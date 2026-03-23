//! AES-256-GCM encryption for Edge CLI.
//!
//! Implements authenticated encryption using AES-256-GCM with
//! 96-bit nonces and 128-bit authentication tags.

use aes_gcm::{Aes256Gcm, Key, KeyInit, Nonce, aead::Aead};
use ed25519_dalek::SigningKey;

use crate::session::crypto::{KEY_SIZE, UsersEncryptionKeys};

use super::types::{CryptoError, CryptoResult, EncryptedData, NONCE_SIZE};

/// Encrypt data using AES-256-GCM.
///
/// Uses a random 96-bit nonce and produces a 128-bit authentication tag.
/// The key must be 256 bits (32 bytes).
///
/// # Arguments
/// * `plaintext` - The data to encrypt
/// * `key` - The encryption key (must be 32 bytes for AES-256)
///
/// # Returns
/// An `EncryptedData` struct containing the nonce and ciphertext (with auth tag).
///
/// # Type Parameters
/// This function accepts any key type that can provide a 32-byte reference.
pub fn encrypt_aes_gcm(plaintext: &[u8], key: &[u8; KEY_SIZE]) -> CryptoResult<EncryptedData> {
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));

    // Generate random nonce
    let mut nonce_bytes = [0u8; NONCE_SIZE];
    getrandom::fill(&mut nonce_bytes).map_err(|_| CryptoError::EncryptionFailed)?;
    let nonce = Nonce::from_slice(&nonce_bytes);

    // Encrypt
    let ciphertext = cipher
        .encrypt(nonce, plaintext)
        .map_err(|_| CryptoError::EncryptionFailed)?;

    Ok(EncryptedData {
        nonce: nonce_bytes,
        ciphertext,
    })
}

/// Decrypt data using AES-256-GCM.
///
/// Verifies the authentication tag before returning the plaintext.
/// Returns an error if the ciphertext is malformed or the auth tag is invalid.
///
/// # Arguments
/// * `encrypted_data` - The encrypted data (nonce + ciphertext with auth tag)
/// * `key` - The decryption key (must be 32 bytes for AES-256)
///
/// # Returns
/// The decrypted plaintext, or an error if decryption fails.
pub fn decrypt_aes_gcm(encrypted_data: &EncryptedData, key: &[u8; KEY_SIZE]) -> CryptoResult<Vec<u8>> {
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
    let nonce = Nonce::from_slice(&encrypted_data.nonce);

    cipher
        .decrypt(nonce, encrypted_data.ciphertext.as_ref())
        .map_err(|e| CryptoError::DecryptionFailed(e.to_string()))
}

/// Encrypt the user encryption key with the key wrapping key.
///
/// This creates the blind_user_key that is stored locally on the filesystem.
/// The UEK is encrypted with the KWK using AES-256-GCM.
///
/// # Arguments
/// * `uek` - The user encryption key to encrypt
/// * `kwk` - The key wrapping key
///
/// # Returns
/// The encrypted UEK (blind_user_key) ready for storage.
pub fn wrap_user_encryption_key(uek: &UsersEncryptionKeys) -> CryptoResult<EncryptedData> {
    encrypt_aes_gcm(&uek.storage, &uek.meta.unwrap())
}

/// Decrypt the user encryption key from the blind_user_key.
///
/// Recovers the UEK from its encrypted form using the KWK.
///
/// # Arguments
/// * `blind_user_key` - The encrypted UEK (blind_user_key from storage)
/// * `kwk` - The key wrapping key
///
/// # Returns
/// The decrypted user encryption key, or an error if decryption fails.
pub fn unwrap_user_encryption_key(blind_user_key: &EncryptedData, kwk: &[u8; 32]) -> CryptoResult<UsersEncryptionKeys> {
    let mut plaintext = decrypt_aes_gcm(blind_user_key, kwk)?;

    if plaintext.len() != KEY_SIZE {
        return Err(CryptoError::InvalidKeyLength);
    }

    let mut key = [0u8; KEY_SIZE];
    key.copy_from_slice(&plaintext);

    // Zeroize the plaintext buffer
    for byte in plaintext.iter_mut() {
        *byte = 0;
    }

    Ok(UsersEncryptionKeys::new(SigningKey::from_bytes(&key), key, Some(*kwk)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aes_gcm_roundtrip() {
        let key = [0x42u8; KEY_SIZE];
        let plaintext = b"Hello, Edge!";

        let encrypted = encrypt_aes_gcm(plaintext, &key).unwrap();
        let decrypted = decrypt_aes_gcm(&encrypted, &key).unwrap();

        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_aes_gcm_different_keys_fail() {
        let key1 = [0x42u8; KEY_SIZE];
        let key2 = [0x43u8; KEY_SIZE];
        let plaintext = b"Secret message";

        let encrypted = encrypt_aes_gcm(plaintext, &key1).unwrap();
        let result = decrypt_aes_gcm(&encrypted, &key2);

        assert!(result.is_err(), "Decryption with wrong key should fail");
    }

    #[test]
    fn test_aes_gcm_different_nonces_produce_different_ciphertexts() {
        let key = [0x42u8; KEY_SIZE];
        let plaintext = b"Secret message";

        let encrypted1 = encrypt_aes_gcm(plaintext, &key).unwrap();
        let encrypted2 = encrypt_aes_gcm(plaintext, &key).unwrap();

        // Nonces should be different (with very high probability)
        assert_ne!(encrypted1.nonce, encrypted2.nonce);

        // Ciphertexts should be different (with very high probability)
        assert_ne!(encrypted1.ciphertext, encrypted2.ciphertext);

        // But both should decrypt to the same plaintext
        let decrypted1 = decrypt_aes_gcm(&encrypted1, &key).unwrap();
        let decrypted2 = decrypt_aes_gcm(&encrypted2, &key).unwrap();
        assert_eq!(decrypted1, plaintext);
        assert_eq!(decrypted2, plaintext);
    }

    #[test]
    fn test_aes_gcm_tampered_ciphertext_fails() {
        let key = [0x42u8; KEY_SIZE];
        let plaintext = b"Secret message";

        let mut encrypted = encrypt_aes_gcm(plaintext, &key).unwrap();
        encrypted.ciphertext[0] ^= 0xFF; // Flip bits in first byte

        let result = decrypt_aes_gcm(&encrypted, &key);
        assert!(result.is_err(), "Tampered ciphertext should fail authentication");
    }

    #[test]
    fn test_encrypt_empty_plaintext() {
        let key = [0x42u8; KEY_SIZE];
        let plaintext = b"";

        let encrypted = encrypt_aes_gcm(plaintext, &key).unwrap();
        let decrypted = decrypt_aes_gcm(&encrypted, &key).unwrap();

        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_encrypt_large_plaintext() {
        let key = [0x42u8; KEY_SIZE];
        let plaintext = vec![0u8; 1024 * 1024]; // 1MB of zeros

        let encrypted = encrypt_aes_gcm(&plaintext, &key).unwrap();
        let decrypted = decrypt_aes_gcm(&encrypted, &key).unwrap();

        assert_eq!(decrypted, plaintext);
    }
}
