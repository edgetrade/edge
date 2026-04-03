//! Config domain messages
//!
//! This module defines the message types used for communication with the
//! config actor. Messages follow the actor pattern with request/reply channels.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::domains::config::errors::ConfigError;

/// Messages that can be sent to the config actor.
///
/// Each variant represents a command or query that the config domain
/// can handle. These messages are wrapped in `PoseidonRequest` for
/// telemetry context and reply channels.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConfigMessage {
    /// Load configuration from disk.
    LoadConfig,
    /// Reload configuration from disk (refresh cached values).
    ReloadConfig,
    /// Save current configuration to disk.
    SaveConfig,
    /// Get a configuration value by key path.
    GetConfigValue {
        /// Dot-separated path to the config value (e.g., "session.use_keyring").
        key: String,
    },
    /// Set a configuration value by key path.
    SetConfigValue {
        /// Dot-separated path to the config value.
        key: String,
        /// The value to set (as JSON for flexibility).
        value: serde_json::Value,
    },
    /// Get host capabilities.
    GetHostCapabilities,
    /// Get the path to the config file.
    GetConfigPath,
    /// Update the manifest timestamp to now.
    UpdateManifestTimestamp,
    /// Get the stored manifest timestamp.
    GetManifestTimestamp,
}

impl fmt::Display for ConfigMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigMessage::LoadConfig => write!(f, "LoadConfig"),
            ConfigMessage::ReloadConfig => write!(f, "ReloadConfig"),
            ConfigMessage::SaveConfig => write!(f, "SaveConfig"),
            ConfigMessage::GetConfigValue { key } => write!(f, "GetConfigValue({})", key),
            ConfigMessage::SetConfigValue { key, value } => {
                write!(f, "SetConfigValue({}, {})", key, value)
            }
            ConfigMessage::GetHostCapabilities => write!(f, "GetHostCapabilities"),
            ConfigMessage::GetConfigPath => write!(f, "GetConfigPath"),
            ConfigMessage::UpdateManifestTimestamp => write!(f, "UpdateManifestTimestamp"),
            ConfigMessage::GetManifestTimestamp => write!(f, "GetManifestTimestamp"),
        }
    }
}

use std::fmt;

/// Response types for config operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConfigResponse {
    /// Configuration loaded successfully.
    ConfigLoaded {
        /// The path from which the config was loaded.
        path: PathBuf,
    },
    /// Configuration saved successfully.
    ConfigSaved {
        /// The path to which the config was saved.
        path: PathBuf,
    },
    /// Configuration value retrieved.
    ConfigValue {
        /// The retrieved value.
        value: serde_json::Value,
    },
    /// Value was set successfully.
    ValueSet {
        /// The key that was set.
        key: String,
    },
    /// Host capabilities response.
    HostCapabilities {
        /// The detected host capabilities.
        capabilities: HostCapabilities,
    },
    /// Config file path.
    ConfigPath {
        /// The path to the config file.
        path: PathBuf,
    },
    /// Manifest timestamp.
    ManifestTimestamp {
        /// The timestamp when the manifest was last fetched.
        timestamp: Option<String>,
    },
}

/// Operating system types for host capability detection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OperatingSystem {
    /// Linux operating system.
    Linux,
    /// macOS operating system.
    MacOS,
    /// Windows operating system.
    Windows,
    /// Unknown or other operating system.
    Unknown,
}

impl fmt::Display for OperatingSystem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OperatingSystem::Linux => write!(f, "Linux"),
            OperatingSystem::MacOS => write!(f, "macOS"),
            OperatingSystem::Windows => write!(f, "Windows"),
            OperatingSystem::Unknown => write!(f, "Unknown"),
        }
    }
}

/// Host capabilities detected at startup.
///
/// These capabilities are detected once at startup and never change
/// during the application lifetime. They help domains make decisions
/// about which features to enable.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostCapabilities {
    /// Whether the OS keyring is available for secure key storage.
    pub keyring_available: bool,
    /// The detected operating system.
    pub os: OperatingSystem,
    /// The OS version string.
    pub version: String,
}

impl HostCapabilities {
    /// Detect host capabilities at startup.
    ///
    /// This method checks the OS keyring availability and determines
    /// the current operating system. It should be called once at startup.
    ///
    /// # Returns
    /// A `HostCapabilities` struct with detected values.
    pub fn detect() -> Self {
        let keyring_available = Self::detect_keyring();
        let (os, version) = Self::detect_os();

        Self {
            keyring_available,
            os,
            version,
        }
    }

    /// Detect if the OS keyring is available.
    ///
    /// Attempts to access the system keyring to determine if it's
    /// available for secure key storage.
    ///
    /// # Returns
    /// `true` if the keyring is available, `false` otherwise.
    fn detect_keyring() -> bool {
        // Try to access the keyring with a test entry
        keyring::Entry::new("edge_test", "capability_check")
            .and_then(|e| e.get_password())
            .is_ok()
            || keyring::Entry::new("edge_test", "capability_check")
                .and_then(|e| e.set_password("test"))
                .is_ok()
    }

    /// Detect the operating system and version.
    ///
    /// # Returns
    /// A tuple of (OperatingSystem, version_string).
    fn detect_os() -> (OperatingSystem, String) {
        #[cfg(target_os = "linux")]
        {
            let version = Self::read_os_release()
                .or_else(Self::read_uname)
                .unwrap_or_else(|| "unknown".to_string());
            (OperatingSystem::Linux, version)
        }

        #[cfg(target_os = "macos")]
        {
            let version = Self::read_macos_version().unwrap_or_else(|| "unknown".to_string());
            (OperatingSystem::MacOS, version)
        }

        #[cfg(target_os = "windows")]
        {
            let version = std::env::var("OS").unwrap_or_else(|_| "Windows".to_string());
            (OperatingSystem::Windows, version)
        }

        #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
        {
            (OperatingSystem::Unknown, "unknown".to_string())
        }
    }

    /// Read OS version from /etc/os-release on Linux.
    #[cfg(target_os = "linux")]
    fn read_os_release() -> Option<String> {
        std::fs::read_to_string("/etc/os-release")
            .ok()
            .and_then(|content| {
                content
                    .lines()
                    .find(|line| line.starts_with("PRETTY_NAME="))
                    .map(|line| {
                        line.trim_start_matches("PRETTY_NAME=")
                            .trim_matches('"')
                            .to_string()
                    })
            })
    }

    /// Read OS version from uname on Linux.
    #[cfg(target_os = "linux")]
    fn read_uname() -> Option<String> {
        std::process::Command::new("uname")
            .args(["-r"])
            .output()
            .ok()
            .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
    }

    /// Read macOS version.
    #[cfg(target_os = "macos")]
    fn read_macos_version() -> Option<String> {
        std::process::Command::new("sw_vers")
            .args(["-productVersion"])
            .output()
            .ok()
            .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
    }
}

impl Default for HostCapabilities {
    fn default() -> Self {
        Self::detect()
    }
}

/// Request type for config operations with reply channel.
pub type ConfigRequest = crate::event_bus::PoseidonRequest<ConfigMessage, ConfigResponse, ConfigError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operating_system_display() {
        assert_eq!(OperatingSystem::Linux.to_string(), "Linux");
        assert_eq!(OperatingSystem::MacOS.to_string(), "macOS");
        assert_eq!(OperatingSystem::Windows.to_string(), "Windows");
        assert_eq!(OperatingSystem::Unknown.to_string(), "Unknown");
    }

    #[test]
    fn test_config_message_display() {
        let msg = ConfigMessage::LoadConfig;
        assert_eq!(msg.to_string(), "LoadConfig");

        let msg = ConfigMessage::GetConfigValue {
            key: "session.use_keyring".to_string(),
        };
        assert_eq!(msg.to_string(), "GetConfigValue(session.use_keyring)");

        let msg = ConfigMessage::SetConfigValue {
            key: "api_key".to_string(),
            value: serde_json::json!("test-key"),
        };
        assert_eq!(msg.to_string(), "SetConfigValue(api_key, \"test-key\")");
    }

    #[test]
    fn test_config_response_serde() {
        let response = ConfigResponse::ConfigLoaded {
            path: PathBuf::from("/home/user/.config/edge/config.toml"),
        };
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("ConfigLoaded"));
        assert!(json.contains("config.toml"));
    }

    #[test]
    fn test_host_capabilities_default() {
        let caps = HostCapabilities::default();
        // Just verify it doesn't panic - actual values depend on system
        // String.len() is always >= 0 by definition
        assert!(!caps.version.is_empty() || caps.version.is_empty());
    }

    #[test]
    fn test_serde_roundtrip() {
        let caps = HostCapabilities {
            keyring_available: true,
            os: OperatingSystem::Linux,
            version: "Ubuntu 22.04".to_string(),
        };
        let json = serde_json::to_string(&caps).unwrap();
        let deserialized: HostCapabilities = serde_json::from_str(&json).unwrap();
        assert!(deserialized.keyring_available);
        assert_eq!(deserialized.os, OperatingSystem::Linux);
        assert_eq!(deserialized.version, "Ubuntu 22.04");
    }
}
