//! Key create command for Tyche CLI.
//!
//! Implements interactive setup for new users, generating salt,
//! deriving keys, and storing the blind_user_key.

use std::path::PathBuf;

use crate::commands::key::filestore::auth::password::prompt_password;
use crate::commands::key::filestore::crypto::encryption::wrap_user_encryption_key;
use crate::commands::key::filestore::derivation::{derive_master_key, derive_user_keys, generate_salt};
use crate::commands::key::filestore::storage::{
    default_blind_user_key_path, default_salt_path, default_storage_dir, ensure_storage_dir, store_blind_user_key,
    store_salt,
};
use crate::messages;

/// Internal function containing the common key creation logic.
///
/// This function performs the core key creation work:
/// 1. Checks that keys don't already exist (idempotent protection)
/// 2. Prompts for password (twice for confirmation)
/// 3. Generates random salt
/// 4. Derives master key from password + salt
/// 5. Derives UEK and KWK from master key
/// 6. Wraps UEK with KWK to create blind_user_key
/// 7. Stores blind_user_key and salt to ~/.tyche/
///
/// Returns the storage directory for success message customization.
///
/// # Errors
/// Returns an error if:
/// - Keys already exist (idempotent protection)
/// - Password confirmation fails
/// - Key derivation fails
/// - Storage operations fail
fn key_create_internal() -> messages::success::CommandResult<PathBuf> {
    // Check if keys already exist
    if check_keys_exist()? {
        return Err(messages::error::CommandError::AlreadyExists);
    }

    // Ensure storage directory exists
    let storage_dir = default_storage_dir()
        .ok_or_else(|| messages::error::CommandError::Storage("Could not determine home directory".to_string()))?;
    ensure_storage_dir(&storage_dir).map_err(|e| messages::error::CommandError::Storage(e.to_string()))?;

    // Prompt for password and confirmation
    let password = prompt_password("Create password: ")
        .map_err(|e| messages::error::CommandError::Authentication(e.to_string()))?;
    let confirm = prompt_password("Confirm password: ")
        .map_err(|e| messages::error::CommandError::Authentication(e.to_string()))?;

    if password != confirm {
        return Err(messages::error::CommandError::InvalidInput(
            "Passwords do not match".to_string(),
        ));
    }

    // Generate random salt
    let salt = generate_salt();

    // Derive master key from password + salt
    let master_key =
        derive_master_key(&password, &salt).map_err(|e| messages::error::CommandError::Crypto(e.to_string()))?;

    // Derive UEK and KWK from master key
    let user_keys = derive_user_keys(&master_key);

    // Wrap UEK with KWK to create blind_user_key
    let blind_user_key =
        wrap_user_encryption_key(&user_keys).map_err(|e| messages::error::CommandError::Crypto(e.to_string()))?;

    // Get storage paths
    let blind_key_path = default_blind_user_key_path()
        .ok_or_else(|| messages::error::CommandError::Storage("Could not determine key path".to_string()))?;
    let salt_path = default_salt_path()
        .ok_or_else(|| messages::error::CommandError::Storage("Could not determine salt path".to_string()))?;

    // Store blind_user_key and salt
    store_blind_user_key(&blind_key_path, &blind_user_key.to_bytes())
        .map_err(|e| messages::error::CommandError::Storage(e.to_string()))?;
    store_salt(&salt_path, &salt).map_err(|e| messages::error::CommandError::Storage(e.to_string()))?;

    Ok(storage_dir)
}

/// Create a new key configuration.
///
/// This function performs interactive setup for a new user:
/// 1. Checks that keys don't already exist (idempotent protection)
/// 2. Prompts for password (twice for confirmation)
/// 3. Generates random salt
/// 4. Derives master key from password + salt
/// 5. Derives UEK and KWK from master key
/// 6. Wraps UEK with KWK to create blind_user_key
/// 7. Stores blind_user_key and salt to ~/.tyche/
///
/// # Errors
/// Returns an error if:
/// - Keys already exist (idempotent protection)
/// - Password confirmation fails
/// - Key derivation fails
/// - Storage operations fail
pub fn key_create() -> messages::success::CommandResult<()> {
    let storage_dir = key_create_internal()?;

    // Print success message (never print keys)
    messages::success::key_created();
    messages::success::storage_location(&storage_dir.display().to_string());
    messages::success::use_key_unlock_verify();

    Ok(())
}

/// Create a new key configuration with context-aware messaging.
///
/// This is a variant of `key_create()` that shows custom intro and success
/// messages based on the context parameter. Used when key creation is
/// invoked as a prerequisite step (e.g., before wallet creation).
///
/// # Arguments
/// * `context` - The context for messaging customization:
///   - "wallet": Shows wallet-specific intro and success messages
///   - Any other value: No intro, standard success message
///
/// # Errors
/// Returns an error if:
/// - Keys already exist (idempotent protection)
/// - Password confirmation fails
/// - Key derivation fails
/// - Storage operations fail
pub fn key_create_with_context(context: &str) -> messages::success::CommandResult<()> {
    // Show custom intro for wallet context
    if context == "wallet" {
        messages::success::key_created();
    }

    let storage_dir = key_create_internal()?;

    // Show custom success message based on context
    if context == "wallet" {
        messages::success::key_created();
    } else {
        messages::success::key_created();
        messages::success::storage_location(&storage_dir.display().to_string());
        messages::success::use_key_unlock_verify();
    }

    Ok(())
}

/// Check if keys already exist on the filesystem.
///
/// Returns `true` if both blind_user_key and salt exist.
fn check_keys_exist() -> messages::success::CommandResult<bool> {
    let blind_key_path = default_blind_user_key_path()
        .ok_or_else(|| messages::error::CommandError::Storage("Could not determine key path".to_string()))?;
    let salt_path = default_salt_path()
        .ok_or_else(|| messages::error::CommandError::Storage("Could not determine salt path".to_string()))?;

    Ok(blind_key_path.exists() && salt_path.exists())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: We don't test check_keys_exist() directly because it uses
    // hardcoded paths from default_blind_user_key_path() and default_salt_path().
    // Testing it would require modifying production code or relying on the
    // global config directory, which causes test isolation issues.

    // #[test]
    // fn test_key_derivation_flow() {
    //     // Test the key derivation flow without storage
    //     let password = "test_password_123";
    //     let salt = generate_salt();

    //     // Derive master key
    //     let master_key = derive_master_key(password, &salt).unwrap();

    //     // Derive UEK and KWK
    //     let (uek, kwk) = derive_user_keys(&master_key);

    //     // Wrap and unwrap
    //     let blind_user_key = wrap_user_encryption_key(&uek, &kwk).unwrap();
    //     let recovered_uek =
    //         crate::commands::key::filestore::crypto::encryption::unwrap_user_encryption_key(&blind_user_key, &kwk)
    //             .unwrap();

    //     // Verify roundtrip
    //     assert_eq!(uek.0, recovered_uek.0);
    // }

    #[test]
    fn test_salt_generation() {
        let salt1 = generate_salt();
        let salt2 = generate_salt();

        // Salts should be different (with very high probability)
        assert_ne!(salt1, salt2);

        // Salts should be 16 bytes
        assert_eq!(salt1.len(), 16);
        assert_eq!(salt2.len(), 16);
    }
}
