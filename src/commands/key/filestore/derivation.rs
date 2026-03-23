//! Key derivation functions for Edge CLI.
//!
//! Implements PBKDF2-SHA256 for password-to-master-key derivation
//! and HKDF-SHA256 for splitting the master key into subkeys.

use ed25519_dalek::SigningKey;
use hkdf::Hkdf;
use pbkdf2::pbkdf2_hmac;
use sha2::Sha256;
use tyche_enclave::types::constants::{USER_ENCRYPTION_KEY_HKDF_INFO, USER_KEY_WRAPPING_KEY_HKDF_INFO};

use crate::session::crypto::{KEY_SIZE, UsersEncryptionKeys};

use super::crypto::types::{CryptoError, CryptoResult, MasterKey, PBKDF2_ITERATIONS, SALT_SIZE};

/// Derive a master key from a password using PBKDF2-SHA256.
///
/// Uses 100,000 iterations and produces a 256-bit (32-byte) key.
/// The salt should be unique per user and stored alongside the blind_user_key.
///
/// # Arguments
/// * `password` - The user's password
/// * `salt` - A random 16-byte salt
///
/// # Returns
/// A `MasterKey` containing the 32-byte derived key.
///
/// # Errors
/// Returns an error if the salt is not exactly 16 bytes.
pub fn derive_master_key(password: &str, salt: &[u8]) -> CryptoResult<MasterKey> {
    if salt.len() != SALT_SIZE {
        return Err(CryptoError::InvalidKeyLength);
    }

    let mut key = [0u8; KEY_SIZE];
    pbkdf2_hmac::<Sha256>(password.as_bytes(), salt, PBKDF2_ITERATIONS, &mut key);
    Ok(MasterKey(key))
}

/// Derive user keys from a master key using HKDF-SHA256.
///
/// Splits the master key into two derived keys using HKDF with
/// domain-separated info strings:
/// - UserEncryptionKey: info="edge-v1:user-encryption-key"
/// - Kwk (Key Wrapping Key): info="edge-v1:key-wrapping-key"
///
/// # Arguments
/// * `master_key` - The 32-byte master key from PBKDF2
///
/// # Returns
/// A tuple of (UserEncryptionKey, Kwk).
pub fn derive_user_keys(master_key: &MasterKey) -> UsersEncryptionKeys {
    let hkdf = Hkdf::<Sha256>::from_prk(&master_key.0).expect("PRK is valid length");

    let mut uek_bytes = [0u8; KEY_SIZE];
    hkdf.expand(USER_ENCRYPTION_KEY_HKDF_INFO, &mut uek_bytes)
        .expect("HKDF expand for UEK");

    let mut kwk_bytes = [0u8; KEY_SIZE];
    hkdf.expand(USER_KEY_WRAPPING_KEY_HKDF_INFO, &mut kwk_bytes)
        .expect("HKDF expand for KWK");

    UsersEncryptionKeys::new(SigningKey::from_bytes(&uek_bytes), uek_bytes, Some(kwk_bytes))
}

/// Generate a random 16-byte salt for PBKDF2.
///
/// Uses the system's cryptographically secure random number generator.
pub fn generate_salt() -> [u8; SALT_SIZE] {
    let mut salt = [0u8; SALT_SIZE];
    getrandom::fill(&mut salt).expect("System RNG available");
    salt
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pbkdf2_deterministic() {
        let password = "test_password_123";
        let salt = [0u8; SALT_SIZE];

        let key1 = derive_master_key(password, &salt).unwrap();
        let key2 = derive_master_key(password, &salt).unwrap();

        assert_eq!(key1.0, key2.0, "Same password/salt must produce same key");
    }

    #[test]
    fn test_pbkdf2_different_salts_produce_different_keys() {
        let password = "test_password_123";
        let salt1 = [0u8; SALT_SIZE];
        let salt2 = [1u8; SALT_SIZE];

        let key1 = derive_master_key(password, &salt1).unwrap();
        let key2 = derive_master_key(password, &salt2).unwrap();

        assert_ne!(key1.0, key2.0, "Different salts must produce different keys");
    }

    #[test]
    fn test_pbkdf2_different_passwords_produce_different_keys() {
        let salt = [0u8; SALT_SIZE];

        let key1 = derive_master_key("password1", &salt).unwrap();
        let key2 = derive_master_key("password2", &salt).unwrap();

        assert_ne!(key1.0, key2.0, "Different passwords must produce different keys");
    }

    #[test]
    fn test_pbkdf2_invalid_salt_size() {
        let result = derive_master_key("password", &[1, 2, 3]);
        assert!(matches!(result, Err(CryptoError::InvalidKeyLength)));
    }

    #[test]
    fn test_generate_salt_is_random() {
        let salt1 = generate_salt();
        let salt2 = generate_salt();

        assert_ne!(
            salt1, salt2,
            "Generated salts should be different (with high probability)"
        );
        assert_eq!(salt1.len(), SALT_SIZE);
        assert_eq!(salt2.len(), SALT_SIZE);
    }
}
