//! Authentication types and traits for Edge CLI.
//!
//! Defines the core types used across authentication methods including
//! the `AuthenticationMethod` enum, `AuthenticationResult`, and the
//! `Authenticator` trait for pluggable authentication.

use crate::domains::keystore::crypto::types::{MasterKey, SALT_SIZE};
use thiserror::Error;

/// The method used to authenticate the user.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthenticationMethod {
    /// Password-based authentication using PBKDF2 derivation.
    Password,
    /// Passkey/WebAuthn authentication using PRF extension.
    Passkey,
}

/// The result of a successful authentication.
///
/// Contains the derived master key and the salt used for derivation.
/// The salt must be stored and reused for subsequent authentication
/// attempts to derive the same master key.
#[derive(Debug, Clone)]
pub struct AuthenticationResult {
    /// The master key derived from the authentication material.
    pub master_key: MasterKey,
    /// The salt used for key derivation (16 bytes).
    pub salt: [u8; SALT_SIZE],
}

/// Errors that can occur during authentication.
#[derive(Debug, Clone, Error)]
pub enum AuthError {
    /// Invalid credentials provided.
    #[error("Invalid credentials")]
    InvalidCredentials,
    /// Authentication operation failed.
    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),
    /// Passkey verification failed.
    #[error("Passkey verification failed")]
    PasskeyVerificationFailed,
    /// Passkey registration failed.
    #[error("Passkey registration failed: {0}")]
    PasskeyRegistrationFailed(String),
    /// Passkey not yet implemented (temporary during development).
    #[error("Passkey authentication not yet implemented")]
    NotImplemented,
    /// IO error during authentication.
    #[error("IO error: {0}")]
    Io(String),
    /// Storage error during authentication.
    #[error("Storage error: {0}")]
    Storage(String),
    /// Crypto error during authentication.
    #[error("Crypto error: {0}")]
    Crypto(String),
    /// User cancelled the operation.
    #[error("Operation cancelled by user")]
    Cancelled,
}

impl From<std::io::Error> for AuthError {
    fn from(e: std::io::Error) -> Self {
        AuthError::Io(e.to_string())
    }
}

impl From<crate::domains::keystore::crypto::types::CryptoError> for AuthError {
    fn from(e: crate::domains::keystore::crypto::types::CryptoError) -> Self {
        AuthError::Crypto(e.to_string())
    }
}

impl From<crate::domains::keystore::crypto::storage::StorageError> for AuthError {
    fn from(e: crate::domains::keystore::crypto::storage::StorageError) -> Self {
        AuthError::Storage(e.to_string())
    }
}

/// Result type for authentication operations.
pub type AuthResult<T> = Result<T, AuthError>;

/// Trait for authentication implementations.
///
/// Implementors of this trait provide a consistent interface for
/// authenticating users and deriving the master key.
pub trait Authenticator {
    /// Authenticate the user and derive the master key.
    ///
    /// # Returns
    /// `AuthenticationResult` containing the master key and salt on success.
    fn authenticate(&self) -> AuthResult<AuthenticationResult>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_authentication_method_enum() {
        assert_eq!(AuthenticationMethod::Password, AuthenticationMethod::Password);
        assert_eq!(AuthenticationMethod::Passkey, AuthenticationMethod::Passkey);
        assert_ne!(AuthenticationMethod::Password, AuthenticationMethod::Passkey);
    }

    #[test]
    fn test_auth_error_display() {
        let err = AuthError::InvalidCredentials;
        assert_eq!(err.to_string(), "Invalid credentials");

        let err = AuthError::AuthenticationFailed("test".to_string());
        assert_eq!(err.to_string(), "Authentication failed: test");

        let err = AuthError::NotImplemented;
        assert_eq!(err.to_string(), "Passkey authentication not yet implemented");
    }

    #[test]
    fn test_authentication_result_creation() {
        let master_key = MasterKey([0x42u8; 32]);
        let salt = [0xABu8; 16];
        let result = AuthenticationResult { master_key, salt };

        assert_eq!(result.master_key.0, [0x42u8; 32]);
        assert_eq!(result.salt, [0xABu8; 16]);
    }

    #[test]
    fn test_io_error_conversion() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let auth_err: AuthError = io_err.into();
        assert!(matches!(auth_err, AuthError::Io(_)));
        assert!(auth_err.to_string().contains("file not found"));
    }
}
