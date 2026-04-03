//! Desktop key create command - keyring only.
//!
//! Creates a new user encryption key and stores it directly in the OS keyring.
//! Supports optional user-provided passwords derived via HKDF-SHA256.

use std::io::Write;

use ed25519_dalek::SigningKey;
use hkdf::Hkdf;
use sha2::Sha256;

use tyche_enclave::envelopes::storage::derive_storage_key;
use tyche_enclave::types::constants::USER_ENCRYPTION_KEY_HKDF_INFO;

use crate::domains::config::Config;
use crate::domains::keystore::KeyringSession as Session;
use crate::domains::keystore::session::crypto::UsersEncryptionKeys;
use crate::error::PoseidonError;
use crate::messages;

/// Create a new key in the OS keyring with context-aware messaging.
///
/// This function:
/// 1. Shows a context-specific intro message
/// 2. Prompts the user for a password (or press Enter for random key)
/// 3. If password provided: derives 32-byte UEK using HKDF-SHA256
/// 4. If no password: generates random 32-byte key with user confirmation
/// 5. Stores it directly in the OS keyring using DesktopSession
/// 6. Shows a context-specific success message
///
/// # Arguments
/// * `context` - The context for messaging ("wallet" for wallet-specific messages, "" for default)
///
/// # Errors
/// Returns an error if:
/// - Key generation fails
/// - Keyring is unavailable
/// - Key already exists (idempotent protection via keyring)
/// - Passwords do not match
/// - HKDF expansion fails
pub fn keyring_create_with_context(context: &str, config: Config) -> crate::error::Result<()> {
    // Show context-specific intro message
    if context == "wallet" {
        messages::success::no_key_found_create();
    } else {
        let session = Session::new(config.clone());
        if session.is_unlocked() {
            messages::success::key_exists();
            return Ok(());
        }
        messages::success::creating_key_os_keyring();
    }

    // Prompt for password
    let password = rpassword::prompt_password(messages::prompt::create_password())
        .map_err(|e| PoseidonError::InvalidInput(format!("Failed to read password: {}", e)))?;
    let password_trimmed = password.trim();

    if password_trimmed.is_empty() {
        // User wants a random key - get confirmation
        print!("Are you sure? (Y/n) ");
        std::io::stdout().flush().map_err(PoseidonError::Io)?;
        let mut confirmation = String::new();
        std::io::stdin()
            .read_line(&mut confirmation)
            .map_err(PoseidonError::Io)?;

        if confirmation.trim().to_lowercase().starts_with("n") {
            messages::success::key_creation_cancelled();
            return Ok(());
        }

        messages::prompt::confirm_no_password();
        let mut continue_input = String::new();
        std::io::stdin()
            .read_line(&mut continue_input)
            .map_err(PoseidonError::Io)?;

        if continue_input.trim().to_lowercase().starts_with("n") {
            messages::success::key_creation_cancelled();
            return Ok(());
        }
    } else {
        // Password provided - confirm it
        let confirm_password = rpassword::prompt_password(messages::prompt::confirm_password())
            .map_err(|e| PoseidonError::InvalidInput(format!("Failed to read password: {}", e)))?;

        if password_trimmed != confirm_password.trim() {
            return Err(PoseidonError::InvalidInput("Passwords do not match".to_string()));
        }
    }

    // Create and store the key
    keyring_create_core(password_trimmed, config)?;

    // Show context-specific success message
    messages::success::key_created();
    messages::success::use_key_unlock_verify();

    Ok(())
}

/// Internal function that creates and stores a key given a password.
///
/// This handles the core logic of key generation and storage without
/// any interactive prompts or output messages.
///
/// # Arguments
/// * `password` - The password to derive the key from, or empty string for random key
///
/// # Errors
/// Returns an error if:
/// - Key generation fails
/// - Keyring is unavailable
/// - Key already exists (idempotent protection via keyring)
/// - HKDF expansion fails
fn keyring_create_core(password: &str, config: Config) -> crate::error::Result<UsersEncryptionKeys> {
    let session = Session::new(config);

    // Check if key already exists
    if session.is_unlocked() {
        return Err(PoseidonError::AlreadyExists("Key".to_string()));
    }

    let uek = if password.is_empty() {
        // Generate random 32-byte key
        let mut uek_bytes = [0u8; 32];
        getrandom::fill(&mut uek_bytes)
            .map_err(|_| PoseidonError::Crypto("Failed to generate random key".to_string()))?;
        let uek = derive_storage_key(&uek_bytes);
        UsersEncryptionKeys::new(SigningKey::from_bytes(&uek_bytes), uek, None)
    } else {
        // Derive 32-byte UEK from password using HKDF-SHA256
        let hkdf = Hkdf::<Sha256>::new(None, password.as_bytes());
        let mut uek_bytes = [0u8; 32];
        hkdf.expand(USER_ENCRYPTION_KEY_HKDF_INFO, &mut uek_bytes)
            .map_err(|e| PoseidonError::Crypto(format!("HKDF expansion failed: {}", e)))?;
        let uek = derive_storage_key(&uek_bytes);
        UsersEncryptionKeys::new(SigningKey::from_bytes(&uek_bytes), uek, None)
    };

    // Store in keyring
    session
        .save(&uek, false)
        .map_err(|e| PoseidonError::Storage(e.to_string()))?;

    Ok(uek)
}

/// Create a new key in the OS keyring.
///
/// This function:
/// 1. Prompts the user for a password (or press Enter for random key)
/// 2. If password provided: derives 32-byte UEK using HKDF-SHA256
/// 3. If no password: generates random 32-byte key with user confirmation
/// 4. Stores it directly in the OS keyring using DesktopSession
/// 5. Prints success message
///
/// # Errors
/// Returns an error if:
/// - Key generation fails
/// - Keyring is unavailable
/// - Key already exists (idempotent protection via keyring)
/// - Passwords do not match
/// - HKDF expansion fails
pub fn keyring_create(config: Config) -> crate::error::Result<()> {
    keyring_create_with_context("", config)
}

#[cfg(all(test, feature = "keyring-tests"))]
mod tests {
    use super::*;
    use crate::domains::keystore::KeyringSession;
    use crate::domains::keystore::session::keyring::{KEYRING_SERVICE, KEYRING_USERNAME};
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

    /// Internal function for testing key creation with password
    fn keyring_create_with_password(password: &str) -> crate::error::Result<UsersEncryptionKeys> {
        keyring_create_core(password, Config::default())
    }

    #[test]
    fn test_keyring_create_random_success() {
        let _lock = KEYRING_TEST_MUTEX
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        cleanup_keyring();
        let _guard = CleanupGuard;

        // Create key with empty password (random generation)
        let result = keyring_create_with_password("");
        assert!(result.is_ok(), "Key creation should succeed: {:?}", result.err());

        // Verify key exists
        let session = KeyringSession::new(Config::default());
        assert!(session.is_unlocked(), "Key should exist after creation");
    }

    #[test]
    fn test_keyring_create_password_success() {
        let _lock = KEYRING_TEST_MUTEX
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        cleanup_keyring();
        let _guard = CleanupGuard;

        // Create key with password
        let result = keyring_create_with_password("test-password-123");
        assert!(
            result.is_ok(),
            "Key creation with password should succeed: {:?}",
            result.err()
        );

        // Verify key exists
        let session = KeyringSession::new(Config::default());
        assert!(session.is_unlocked(), "Key should exist after creation");
    }

    #[test]
    fn test_keyring_create_password_deterministic() {
        // Test that same password always produces same key
        let password = "deterministic-test-password";

        // First derivation
        let hkdf1 = Hkdf::<Sha256>::new(None, password.as_bytes());
        let mut uek_bytes1 = [0u8; 32];
        hkdf1
            .expand(b"edge-keyring-v1:uek", &mut uek_bytes1)
            .unwrap();

        // Second derivation
        let hkdf2 = Hkdf::<Sha256>::new(None, password.as_bytes());
        let mut uek_bytes2 = [0u8; 32];
        hkdf2
            .expand(b"edge-keyring-v1:uek", &mut uek_bytes2)
            .unwrap();

        assert_eq!(uek_bytes1, uek_bytes2, "Same password should produce same key");
    }

    #[test]
    fn test_keyring_create_already_exists() {
        let _lock = KEYRING_TEST_MUTEX
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        cleanup_keyring();
        let _guard = CleanupGuard;

        // Create first key
        keyring_create_with_password("").unwrap();

        // Try to create again - should fail with AlreadyExists
        let result = keyring_create_with_password("");
        assert!(result.is_err(), "Should fail when key already exists");
    }

    #[test]
    fn test_keyring_create_generates_valid_key() {
        let _lock = KEYRING_TEST_MUTEX
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        cleanup_keyring();
        let _guard = CleanupGuard;

        keyring_create_with_password("").unwrap();

        // Verify we can retrieve the key
        let session = KeyringSession::new(Config::default());
        let key = session.get().unwrap();
        assert!(key.is_some(), "Key should be retrievable");

        // Key should be 32 bytes
        let uek = key.unwrap();
        assert_eq!(uek.storage.len(), 32, "Key should be 32 bytes");
    }

    #[test]
    fn test_password_derived_key_is_32_bytes() {
        let _lock = KEYRING_TEST_MUTEX
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        cleanup_keyring();
        let _guard = CleanupGuard;

        let uek = keyring_create_with_password("test-password").unwrap();
        assert_eq!(uek.storage.len(), 32, "Derived key should be 32 bytes");
    }
}
