//! Desktop key update command - keyring only.
//!
//! Generates a new user encryption key and replaces the existing one
//! in the OS keyring. No password prompts, no file storage.

use crate::client::{IrisClient, rotate_user_encryption_key};
use crate::commands::key::keyring::keyring_create;
use crate::messages;
use crate::session::KeyringSession as Session;

// TODO: trigger the rotate key operation in tyche

/// Update the key by generating a new one.
///
/// This function:
/// 1. Checks if a key exists in the keyring
/// 2. Generates a new random 32-byte UserEncryptionKey
/// 3. Replaces the existing key in the OS keyring
/// 4. Prints success message
///
/// # Errors
/// Returns an error if:
/// - No existing key exists (must create first)
/// - Key generation fails
/// - Keyring is inaccessible
pub async fn keyring_update(client: &IrisClient) -> messages::success::CommandResult<()> {
    let session = Session::new();

    // Check if key exists first
    if !session.is_unlocked() {
        return Err(messages::error::CommandError::Session(
            "No key found. Run 'edge key create' first.".to_string(),
        ));
    }

    let old = session.get_user_encryption_key().unwrap();
    if old.is_none() {
        return Err(messages::error::CommandError::Session(
            "No key found. Run 'edge key create' first.".to_string(),
        ));
    }

    let old_uek = old.unwrap();

    keyring_create()?;

    let new = session.get_user_encryption_key().unwrap();
    if new.is_none() {
        return Err(messages::error::CommandError::Session(
            "No key found. Run 'edge key create' first.".to_string(),
        ));
    }

    let new_uek = new.unwrap();
    rotate_user_encryption_key(&new_uek, &old_uek, client)
        .await
        .map_err(|e| messages::error::CommandError::Session(e.to_string()))?;

    messages::success::key_updated();
    Ok(())
}

#[cfg(all(test, feature = "keyring-tests"))]
mod tests {
    use super::*;
    use crate::session::KeyringSession;
    use crate::session::keyring::{KEYRING_SERVICE, KEYRING_USERNAME};
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
    fn test_keyring_update_success() {
        let _lock = KEYRING_TEST_MUTEX
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        cleanup_keyring();
        let _guard = CleanupGuard;

        // Setup: create initial key
        let session = KeyringSession::new();
        let initial_key = UserEncryptionKey([0x42u8; 32]);
        session.save(&initial_key, false).unwrap();

        // Update
        let result = keyring_update();
        assert!(result.is_ok(), "Update should succeed: {:?}", result.err());

        // Verify key changed
        let new_key = session
            .get()
            .unwrap()
            .expect("Key should exist after update");
        assert_ne!(new_key.0, initial_key.0, "Key should be different after update");
    }

    #[test]
    fn test_keyring_update_without_existing_key() {
        let _lock = KEYRING_TEST_MUTEX
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        cleanup_keyring();
        let _guard = CleanupGuard;

        // Update without creating first should fail
        let result = keyring_update();
        assert!(
            matches!(result, Err(messages::error::CommandError::Session(_))),
            "Should fail when no key exists"
        );
    }

    #[test]
    fn test_keyring_update_generates_valid_key() {
        let _lock = KEYRING_TEST_MUTEX
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        cleanup_keyring();
        let _guard = CleanupGuard;

        // Setup: create initial key
        let session = KeyringSession::new();
        let initial_key = UserEncryptionKey([0x42u8; 32]);
        session.save(&initial_key, false).unwrap();

        // Update
        keyring_update().unwrap();

        // Verify new key is valid
        let new_key = session.get().unwrap().expect("Key should exist");
        assert_eq!(new_key.0.len(), 32, "Key should be 32 bytes");

        // Key should be different from initial
        assert_ne!(new_key.0, [0x42u8; 32], "Key should be randomly generated");
    }

    #[test]
    fn test_keyring_update_multiple_times() {
        let _lock = KEYRING_TEST_MUTEX
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        cleanup_keyring();
        let _guard = CleanupGuard;

        // Setup
        let session = KeyringSession::new();
        let key1 = UserEncryptionKey([0x42u8; 32]);
        session.save(&key1, false).unwrap();

        // Update multiple times
        keyring_update().unwrap();
        let after_first = session.get().unwrap().unwrap();

        keyring_update().unwrap();
        let after_second = session.get().unwrap().unwrap();

        // Each update should produce a different key
        assert_ne!(after_first.0, after_second.0, "Each update should generate a new key");
    }
}
