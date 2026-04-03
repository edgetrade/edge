//! Desktop session management using OS keyring only.
//!
//! This module provides session storage exclusively via the OS keyring.
//! There is NO file fallback - operations fail if the keyring is unavailable.

use std::sync::Arc;

use ed25519_dalek::SigningKey;

use crate::domains::config::Config;

use super::crypto::UsersEncryptionKeys;

/// Re-export keyring Entry for convenience
pub use keyring::Entry;

/// Keyring service name for storing the user encryption key.
pub const KEYRING_SERVICE: &str = "edge";
/// Keyring username for storing the user encryption key.
pub const KEYRING_USERNAME: &str = "user-encryption-key";

/// Error type for desktop session operations.
#[derive(Debug, Clone, thiserror::Error)]
pub enum SessionError {
    #[error("Keyring error: {0}")]
    Keyring(String),
    #[error("Session not found")]
    NotFound,
    #[error("Session corrupted")]
    Corrupted,
}

impl From<keyring::Error> for SessionError {
    fn from(e: keyring::Error) -> Self {
        SessionError::Keyring(e.to_string())
    }
}

/// Desktop session manager using OS keyring exclusively.
///
/// This struct provides operations to save, retrieve, and delete
/// the user encryption key from the OS keyring. There is no file
/// fallback - all operations require a functional OS keyring.
#[derive(Debug, Clone)]
pub struct Session {
    _private: (),

    /// Config
    config: Arc<Config>,
}

impl Session {
    /// Create a new desktop session manager.
    pub fn new(config: Config) -> Self {
        Self {
            _private: (),
            config: Arc::new(config),
        }
    }

    /// Get the keyring entry for the user encryption key.
    fn get_entry(&self) -> Result<keyring::Entry, SessionError> {
        Ok(keyring::Entry::new(KEYRING_SERVICE, KEYRING_USERNAME)?)
    }

    /// Check if the session is unlocked (key exists in keyring).
    ///
    /// Get the user encryption key from the session.
    ///
    /// # Returns
    /// `Ok(Some(UsersEncryptionKeys))` if a key exists,
    /// `Ok(None)` if no key is stored.
    pub fn get_user_encryption_key(&self) -> Result<Option<UsersEncryptionKeys>, SessionError> {
        self.get()
    }

    /// Get the agent ID from the session.
    pub fn get_config(&self) -> Result<&Config, SessionError> {
        Ok(&self.config)
    }

    /// Save the user encryption key to the keyring.
    ///
    /// Overwrites any existing key.
    ///
    /// # Arguments
    /// * `uek` - The user encryption key to store
    ///
    /// # Returns
    /// `Ok(())` on success, or `SessionError` on failure.
    pub fn save(&self, uek: &UsersEncryptionKeys, overwrite: bool) -> Result<(), SessionError> {
        if self.is_unlocked() && !overwrite {
            return Ok(());
        }

        let entry = self.get_entry()?;
        entry.set_secret(&uek.storage)?;

        Ok(())
    }

    /// Noop
    ///
    /// This stores the UEK in the keyring. For desktop, this is equivalent to save().
    ///
    /// # Arguments
    /// * `uek` - The user encryption key to store
    pub fn unlock(&self, uek: &UsersEncryptionKeys) -> Result<(), SessionError> {
        self.save(uek, false)
    }

    /// Check if the session is unlocked (key exists).
    pub fn is_unlocked(&self) -> bool {
        self.check_unlocked().unwrap_or(false)
    }

    /// Internal method to check if a key exists.
    fn check_unlocked(&self) -> Result<bool, SessionError> {
        let entry = self.get_entry()?;
        match entry.get_secret() {
            Ok(_) => Ok(true),
            Err(keyring::Error::NoEntry) => Ok(false),
            Err(e) => Err(e.into()),
        }
    }

    /// Lock the session by deleting the stored key.
    ///
    /// This removes the user encryption key from the keyring.
    /// The operation is idempotent - it succeeds even if no key exists.
    ///
    /// # Returns
    /// `Ok(())` on success, or `SessionError` on failure.
    pub fn lock(&self) -> Result<(), SessionError> {
        let entry = self.get_entry()?;
        // Ignore NoEntry errors (already locked)
        match entry.delete_credential() {
            Ok(()) | Err(keyring::Error::NoEntry) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }

    /// Change the stored user encryption key.
    ///
    /// This is equivalent to `save()` but semantically indicates
    /// an update to an existing session.
    ///
    /// # Arguments
    /// * `uek` - The new user encryption key to store
    ///
    /// # Returns
    /// `Ok(())` on success, or `SessionError` on failure.
    pub fn change(&self, uek: &UsersEncryptionKeys) -> Result<(), SessionError> {
        self.save(uek, true)
    }

    /// Retrieve the user encryption key from the keyring.
    ///
    /// # Returns
    /// `Ok(Some(UsersEncryptionKeys))` if a key exists,
    /// `Ok(None)` if no key is stored,
    /// or `SessionError` if retrieval fails or data is corrupted.
    pub fn get(&self) -> Result<Option<UsersEncryptionKeys>, SessionError> {
        let entry = self.get_entry()?;

        match entry.get_secret() {
            Ok(bytes) => {
                if bytes.len() != 32 {
                    return Err(SessionError::Corrupted);
                }
                let mut key = [0u8; 32];
                key.copy_from_slice(&bytes);
                Ok(Some(UsersEncryptionKeys::new(SigningKey::from_bytes(&key), key, None)))
            }
            Err(keyring::Error::NoEntry) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }
}

#[cfg(all(test, feature = "keyring-tests"))]
mod tests {
    use super::*;
    use crate::test_utils::KEYRING_TEST_MUTEX;

    /// Cleans up the keyring entry. Panics if keyring is inaccessible.
    fn cleanup_keyring() {
        let entry =
            keyring::Entry::new(KEYRING_SERVICE, KEYRING_USERNAME).expect("Failed to access keyring for cleanup");
        let _ = entry.delete_credential();
    }

    /// Guard that ensures cleanup runs even if a test panics.
    struct CleanupGuard;

    impl Drop for CleanupGuard {
        fn drop(&mut self) {
            cleanup_keyring();
        }
    }

    #[test]
    fn test_new_session_is_locked() {
        let _lock = KEYRING_TEST_MUTEX
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        cleanup_keyring();
        let _guard = CleanupGuard;

        let config = Config::default();
        let session = Session::new(config);
        assert!(!session.is_unlocked());
        assert!(session.get().unwrap().is_none());
    }

    #[test]
    fn test_save_and_get() {
        let _lock = KEYRING_TEST_MUTEX
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        cleanup_keyring();
        let _guard = CleanupGuard;

        let config = Config::default();
        let session = Session::new(config);
        let key = UsersEncryptionKeys::new(SigningKey::from_bytes(&[0x42u8; 32]), [0x42u8; 32], None);

        // Save key
        session.save(&key, false).unwrap();
        assert!(session.is_unlocked());

        // Retrieve key
        let retrieved = session.get().unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().storage, [0x42u8; 32]);
    }

    #[test]
    fn test_lock() {
        let _lock = KEYRING_TEST_MUTEX
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        cleanup_keyring();
        let _guard = CleanupGuard;

        let config = Config::default();
        let session = Session::new(config);
        let key = UsersEncryptionKeys::new(SigningKey::from_bytes(&[0x42u8; 32]), [0x42u8; 32], None);

        // Save and verify
        session.save(&key, false).unwrap();
        assert!(session.is_unlocked());

        // Lock
        session.lock().unwrap();
        assert!(!session.is_unlocked());
        assert!(session.get().unwrap().is_none());
    }

    #[test]
    fn test_change_updates_key() {
        let _lock = KEYRING_TEST_MUTEX
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        cleanup_keyring();
        let _guard = CleanupGuard;

        let config = Config::default();
        let session = Session::new(config);
        let key1 = UsersEncryptionKeys::new(SigningKey::from_bytes(&[0x42u8; 32]), [0x42u8; 32], None);
        let key2 = UsersEncryptionKeys::new(SigningKey::from_bytes(&[0xABu8; 32]), [0xABu8; 32], None);

        // Save initial key
        session.save(&key1, false).unwrap();

        // Change to new key
        session.change(&key2).unwrap();

        // Verify new key
        let retrieved = session.get().unwrap();
        assert_eq!(retrieved.unwrap().storage, [0xABu8; 32]);
    }

    #[test]
    fn test_save_overwrites_existing() {
        let _lock = KEYRING_TEST_MUTEX
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        cleanup_keyring();
        let _guard = CleanupGuard;

        let config = Config::default();
        let session = Session::new(config);
        let key1 = UsersEncryptionKeys::new(SigningKey::from_bytes(&[0x42u8; 32]), [0x42u8; 32], None);
        let key2 = UsersEncryptionKeys::new(SigningKey::from_bytes(&[0xCDu8; 32]), [0xCDu8; 32], None);

        session.save(&key1, false).unwrap();
        session.save(&key2, true).unwrap();

        let retrieved = session.get().unwrap();
        assert_eq!(retrieved.unwrap().storage, [0xCDu8; 32]);
    }

    #[test]
    fn test_lock_idempotent() {
        let _lock = KEYRING_TEST_MUTEX
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        cleanup_keyring();
        let _guard = CleanupGuard;

        let config = Config::default();
        let session = Session::new(config);

        // Lock when already locked should succeed
        session.lock().unwrap();
        assert!(!session.is_unlocked());
    }

    #[test]
    fn test_unlock_method() {
        let _lock = KEYRING_TEST_MUTEX
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        cleanup_keyring();
        let _guard = CleanupGuard;

        let config = Config::default();
        let session = Session::new(config);
        let key = UsersEncryptionKeys::new(SigningKey::from_bytes(&[0x42u8; 32]), [0x42u8; 32], None);

        // Unlock should save the key
        session.unlock(&key).unwrap();
        assert!(session.is_unlocked());

        // Verify key was stored correctly
        let retrieved = session.get().unwrap();
        assert_eq!(retrieved.unwrap().storage, [0x42u8; 32]);
    }

    #[test]
    fn test_unlock_is_noop_when_already_unlocked() {
        let _lock = KEYRING_TEST_MUTEX
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        cleanup_keyring();
        let _guard = CleanupGuard;

        let config = Config::default();
        let session = Session::new(config);
        let key1 = UsersEncryptionKeys::new(SigningKey::from_bytes(&[0x42u8; 32]), [0x42u8; 32], None);
        let key2 = UsersEncryptionKeys::new(SigningKey::from_bytes(&[0xABu8; 32]), [0xABu8; 32], None);

        // Save initial key via unlock
        session.unlock(&key1).unwrap();
        assert!(session.is_unlocked());

        // Try to unlock with different key (should be noop)
        session.unlock(&key2).unwrap();

        // Original key should still be there
        let retrieved = session.get().unwrap();
        assert_eq!(retrieved.unwrap().storage, [0x42u8; 32]);
    }

    #[test]
    fn test_save_without_overwrite_preserves_existing() {
        let _lock = KEYRING_TEST_MUTEX
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        cleanup_keyring();
        let _guard = CleanupGuard;

        let config = Config::default();
        let session = Session::new(config);
        let key1 = UsersEncryptionKeys::new(SigningKey::from_bytes(&[0x42u8; 32]), [0x42u8; 32], None);
        let key2 = UsersEncryptionKeys::new(SigningKey::from_bytes(&[0xABu8; 32]), [0xABu8; 32], None);

        // Save initial key
        session.save(&key1, false).unwrap();

        // Try to save with overwrite=false (should be noop)
        session.save(&key2, false).unwrap();

        // Original key should still be there
        let retrieved = session.get().unwrap();
        assert_eq!(retrieved.unwrap().storage, [0x42u8; 32]);
    }

    #[test]
    fn test_get_user_encryption_key_alias() {
        let _lock = KEYRING_TEST_MUTEX
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        cleanup_keyring();
        let _guard = CleanupGuard;

        let config = Config::default();
        let session = Session::new(config);
        let key = UsersEncryptionKeys::new(SigningKey::from_bytes(&[0x42u8; 32]), [0x42u8; 32], None);

        // Save key
        session.save(&key, false).unwrap();

        // get_user_encryption_key should be equivalent to get()
        let retrieved = session.get_user_encryption_key().unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().storage, [0x42u8; 32]);
    }
}
