//! Key unlock command for filestore.
//!
//! Implements session unlock by prompting for password, deriving keys,
use crate::domains::config::Config;
use crate::domains::keystore::Session;
/// and loading the user encryption key into the session.
use crate::domains::keystore::auth::password::prompt_password;
use crate::domains::keystore::crypto::derivation::{derive_master_key, derive_user_keys};
use crate::domains::keystore::crypto::encryption::unwrap_user_encryption_key;
use crate::domains::keystore::crypto::storage::{
    default_blind_user_key_path, default_salt_path, load_blind_user_key, load_salt,
};
use crate::domains::keystore::crypto::types::EncryptedData;
use crate::error::PoseidonError;
use crate::messages;

/// Internal unlock logic shared between `key_unlock` and `key_unlock_with_context`.
///
/// Performs the core unlock flow without any user-facing messages.
/// Returns Ok(true) if already unlocked, Ok(false) if successfully unlocked,
/// or Err on failure.
fn key_unlock_internal(session: &Session) -> crate::error::Result<bool> {
    // Check if already unlocked
    if session.is_unlocked() {
        return Ok(true);
    }

    // Get storage paths
    let salt_path =
        default_salt_path().ok_or_else(|| PoseidonError::Storage("Could not determine salt path".to_string()))?;
    let blind_key_path = default_blind_user_key_path()
        .ok_or_else(|| PoseidonError::Storage("Could not determine key path".to_string()))?;

    // Load salt from storage
    let salt_bytes = load_salt(&salt_path).map_err(|e| PoseidonError::Storage(e.to_string()))?;

    if salt_bytes.len() != 16 {
        return Err(PoseidonError::Crypto(format!(
            "Invalid salt size: expected 16, got {}",
            salt_bytes.len()
        )));
    }

    let mut salt = [0u8; 16];
    salt.copy_from_slice(&salt_bytes);

    // Prompt for password
    let password = prompt_password("Enter password: ").map_err(|e| PoseidonError::Authentication(e.to_string()))?;

    // Derive master key from password + salt
    let master_key = derive_master_key(&password, &salt).map_err(|e| PoseidonError::Crypto(e.to_string()))?;

    // Derive KWK from master key (we only need KWK for unwrapping)
    let user_keys = derive_user_keys(&master_key);

    // Load blind_user_key from disk
    let blind_key_bytes = load_blind_user_key(&blind_key_path).map_err(|e| PoseidonError::Storage(e.to_string()))?;

    let blind_user_key = EncryptedData::from_bytes(&blind_key_bytes)
        .ok_or_else(|| PoseidonError::Crypto("Invalid blind_user_key format".to_string()))?;

    // Unwrap to get UEK - this will fail if password is wrong
    let uek = unwrap_user_encryption_key(&blind_user_key, &user_keys.meta.unwrap())
        .map_err(|_| PoseidonError::Authentication("Invalid password".to_string()))?;

    // Store UEK in session (keyring or file)
    session
        .unlock(&uek)
        .map_err(|e| PoseidonError::Storage(e.to_string()))?;

    Ok(false)
}

/// Unlock the session.
///
/// This function performs the unlock flow:
/// 1. Checks if already unlocked (returns early with message)
/// 2. Loads salt from ~/.tyche/salt
/// 3. Prompts for password
/// 4. Derives master key from password + salt
/// 5. Derives KWK from master key
/// 6. Loads blind_user_key from disk
/// 7. Unwraps to get UEK
/// 8. Stores UEK in OS keyring via Session
///
/// # Errors
/// Returns an error if:
/// - Session is already unlocked
/// - Salt or blind_user_key files are missing
/// - Password is incorrect
/// - Key derivation fails
/// - Decryption fails
pub fn key_unlock(config: Config) -> crate::error::Result<()> {
    let session = Session::new(config);

    match key_unlock_internal(&session)? {
        true => {
            messages::success::session_already_unlocked();
        }
        false => {
            messages::success::session_unlocked();
        }
    }

    Ok(())
}

/// Unlock the session with context-specific messaging.
///
/// Similar to `key_unlock` but with custom intro and success messages
/// based on the provided context.
///
/// - If context is "wallet": prints "Session locked. Unlocking now..." before unlock
///   and "Session unlocked successfully" after unlock
/// - For other contexts: no intro message, uses existing success message
///
/// # Errors
/// Returns an error if:
/// - Session is already unlocked
/// - Salt or blind_user_key files are missing
/// - Password is incorrect
/// - Key derivation fails
/// - Decryption fails
pub fn key_unlock_with_context(context: &str, config: Config) -> crate::error::Result<()> {
    let session = Session::new(config);

    if context == "wallet" {
        messages::success::session_unlocking();
    }

    match key_unlock_internal(&session)? {
        true => {
            messages::success::session_already_unlocked();
        }
        false => {
            messages::success::session_unlocked();
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domains::keystore::crypto::derivation::generate_salt;
    use crate::domains::keystore::crypto::encryption::wrap_user_encryption_key;
    use std::fs;

    // #[test]
    // fn test_unwrap_with_correct_password() {
    //     let temp = tempfile::tempdir().expect("Failed to create temp dir");

    //     // Setup: create salt and blind_user_key
    //     let password = "correct_password";
    //     let salt = generate_salt();

    //     let master_key = derive_master_key(password, &salt).unwrap();
    //     let (uek, kwk) = derive_user_keys(&master_key);
    //     let blind_user_key = wrap_user_encryption_key(&uek, &kwk).unwrap();

    //     // Store files in temp directory
    //     let key_path = temp.path().join("blind_user_key");
    //     fs::write(&key_path, blind_user_key.to_bytes()).unwrap();

    //     // Test: verify the wrapped key can be unwrapped with correct password
    //     let master_key2 = derive_master_key(password, &salt).unwrap();
    //     let (_, kwk2) = derive_user_keys(&master_key2);

    //     let blind_key_bytes = fs::read(&key_path).unwrap();
    //     let blind_user_key2 = EncryptedData::from_bytes(&blind_key_bytes).expect("valid encrypted data");

    //     let recovered_uek = unwrap_user_encryption_key(&blind_user_key2, &kwk2).unwrap();
    //     assert_eq!(uek.0, recovered_uek.0);
    // }

    #[test]
    fn test_unwrap_with_wrong_password() {
        let temp = tempfile::tempdir().expect("Failed to create temp dir");

        // Setup: create salt and blind_user_key with correct password
        let correct_password = "correct_password";
        let wrong_password = "wrong_password";
        let salt = generate_salt();

        let master_key = derive_master_key(correct_password, &salt).unwrap();
        let user_keys = derive_user_keys(&master_key);
        let blind_user_key = wrap_user_encryption_key(&user_keys).unwrap();

        // Store files in temp directory
        let key_path = temp.path().join("blind_user_key");
        fs::write(&key_path, blind_user_key.to_bytes()).unwrap();

        // Test: verify unwrap fails with wrong password
        let wrong_master_key = derive_master_key(wrong_password, &salt).unwrap();
        let wrong_user_keys = derive_user_keys(&wrong_master_key);

        let blind_key_bytes = fs::read(&key_path).unwrap();
        let blind_user_key2 = EncryptedData::from_bytes(&blind_key_bytes).expect("valid encrypted data");

        let result = unwrap_user_encryption_key(&blind_user_key2, &wrong_user_keys.meta.unwrap());
        assert!(result.is_err(), "Unwrap with wrong password should fail");
    }
}
