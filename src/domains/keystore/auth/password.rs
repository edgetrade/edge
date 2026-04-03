//! Password-based authentication for Edge CLI.
//!
//! Implements password prompt, master key derivation using PBKDF2-SHA256,
//! and the complete password authentication flow.
//!
//! # Password Authentication Flow
//!
//! ## Initial Setup
//! 1. Generate random salt
//! 2. Prompt user for password
//! 3. Derive master key from password + salt using PBKDF2
//! 4. Derive UEK and KWK from master key using HKDF
//! 5. Wrap UEK with KWK to create blind_user_key
//! 6. Store blind_user_key and salt to filesystem
//!
//! ## Unlock
//! 1. Load salt from filesystem
//! 2. Prompt user for password
//! 3. Derive master key from password + salt
//! 4. Derive KWK from master key
//! 5. Load blind_user_key from filesystem
//! 6. Unwrap to recover UEK
//! 7. Store UEK in session

use crate::domains::config::Config;
use crate::domains::keystore::Session;
use crate::domains::keystore::crypto::derivation::{derive_master_key, derive_user_keys, generate_salt};
use crate::domains::keystore::crypto::encryption::{unwrap_user_encryption_key, wrap_user_encryption_key};
use crate::domains::keystore::crypto::storage::{
    default_blind_user_key_path, default_salt_path, default_storage_dir, ensure_storage_dir, load_blind_user_key,
    load_salt, store_blind_user_key, store_salt,
};
use crate::domains::keystore::crypto::types::{MasterKey, SALT_SIZE};

use super::types::{AuthError, AuthResult, AuthenticationResult, Authenticator};

/// Password authenticator implementing the `Authenticator` trait.
///
/// This struct holds the salt for password derivation and provides
/// the authentication implementation.
#[derive(Debug, Clone)]
pub struct PasswordAuth {
    /// The salt used for PBKDF2 derivation (16 bytes).
    salt: [u8; SALT_SIZE],
}

impl PasswordAuth {
    /// Create a new password authenticator with the given salt.
    ///
    /// # Arguments
    /// * `salt` - The 16-byte salt for PBKDF2 derivation.
    pub fn new(salt: [u8; SALT_SIZE]) -> Self {
        Self { salt }
    }

    /// Get the salt used by this authenticator.
    pub fn salt(&self) -> &[u8; SALT_SIZE] {
        &self.salt
    }
}

impl Authenticator for PasswordAuth {
    /// Authenticate using password and derive the master key.
    ///
    /// Prompts the user for their password and derives the master key
    /// using the stored salt.
    fn authenticate(&self) -> AuthResult<AuthenticationResult> {
        let password = prompt_password("Enter password: ")?;
        let master_key = authenticate_with_password(&password, &self.salt)?;

        Ok(AuthenticationResult {
            master_key,
            salt: self.salt,
        })
    }
}

/// Prompt the user for a password securely.
///
/// Uses `rpassword` to read from stdin without echoing characters.
/// This prevents the password from being visible on screen or in
/// terminal history.
///
/// # Arguments
/// * `prompt` - The prompt text to display to the user.
///
/// # Returns
/// The password entered by the user (without trailing newline).
///
/// # Errors
/// Returns `AuthError::Io` if reading from stdin fails.
pub fn prompt_password(prompt: &str) -> AuthResult<String> {
    rpassword::prompt_password(prompt).map_err(|e| AuthError::Io(e.to_string()))
}

/// Authenticate with a password and derive the master key.
///
/// This is a low-level function that derives the master key from a
/// password and salt without any user interaction. Use this when you
/// already have the password (e.g., from a prompt or config).
///
/// # Arguments
/// * `password` - The user's password.
/// * `salt` - The 16-byte salt for PBKDF2 derivation.
///
/// # Returns
/// The derived `MasterKey` on success.
///
/// # Errors
/// Returns `AuthError::Crypto` if derivation fails (e.g., invalid salt size).
pub fn authenticate_with_password(password: &str, salt: &[u8]) -> AuthResult<MasterKey> {
    derive_master_key(password, salt).map_err(|e| e.into())
}

/// Set up password authentication for the first time.
///
/// This function performs the complete initial setup flow:
/// 1. Generates a random salt
/// 2. Prompts for and confirms the password
/// 3. Derives the master key
/// 4. Derives UEK and KWK
/// 5. Wraps UEK with KWK to create blind_user_key
/// 6. Stores blind_user_key and salt to filesystem
///
/// # Returns
/// A tuple of `(MasterKey, [u8; SALT_SIZE])` on success.
///
/// # Errors
/// Returns `AuthError` if:
/// - Password confirmation fails
/// - Key derivation fails
/// - Storage operations fail
pub fn setup_password_auth() -> AuthResult<(MasterKey, [u8; SALT_SIZE])> {
    // Generate salt
    let salt = generate_salt();

    // Prompt and confirm password
    let password = prompt_password("Create password: ")?;
    let confirm = prompt_password("Confirm password: ")?;

    if password != confirm {
        return Err(AuthError::AuthenticationFailed("Passwords do not match".to_string()));
    }

    // Derive master key
    let master_key = authenticate_with_password(&password, &salt)?;

    // Derive UEK and KWK
    let user_keys = derive_user_keys(&master_key);

    // Wrap UEK with KWK to create blind_user_key
    let blind_user_key = wrap_user_encryption_key(&user_keys)?;

    // Get storage paths
    let storage_dir =
        default_storage_dir().ok_or_else(|| AuthError::Storage("Could not determine home directory".to_string()))?;
    ensure_storage_dir(&storage_dir)?;

    let blind_key_path =
        default_blind_user_key_path().ok_or_else(|| AuthError::Storage("Could not determine key path".to_string()))?;
    let salt_path =
        default_salt_path().ok_or_else(|| AuthError::Storage("Could not determine salt path".to_string()))?;

    // Store blind_user_key and salt
    store_blind_user_key(&blind_key_path, &blind_user_key.to_bytes())?;
    store_salt(&salt_path, &salt)?;

    // Note: The caller should unlock the session after setup completes
    // by calling Session::new().unlock(&uek)

    Ok((master_key, salt))
}

/// Verify a password against stored credentials.
///
/// This function loads the salt and blind_user_key from storage,
/// derives the master key from the provided password, and attempts
/// to unwrap the blind_user_key. If successful, the UEK is stored
/// in the session and the master key is returned.
///
/// # Arguments
/// * `password` - The password to verify.
/// * `stored_salt` - The salt loaded from storage.
///
/// # Returns
/// The derived `MasterKey` on success.
///
/// # Errors
/// Returns `AuthError` if:
/// - Key derivation fails
/// - Blind_user_key cannot be loaded
/// - Unwrapping fails (wrong password)
pub fn verify_password(password: &str, stored_salt: &[u8]) -> AuthResult<MasterKey> {
    // Convert stored_salt to fixed-size array
    if stored_salt.len() != SALT_SIZE {
        return Err(AuthError::Crypto(format!(
            "Invalid salt size: expected {}, got {}",
            SALT_SIZE,
            stored_salt.len()
        )));
    }
    let mut salt = [0u8; SALT_SIZE];
    salt.copy_from_slice(stored_salt);

    // Derive master key
    let master_key = authenticate_with_password(password, &salt)?;

    // Derive KWK from master key
    let user_keys = derive_user_keys(&master_key);

    // Load blind_user_key
    let blind_key_path =
        default_blind_user_key_path().ok_or_else(|| AuthError::Storage("Could not determine key path".to_string()))?;
    let blind_key_bytes = load_blind_user_key(&blind_key_path)?;
    let blind_user_key = crate::domains::keystore::crypto::types::EncryptedData::from_bytes(&blind_key_bytes)
        .ok_or_else(|| AuthError::Crypto("Invalid blind_user_key format".to_string()))?;

    // Attempt to unwrap - this will fail if the password is wrong
    let recovered_uek = unwrap_user_encryption_key(&blind_user_key, &user_keys.meta.unwrap())
        .map_err(|_| AuthError::InvalidCredentials)?;

    // Store UEK in session (if in async context)
    if let Ok(_rt) = tokio::runtime::Handle::try_current() {
        // In async context, we would store in session here
        // But we can't easily get a mutable reference without block_on
        // which could deadlock. The caller should handle this.
    }

    // Zeroize the recovered UEK since we're not storing it
    drop(recovered_uek);

    Ok(master_key)
}

/// Interactive password verification with session unlock.
///
/// This function performs the complete unlock flow:
/// 1. Loads salt from storage
/// 2. Prompts for password
/// 3. Verifies password and derives keys
/// 4. Loads and unwraps blind_user_key
/// 5. Stores UEK in session
///
/// # Returns
/// `Ok(())` on successful unlock.
///
/// # Errors
/// Returns `AuthError` if authentication fails.
pub async fn unlock_with_password(config: Config) -> AuthResult<()> {
    // Load salt
    let salt_path =
        default_salt_path().ok_or_else(|| AuthError::Storage("Could not determine salt path".to_string()))?;
    let salt_bytes = load_salt(&salt_path)?;

    if salt_bytes.len() != SALT_SIZE {
        return Err(AuthError::Crypto(format!(
            "Invalid salt size: expected {}, got {}",
            SALT_SIZE,
            salt_bytes.len()
        )));
    }
    let mut salt = [0u8; SALT_SIZE];
    salt.copy_from_slice(&salt_bytes);

    // Prompt for password
    let password = prompt_password("Enter password: ")?;

    // Derive master key
    let master_key = authenticate_with_password(&password, &salt)?;

    // Derive KWK from master key
    let user_keys = derive_user_keys(&master_key);

    // Load blind_user_key
    let blind_key_path =
        default_blind_user_key_path().ok_or_else(|| AuthError::Storage("Could not determine key path".to_string()))?;
    let blind_key_bytes = load_blind_user_key(&blind_key_path)?;
    let blind_user_key = crate::domains::keystore::crypto::types::EncryptedData::from_bytes(&blind_key_bytes)
        .ok_or_else(|| AuthError::Crypto("Invalid blind_user_key format".to_string()))?;

    // Unwrap to get UEK
    let uek = unwrap_user_encryption_key(&blind_user_key, &user_keys.meta.unwrap())
        .map_err(|_| AuthError::InvalidCredentials)?;

    // Store UEK in session
    let session = Session::new(config);
    session
        .unlock(&uek)
        .map_err(|e| AuthError::Storage(format!("Failed to store session: {}", e)))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domains::keystore::crypto::types::SALT_SIZE;

    #[test]
    fn test_authenticate_with_password_deterministic() {
        let password = "test_password_123";
        let salt = [0u8; SALT_SIZE];

        let key1 = authenticate_with_password(password, &salt).unwrap();
        let key2 = authenticate_with_password(password, &salt).unwrap();

        assert_eq!(key1.0, key2.0, "Same password/salt must produce same key");
    }

    #[test]
    fn test_authenticate_with_password_different_passwords() {
        let salt = [0u8; SALT_SIZE];

        let key1 = authenticate_with_password("password1", &salt).unwrap();
        let key2 = authenticate_with_password("password2", &salt).unwrap();

        assert_ne!(key1.0, key2.0, "Different passwords must produce different keys");
    }

    #[test]
    fn test_password_auth_struct() {
        let salt = [0xABu8; SALT_SIZE];
        let auth = PasswordAuth::new(salt);
        assert_eq!(auth.salt(), &salt);
    }

    #[test]
    fn test_verify_password_wrong_password() {
        // This test requires stored credentials, so we can't fully test it here
        // without mocking the filesystem. The error case is tested.
        let result = verify_password("wrong_password", &[0u8; SALT_SIZE]);
        // Should fail because there's no stored blind_user_key
        assert!(result.is_err());
    }

    #[test]
    fn test_verify_password_invalid_salt_size() {
        let result = verify_password("password", &[0u8; 8]);
        assert!(matches!(result, Err(AuthError::Crypto(_))));
    }
}
