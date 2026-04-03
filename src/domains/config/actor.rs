//! Config actor - State owner for configuration management.
//!
//! This module contains the `ConfigActor` which owns the configuration state
//! and handles all config-related operations. It receives messages via a channel
//! and emits state events via the EventBus.

use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use uuid::Uuid;

use crate::domains::config::errors::ConfigError;
use crate::domains::config::messages::{ConfigMessage, ConfigRequest, ConfigResponse, HostCapabilities};
use crate::domains::config::types::{CONFIG_DIR_NAME, CONFIG_FILE_NAME, Config};
use crate::event_bus::ServerFeature;
use crate::event_bus::{EventBus, StateEvent};

/// Config actor that owns configuration state.
pub struct ConfigActor {
    /// The loaded configuration.
    config: Config,
    /// The path to the config file.
    config_path: PathBuf,
    /// Host capabilities detected at startup.
    host_capabilities: HostCapabilities,
    /// EventBus for publishing state events.
    event_bus: EventBus,
}

impl ConfigActor {
    /// Create a new ConfigActor.
    pub async fn new(config_path: Option<PathBuf>, event_bus: EventBus) -> Result<Self, ConfigError> {
        let config_path = match config_path {
            Some(p) => p,
            None => Self::default_config_path()?,
        };

        // Load config or create default
        let config = if config_path.exists() {
            let contents = fs::read_to_string(&config_path)?;
            toml::from_str(&contents)?
        } else {
            Config::default()
        };

        // Detect host capabilities
        let host_capabilities = HostCapabilities::detect();

        let actor = Self {
            config,
            config_path,
            host_capabilities,
            event_bus,
        };

        // Publish ConfigLoaded event
        let _ = actor.event_bus.publish(StateEvent::ConfigLoaded {
            path: actor.config_path.clone(),
        });

        Ok(actor)
    }

    /// Spawn the actor with an existing receiver.
    ///
    /// The channel pair must be created by the caller. The receiver is passed here
    /// for the actor to listen on, and the caller keeps the sender for the handle.
    pub async fn spawn_with_receiver(
        config_path: Option<PathBuf>,
        receiver: mpsc::Receiver<ConfigRequest>,
        event_bus: EventBus,
    ) -> Result<(), ConfigError> {
        let actor = Self::new(config_path, event_bus).await?;

        tokio::spawn(async move {
            actor.run(receiver).await;
        });

        Ok(())
    }

    /// Get the default config file path.
    fn default_config_path() -> Result<PathBuf, ConfigError> {
        // Check for EDGE_CONFIG env var first
        if let Ok(env_path) = std::env::var("EDGE_CONFIG") {
            return Ok(PathBuf::from(env_path));
        }

        // Fall back to XDG config directory
        dirs::config_dir()
            .map(|d| d.join(CONFIG_DIR_NAME).join(CONFIG_FILE_NAME))
            .ok_or(ConfigError::NoConfigDir)
    }

    /// Get the config directory path.
    fn config_dir(&self) -> Result<PathBuf, ConfigError> {
        self.config_path
            .parent()
            .map(|p| p.to_path_buf())
            .ok_or(ConfigError::NoConfigDir)
    }

    /// Run the actor loop.
    pub async fn run(mut self, mut receiver: mpsc::Receiver<ConfigRequest>) {
        while let Some(req) = receiver.recv().await {
            let reply = match req.payload {
                ConfigMessage::LoadConfig => self.handle_load_config(),
                ConfigMessage::ReloadConfig => self.handle_reload_config().await,
                ConfigMessage::SaveConfig => self.handle_save_config().await,
                ConfigMessage::GetConfigValue { key } => self.handle_get_config_value(key),
                ConfigMessage::SetConfigValue { key, value } => self.handle_set_config_value(key, value).await,
                ConfigMessage::GetHostCapabilities => self.handle_get_host_capabilities(),
                ConfigMessage::GetConfigPath => self.handle_get_config_path(),
                ConfigMessage::UpdateManifestTimestamp => self.handle_update_manifest_timestamp().await,
                ConfigMessage::GetManifestTimestamp => self.handle_get_manifest_timestamp(),
            };

            let _ = req.reply_to.send(reply);
        }
    }

    /// Handle LoadConfig message.
    fn handle_load_config(&self) -> Result<ConfigResponse, ConfigError> {
        Ok(ConfigResponse::ConfigLoaded {
            path: self.config_path.clone(),
        })
    }

    /// Handle ReloadConfig message.
    async fn handle_reload_config(&mut self) -> Result<ConfigResponse, ConfigError> {
        if self.config_path.exists() {
            let contents = fs::read_to_string(&self.config_path)?;
            self.config = toml::from_str(&contents)?;

            let _ = self.event_bus.publish(StateEvent::ConfigLoaded {
                path: self.config_path.clone(),
            });
        }

        Ok(ConfigResponse::ConfigLoaded {
            path: self.config_path.clone(),
        })
    }

    /// Handle SaveConfig message.
    async fn handle_save_config(&mut self) -> Result<ConfigResponse, ConfigError> {
        let config_dir = self.config_dir()?;

        if !config_dir.exists() {
            fs::create_dir_all(&config_dir)?;
        }

        let contents = toml::to_string_pretty(&self.config)?;
        let mut file = fs::File::create(&self.config_path)?;
        file.write_all(contents.as_bytes())?;

        Ok(ConfigResponse::ConfigSaved {
            path: self.config_path.clone(),
        })
    }

    /// Handle GetConfigValue message.
    fn handle_get_config_value(&self, key: String) -> Result<ConfigResponse, ConfigError> {
        let value = match key.as_str() {
            "api_key" => serde_json::to_value(&self.config.api_key).unwrap_or(serde_json::Value::Null),
            "session.use_keyring" => {
                serde_json::to_value(self.config.session.use_keyring).unwrap_or(serde_json::Value::Null)
            }
            "enclave.verify_attestation" => {
                serde_json::to_value(self.config.enclave.verify_attestation).unwrap_or(serde_json::Value::Null)
            }
            "enclave.transport_key_ttl_minutes" => {
                serde_json::to_value(self.config.enclave.transport_key_ttl_minutes).unwrap_or(serde_json::Value::Null)
            }
            "manifest_last_fetched" => {
                serde_json::to_value(&self.config.manifest_last_fetched).unwrap_or(serde_json::Value::Null)
            }
            "agent_id" => serde_json::to_value(self.config.agent_id).unwrap_or(serde_json::Value::Null),
            "mcp_server_enabled" => {
                serde_json::to_value(self.config.mcp_server_enabled).unwrap_or(serde_json::Value::Null)
            }
            _ => serde_json::Value::Null,
        };

        Ok(ConfigResponse::ConfigValue { value })
    }

    /// Handle SetConfigValue message.
    async fn handle_set_config_value(
        &mut self,
        key: String,
        value: serde_json::Value,
    ) -> Result<ConfigResponse, ConfigError> {
        match key.as_str() {
            "api_key" => {
                if let Ok(v) = serde_json::from_value::<Option<String>>(value.clone()) {
                    self.config.api_key = v;
                }
            }
            "session.use_keyring" => {
                if let Ok(v) = serde_json::from_value::<Option<bool>>(value.clone()) {
                    self.config.session.use_keyring = v;
                }
            }
            "enclave.verify_attestation" => {
                if let Ok(v) = serde_json::from_value::<bool>(value.clone()) {
                    self.config.enclave.verify_attestation = v;
                }
            }
            "enclave.transport_key_ttl_minutes" => {
                if let Ok(v) = serde_json::from_value::<u64>(value.clone()) {
                    self.config.enclave.transport_key_ttl_minutes = v;
                }
            }
            "mcp_server_enabled" => {
                if let Ok(v) = serde_json::from_value::<bool>(value.clone()) {
                    self.config.mcp_server_enabled = v;
                }
            }
            "agent_id" => {
                if let Ok(v) = serde_json::from_value::<Option<Uuid>>(value.clone()) {
                    self.config.agent_id = v;
                }
            }
            _ => {
                return Err(ConfigError::InvalidValue {
                    key: key.clone(),
                    expected: "valid config key".to_string(),
                });
            }
        }

        let _ = self.event_bus.publish(StateEvent::ConfigChanged {
            key: key.clone(),
            value: value.clone(),
        });

        self.handle_save_config().await?;

        Ok(ConfigResponse::ValueSet { key })
    }

    /// Handle GetHostCapabilities message.
    fn handle_get_host_capabilities(&self) -> Result<ConfigResponse, ConfigError> {
        Ok(ConfigResponse::HostCapabilities {
            capabilities: self.host_capabilities.clone(),
        })
    }

    /// Handle GetConfigPath message.
    fn handle_get_config_path(&self) -> Result<ConfigResponse, ConfigError> {
        Ok(ConfigResponse::ConfigPath {
            path: self.config_path.clone(),
        })
    }

    /// Handle UpdateManifestTimestamp message.
    async fn handle_update_manifest_timestamp(&mut self) -> Result<ConfigResponse, ConfigError> {
        self.config.manifest_last_fetched = Some(chrono::Utc::now().to_rfc3339());
        self.handle_save_config().await?;

        Ok(ConfigResponse::ManifestTimestamp {
            timestamp: self.config.manifest_last_fetched.clone(),
        })
    }

    /// Handle GetManifestTimestamp message.
    fn handle_get_manifest_timestamp(&self) -> Result<ConfigResponse, ConfigError> {
        Ok(ConfigResponse::ManifestTimestamp {
            timestamp: self.config.manifest_last_fetched.clone(),
        })
    }

    /// Get the internal Config reference.
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// Get the host capabilities.
    pub fn host_capabilities(&self) -> &HostCapabilities {
        &self.host_capabilities
    }
}

/// Server feature configuration with feature flags.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureServerConfig {
    /// Feature flags map - each feature can be enabled or disabled.
    #[serde(default = "default_feature_flags")]
    pub feature_flags: HashMap<ServerFeature, bool>,
}

impl Default for FeatureServerConfig {
    fn default() -> Self {
        Self {
            feature_flags: default_feature_flags(),
        }
    }
}

impl FeatureServerConfig {
    /// Creates a new FeatureServerConfig with all features enabled.
    pub fn new() -> Self {
        Self::default()
    }

    /// Checks if a specific feature is enabled.
    pub fn is_feature_enabled(&self, feature: ServerFeature) -> bool {
        self.feature_flags.get(&feature).copied().unwrap_or(false)
    }

    /// Toggles a feature's enabled state.
    pub fn toggle_feature(&mut self, feature: ServerFeature) -> bool {
        let current = self.feature_flags.get(&feature).copied().unwrap_or(false);
        let new_state = !current;
        self.feature_flags.insert(feature, new_state);
        new_state
    }

    /// Sets a feature to a specific enabled state.
    pub fn set_feature(&mut self, feature: ServerFeature, enabled: bool) {
        self.feature_flags.insert(feature, enabled);
    }

    /// Returns a list of all enabled features.
    pub fn enabled_features(&self) -> Vec<ServerFeature> {
        self.feature_flags
            .iter()
            .filter(|(_, enabled)| **enabled)
            .map(|(feature, _)| *feature)
            .collect()
    }

    /// Returns a list of all disabled features.
    pub fn disabled_features(&self) -> Vec<ServerFeature> {
        self.feature_flags
            .iter()
            .filter(|(_, enabled)| !**enabled)
            .map(|(feature, _)| *feature)
            .collect()
    }
}

/// Creates default feature flags with all features enabled.
fn default_feature_flags() -> HashMap<ServerFeature, bool> {
    let mut flags = HashMap::new();
    flags.insert(ServerFeature::McpServer, true);
    flags.insert(ServerFeature::Subscriptions, true);
    flags.insert(ServerFeature::Alerts, true);
    flags
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn test_config_actor_spawn() {
        let event_bus = EventBus::new(128);
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_path_buf();

        let (_tx, rx) = mpsc::channel(64);
        let handle = ConfigActor::spawn_with_receiver(Some(path), rx, event_bus).await;
        assert!(handle.is_ok());
    }

    #[test]
    fn test_default_config_path() {
        unsafe {
            std::env::set_var("EDGE_CONFIG", "/tmp/test-config.toml");
        }

        let path = ConfigActor::default_config_path();
        assert!(path.is_ok());
        assert_eq!(path.unwrap(), PathBuf::from("/tmp/test-config.toml"));

        unsafe {
            std::env::remove_var("EDGE_CONFIG");
        }
    }

    #[test]
    fn test_feature_server_config_default() {
        let config = FeatureServerConfig::default();
        assert!(config.is_feature_enabled(ServerFeature::McpServer));
        assert!(config.is_feature_enabled(ServerFeature::Subscriptions));
        assert!(config.is_feature_enabled(ServerFeature::Alerts));
    }

    #[test]
    fn test_feature_server_config_new() {
        let new_config = FeatureServerConfig::new();
        let default_config = FeatureServerConfig::default();

        assert_eq!(
            new_config.is_feature_enabled(ServerFeature::McpServer),
            default_config.is_feature_enabled(ServerFeature::McpServer)
        );
    }

    #[test]
    fn test_feature_toggle() {
        let mut config = FeatureServerConfig::default();

        let result = config.toggle_feature(ServerFeature::McpServer);
        assert!(!result);
        assert!(!config.is_feature_enabled(ServerFeature::McpServer));

        let result = config.toggle_feature(ServerFeature::McpServer);
        assert!(result);
        assert!(config.is_feature_enabled(ServerFeature::McpServer));
    }

    #[test]
    fn test_feature_set() {
        let mut config = FeatureServerConfig::default();

        config.set_feature(ServerFeature::Alerts, false);
        assert!(!config.is_feature_enabled(ServerFeature::Alerts));

        config.set_feature(ServerFeature::Alerts, true);
        assert!(config.is_feature_enabled(ServerFeature::Alerts));
    }

    #[test]
    fn test_enabled_disabled_features() {
        let mut config = FeatureServerConfig::default();
        config.set_feature(ServerFeature::Alerts, false);

        let enabled = config.enabled_features();
        assert!(enabled.contains(&ServerFeature::McpServer));
        assert!(!enabled.contains(&ServerFeature::Alerts));

        let disabled = config.disabled_features();
        assert!(disabled.contains(&ServerFeature::Alerts));
    }
}
