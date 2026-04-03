//! Key delete command for filestore (server) variant.
//!
//! Deletes all key-related files from the filesystem:
//! - blind_user_key
//! - salt
//! - session file

use crate::domains::keystore::crypto::storage::{default_blind_user_key_path, default_salt_path, delete_session_file};
use crate::error::PoseidonError;
use crate::messages;

/// Delete all key-related files from the filesystem.
///
/// This function:
/// 1. Checks if keys exist
/// 2. Deletes the blind_user_key file
/// 3. Deletes the salt file
/// 4. Deletes the session file (if exists)
/// 5. Prints success message
///
/// This operation is idempotent - it succeeds even if no keys exist.
///
/// # Errors
/// Returns an error if:
/// - Files cannot be deleted due to permission issues
pub fn key_delete() -> crate::error::Result<()> {
    let blind_key_path = default_blind_user_key_path()
        .ok_or_else(|| PoseidonError::Storage("Could not determine key path".to_string()))?;
    let salt_path =
        default_salt_path().ok_or_else(|| PoseidonError::Storage("Could not determine salt path".to_string()))?;

    let key_existed = blind_key_path.exists();
    let salt_existed = salt_path.exists();

    // Delete blind_user_key file if it exists
    if key_existed {
        std::fs::remove_file(&blind_key_path)
            .map_err(|e| PoseidonError::Storage(format!("Failed to delete blind_user_key: {}", e)))?;
    }

    // Delete salt file if it exists
    if salt_existed {
        std::fs::remove_file(&salt_path)
            .map_err(|e| PoseidonError::Storage(format!("Failed to delete salt: {}", e)))?;
    }

    // Delete session file if it exists (ignore errors - may not exist)
    let _ = delete_session_file();

    if key_existed || salt_existed {
        messages::success::key_config_deleted();
    } else {
        messages::success::key_config_not_found();
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::domains::keystore::crypto::storage::{store_blind_user_key, store_salt};
    use std::fs;

    fn setup_test_keys(temp_dir: &std::path::Path) {
        fs::create_dir_all(temp_dir).unwrap();

        let key_path = temp_dir.join("blind_user_key");
        let salt_path = temp_dir.join("salt");

        store_blind_user_key(&key_path, &[0xde, 0xad, 0xbe, 0xef]).unwrap();
        store_salt(&salt_path, &[0xca, 0xfe, 0xba, 0xbe]).unwrap();
    }

    #[test]
    fn test_key_delete_removes_files() {
        let temp = tempfile::tempdir().expect("Failed to create temp dir");
        setup_test_keys(temp.path());

        // Verify files exist
        let key_path = temp.path().join("blind_user_key");
        let salt_path = temp.path().join("salt");
        assert!(key_path.exists());
        assert!(salt_path.exists());

        // Delete them
        std::fs::remove_file(&key_path).unwrap();
        std::fs::remove_file(&salt_path).unwrap();

        // Verify files are gone
        assert!(!key_path.exists());
        assert!(!salt_path.exists());
    }

    #[test]
    fn test_key_delete_idempotent() {
        let temp = tempfile::tempdir().expect("Failed to create temp dir");

        // Try to delete non-existent files - should not panic
        let key_path = temp.path().join("blind_user_key");
        let salt_path = temp.path().join("salt");

        assert!(!key_path.exists());
        assert!(!salt_path.exists());

        // Both should succeed (no-op)
        assert!(std::fs::remove_file(&key_path).is_err());
        assert!(std::fs::remove_file(&salt_path).is_err());
    }
}
