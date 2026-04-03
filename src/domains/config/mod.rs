//! config domain - File configuration and host capabilities
//!
//! This domain manages:
//! - Loading/saving configuration from `~/.config/edge/config.toml`
//! - Detecting host capabilities (keyring availability, OS, version)
//! - Providing configuration values to other domains
//!
//! # Architecture
//!
//! Following the actor/handler pattern:
//! - `ConfigActor`: Owns the configuration state, runs in a background task
//! - `ConfigHandle`: Public API for sending messages to the actor
//! - `ConfigMessage`: Command/query message types
//! - `ConfigError`: Domain-specific error types
//!
//! # Example Usage
//!
//! ```rust,no_run
//! use poseidon::domains::config::initialize_config_domain;
//! use poseidon::event_bus::EventBus;
//!
//! # async fn example() {
//! let event_bus = EventBus::new(128);
//! let handle = initialize_config_domain(None, event_bus).await.unwrap();
//!
//! // Load configuration
//! let _ = handle.load().await.unwrap();
//!
//! // Get host capabilities
//! let caps = handle.get_host_capabilities().await.unwrap();
//! println!("Keyring available: {}", caps.keyring_available);
//! # }
//! ```

pub mod actor;
pub mod errors;
pub mod handle;
pub mod messages;
pub mod state;
pub mod types;

// Public exports
pub use actor::{ConfigActor, FeatureServerConfig};
pub use errors::ConfigError;
pub use handle::ConfigHandle;
pub use messages::{ConfigMessage, ConfigRequest, ConfigResponse};
pub use state::{ConfigState, HostCapabilities, OperatingSystem};
pub use types::{CONFIG_DIR_NAME, CONFIG_FILE_NAME, Config, EnclaveConfig, SessionConfig, default_config_path_buf};

use std::path::PathBuf;

use crate::event_bus::EventBus;

/// Initialize the config domain.
///
/// Creates the config actor and returns a handle for public API access.
/// This is the primary entry point for other domains to use the config domain.
///
/// # Arguments
/// * `config_path` - Optional path to config file. If None, uses default XDG location.
/// * `event_bus` - EventBus for publishing state events.
///
/// # Returns
/// `Ok(ConfigHandle)` on success, or `ConfigError` if initialization fails.
///
/// # Example
/// ```rust,no_run
/// use poseidon::domains::config::initialize_config_domain;
/// use poseidon::event_bus::EventBus;
///
/// # async fn example() {
/// let event_bus = EventBus::new(128);
/// let config_handle = initialize_config_domain(None, event_bus).await.unwrap();
/// # }
/// ```
pub async fn initialize_config_domain(
    config_path: Option<PathBuf>,
    event_bus: EventBus,
) -> Result<ConfigHandle, ConfigError> {
    let handle = ConfigHandle::new(config_path, event_bus).await?;
    Ok(handle)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_exports() {
        // Just verify types are exported by calling the function
        // The function signature test verifies the types are correct
        std::mem::drop(Box::pin(initialize_config_domain(None, EventBus::new(1))));
    }
}
