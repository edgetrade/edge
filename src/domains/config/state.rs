//! Config domain state
//!
//! This module defines the state structures for the config domain,
//! including the configuration and host capabilities.

use std::path::PathBuf;

use crate::domains::config::types::Config;
use serde::{Deserialize, Serialize};

/// Configuration state owned by the config actor.
///
/// This struct holds the loaded configuration and detected host capabilities.
/// It is owned exclusively by the ConfigActor and never shared.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigState {
    /// Config loaded from ~/.config/edge/config.toml
    pub config: Config,
    /// Host-level capabilities (detected at startup, never changes)
    pub host_capabilities: HostCapabilities,
    /// Path to the config file
    pub config_path: PathBuf,
}

impl ConfigState {
    /// Create a new ConfigState with the given config and capabilities.
    ///
    /// # Arguments
    /// * `config` - The loaded configuration
    /// * `host_capabilities` - The detected host capabilities
    /// * `config_path` - The path to the config file
    ///
    /// # Returns
    /// A new ConfigState instance
    pub fn new(config: Config, host_capabilities: HostCapabilities, config_path: PathBuf) -> Self {
        Self {
            config,
            host_capabilities,
            config_path,
        }
    }

    /// Get the config reference
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// Get the config mutable reference
    pub fn config_mut(&mut self) -> &mut Config {
        &mut self.config
    }

    /// Get the host capabilities
    pub fn host_capabilities(&self) -> &HostCapabilities {
        &self.host_capabilities
    }

    /// Get the config path
    pub fn config_path(&self) -> &PathBuf {
        &self.config_path
    }

    /// Update the config path
    pub fn set_config_path(&mut self, path: PathBuf) {
        self.config_path = path;
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

impl std::fmt::Display for OperatingSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OperatingSystem::Linux => write!(f, "Linux"),
            OperatingSystem::MacOS => write!(f, "macOS"),
            OperatingSystem::Windows => write!(f, "Windows"),
            OperatingSystem::Unknown => write!(f, "Unknown"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_state_creation() {
        let config = Config::default();
        let caps = HostCapabilities::default();
        let path = PathBuf::from("/tmp/test.toml");

        let state = ConfigState::new(config.clone(), caps, path.clone());

        assert_eq!(state.config_path(), &path);
    }

    #[test]
    fn test_host_capabilities_default() {
        let caps = HostCapabilities::default();
        // Just verify it doesn't panic - actual values depend on system
        assert!(!caps.version.is_empty());
    }

    #[test]
    fn test_operating_system_display() {
        assert_eq!(OperatingSystem::Linux.to_string(), "Linux");
        assert_eq!(OperatingSystem::MacOS.to_string(), "macOS");
        assert_eq!(OperatingSystem::Windows.to_string(), "Windows");
        assert_eq!(OperatingSystem::Unknown.to_string(), "Unknown");
    }

    #[test]
    fn test_config_state_serde() {
        let config = Config::default();
        let caps = HostCapabilities::default();
        let path = PathBuf::from("/tmp/test.toml");

        let state = ConfigState::new(config, caps, path);
        let json = serde_json::to_string(&state).unwrap();
        let deserialized: ConfigState = serde_json::from_str(&json).unwrap();

        assert_eq!(state.config_path, deserialized.config_path);
    }
}
