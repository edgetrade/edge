use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use ed25519_dalek::VerifyingKey;
use serde_json::json;

use tyche_enclave::shared::attestation::TransportKeyReceiver;

use crate::session::transport::{
    CachedTransportKeys, delete_cached_transport_keys, is_cache_fresh, load_cached_transport_keys, save_transport_keys,
};
use crate::{client::IrisClient, config::Config, wallet::types::GetTransportKeyResponse};

use crate::wallet::types::{WalletError, WalletResult};

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
pub async fn get_transport_key(client: &IrisClient) -> WalletResult<TransportKeyReceiver> {
    // Load config to get TTL and verification settings
    let config = Config::load().map_err(|e| WalletError::StorageFailed(e.to_string()))?;
    let ttl_minutes = config.enclave.transport_key_ttl_minutes;
    let verify_attestation = config.enclave.verify_attestation;

    // Get config directory for cache storage
    let config_dir = Config::config_path()
        .map_err(|e| WalletError::StorageFailed(e.to_string()))?
        .parent()
        .ok_or_else(|| WalletError::StorageFailed("No config dir".to_string()))?
        .to_path_buf();

    // Check cache first
    if let Some(cached) = load_cached_transport_keys(&config_dir)
        && let Some(timestamp) = cached.timestamp()
        && is_cache_fresh(timestamp, ttl_minutes)
    {
        println!("Cache is fresh, decoding and returning");

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
    let response: GetTransportKeyResponse = client
        .query("agent.getTransportKey", json!({}))
        .await
        .map_err(|e| WalletError::StorageFailed(e.to_string()))?;

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
        eprintln!("Warning: Failed to cache transport keys: {}", e);
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

/// Clear the transport key cache.
///
/// Useful when wanting to force a fresh attestation or when
/// switching between different enclave instances.
///
/// # Returns
/// `Ok(())` on success
///
/// # Errors
/// Returns `WalletError` if cache deletion fails
#[allow(unused)]
pub fn clear_transport_key_cache() -> WalletResult<()> {
    let config_dir = Config::config_path()
        .map_err(|e| WalletError::StorageFailed(e.to_string()))?
        .parent()
        .ok_or_else(|| WalletError::StorageFailed("No config dir".to_string()))?
        .to_path_buf();

    delete_cached_transport_keys(&config_dir).map_err(|e| WalletError::StorageFailed(e.to_string()))?;

    Ok(())
}
