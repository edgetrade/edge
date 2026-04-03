//! Session management for filestore backend.
//!
//! Provides simple file-based session storage for the password-based
//! authentication flow. No keyring - the session is secured by the
//! password-derived encryption.

use std::sync::Arc;

use base64::Engine;
use ed25519_dalek::SigningKey;

use crate::domains::config::Config;
use crate::domains::keystore::crypto::storage::{
    StorageError, delete_session_file, load_session_file, store_session_file,
};

use super::crypto::UsersEncryptionKeys;

/// Error type for session operations.
#[derive(Debug, Clone, thiserror::Error)]
pub enum SessionError {
    #[error("Storage error: {0}")]
    Storage(String),
    #[error("Encoding error: {0}")]
    Encoding(String),
    #[error("Session corrupted")]
    Corrupted,
}

impl From<StorageError> for SessionError {
    fn from(e: StorageError) -> Self {
        SessionError::Storage(e.to_string())
    }
}

/// Session management using simple file-based storage.
///
/// The user encryption key is stored in a file, base64-encoded.
/// Security is provided by the fact that the UEK itself is only
/// obtainable after password authentication unlocks the blind_user_key.
#[derive(Debug, Clone)]
pub struct Session {
    _private: (),
    /// Config
    config: Arc<Config>,
}

impl Session {
    /// Create a new session manager.
    pub fn new(config: Config) -> Self {
        Self {
            _private: (),
            config: Arc::new(config),
        }
    }

    /// Store the user encryption key to the session file.
    ///
    /// Overwrites any existing session.
    ///
    /// # Arguments
    /// * `uek` - The user encryption key to store
    pub fn unlock(&self, uek: &UsersEncryptionKeys) -> Result<(), SessionError> {
        let encoded = base64::engine::general_purpose::STANDARD.encode(uek.storage);
        store_session_file(encoded.as_bytes())?;
        Ok(())
    }

    /// Check if a session exists (key file exists).
    pub fn is_unlocked(&self) -> bool {
        load_session_file()
            .map(|opt| opt.is_some())
            .unwrap_or(false)
    }

    /// Lock the session by deleting the session file.
    ///
    /// This operation is idempotent - it succeeds even if no session exists.
    pub fn lock(&self) -> Result<(), SessionError> {
        let _ = delete_session_file();
        Ok(())
    }

    /// Get the user encryption key from the session file.
    ///
    /// # Returns
    /// `Ok(Some(UserEncryptionKey))` if a session exists,
    /// `Ok(None)` if no session exists.
    pub fn get_user_encryption_key(&self) -> Result<Option<UsersEncryptionKeys>, SessionError> {
        match load_session_file()? {
            Some(data) => {
                let encoded = String::from_utf8(data).map_err(|_| SessionError::Corrupted)?;
                let bytes = base64::engine::general_purpose::STANDARD
                    .decode(&encoded)
                    .map_err(|_| SessionError::Corrupted)?;

                // Validate key length before copying
                if bytes.len() != 32 {
                    return Err(SessionError::Corrupted);
                }

                let mut key = [0u8; 32];
                key.copy_from_slice(&bytes);
                Ok(Some(UsersEncryptionKeys::new(SigningKey::from_bytes(&key), key, None)))
            }
            None => Ok(None),
        }
    }

    /// Get the config from the session file.
    pub fn get_config(&self) -> Result<&Config, SessionError> {
        Ok(&self.config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domains::keystore::crypto::storage::{SESSION_FILENAME, config_dir, store_session_file};
    use crate::test_utils::FILESTORE_TEST_MUTEX;

    /// Cleanup the session file before and after tests to ensure isolation.
    fn cleanup_session() {
        if let Some(dir) = config_dir() {
            let _ = std::fs::remove_file(dir.join(SESSION_FILENAME));
        }
    }

    #[test]
    fn test_session_unlock_and_get() {
        let _lock = FILESTORE_TEST_MUTEX
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        cleanup_session();

        let session = Session::new(Config::default());
        assert!(!session.is_unlocked(), "Fresh session should be locked");

        let uek = UsersEncryptionKeys::new(SigningKey::from_bytes(&[1u8; 32]), [1u8; 32], None);
        session.unlock(&uek).unwrap();
        assert!(session.is_unlocked(), "Session should be unlocked after unlock");

        let retrieved = session.get_user_encryption_key().unwrap();
        assert!(retrieved.is_some(), "Should retrieve key from unlocked session");
        assert_eq!(
            retrieved.unwrap().storage,
            uek.storage,
            "Retrieved key should match original"
        );

        cleanup_session();
    }

    #[test]
    fn test_session_lock() {
        let _lock = FILESTORE_TEST_MUTEX
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        cleanup_session();

        let session = Session::new(Config::default());
        let uek = UsersEncryptionKeys::new(SigningKey::from_bytes(&[2u8; 32]), [2u8; 32], None);
        session.unlock(&uek).unwrap();
        assert!(session.is_unlocked(), "Session should be unlocked");

        session.lock().unwrap();
        assert!(!session.is_unlocked(), "Session should be locked after lock");
        assert!(
            session.get_user_encryption_key().unwrap().is_none(),
            "Key should not be retrievable after lock"
        );

        cleanup_session();
    }

    #[test]
    fn test_session_get_when_locked() {
        let _lock = FILESTORE_TEST_MUTEX
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        cleanup_session();

        let session = Session::new(Config::default());
        assert!(!session.is_unlocked(), "Fresh session should be locked");

        let result = session.get_user_encryption_key().unwrap();
        assert!(result.is_none(), "Should return None when session is locked");

        cleanup_session();
    }

    #[test]
    fn test_session_corrupted_data() {
        let _lock = FILESTORE_TEST_MUTEX
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        cleanup_session();

        // Store invalid data (not valid base64)
        store_session_file(b"not-valid-base64!!!").unwrap();

        let session = Session::new(Config::default());
        let result = session.get_user_encryption_key();
        assert!(
            matches!(result, Err(SessionError::Corrupted)),
            "Should return Corrupted error for invalid base64"
        );

        cleanup_session();
    }

    #[test]
    fn test_session_wrong_length() {
        let _lock = FILESTORE_TEST_MUTEX
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        cleanup_session();

        // Store valid base64 but wrong length (16 bytes instead of 32)
        let short = base64::engine::general_purpose::STANDARD.encode([0u8; 16]);
        store_session_file(short.as_bytes()).unwrap();

        let session = Session::new(Config::default());
        let result = session.get_user_encryption_key();
        assert!(
            matches!(result, Err(SessionError::Corrupted)),
            "Should return Corrupted error for wrong key length"
        );

        cleanup_session();
    }

    #[test]
    fn test_session_lock_idempotent() {
        let _lock = FILESTORE_TEST_MUTEX
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        cleanup_session();

        let session = Session::new(Config::default());
        assert!(!session.is_unlocked(), "Fresh session should be locked");

        // Lock when already locked should succeed (idempotent)
        session.lock().unwrap();
        session.lock().unwrap();
        assert!(!session.is_unlocked(), "Session should remain locked");

        cleanup_session();
    }
}
