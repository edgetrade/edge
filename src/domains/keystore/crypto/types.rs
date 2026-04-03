//! Key types with secure memory zeroization.
//!
//! All secret key types implement `ZeroizeOnDrop` to ensure sensitive
//! key material is cleared from memory when the keys go out of scope.

use serde::{Deserialize, Serialize};
use zeroize::ZeroizeOnDrop;

use crate::domains::keystore::session::crypto::KEY_SIZE;

/// Size of a 128-bit salt in bytes.
pub const SALT_SIZE: usize = 16;

/// Size of AES-GCM nonce in bytes.
pub const NONCE_SIZE: usize = 12;

/// Size of AES-GCM authentication tag in bytes.
pub const TAG_SIZE: usize = 16;

/// Number of PBKDF2 iterations for master key derivation.
pub const PBKDF2_ITERATIONS: u32 = 100_000;

/// Domain separator prefix for all HKDF info strings.
pub const DOMAIN_PREFIX: &[u8] = b"edge-v1";

/// Master key derived from password via PBKDF2.
///
/// This is the intermediate key from which user-facing keys are derived.
#[derive(Clone, ZeroizeOnDrop)]
pub struct MasterKey(pub [u8; KEY_SIZE]);

impl std::fmt::Debug for MasterKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MasterKey")
            .field("0", &"[REDACTED]")
            .finish()
    }
}

/// Encrypted data from AES-256-GCM encryption.
///
/// Contains the nonce, ciphertext, and authentication tag.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedData {
    /// The 12-byte nonce used for encryption.
    #[serde(with = "serde_bytes")]
    pub nonce: [u8; NONCE_SIZE],
    /// The encrypted ciphertext (includes 16-byte auth tag at end).
    #[serde(with = "serde_bytes")]
    pub ciphertext: Vec<u8>,
}

impl EncryptedData {
    /// Serialize encrypted data to bytes: [nonce | ciphertext].
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut result = Vec::with_capacity(NONCE_SIZE + self.ciphertext.len());
        result.extend_from_slice(&self.nonce);
        result.extend_from_slice(&self.ciphertext);
        result
    }

    /// Deserialize encrypted data from bytes.
    ///
    /// Returns `None` if the input is too short to contain a valid nonce.
    pub fn from_bytes(data: &[u8]) -> Option<Self> {
        if data.len() < NONCE_SIZE {
            return None;
        }
        let mut nonce = [0u8; NONCE_SIZE];
        nonce.copy_from_slice(&data[..NONCE_SIZE]);
        let ciphertext = data[NONCE_SIZE..].to_vec();
        Some(Self { nonce, ciphertext })
    }
}

/// Result type for crypto operations.
pub type CryptoResult<T> = Result<T, CryptoError>;

/// Errors that can occur during cryptographic operations.
#[derive(Debug, Clone, thiserror::Error)]
pub enum CryptoError {
    #[error("Encryption failed")]
    EncryptionFailed,
    #[error("Decryption failed: {0}")]
    DecryptionFailed(String),
    #[error("Invalid key length")]
    InvalidKeyLength,
    #[error("Invalid ciphertext")]
    InvalidCiphertext,
    #[error("Derivation failed")]
    DerivationFailed,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_sizes() {
        assert_eq!(KEY_SIZE, 32);
        assert_eq!(SALT_SIZE, 16);
        assert_eq!(NONCE_SIZE, 12);
        assert_eq!(TAG_SIZE, 16);
    }

    #[test]
    fn test_encrypted_data_roundtrip() {
        let data = EncryptedData {
            nonce: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            ciphertext: vec![0xde, 0xad, 0xbe, 0xef],
        };
        let bytes = data.to_bytes();
        let recovered = EncryptedData::from_bytes(&bytes).unwrap();
        assert_eq!(recovered.nonce, data.nonce);
        assert_eq!(recovered.ciphertext, data.ciphertext);
    }

    #[test]
    fn test_encrypted_data_from_bytes_too_short() {
        assert!(EncryptedData::from_bytes(&[1, 2, 3]).is_none());
    }
}
