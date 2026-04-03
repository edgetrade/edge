use ed25519_dalek::{SigningKey, VerifyingKey};

/// Result type for crypto operations.
pub type CryptoResult<T> = Result<T, CryptoError>;

/// Errors that can occur during cryptographic operations.
#[derive(Debug, Clone, thiserror::Error)]
pub enum CryptoError {
    #[error("Encryption failed")]
    EncryptionFailed,
    #[error("Decryption failed: {0}")]
    DecryptionFailed(String),
    #[error("Transport key error: {0}")]
    TransportKeyError(String),
    #[error("Invalid key length")]
    InvalidKeyLength,
    #[error("Invalid ciphertext")]
    InvalidCiphertext,
    #[error("Derivation failed")]
    DerivationFailed,
}

pub const KEY_SIZE: usize = 32;

#[derive(Debug, Clone)]
pub struct EnclaveTransportKeys {
    pub ephemeral: VerifyingKey,
    pub deterministic: VerifyingKey,
    pub attestation: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct UsersEncryptionKeys {
    pub transport: SigningKey,
    pub storage: [u8; 32],
    pub meta: Option<[u8; 32]>,
}

impl UsersEncryptionKeys {
    pub fn new(transport: SigningKey, storage: [u8; 32], meta: Option<[u8; 32]>) -> Self {
        Self {
            transport,
            storage,
            meta,
        }
    }
}
