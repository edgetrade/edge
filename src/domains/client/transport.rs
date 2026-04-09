//! Transport key cache for enclave transport keys.
//!
//! Provides filesystem-backed caching of enclave transport keys with TTL-based
//! expiration. Transport keys are cached in the XDG config directory to avoid
//! repeated attestation round-trips.

use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use chrono::{DateTime, Utc};
use ed25519_dalek::VerifyingKey;
use serde::{Deserialize, Serialize};

use erato::services::tyche::attestation::TransportKeyReceiver;

use crate::domains::client::generated::routes::requests::agent_get_transport_key;
use crate::domains::client::trpc::RouteExecutor;
use crate::domains::config::{Config, default_config_path_buf};
use crate::domains::enclave::wallet::types::{WalletError, WalletResult};

/// Default cache TTL in minutes
pub const DEFAULT_TRANSPORT_KEY_TTL_MINUTES: u64 = 15;
/// Transport key cache filename
pub const TRANSPORT_KEY_FILENAME: &str = "transport_keys.json";

/// Error type for transport cache operations.
#[derive(Debug, thiserror::Error)]
pub enum TransportCacheError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON parse error: {0}")]
    Parse(#[from] serde_json::Error),
    #[error("Config directory not found")]
    NoConfigDir,
    #[error("Invalid key encoding")]
    InvalidEncoding,
}

/// Cached transport keys with timestamp for TTL tracking.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedTransportKeys {
    /// Ephemeral public key (base64 encoded)
    pub ephemeral: String,
    /// Deterministic public key (base64 encoded)
    pub deterministic: String,
    /// Attestation document (base64 encoded)
    pub attestation: String,
    /// ISO 8601 timestamp of when keys were cached
    pub cached_at: String,
}

impl CachedTransportKeys {
    /// Create new cached transport keys with current timestamp.
    pub fn new(ephemeral: String, deterministic: String, attestation: String) -> Self {
        Self {
            ephemeral,
            deterministic,
            attestation,
            cached_at: Utc::now().to_rfc3339(),
        }
    }

    /// Get the cache timestamp as DateTime.
    pub fn timestamp(&self) -> Option<DateTime<Utc>> {
        DateTime::parse_from_rfc3339(&self.cached_at)
            .ok()
            .map(|dt| dt.with_timezone(&Utc))
    }
}

/// Get transport keys from cache or fetch from API.
///
/// Checks the cache first based on TTL from config. If cache is fresh,
/// returns cached keys. Otherwise fetches from API, optionally verifies
/// attestation (based on config), and caches the result.
///
/// # Arguments
/// * `client` - The Iris API client
///
/// # Returns
/// Transport keys for encrypting wallet data
///
/// # Errors
/// Returns `WalletError` if fetching from API fails, attestation verification
/// fails (when enabled), or cache operations fail.
pub async fn get_transport_key(client: &impl RouteExecutor) -> WalletResult<TransportKeyReceiver> {
    // Load config to get TTL and verification settings from default location
    let mut config = Config::load(None).map_err(|e| WalletError::StorageFailed(e.to_string()))?;
    let ttl_minutes = config.enclave.transport_key_ttl_minutes;
    let verify_attestation = config.enclave.verify_attestation;

    // Get config directory for cache storage
    let config_dir = default_config_path_buf()
        .ok_or_else(|| WalletError::StorageFailed("No config dir".to_string()))?
        .parent()
        .ok_or_else(|| WalletError::StorageFailed("No config dir".to_string()))?
        .to_path_buf();

    // Check cache first
    if let Some(cached) = load_cached_transport_keys(&config_dir)
        && let Some(timestamp) = cached.timestamp()
        && is_cache_fresh(timestamp, ttl_minutes)
    {
        // Cache is fresh, decode and return
        let ephemeral_bytes = STANDARD
            .decode(&cached.ephemeral)
            .map_err(|_| WalletError::Crypto("Invalid cached ephemeral key".to_string()))?;
        let deterministic_bytes = STANDARD
            .decode(&cached.deterministic)
            .map_err(|_| WalletError::Crypto("Invalid cached deterministic key".to_string()))?;
        let attestation = STANDARD
            .decode(&cached.attestation)
            .map_err(|_| WalletError::Crypto("Invalid cached attestation".to_string()))?;

        // Decode base64 to raw bytes and convert to fixed-size arrays
        let ephemeral = VerifyingKey::from_bytes(ephemeral_bytes.as_slice().try_into().unwrap())
            .map_err(|_| WalletError::Crypto("Invalid ephemeral key length".to_string()))?;
        let deterministic = VerifyingKey::from_bytes(deterministic_bytes.as_slice().try_into().unwrap())
            .map_err(|_| WalletError::Crypto("Invalid deterministic key length".to_string()))?;

        return Ok(TransportKeyReceiver::from_message(
            ephemeral,
            deterministic,
            attestation,
        ));
    }

    // Cache miss or stale - fetch from API
    let response: agent_get_transport_key::GetTransportKeyResponse = client
        .execute(&agent_get_transport_key::ROUTE, &())
        .await
        .map_err(|e| WalletError::StorageFailed(e.to_string()))?;

    // Extract and save agent_id from response if not already configured
    if config.agent_id.is_none() {
        config.agent_id = Some(response.agent_id);
        if let Err(e) = config.save() {
            // Log config save failure but don't fail the operation
            eprintln!("Warning: Failed to save agent_id to config: {e}");
        }
    }

    let ephemeral_bytes = STANDARD
        .decode(&response.ephemeral)
        .map_err(|e| WalletError::StorageFailed(e.to_string()))?;

    let deterministic_bytes = STANDARD
        .decode(&response.deterministic)
        .map_err(|e| WalletError::StorageFailed(e.to_string()))?;

    let attestation = STANDARD
        .decode(&response.attestation)
        .map_err(|e| WalletError::StorageFailed(e.to_string()))?;

    // Verify attestation if configured
    if verify_attestation {
        verify_attestation_document(&attestation).map_err(|e| WalletError::Crypto(e.to_string()))?;
    }

    // Decode base64 to raw bytes and convert to fixed-size arrays
    let ephemeral = VerifyingKey::from_bytes(ephemeral_bytes.as_slice().try_into().unwrap())
        .map_err(|_| WalletError::Crypto("Invalid ephemeral key length".to_string()))?;
    let deterministic = VerifyingKey::from_bytes(deterministic_bytes.as_slice().try_into().unwrap())
        .map_err(|_| WalletError::Crypto("Invalid deterministic key length".to_string()))?;

    let transport_key = TransportKeyReceiver::from_message(ephemeral, deterministic, attestation.clone());

    // Save to cache
    let cached_keys = CachedTransportKeys::new(response.ephemeral, response.deterministic, response.attestation);

    if let Err(e) = save_transport_keys(&config_dir, &cached_keys) {
        // Log cache save failure but don't fail the operation
        eprintln!("Warning: Failed to cache transport keys: {e}");
    }

    Ok(transport_key)
}

/// Verify the attestation document from the enclave.
///
/// This validates the cryptographic attestation to ensure the enclave
/// is authentic and running the expected code.
///
/// # Arguments
/// * `attestation` - Raw attestation document bytes
///
/// # Returns
/// `Ok(())` if verification succeeds
///
/// # Errors
/// Returns error string if verification fails
pub fn verify_attestation_document(attestation: &[u8]) -> Result<(), String> {
    // TODO: Implement actual attestation verification
    // For now, this is a placeholder that always succeeds
    // In production, this should verify:
    // 1. Attestation signature using AWS KMS or NSM certificate
    // 2. PCR values match expected build measurements
    // 3. Timestamp is recent (anti-replay)
    // 4. Enclave identity matches expected value

    if attestation.is_empty() {
        return Err("Attestation document is empty".to_string());
    }

    // Placeholder - in production this would use the actual verification
    // tyche_enclave::attestation::verify(attestation)

    Ok(())
}

/// Returns the full path to the transport key cache file.
pub fn transport_key_path(config_dir: &Path) -> PathBuf {
    config_dir.join(TRANSPORT_KEY_FILENAME)
}

/// Load cached transport keys from disk if they exist.
///
/// Returns `Some(CachedTransportKeys)` if found and parsed successfully,
/// `None` if the file doesn't exist or cannot be parsed.
pub fn load_cached_transport_keys(config_dir: &Path) -> Option<CachedTransportKeys> {
    let path = transport_key_path(config_dir);

    if !path.exists() {
        return None;
    }

    let mut file = fs::File::open(&path).ok()?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).ok()?;

    let cached: CachedTransportKeys = serde_json::from_str(&contents).ok()?;
    Some(cached)
}

/// Save transport keys to disk with current timestamp.
///
/// Creates the config directory if it doesn't exist and sets proper
/// permissions (0o700 for directory, 0o600 for file on Unix).
pub fn save_transport_keys(config_dir: &Path, keys: &CachedTransportKeys) -> Result<(), TransportCacheError> {
    if !config_dir.exists() {
        fs::create_dir_all(config_dir)?;

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(config_dir, fs::Permissions::from_mode(0o700))?;
        }
    }

    let path = transport_key_path(config_dir);
    let json = serde_json::to_vec_pretty(keys)?;

    let mut file = fs::File::create(&path)?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&path, fs::Permissions::from_mode(0o600))?;
    }

    file.write_all(&json)?;
    file.sync_all()?;

    Ok(())
}

/// Delete cached transport keys from disk.
///
/// This is idempotent - succeeds even if the cache file doesn't exist.
pub fn delete_cached_transport_keys(config_dir: &Path) -> Result<(), TransportCacheError> {
    let path = transport_key_path(config_dir);
    if path.exists() {
        fs::remove_file(&path)?;
    }
    Ok(())
}

/// Check if the given timestamp is within the TTL window.
///
/// Returns `true` if the elapsed time since `cached_at` is less
/// than `ttl_minutes`, `false` otherwise.
pub fn is_cache_fresh(cached_at: DateTime<Utc>, ttl_minutes: u64) -> bool {
    let now = Utc::now();
    let elapsed = now.signed_duration_since(cached_at);
    elapsed.num_minutes() < ttl_minutes as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transport_key_path_construction() {
        let config_dir = Path::new("/tmp/test-config");
        let path = transport_key_path(config_dir);
        assert_eq!(path, PathBuf::from("/tmp/test-config/transport_keys.json"));
    }

    #[test]
    fn test_is_cache_fresh_within_ttl() {
        let recent = Utc::now() - chrono::Duration::minutes(5);
        assert!(is_cache_fresh(recent, 15));
    }

    #[test]
    fn test_is_cache_fresh_expired() {
        let old = Utc::now() - chrono::Duration::minutes(20);
        assert!(!is_cache_fresh(old, 15));
    }

    #[test]
    fn test_is_cache_fresh_at_exact_boundary() {
        let exactly_fifteen_minutes_ago = Utc::now() - chrono::Duration::minutes(15);
        // At exactly the boundary, it should NOT be fresh (<, not <=)
        assert!(!is_cache_fresh(exactly_fifteen_minutes_ago, 15));
    }

    #[test]
    fn test_cached_transport_keys_new() {
        let keys = CachedTransportKeys::new(
            "ephemeral123".to_string(),
            "deterministic456".to_string(),
            "attestation789".to_string(),
        );

        assert_eq!(keys.ephemeral, "ephemeral123");
        assert_eq!(keys.deterministic, "deterministic456");
        assert_eq!(keys.attestation, "attestation789");
        assert!(!keys.cached_at.is_empty());
    }

    #[test]
    fn test_cached_transport_keys_timestamp() {
        let _now = Utc::now();
        let keys = CachedTransportKeys::new(
            "ephemeral".to_string(),
            "deterministic".to_string(),
            "attestation".to_string(),
        );

        let timestamp = keys.timestamp().expect("Should parse timestamp");
        let elapsed = Utc::now().signed_duration_since(timestamp);
        assert!(elapsed.num_seconds() < 5);
    }

    #[test]
    fn test_load_nonexistent_cache() {
        let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
        let result = load_cached_transport_keys(temp_dir.path());
        assert!(result.is_none());
    }

    #[test]
    fn test_save_and_load_cache() {
        let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");

        let keys = CachedTransportKeys::new(
            "base64ephemeral".to_string(),
            "base64deterministic".to_string(),
            "base64attestation".to_string(),
        );

        save_transport_keys(temp_dir.path(), &keys).expect("Failed to save transport keys");

        let loaded = load_cached_transport_keys(temp_dir.path());
        assert!(loaded.is_some());

        let loaded_keys = loaded.unwrap();
        assert_eq!(loaded_keys.ephemeral, "base64ephemeral");
        assert_eq!(loaded_keys.deterministic, "base64deterministic");
        assert_eq!(loaded_keys.attestation, "base64attestation");
    }

    #[test]
    fn test_delete_cached_transport_keys() {
        let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");

        let keys = CachedTransportKeys::new("test".to_string(), "test".to_string(), "test".to_string());

        save_transport_keys(temp_dir.path(), &keys).unwrap();
        assert!(transport_key_path(temp_dir.path()).exists());

        delete_cached_transport_keys(temp_dir.path()).unwrap();
        assert!(!transport_key_path(temp_dir.path()).exists());
    }

    #[test]
    fn test_delete_nonexistent_cache() {
        let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");

        // Should not error when file doesn't exist
        let result = delete_cached_transport_keys(temp_dir.path());
        assert!(result.is_ok());
    }

    #[test]
    fn test_cache_serialization_roundtrip() {
        let keys = CachedTransportKeys::new("abc123".to_string(), "def456".to_string(), "ghi789".to_string());

        let json = serde_json::to_string(&keys).expect("Should serialize");
        let deserialized: CachedTransportKeys = serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(deserialized.ephemeral, keys.ephemeral);
        assert_eq!(deserialized.deterministic, keys.deterministic);
        assert_eq!(deserialized.attestation, keys.attestation);
        assert_eq!(deserialized.cached_at, keys.cached_at);
    }
}
