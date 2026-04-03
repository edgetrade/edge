//! Keystore domain error types.
//!
//! Defines all error variants for keystore operations including
//! authentication, storage, crypto, and keyring errors.

use thiserror::Error;

/// Result type for keystore operations.
pub type KeystoreResult<T> = Result<T, KeystoreError>;

/// Errors that can occur in the keystore domain.
#[derive(Debug, Error, Clone)]
pub enum KeystoreError {
    /// Wrong password provided.
    #[error("Wrong password")]
    WrongPassword,

    /// Keystore is locked.
    #[error("Keystore locked")]
    Locked,

    /// Backend unavailable.
    #[error("Backend unavailable: {0}")]
    BackendUnavailable(String),

    /// Key derivation failed.
    #[error("Key derivation failed: {0}")]
    DerivationFailed(String),

    /// Storage operation failed.
    #[error("Storage error: {0}")]
    Storage(String),

    /// Channel communication error.
    #[error("Channel error")]
    ChannelError,

    /// Authentication failed (wrong password, invalid credentials).
    #[error("Authentication failed: {0}")]
    Authentication(String),

    /// Keyring is unavailable or operation failed.
    #[error("Keyring error: {0}")]
    Keyring(String),

    /// Cryptographic operation failed (encryption, decryption, derivation).
    #[error("Crypto error: {0}")]
    Crypto(String),

    /// Session is not unlocked or key not available.
    #[error("Session locked: {0}")]
    SessionLocked(String),

    /// Key already exists (idempotent protection).
    #[error("Key already exists: {0}")]
    AlreadyExists(String),

    /// Key not found.
    #[error("Key not found: {0}")]
    NotFound(String),

    /// Invalid input (password mismatch, bad format).
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    /// Passkey/WebAuthn not implemented.
    #[error("Passkey not implemented: {0}")]
    PasskeyNotImplemented(String),

    /// Operation cancelled by user.
    #[error("Operation cancelled")]
    Cancelled,

    /// Channel send error (actor communication).
    #[error("Channel send error")]
    ChannelSend,

    /// Channel receive error (actor communication).
    #[error("Channel receive error")]
    ChannelRecv,

    /// Oneshot reply error.
    #[error("Oneshot reply error")]
    OneshotReply,

    /// Domain configuration error.
    #[error("Configuration error: {0}")]
    Config(String),

    /// Unknown/internal error.
    #[error("Internal error: {0}")]
    Internal(String),
}

impl KeystoreError {
    /// Returns true if this error indicates authentication failure.
    pub fn is_auth_error(&self) -> bool {
        matches!(
            self,
            KeystoreError::Authentication(_) | KeystoreError::SessionLocked(_) | KeystoreError::WrongPassword
        )
    }

    /// Returns true if this error indicates the keystore is locked.
    pub fn is_locked(&self) -> bool {
        matches!(self, KeystoreError::Locked | KeystoreError::SessionLocked(_))
    }

    /// Returns true if this is a "not found" error.
    pub fn is_not_found(&self) -> bool {
        matches!(self, KeystoreError::NotFound(_))
    }

    /// Returns true if this is an "already exists" error.
    pub fn is_already_exists(&self) -> bool {
        matches!(self, KeystoreError::AlreadyExists(_))
    }
}

// Conversions from crypto types
impl From<crate::domains::keystore::crypto::types::CryptoError> for KeystoreError {
    fn from(e: crate::domains::keystore::crypto::types::CryptoError) -> Self {
        KeystoreError::Crypto(e.to_string())
    }
}

// Conversions from storage types
impl From<crate::domains::keystore::crypto::storage::StorageError> for KeystoreError {
    fn from(e: crate::domains::keystore::crypto::storage::StorageError) -> Self {
        KeystoreError::Storage(e.to_string())
    }
}

// Conversions from auth types
impl From<crate::domains::keystore::auth::types::AuthError> for KeystoreError {
    fn from(e: crate::domains::keystore::auth::types::AuthError) -> Self {
        match e {
            crate::domains::keystore::auth::types::AuthError::InvalidCredentials => KeystoreError::WrongPassword,
            crate::domains::keystore::auth::types::AuthError::AuthenticationFailed(msg) => {
                KeystoreError::Authentication(msg)
            }
            crate::domains::keystore::auth::types::AuthError::PasskeyVerificationFailed => {
                KeystoreError::PasskeyNotImplemented("Passkey verification not implemented".to_string())
            }
            crate::domains::keystore::auth::types::AuthError::PasskeyRegistrationFailed(msg) => {
                KeystoreError::PasskeyNotImplemented(msg)
            }
            crate::domains::keystore::auth::types::AuthError::NotImplemented => {
                KeystoreError::PasskeyNotImplemented("Passkey authentication not implemented".to_string())
            }
            crate::domains::keystore::auth::types::AuthError::Io(msg) => KeystoreError::Storage(msg),
            crate::domains::keystore::auth::types::AuthError::Storage(msg) => KeystoreError::Storage(msg),
            crate::domains::keystore::auth::types::AuthError::Crypto(msg) => KeystoreError::Crypto(msg),
            crate::domains::keystore::auth::types::AuthError::Cancelled => KeystoreError::Cancelled,
        }
    }
}

// Conversions from session types
impl From<crate::domains::keystore::session::keyring::SessionError> for KeystoreError {
    fn from(e: crate::domains::keystore::session::keyring::SessionError) -> Self {
        match e {
            crate::domains::keystore::session::keyring::SessionError::Keyring(msg) => KeystoreError::Keyring(msg),
            crate::domains::keystore::session::keyring::SessionError::NotFound => {
                KeystoreError::SessionLocked("No key found in keyring".to_string())
            }
            crate::domains::keystore::session::keyring::SessionError::Corrupted => {
                KeystoreError::Crypto("Session data corrupted".to_string())
            }
        }
    }
}

impl From<crate::domains::keystore::session::filestore::SessionError> for KeystoreError {
    fn from(e: crate::domains::keystore::session::filestore::SessionError) -> Self {
        match e {
            crate::domains::keystore::session::filestore::SessionError::Storage(msg) => KeystoreError::Storage(msg),
            crate::domains::keystore::session::filestore::SessionError::Encoding(msg) => {
                KeystoreError::Crypto(format!("Encoding error: {}", msg))
            }
            crate::domains::keystore::session::filestore::SessionError::Corrupted => {
                KeystoreError::Crypto("Session data corrupted".to_string())
            }
        }
    }
}

impl From<std::io::Error> for KeystoreError {
    fn from(e: std::io::Error) -> Self {
        KeystoreError::Storage(e.to_string())
    }
}
