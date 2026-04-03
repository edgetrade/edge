//! Desktop key delete command - keyring only.
//!
//! Removes the user encryption key from the OS keyring.
//! This operation is idempotent - it succeeds even if no key exists.

use crate::domains::keystore::session::keyring::{KEYRING_SERVICE, KEYRING_USERNAME};
use crate::error::PoseidonError;
use crate::messages;

/// Delete the key from the OS keyring.
///
/// This function:
/// 1. Prompts the user for confirmation
/// 2. Removes the user encryption key from the OS keyring if confirmed
/// 3. Prints success message
///
/// This operation is idempotent - it succeeds even if no key exists.
///
/// # Errors
/// Returns an error if:
/// - Keyring is inaccessible
pub fn keyring_delete() -> crate::error::Result<()> {
    let check = rpassword::prompt_password(
        "If you have not saved your password, you WILL lose access to your wallets.\nAre you sure you want to delete? (y/N) ",
    )
    .map_err(|e| PoseidonError::InvalidInput(format!("Failed to read confirmation: {}", e)))?;
    let check_trimmed = check.trim();
    if !check_trimmed.to_lowercase().starts_with("y") {
        return Ok(());
    }

    keyring_delete_internal()
}

/// Delete the key from the OS keyring (internal function without prompt).
///
/// This function:
/// 1. Removes the user encryption key from the OS keyring
/// 2. Prints success message
///
/// This operation is idempotent - it succeeds even if no key exists.
///
/// # Errors
/// Returns an error if:
/// - Keyring is inaccessible
fn keyring_delete_internal() -> crate::error::Result<()> {
    let entry = keyring::Entry::new(KEYRING_SERVICE, KEYRING_USERNAME)
        .map_err(|e| PoseidonError::Storage(format!("Failed to access keyring: {}", e)))?;

    match entry.delete_credential() {
        Ok(()) => {
            messages::success::key_config_deleted();
            Ok(())
        }
        Err(keyring::Error::NoEntry) => {
            messages::success::key_config_not_found();
            Ok(())
        }
        Err(e) => Err(PoseidonError::Storage(format!("Failed to delete key: {}", e))),
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
    fn test_keyring_delete_when_already_deleted() {
        let _lock = KEYRING_TEST_MUTEX
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        cleanup_keyring();
        let _guard = CleanupGuard;

        // Delete - should succeed (idempotent)
        let result = keyring_delete_internal();
        assert!(result.is_ok(), "Delete should succeed even when no key exists");
    }

    #[test]
    fn test_keyring_delete_idempotent() {
        let _lock = KEYRING_TEST_MUTEX
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        cleanup_keyring();
        let _guard = CleanupGuard;

        // Delete when never existed
        let result1 = keyring_delete_internal();
        assert!(result1.is_ok(), "First delete should succeed");

        // Delete again
        let result2 = keyring_delete_internal();
        assert!(result2.is_ok(), "Second delete should succeed");
    }

    #[test]
    fn test_keyring_delete_removes_key() {
        use crate::domains::config::Config;
        use crate::domains::keystore::KeyringSession;
        use crate::domains::keystore::UsersEncryptionKeys;
        use ed25519_dalek::SigningKey;

        let _lock = KEYRING_TEST_MUTEX
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        cleanup_keyring();
        let _guard = CleanupGuard;

        // Setup: store a key
        let session = KeyringSession::new(Config::default());
        let key = UsersEncryptionKeys::new(SigningKey::from_bytes(&[0x42u8; 32]), [0x42u8; 32], None);
        session.save(&key, false).unwrap();
        assert!(session.is_unlocked(), "Key should exist before deletion");

        // Delete
        let result = keyring_delete_internal();
        assert!(result.is_ok(), "Delete should succeed");

        // Verify key is gone
        assert!(!session.is_unlocked(), "Key should not exist after deletion");
    }
}
