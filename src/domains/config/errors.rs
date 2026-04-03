//! Config domain errors
//!
//! This module defines error types specific to the config domain,
//! including file I/O errors, parse errors, and capability detection failures.

use std::fmt;
use std::io;

use serde::{Deserialize, Serialize};

/// Errors that can occur in the config domain.
///
/// This enum covers all error cases related to configuration loading,
/// saving, parsing, and host capability detection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConfigError {
    /// I/O error when reading or writing config files.
    Io {
        /// The path that caused the error.
        path: String,
        /// The error message.
        message: String,
    },
    /// TOML parsing error.
    Parse {
        /// The error message from the TOML parser.
        message: String,
    },
    /// TOML serialization error.
    Serialize {
        /// The error message from the TOML serializer.
        message: String,
    },
    /// Config directory could not be determined.
    NoConfigDir,
    /// Configuration file not found at the expected location.
    ConfigNotFound {
        /// The path that was expected.
        path: String,
    },
    /// Invalid configuration value.
    InvalidValue {
        /// The key that had an invalid value.
        key: String,
        /// The expected type or format.
        expected: String,
    },
    /// Host capability detection failed.
    CapabilityDetectionFailed {
        /// The capability that could not be detected.
        capability: String,
        /// The reason for failure.
        reason: String,
    },
    /// Channel send error (actor communication).
    ChannelSend,
    /// Channel receive error (actor communication).
    ChannelRecv,
    /// Oneshot reply channel closed.
    OneshotReply,
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::Io { path, message } => {
                write!(f, "IO error for path '{}': {}", path, message)
            }
            ConfigError::Parse { message } => {
                write!(f, "Failed to parse config: {}", message)
            }
            ConfigError::Serialize { message } => {
                write!(f, "Failed to serialize config: {}", message)
            }
            ConfigError::NoConfigDir => {
                write!(f, "Could not determine config directory")
            }
            ConfigError::ConfigNotFound { path } => {
                write!(f, "Config file not found: {}", path)
            }
            ConfigError::InvalidValue { key, expected } => {
                write!(f, "Invalid value for '{}': expected {}", key, expected)
            }
            ConfigError::CapabilityDetectionFailed { capability, reason } => {
                write!(f, "Failed to detect capability '{}': {}", capability, reason)
            }
            ConfigError::ChannelSend => {
                write!(f, "Failed to send message to config actor")
            }
            ConfigError::ChannelRecv => {
                write!(f, "Failed to receive message from config actor")
            }
            ConfigError::OneshotReply => {
                write!(f, "Config actor reply channel closed")
            }
        }
    }
}

impl std::error::Error for ConfigError {}

impl From<io::Error> for ConfigError {
    fn from(err: io::Error) -> Self {
        ConfigError::Io {
            path: String::new(),
            message: err.to_string(),
        }
    }
}

impl From<toml::de::Error> for ConfigError {
    fn from(err: toml::de::Error) -> Self {
        ConfigError::Parse {
            message: err.to_string(),
        }
    }
}

impl From<toml::ser::Error> for ConfigError {
    fn from(err: toml::ser::Error) -> Self {
        ConfigError::Serialize {
            message: err.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_io_error_display() {
        let err = ConfigError::Io {
            path: "/etc/config.toml".to_string(),
            message: "Permission denied".to_string(),
        };
        assert!(err.to_string().contains("IO error"));
        assert!(err.to_string().contains("/etc/config.toml"));
    }

    #[test]
    fn test_parse_error_display() {
        let err = ConfigError::Parse {
            message: "Invalid syntax at line 5".to_string(),
        };
        assert!(err.to_string().contains("Failed to parse"));
    }

    #[test]
    fn test_no_config_dir_display() {
        let err = ConfigError::NoConfigDir;
        assert!(err.to_string().contains("config directory"));
    }

    #[test]
    fn test_invalid_value_display() {
        let err = ConfigError::InvalidValue {
            key: "api_key".to_string(),
            expected: "non-empty string".to_string(),
        };
        assert!(err.to_string().contains("api_key"));
        assert!(err.to_string().contains("non-empty string"));
    }

    #[test]
    fn test_capability_detection_failed_display() {
        let err = ConfigError::CapabilityDetectionFailed {
            capability: "keyring".to_string(),
            reason: "DBus not available".to_string(),
        };
        assert!(err.to_string().contains("keyring"));
        assert!(err.to_string().contains("DBus"));
    }

    #[test]
    fn test_from_io_error() {
        let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
        let config_err: ConfigError = io_err.into();
        match config_err {
            ConfigError::Io { message, .. } => {
                assert!(message.contains("file not found"));
            }
            _ => panic!("Expected Io error variant"),
        }
    }

    #[test]
    fn test_serde_roundtrip() {
        let err = ConfigError::ConfigNotFound {
            path: "/home/user/.config/edge/config.toml".to_string(),
        };
        let json = serde_json::to_string(&err).unwrap();
        let deserialized: ConfigError = serde_json::from_str(&json).unwrap();
        match deserialized {
            ConfigError::ConfigNotFound { path } => {
                assert_eq!(path, "/home/user/.config/edge/config.toml");
            }
            _ => panic!("Wrong variant after deserialization"),
        }
    }
}
