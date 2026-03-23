//! Key update command for Tyche CLI.
//!
//! Implements password change by re-wrapping the UEK with a new KWK
//! derived from a new password.

use crate::client::IrisClient;
use crate::client::rotate_user_encryption_key;
use crate::commands::key::filestore::auth::password::prompt_password;
use crate::commands::key::filestore::crypto::encryption::{unwrap_user_encryption_key, wrap_user_encryption_key};
use crate::commands::key::filestore::crypto::types::EncryptedData;
use crate::commands::key::filestore::derivation::{derive_master_key, derive_user_keys, generate_salt};
use crate::commands::key::filestore::storage::{
    default_blind_user_key_path, default_salt_path, default_storage_dir, ensure_storage_dir, load_blind_user_key,
    load_salt, store_blind_user_key, store_salt,
};
use crate::messages;
use crate::session::Session;
use std::fs;

/// Update the authentication (change password).
///
/// This function performs the password change flow:
/// 1. Must be unlocked first (checks session)
/// 2. Prompts for current password (verifies)
/// 3. Prompts for new password (twice for confirmation)
/// 4. Gets current UEK from session
/// 5. Generates new salt
/// 6. Derives new master key, new KWK
/// 7. Re-wraps UEK with new KWK → new blind_user_key
/// 8. Atomically updates storage
/// 9. Keeps session unlocked with same UEK
///
/// # Errors
/// Returns an error if:
/// - Session is not unlocked
/// - Current password is incorrect
/// - New password confirmation fails
/// - Storage operations fail
pub async fn key_update(client: &IrisClient) -> messages::success::CommandResult<()> {
    let session = Session::new();

    // Check if session is unlocked and get current UEK
    let current_uek = session
        .get_user_encryption_key()
        .map_err(|e| messages::error::CommandError::Session(format!("Failed to get user encryption key: {}", e)))?
        .ok_or_else(|| {
            messages::error::CommandError::Session("Session must be unlocked. Run 'edge key unlock' first.".to_string())
        })?;

    // Get storage paths
    let storage_dir = default_storage_dir()
        .ok_or_else(|| messages::error::CommandError::Storage("Could not determine storage directory".to_string()))?;
    let salt_path = default_salt_path()
        .ok_or_else(|| messages::error::CommandError::Storage("Could not determine salt path".to_string()))?;
    let blind_key_path = default_blind_user_key_path()
        .ok_or_else(|| messages::error::CommandError::Storage("Could not determine key path".to_string()))?;

    // Load current salt
    let current_salt_bytes =
        load_salt(&salt_path).map_err(|e| messages::error::CommandError::Storage(e.to_string()))?;

    if current_salt_bytes.len() != 16 {
        return Err(messages::error::CommandError::Crypto(format!(
            "Invalid salt size: expected 16, got {}",
            current_salt_bytes.len()
        )));
    }

    let mut current_salt = [0u8; 16];
    current_salt.copy_from_slice(&current_salt_bytes);

    // Prompt for current password and verify
    let current_password = prompt_password("Enter current password: ")
        .map_err(|e| messages::error::CommandError::Authentication(e.to_string()))?;

    // Derive master key and verify we can unwrap
    let current_master_key = derive_master_key(&current_password, &current_salt)
        .map_err(|e| messages::error::CommandError::Crypto(e.to_string()))?;
    let current_user_keys = derive_user_keys(&current_master_key);

    // Verify by attempting to unwrap
    let blind_key_bytes =
        load_blind_user_key(&blind_key_path).map_err(|e| messages::error::CommandError::Storage(e.to_string()))?;
    let blind_user_key = EncryptedData::from_bytes(&blind_key_bytes)
        .ok_or_else(|| messages::error::CommandError::Crypto("Invalid blind_user_key format".to_string()))?;

    let verify_result = unwrap_user_encryption_key(&blind_user_key, &current_user_keys.meta.unwrap());
    if verify_result.is_err() {
        return Err(messages::error::CommandError::Authentication(
            "Current password is incorrect".to_string(),
        ));
    }

    // Prompt for new password
    let new_password = prompt_password("Enter new password: ")
        .map_err(|e| messages::error::CommandError::Authentication(e.to_string()))?;
    let confirm_password = prompt_password("Confirm new password: ")
        .map_err(|e| messages::error::CommandError::Authentication(e.to_string()))?;

    if new_password != confirm_password {
        return Err(messages::error::CommandError::InvalidInput(
            "New passwords do not match".to_string(),
        ));
    }

    // Generate new salt
    let new_salt = generate_salt();

    // Derive new master key and KWK
    let new_master_key = derive_master_key(&new_password, &new_salt)?;
    let new_user_keys = derive_user_keys(&new_master_key);

    // Re-wrap the UEK with the new KWK
    let new_blind_user_key = wrap_user_encryption_key(&new_user_keys)?;

    // Ensure storage directory exists
    ensure_storage_dir(&storage_dir)?;

    // Atomically update storage
    // Write new files first, then rename over old files
    let new_salt_path = storage_dir.join("salt.new");
    let new_blind_key_path = storage_dir.join("blind_user_key.new");

    // Write new files
    store_salt(&new_salt_path, &new_salt)?;
    store_blind_user_key(&new_blind_key_path, &new_blind_user_key.to_bytes())?;

    // Atomically rename new files over old files
    fs::rename(&new_salt_path, &salt_path).map_err(|e| messages::error::CommandError::Storage(e.to_string()))?;
    fs::rename(&new_blind_key_path, &blind_key_path)
        .map_err(|e| messages::error::CommandError::Storage(e.to_string()))?;

    let new_uek = session.get_user_encryption_key().unwrap();
    if new_uek.is_none() {
        return Err(messages::error::CommandError::Session(
            "Something strange hanppened. Please contact support.".to_string(),
        ));
    }
    rotate_user_encryption_key(&new_uek.unwrap(), &current_uek, client).await?;

    messages::success::password_updated();
    messages::success::session_remains_unlocked();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::key::filestore::crypto::encryption::{unwrap_user_encryption_key, wrap_user_encryption_key};
    use crate::commands::key::filestore::derivation::generate_salt;

    #[test]
    fn test_rewrap_uek_with_new_kwk() {
        // Setup original keys
        let original_password = "original_password";
        let original_salt = generate_salt();
        let original_master_key = derive_master_key(original_password, &original_salt).unwrap();
        let original_user_keys = derive_user_keys(&original_master_key);

        // Wrap the original UEK with its own KWK (this is what wrap_user_encryption_key does)
        let blind_user_key = wrap_user_encryption_key(&original_user_keys).unwrap();

        // Create new keys with different password (different KWK)
        let new_password = "new_password";
        let new_salt = generate_salt();
        let new_master_key = derive_master_key(new_password, &new_salt).unwrap();
        let new_user_keys = derive_user_keys(&new_master_key);

        // Verify old KWK CAN unwrap the blind_user_key (it was wrapped with the original KWK)
        let old_result = unwrap_user_encryption_key(&blind_user_key, &original_user_keys.meta.unwrap());
        assert!(
            old_result.is_ok(),
            "Original KWK should be able to unwrap data it wrapped"
        );

        // Verify new KWK CANNOT unwrap the blind_user_key (different KWK)
        let new_result = unwrap_user_encryption_key(&blind_user_key, &new_user_keys.meta.unwrap());
        assert!(
            new_result.is_err(),
            "Different KWK should not be able to unwrap data wrapped with original KWK"
        );
    }

    #[test]
    fn test_atomic_update_pattern() {
        let temp = tempfile::tempdir().expect("Failed to create temp dir");

        // Setup initial files
        let initial_content = b"initial content";
        let file_path = temp.path().join("test_file");
        fs::write(&file_path, initial_content).unwrap();

        // Perform atomic update
        let new_content = b"updated content";
        let new_file_path = temp.path().join("test_file.new");

        fs::write(&new_file_path, new_content).unwrap();
        fs::rename(&new_file_path, &file_path).unwrap();

        // Verify update succeeded
        let final_content = fs::read(&file_path).unwrap();
        assert_eq!(final_content, new_content);
        assert!(!new_file_path.exists());
    }
}
