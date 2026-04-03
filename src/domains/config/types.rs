//! Config types - Configuration data structures
//!
//! This module defines the Config struct and supporting types for
//! the Edge CLI configuration system.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Default config directory name
pub const CONFIG_DIR_NAME: &str = "edge";
/// Default config file name
pub const CONFIG_FILE_NAME: &str = "config.toml";

/// Returns the default config file path, checking EDGE_CONFIG env var first.
///
/// This function is used by both the CLI (for default_value) and Config::config_path()
/// to ensure consistent path resolution.
pub fn default_config_path_buf() -> Option<PathBuf> {
    // Check for EDGE_CONFIG env var first
    if let Ok(env_path) = std::env::var("EDGE_CONFIG") {
        return Some(PathBuf::from(env_path));
    }

    // Fall back to XDG config directory
    dirs::config_dir().map(|d| d.join(CONFIG_DIR_NAME).join(CONFIG_FILE_NAME))
}

/// Edge CLI configuration.
///
/// This struct represents the user-configurable settings for the Edge CLI,
/// stored in `~/.config/edge/config.toml` (XDG config directory).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    /// Edge API key for authentication
    #[serde(default)]
    pub api_key: Option<String>,
    /// Session storage configuration
    #[serde(default)]
    pub session: SessionConfig,
    /// ISO 8601 timestamp of last manifest fetch
    #[serde(default)]
    pub manifest_last_fetched: Option<String>,
    /// Enclave security configuration
    #[serde(default)]
    pub enclave: EnclaveConfig,
    /// Agent identifier for tracking
    #[serde(default)]
    pub agent_id: Option<Uuid>,
    /// MCP server enabled state (persisted across restarts)
    #[serde(default)]
    pub mcp_server_enabled: bool,
}

/// Enclave security and transport key configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnclaveConfig {
    /// Whether to verify attestation documents from the enclave.
    ///
    /// When `true`, attestation documents are cryptographically verified.
    /// When `false`, verification is skipped (useful for testing/local dev).
    /// Default: `true`.
    #[serde(default = "default_verify_attestation")]
    pub verify_attestation: bool,
    /// TTL for cached transport keys in minutes.
    ///
    /// Transport keys are cached locally to avoid repeated attestation
    /// round-trips. This specifies how long cached keys remain valid.
    /// Default: 15 minutes.
    #[serde(default = "default_transport_key_ttl")]
    pub transport_key_ttl_minutes: u64,
}

impl Default for EnclaveConfig {
    fn default() -> Self {
        Self {
            verify_attestation: default_verify_attestation(),
            transport_key_ttl_minutes: default_transport_key_ttl(),
        }
    }
}

fn default_verify_attestation() -> bool {
    true
}

fn default_transport_key_ttl() -> u64 {
    15
}

/// Session storage configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SessionConfig {
    /// Whether to use the OS keyring for session storage.
    ///
    /// If `true`, the keyring will be used. If `false` or not set,
    /// file-based storage will be used as a fallback.
    ///
    /// This is automatically detected on first run and cached here.
    /// Users can manually edit this to force a specific storage backend.
    #[serde(default)]
    pub use_keyring: Option<bool>,
}

impl Config {
    /// Load configuration from the specified path or default location.
    ///
    /// If `path` is `Some`, loads from that path. Otherwise, uses the default
    /// config location (`~/.config/edge/config.toml` or `$EDGE_CONFIG` env var).
    /// If the file doesn't exist, returns a default configuration.
    ///
    /// # Arguments
    /// - `path` - Optional path to the config file. If `None`, uses default location.
    ///
    /// # Returns
    /// - `Ok(Config)` - The loaded or default configuration
    /// - `Err(ConfigError)` - If there was an error reading the config file
    pub fn load(path: Option<PathBuf>) -> Result<Self, crate::domains::config::errors::ConfigError> {
        match path {
            Some(p) => Self::load_from(p),
            None => Self::load_default(),
        }
    }

    /// Load configuration from a specific file path.
    ///
    /// If the file doesn't exist, returns a default configuration.
    ///
    /// # Arguments
    /// - `path` - Path to the config file
    ///
    /// # Returns
    /// - `Ok(Config)` - The loaded or default configuration
    /// - `Err(ConfigError)` - If there was an error reading the config file
    pub fn load_from(path: PathBuf) -> Result<Self, crate::domains::config::errors::ConfigError> {
        if !path.exists() {
            return Ok(Self::default());
        }

        let contents = std::fs::read_to_string(&path)?;
        let config: Config = toml::from_str(&contents)?;
        Ok(config)
    }

    /// Load configuration from the default location.
    ///
    /// Uses the XDG config directory (`~/.config/edge/config.toml`)
    /// or `$EDGE_CONFIG` env var if set.
    /// If the file doesn't exist, returns a default configuration.
    ///
    /// # Returns
    /// - `Ok(Config)` - The loaded or default configuration
    /// - `Err(ConfigError)` - If there was an error reading the config file
    pub fn load_default() -> Result<Self, crate::domains::config::errors::ConfigError> {
        let config_path = default_config_path_buf().ok_or(crate::domains::config::errors::ConfigError::NoConfigDir)?;
        Self::load_from(config_path)
    }

    /// Save configuration to the XDG config directory.
    ///
    /// Writes the configuration to `~/.config/edge/config.toml`,
    /// creating the directory if it doesn't exist.
    ///
    /// # Returns
    /// - `Ok(())` - On successful save
    /// - `Err(ConfigError)` - If there was an error writing the config file
    pub fn save(&self) -> Result<(), crate::domains::config::errors::ConfigError> {
        let config_path = default_config_path_buf().ok_or(crate::domains::config::errors::ConfigError::NoConfigDir)?;
        let config_dir = config_path
            .parent()
            .ok_or(crate::domains::config::errors::ConfigError::NoConfigDir)?;

        std::fs::create_dir_all(config_dir)?;

        let contents = toml::to_string_pretty(self)?;
        let mut file = std::fs::File::create(&config_path)?;
        file.write_all(contents.as_bytes())?;

        Ok(())
    }

    /// Get the default config file path.
    ///
    /// # Returns
    /// - `Some(PathBuf)` - The default config file path
    /// - `None` - If the config directory cannot be determined
    pub fn config_path() -> Option<PathBuf> {
        default_config_path_buf()
    }

    /// Update the manifest timestamp to now.
    ///
    /// Sets the `manifest_last_fetched` field to the current UTC time
    /// and saves the configuration.
    ///
    /// # Returns
    /// - `Ok(())` - On successful update
    /// - `Err(ConfigError)` - If there was an error saving the config
    pub fn update_manifest_timestamp(&mut self) -> Result<(), crate::domains::config::errors::ConfigError> {
        self.manifest_last_fetched = Some(chrono::Utc::now().to_rfc3339());
        self.save()
    }
}

use std::io::Write;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert!(config.api_key.is_none());
        assert!(config.manifest_last_fetched.is_none());
        assert!(config.agent_id.is_none());
        assert!(!config.mcp_server_enabled);
    }

    #[test]
    fn test_session_config_default() {
        let session = SessionConfig::default();
        assert!(session.use_keyring.is_none());
    }

    #[test]
    fn test_enclave_config_default() {
        let enclave = EnclaveConfig::default();
        assert!(enclave.verify_attestation);
        assert_eq!(enclave.transport_key_ttl_minutes, 15);
    }
}
