//! Config handle - Public API for the config domain.
//!
//! This module provides `ConfigHandle`, a thin gateway that sends messages
//! to the config actor. This is the public interface for other domains
//! and the CLI to interact with configuration.

use std::path::PathBuf;

use tokio::sync::{mpsc, oneshot};

use crate::domains::config::errors::ConfigError;
use crate::domains::config::messages::{ConfigMessage, ConfigRequest, ConfigResponse, HostCapabilities};
use crate::event_bus::PoseidonRequest;

/// Handle for interacting with the config domain.
///
/// This is a thin gateway that sends messages to the config actor.
/// All operations are asynchronous and return results via oneshot channels.
///
/// # Example
/// ```rust,no_run
/// use poseidon::domains::config::initialize_config_domain;
/// use poseidon::event_bus::EventBus;
///
/// # async fn example() {
/// let event_bus = EventBus::new(128);
/// let handle = initialize_config_domain(None, event_bus).await.unwrap();
///
/// // Load config
/// let _ = handle.load().await.unwrap();
///
/// // Get a config value
/// let value = handle.get_value("session.use_keyring").await.unwrap();
///
/// // Get host capabilities
/// let caps = handle.get_host_capabilities().await.unwrap();
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct ConfigHandle {
    /// Sender channel for sending requests to the actor.
    sender: mpsc::Sender<ConfigRequest>,
}

impl ConfigHandle {
    /// Create a new ConfigHandle with config path, and EventBus.
    ///
    /// This creates a channel pair, spawns the ConfigActor with the receiver,
    /// and returns a handle containing the sender.
    ///
    /// # Arguments
    /// * `config_path` - Optional path to config file (uses default if None)
    /// * `event_bus` - EventBus for publishing state events
    pub async fn new(config_path: Option<PathBuf>, event_bus: crate::event_bus::EventBus) -> Result<Self, ConfigError> {
        // Create the channel pair - sender goes to handle, receiver goes to actor
        let (sender, receiver) = mpsc::channel::<ConfigRequest>(64);

        // Spawn the actor with the receiver
        crate::domains::config::actor::ConfigActor::spawn_with_receiver(config_path, receiver, event_bus).await?;

        Ok(Self { sender })
    }

    /// Create a ConfigHandle from an existing sender.
    ///
    /// Used internally when the actor is already spawned.
    ///
    /// # Arguments
    /// * `sender` - The mpsc sender channel connected to the actor.
    pub fn from_sender(sender: mpsc::Sender<ConfigRequest>) -> Self {
        Self { sender }
    }

    /// Get the sender channel for this handle.
    ///
    /// Used by the orchestrator to wire up domain gateways.
    pub fn sender(&self) -> &mpsc::Sender<ConfigRequest> {
        &self.sender
    }

    /// Send a message to the config actor and await the response.
    ///
    /// This is the internal method used by all public methods.
    ///
    /// # Arguments
    /// * `message` - The message to send.
    ///
    /// # Returns
    /// `Result<ConfigResponse, ConfigError>` with the actor's response.
    async fn send_message(&self, message: ConfigMessage) -> Result<ConfigResponse, ConfigError> {
        let (reply_to, rx) = oneshot::channel();

        let request = PoseidonRequest {
            payload: message,
            trace_ctx: crate::event_bus::TraceContext::current(),
            reply_to,
        };

        self.sender
            .send(request)
            .await
            .map_err(|_| ConfigError::ChannelSend)?;

        rx.await.map_err(|_| ConfigError::OneshotReply)?
    }

    /// Load the configuration from disk.
    ///
    /// Returns a `ConfigLoaded` response with the path from which
    /// the config was loaded.
    ///
    /// # Returns
    /// `Ok(ConfigResponse)` on success, or `ConfigError` on failure.
    pub async fn load(&self) -> Result<ConfigResponse, ConfigError> {
        self.send_message(ConfigMessage::LoadConfig).await
    }

    /// Reload the configuration from disk.
    ///
    /// Refreshes the in-memory configuration from the config file.
    /// This is useful when the config file has been modified externally.
    ///
    /// # Returns
    /// `Ok(ConfigResponse)` on success, or `ConfigError` on failure.
    pub async fn reload(&self) -> Result<ConfigResponse, ConfigError> {
        self.send_message(ConfigMessage::ReloadConfig).await
    }

    /// Save the current configuration to disk.
    ///
    /// Persists any changes made to the configuration.
    ///
    /// # Returns
    /// `Ok(ConfigResponse)` on success, or `ConfigError` on failure.
    pub async fn save(&self) -> Result<ConfigResponse, ConfigError> {
        self.send_message(ConfigMessage::SaveConfig).await
    }

    /// Get a configuration value by key.
    ///
    /// # Arguments
    /// * `key` - Dot-separated path to the config value (e.g., "session.use_keyring").
    ///
    /// # Returns
    /// `Ok(ConfigValue)` with the value, or `ConfigError` if the key is invalid.
    pub async fn get_value(&self, key: &str) -> Result<serde_json::Value, ConfigError> {
        let response = self
            .send_message(ConfigMessage::GetConfigValue { key: key.to_string() })
            .await?;

        match response {
            ConfigResponse::ConfigValue { value } => Ok(value),
            _ => Err(ConfigError::InvalidValue {
                key: key.to_string(),
                expected: "valid config key".to_string(),
            }),
        }
    }

    /// Set a configuration value by key.
    ///
    /// Automatically saves the configuration after setting the value.
    ///
    /// # Arguments
    /// * `key` - Dot-separated path to the config value.
    /// * `value` - The value to set (as JSON).
    ///
    /// # Returns
    /// `Ok(())` on success, or `ConfigError` on failure.
    pub async fn set_value(&self, key: &str, value: serde_json::Value) -> Result<(), ConfigError> {
        let response = self
            .send_message(ConfigMessage::SetConfigValue {
                key: key.to_string(),
                value,
            })
            .await?;

        match response {
            ConfigResponse::ValueSet { .. } => Ok(()),
            _ => Err(ConfigError::InvalidValue {
                key: key.to_string(),
                expected: "valid config key".to_string(),
            }),
        }
    }

    /// Get host capabilities.
    ///
    /// Returns information about the host system, including keyring
    /// availability, operating system, and version.
    ///
    /// # Returns
    /// `Ok(HostCapabilities)` on success.
    pub async fn get_host_capabilities(&self) -> Result<HostCapabilities, ConfigError> {
        let response = self
            .send_message(ConfigMessage::GetHostCapabilities)
            .await?;

        match response {
            ConfigResponse::HostCapabilities { capabilities } => Ok(capabilities),
            _ => Err(ConfigError::CapabilityDetectionFailed {
                capability: "host".to_string(),
                reason: "Unexpected response".to_string(),
            }),
        }
    }

    /// Get the configuration file path.
    ///
    /// # Returns
    /// `Ok(PathBuf)` with the path to the config file.
    pub async fn get_config_path(&self) -> Result<PathBuf, ConfigError> {
        let response = self.send_message(ConfigMessage::GetConfigPath).await?;

        match response {
            ConfigResponse::ConfigPath { path } => Ok(path),
            _ => Err(ConfigError::ConfigNotFound {
                path: "unknown".to_string(),
            }),
        }
    }

    /// Update the manifest timestamp to the current time.
    ///
    /// Sets the `manifest_last_fetched` field to the current UTC time
    /// and saves the configuration.
    ///
    /// # Returns
    /// `Ok(String)` with the new timestamp, or `ConfigError` on failure.
    pub async fn update_manifest_timestamp(&self) -> Result<String, ConfigError> {
        let response = self
            .send_message(ConfigMessage::UpdateManifestTimestamp)
            .await?;

        match response {
            ConfigResponse::ManifestTimestamp { timestamp } => Ok(timestamp.unwrap_or_default()),
            _ => Err(ConfigError::InvalidValue {
                key: "manifest_last_fetched".to_string(),
                expected: "timestamp".to_string(),
            }),
        }
    }

    /// Get the stored manifest timestamp.
    ///
    /// # Returns
    /// `Ok(Option<String>)` with the timestamp, or `None` if not set.
    pub async fn get_manifest_timestamp(&self) -> Result<Option<String>, ConfigError> {
        let response = self
            .send_message(ConfigMessage::GetManifestTimestamp)
            .await?;

        match response {
            ConfigResponse::ManifestTimestamp { timestamp } => Ok(timestamp),
            _ => Err(ConfigError::InvalidValue {
                key: "manifest_last_fetched".to_string(),
                expected: "timestamp".to_string(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Mock sender for testing
    fn mock_handle() -> ConfigHandle {
        let (tx, _rx) = mpsc::channel(64);
        ConfigHandle::from_sender(tx)
    }

    #[tokio::test]
    async fn test_handle_creation() {
        let _handle = mock_handle();
        // Just verify it can be created
    }

    #[test]
    fn test_handle_clone() {
        let _handle = mock_handle();
        let _cloned = _handle.clone();
        // Verify clone works
    }
}
