//! Session management for the keystore domain.
//!
//! Provides unified session management across both filestore (server)
//! and keyring (desktop) backends.

pub mod crypto;
pub mod filestore;
pub mod keyring;

use ed25519_dalek::SigningKey;
use hkdf::Hkdf;
use sha2::Sha256;

use erato::messages::envelopes::storage::derive_storage_key;
use erato::types::cryptography::USER_ENCRYPTION_KEY_HKDF_INFO;

use crate::domains::config::Config;
use crate::domains::keystore::session::crypto::UsersEncryptionKeys;

/// Unified session type that abstracts over keyring and filestore backends.
///
/// This enum provides a common interface for session operations regardless
/// of whether the session is backed by the OS keyring or file-based storage.
#[derive(Debug, Clone)]
pub enum Session {
    /// Keyring-backed session for desktop environments
    Keyring(keyring::Session),
    /// Filestore-backed session for server environments
    File(filestore::Session),
}

impl Session {
    /// Create a new session with the appropriate backend based on host capabilities.
    ///
    /// If keyring is available, uses the keyring backend. Otherwise falls back
    /// to filestore.
    ///
    /// # Arguments
    /// * `config` - Configuration for the session
    pub fn new(config: Config) -> Self {
        if crate::domains::keystore::keyring_available() {
            Session::Keyring(keyring::Session::new(config))
        } else {
            Session::File(filestore::Session::new(config))
        }
    }

    /// Unlock the session with the user encryption key.
    ///
    /// Stores the UEK in the underlying session storage.
    ///
    /// # Arguments
    /// * `uek` - The user encryption key to store
    pub fn unlock(&self, uek: &UsersEncryptionKeys) -> Result<(), SessionError> {
        match self {
            Session::Keyring(s) => s.unlock(uek).map_err(SessionError::from),
            Session::File(s) => s.unlock(uek).map_err(SessionError::from),
        }
    }

    /// Lock the session, removing stored credentials.
    ///
    /// This is idempotent - succeeds even if no session exists.
    pub fn lock(&self) -> Result<(), SessionError> {
        match self {
            Session::Keyring(s) => s.lock().map_err(SessionError::from),
            Session::File(s) => s.lock().map_err(SessionError::from),
        }
    }

    /// Get the user encryption key from the session.
    ///
    /// # Returns
    /// `Ok(Some(UserEncryptionKey))` if a session exists,
    /// `Ok(None)` if no session exists.
    pub fn get_user_encryption_key(&self) -> Result<Option<UsersEncryptionKeys>, SessionError> {
        match self {
            Session::Keyring(s) => s.get_user_encryption_key().map_err(SessionError::from),
            Session::File(s) => s.get_user_encryption_key().map_err(SessionError::from),
        }
    }

    /// Check if the session is currently unlocked.
    ///
    /// # Returns
    /// `true` if the session has a stored UEK, `false` otherwise.
    pub fn is_unlocked(&self) -> bool {
        match self {
            Session::Keyring(s) => s.is_unlocked(),
            Session::File(s) => s.is_unlocked(),
        }
    }

    /// Get the config from the session.
    pub fn get_config(&self) -> Result<&Config, SessionError> {
        match self {
            Session::Keyring(s) => s.get_config().map_err(SessionError::from),
            Session::File(s) => s.get_config().map_err(SessionError::from),
        }
    }

    /// Unlock the session with a password.
    ///
    /// Derives the user encryption key from the password
    /// and stores it in the session.
    ///
    /// # Arguments
    /// * `password` - The password to derive the key from.
    ///
    /// # Returns
    /// `Ok(())` on success, or an error if unlocking fails.
    pub fn unlock_with_password(&self, password: &str) -> Result<(), SessionError> {
        // Derive 32-byte UEK from password using HKDF-SHA256
        let hkdf = Hkdf::<Sha256>::new(None, password.as_bytes());
        let mut uek_bytes = [0u8; 32];
        hkdf.expand(USER_ENCRYPTION_KEY_HKDF_INFO, &mut uek_bytes)
            .map_err(|e| SessionError::Keyring(format!("HKDF expansion failed: {}", e)))?;
        let uek = derive_storage_key(&uek_bytes);
        let user_key = UsersEncryptionKeys::new(SigningKey::from_bytes(&uek_bytes), uek, None);

        self.unlock(&user_key)
    }
}

/// Error type for unified session operations.
#[derive(Debug, Clone, thiserror::Error)]
pub enum SessionError {
    /// Keyring session error
    #[error("Keyring error: {0}")]
    Keyring(String),
    /// Filestore session error
    #[error("Filestore error: {0}")]
    Filestore(String),
    /// Session not found
    #[error("Session not found")]
    NotFound,
}

impl From<keyring::SessionError> for SessionError {
    fn from(e: keyring::SessionError) -> Self {
        SessionError::Keyring(e.to_string())
    }
}

impl From<filestore::SessionError> for SessionError {
    fn from(e: filestore::SessionError) -> Self {
        SessionError::Filestore(e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_error_display() {
        let err = SessionError::Keyring("test error".to_string());
        assert!(err.to_string().contains("Keyring error"));
        assert!(err.to_string().contains("test error"));
    }
}
