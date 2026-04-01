//! Unified error types for Poseidon.
//!
//! This module provides a top-level error enum that aggregates all error types
//! from the various subsystems (config, session, client, state, etc.).
//! It enables the refactored commands and handlers to return `Result<T, PoseidonError>`
//! instead of exit codes, allowing for proper error propagation and handling.

use thiserror::Error;

/// Unified error type for Poseidon operations.
///
/// This enum aggregates errors from all subsystems and provides a consistent
/// interface for error handling across the CLI. Each variant wraps a specific
/// error type from a subsystem, with automatic conversions via `From` traits.
///
/// # Example
///
/// ```rust
/// use poseidon::error::PoseidonError;
///
/// fn may_fail() -> Result<(), PoseidonError> {
///     // Config operations automatically convert
///     let config = poseidon::config::Config::load_default()?;
///     
///     // Session operations automatically convert  
///     let session = poseidon::session::Session::new(config);
///     
///     Ok(())
/// }
/// ```
#[derive(Debug, Error)]
pub enum PoseidonError {
    /// Configuration-related errors.
    #[error("Config error: {0}")]
    Config(#[from] crate::config::ConfigError),

    /// Orders errors.
    #[error("Orders error: {0}")]
    Orders(#[from] crate::orders::OrdersError),

    /// Session management errors.
    #[error("Session error: {0}")]
    Session(#[from] crate::session::SessionError),

    /// Client/API communication errors.
    #[error("Client error: {0}")]
    Client(#[from] crate::messages::IrisClientError),

    /// State management errors.
    #[error("State error: {0}")]
    State(#[from] crate::state::StateError),

    /// I/O errors.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Command execution errors.
    #[error("Command error: {0}")]
    Command(String),

    /// Authentication errors.
    #[error("Authentication failed: {0}")]
    Authentication(String),

    /// Crypto/encryption errors.
    #[error("Crypto error: {0}")]
    Crypto(String),

    /// Storage errors (file, keyring, etc.).
    #[error("Storage error: {0}")]
    Storage(String),

    /// Wallet operation errors.
    #[error("Wallet error: {0}")]
    Wallet(String),

    /// Invalid user input.
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    /// Resource not found.
    #[error("Not found: {0}")]
    NotFound(String),

    /// Resource already exists.
    #[error("Already exists: {0}")]
    AlreadyExists(String),

    /// Initialization errors.
    #[error("Initialization error: {0}")]
    Initialization(String),

    /// Lock acquisition errors.
    #[error("Lock error: {0}")]
    LockError(String),

    /// Serialization/deserialization errors.
    #[error("Serialization error: {0}")]
    Serialization(String),

    /// Manifest-related errors.
    #[error("Manifest error: {0}")]
    Manifest(String),

    /// Transport/transport key cache errors.
    #[error("Transport error: {0}")]
    Transport(String),

    /// Generic/other errors.
    #[error("{0}")]
    Other(String),
}

impl PoseidonError {
    /// Returns true if this error is authentication-related.
    ///
    /// Authentication errors may require re-authenticating the user.
    pub fn is_auth_error(&self) -> bool {
        matches!(
            self,
            PoseidonError::Authentication(_) | PoseidonError::Client(crate::messages::IrisClientError::Auth(_))
        )
    }

    /// Returns true if this error indicates the state is not initialized.
    ///
    /// This typically means the application hasn't been properly started.
    pub fn is_not_initialized(&self) -> bool {
        matches!(self, PoseidonError::State(crate::state::StateError::NotInitialized))
    }

    /// Returns true if this error indicates a resource was not found.
    pub fn is_not_found(&self) -> bool {
        matches!(
            self,
            PoseidonError::NotFound(_)
                | PoseidonError::Session(crate::session::SessionError::NotFound)
                | PoseidonError::State(crate::state::StateError::NotInitialized)
        )
    }

    /// Returns true if this error indicates a resource already exists.
    pub fn is_already_exists(&self) -> bool {
        matches!(
            self,
            PoseidonError::AlreadyExists(_) | PoseidonError::State(crate::state::StateError::AlreadyInitialized)
        )
    }

    /// Get a user-friendly error message.
    ///
    /// This provides a message suitable for displaying to the user,
    /// potentially with more context than the raw error display.
    pub fn user_message(&self) -> String {
        match self {
            PoseidonError::Authentication(msg) => {
                format!("Authentication failed: {}. Please check your API key.", msg)
            }
            PoseidonError::NotFound(msg) => {
                format!("Not found: {}. Please check the resource exists.", msg)
            }
            PoseidonError::AlreadyExists(msg) => {
                format!(
                    "Already exists: {}. Use a different name or delete the existing resource.",
                    msg
                )
            }
            PoseidonError::InvalidInput(msg) => {
                format!("Invalid input: {}. Please check your input and try again.", msg)
            }
            PoseidonError::Session(crate::session::SessionError::NotFound) => {
                "No active session found. Please unlock your session first.".to_string()
            }
            PoseidonError::State(crate::state::StateError::NotInitialized) => {
                "Application not initialized. Please run the init command first.".to_string()
            }
            _ => self.to_string(),
        }
    }
}

// Additional From implementations for converting from string-based errors
// commonly used in the codebase

impl From<crate::commands::key::filestore::crypto::types::CryptoError> for PoseidonError {
    fn from(e: crate::commands::key::filestore::crypto::types::CryptoError) -> Self {
        PoseidonError::Crypto(e.to_string())
    }
}

impl From<crate::commands::key::filestore::storage::StorageError> for PoseidonError {
    fn from(e: crate::commands::key::filestore::storage::StorageError) -> Self {
        PoseidonError::Storage(e.to_string())
    }
}

impl From<crate::wallet::types::WalletError> for PoseidonError {
    fn from(e: crate::wallet::types::WalletError) -> Self {
        PoseidonError::Wallet(e.to_string())
    }
}

impl From<crate::commands::wallet::game::game_state::GameStateError> for PoseidonError {
    fn from(e: crate::commands::wallet::game::game_state::GameStateError) -> Self {
        PoseidonError::Storage(e.to_string())
    }
}

impl From<crate::messages::CommandError> for PoseidonError {
    fn from(e: crate::messages::CommandError) -> Self {
        match e {
            crate::messages::CommandError::Authentication(msg) => PoseidonError::Authentication(msg),
            crate::messages::CommandError::Crypto(msg) => PoseidonError::Crypto(msg),
            crate::messages::CommandError::Storage(msg) => PoseidonError::Storage(msg),
            crate::messages::CommandError::Session(msg) => {
                PoseidonError::Session(crate::session::SessionError::Keyring(msg))
            }
            crate::messages::CommandError::Io(msg) => PoseidonError::Io(std::io::Error::other(msg)),
            crate::messages::CommandError::AlreadyExists => PoseidonError::AlreadyExists("Resource".to_string()),
            crate::messages::CommandError::NotFound => PoseidonError::NotFound("Resource".to_string()),
            crate::messages::CommandError::InvalidInput(msg) => PoseidonError::InvalidInput(msg),
            crate::messages::CommandError::Wallet(msg) => PoseidonError::Wallet(msg),
        }
    }
}

impl From<crate::manifest::ManifestError> for PoseidonError {
    fn from(e: crate::manifest::ManifestError) -> Self {
        PoseidonError::Manifest(e.to_string())
    }
}

impl From<crate::manifest::fetch::FetchError> for PoseidonError {
    fn from(e: crate::manifest::fetch::FetchError) -> Self {
        PoseidonError::Manifest(e.to_string())
    }
}

impl From<crate::session::transport::TransportCacheError> for PoseidonError {
    fn from(e: crate::session::transport::TransportCacheError) -> Self {
        PoseidonError::Transport(e.to_string())
    }
}

impl From<toml::de::Error> for PoseidonError {
    fn from(e: toml::de::Error) -> Self {
        PoseidonError::Serialization(format!("TOML parse error: {}", e))
    }
}

impl From<toml::ser::Error> for PoseidonError {
    fn from(e: toml::ser::Error) -> Self {
        PoseidonError::Serialization(format!("TOML serialization error: {}", e))
    }
}

impl From<serde_json::Error> for PoseidonError {
    fn from(e: serde_json::Error) -> Self {
        PoseidonError::Serialization(format!("JSON error: {}", e))
    }
}

/// Type alias for Results using PoseidonError.
pub type Result<T> = std::result::Result<T, PoseidonError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = PoseidonError::Authentication("test".to_string());
        assert_eq!(err.to_string(), "Authentication failed: test");

        let err = PoseidonError::NotFound("config".to_string());
        assert_eq!(err.to_string(), "Not found: config");
    }

    #[test]
    fn test_is_auth_error() {
        let err = PoseidonError::Authentication("test".to_string());
        assert!(err.is_auth_error());

        let err = PoseidonError::NotFound("test".to_string());
        assert!(!err.is_auth_error());
    }

    #[test]
    fn test_is_not_found() {
        let err = PoseidonError::NotFound("config".to_string());
        assert!(err.is_not_found());

        let err = PoseidonError::Authentication("test".to_string());
        assert!(!err.is_not_found());
    }

    #[test]
    fn test_user_message() {
        let err = PoseidonError::Authentication("invalid key".to_string());
        assert!(err.user_message().contains("Authentication failed"));

        let err = PoseidonError::NotFound("wallet".to_string());
        assert!(err.user_message().contains("Not found"));
    }

    #[test]
    fn test_from_io_error() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let err: PoseidonError = io_err.into();
        assert!(matches!(err, PoseidonError::Io(_)));
    }

    #[test]
    fn test_send_sync() {
        fn assert_send_sync<T: Send + Sync + 'static>() {}
        assert_send_sync::<PoseidonError>();
    }
}
