//! Key lock command for Tyche CLI.
//!
//! Implements session lock by clearing session keys from the OS keyring.

use crate::messages;
use crate::session::Session;

/// Lock the session.
///
/// This function performs the lock flow:
/// 1. Checks if session is unlocked via Session::is_unlocked()
/// 2. If unlocked: calls Session::lock(), prints success
/// 3. If already locked: prints info message
///
/// All sensitive key material is removed from the keyring when locked.
///
/// # Errors
/// Returns an error if:
/// - The session cannot be accessed (should not happen in normal operation)
pub fn key_lock() -> messages::success::CommandResult<()> {
    let session = Session::new();

    if session.is_unlocked() {
        session
            .lock()
            .map_err(|e| messages::error::CommandError::Storage(e.to_string()))?;
        messages::success::session_locked();
    } else {
        messages::success::session_already_locked();
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use ed25519_dalek::SigningKey;

    use crate::commands::key::filestore::storage::delete_session_file;
    use crate::session::FileStoreSession;
    use crate::session::crypto::UsersEncryptionKeys;
    use crate::test_utils::FILESTORE_TEST_MUTEX;

    /// Cleanup any existing session file before/after tests
    fn cleanup_session_file() {
        let _ = delete_session_file();
    }

    #[test]
    fn test_lock_unlocked_session() {
        let _lock = FILESTORE_TEST_MUTEX
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        cleanup_session_file();

        // Use explicit file-based session for consistent test behavior
        let session = FileStoreSession::new();

        // First unlock with a test key
        let test_key = UsersEncryptionKeys::new(SigningKey::from_bytes(&[0x42u8; 32]), [0x42u8; 32], None);
        session.unlock(&test_key).expect("unlock should succeed");
        assert!(session.is_unlocked());

        // Now lock it
        session.lock().expect("lock should succeed");
        assert!(!session.is_unlocked());

        cleanup_session_file();
    }

    #[test]
    fn test_lock_already_locked_session() {
        let _lock = FILESTORE_TEST_MUTEX
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        cleanup_session_file();

        // Use explicit file-based session for consistent test behavior
        let session = FileStoreSession::new();

        // Ensure it's locked (fresh session should be locked)
        assert!(!session.is_unlocked());

        // Locking an already locked session should succeed (idempotent)
        session
            .lock()
            .expect("lock should succeed on already locked");
        assert!(!session.is_unlocked());

        cleanup_session_file();
    }

    #[test]
    fn test_is_unlocked_after_unlock() {
        let _lock = FILESTORE_TEST_MUTEX
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        cleanup_session_file();

        // Use explicit file-based session for consistent test behavior
        let session = FileStoreSession::new();

        // Initially locked
        assert!(!session.is_unlocked());

        // Unlock
        let test_key = UsersEncryptionKeys::new(SigningKey::from_bytes(&[0x42u8; 32]), [0x42u8; 32], None);
        session.unlock(&test_key).expect("unlock should succeed");
        assert!(session.is_unlocked());

        // Cleanup
        session.lock().expect("lock should succeed");
        assert!(!session.is_unlocked());

        cleanup_session_file();
    }
}
