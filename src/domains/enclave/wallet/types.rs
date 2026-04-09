//! Wallet types for Edge CLI.
//!
//! Defines the core wallet data structures including chain types,
//! wallet metadata, encrypted wallet blobs, and error types for
//! wallet operations.

use serde::{Deserialize, Serialize};
use thiserror::Error;

use erato::ChainType;

/// A wallet holding encrypted private key material.
///
/// The private key is encrypted with the user's encryption key (UEK)
/// and stored alongside metadata about the wallet.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wallet {
    /// The blockchain chain this wallet belongs to.
    pub chain: ChainType,
    /// Human-readable name for the wallet.
    pub name: String,
    /// The wallet's public address.
    pub address: String,
    /// The encrypted private key (AES-GCM encrypted).
    pub encrypted_private_key: Vec<u8>,
}

/// A wallet holding encrypted private key material.
///
/// The private key is encrypted with the user's encryption key (UEK)
/// and stored alongside metadata about the wallet.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletList {
    /// The blockchain chain this wallet belongs to.
    pub chain_type: ChainType,
    /// Human-readable name for the wallet.
    pub name: String,
    /// The wallet's public address.
    pub address: String,
}

/// The encrypted wallet blob format for storage.
///
/// This is the structure that gets stored in the database via the API.
/// It contains the encrypted wallet data along with any additional
/// encryption layers applied by the TEE platform.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedWalletBlob {
    /// The inner encrypted data (UEK-encrypted wallet).
    pub inner_blob: Vec<u8>,
    /// Optional outer encryption layer (TEE Platform Key encrypted).
    /// This is added when the wallet is stored server-side.
    pub outer_blob: Option<Vec<u8>>,
    /// The chain this wallet belongs to.
    pub chain: ChainType,
    /// The wallet address (unencrypted for indexing).
    pub address: String,
    /// Version of the blob format.
    pub version: u32,
}

impl EncryptedWalletBlob {
    /// Current version of the encrypted wallet blob format.
    pub const CURRENT_VERSION: u32 = 1;

    /// Create a new encrypted wallet blob from inner blob data.
    pub fn new(inner_blob: Vec<u8>, chain: ChainType, address: String) -> Self {
        Self {
            inner_blob,
            outer_blob: None,
            chain,
            address,
            version: Self::CURRENT_VERSION,
        }
    }

    /// Serialize the blob to bytes for storage.
    pub fn to_bytes(&self) -> Result<Vec<u8>, WalletError> {
        serde_json::to_vec(self).map_err(|e| WalletError::Serialization(e.to_string()))
    }

    /// Deserialize the blob from bytes.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, WalletError> {
        serde_json::from_slice(bytes).map_err(|e| WalletError::Serialization(e.to_string()))
    }
}

/// Size of an EVM private key in bytes (32 bytes).
pub const EVM_PRIVATE_KEY_SIZE: usize = 32;

/// Size of a Solana private key seed in bytes (32 bytes).
pub const SOLANA_PRIVATE_KEY_SIZE: usize = 32;

/// Size of an EVM address in bytes (20 bytes).
pub const EVM_ADDRESS_SIZE: usize = 20;

/// Size of a Solana public key in bytes (32 bytes).
pub const SOLANA_PUBKEY_SIZE: usize = 32;

/// Errors that can occur during wallet operations.
#[derive(Debug, Clone, Error)]
pub enum WalletError {
    /// Invalid chain specified.
    #[error("Invalid chain: {0}")]
    InvalidChain(String),

    /// Invalid private key format.
    #[error("Invalid private key: {0}")]
    InvalidPrivateKey(String),

    /// Invalid address format.
    #[error("Invalid address: {0}")]
    InvalidAddress(String),

    /// Cryptographic operation failed.
    #[error("Crypto error: {0}")]
    Crypto(String),

    /// Encryption failed.
    #[error("Encryption failed: {0}")]
    EncryptionFailed(String),

    /// Decryption failed.
    #[error("Decryption failed: {0}")]
    DecryptionFailed(String),

    /// Serialization/deserialization failed.
    #[error("Serialization error: {0}")]
    Serialization(String),

    /// Key generation failed.
    #[error("Key generation failed: {0}")]
    KeyGenerationFailed(String),

    /// Address derivation failed.
    #[error("Address derivation failed: {0}")]
    AddressDerivationFailed(String),

    /// Wallet already exists.
    #[error("Wallet already exists: {0}")]
    WalletAlreadyExists(String),

    /// Wallet not found.
    #[error("Wallet not found: {0}")]
    WalletNotFound(String),

    /// Storage failed.
    #[error("Storage error: {0}")]
    StorageFailed(String),

    /// Could not parse the wallet list.
    #[error("Could not parse the wallet list")]
    ParsingWalletList,

    /// Transport cache error.
    #[error("Transport cache error: {0}")]
    TransportCache(String),
}

/// Result type for wallet operations.
pub type WalletResult<T> = Result<T, WalletError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chain_type_enum() {
        assert_eq!(ChainType::EVM.to_string(), "EVM");
        assert_eq!(ChainType::SVM.to_string(), "SVM");
    }

    #[test]
    fn test_encrypted_wallet_blob_new() {
        let blob = EncryptedWalletBlob::new(vec![1, 2, 3], ChainType::EVM, "0x1234".to_string());
        assert_eq!(blob.inner_blob, vec![1, 2, 3]);
        assert!(blob.outer_blob.is_none());
        assert_eq!(blob.chain, ChainType::EVM);
        assert_eq!(blob.address, "0x1234");
        assert_eq!(blob.version, 1);
    }

    #[test]
    fn test_encrypted_wallet_blob_serialization() {
        let blob = EncryptedWalletBlob::new(vec![1, 2, 3], ChainType::SVM, "ABC123".to_string());
        let bytes = blob.to_bytes().unwrap();
        let recovered = EncryptedWalletBlob::from_bytes(&bytes).unwrap();
        assert_eq!(recovered.address, "ABC123");
        assert_eq!(recovered.chain, ChainType::SVM);
        assert_eq!(recovered.inner_blob, vec![1, 2, 3]);
    }

    #[test]
    fn test_wallet_error_display() {
        let err = WalletError::InvalidChain("bitcoin".to_string());
        assert_eq!(err.to_string(), "Invalid chain: bitcoin");

        let err = WalletError::InvalidPrivateKey("bad format".to_string());
        assert_eq!(err.to_string(), "Invalid private key: bad format");
    }

    #[test]
    fn test_key_sizes() {
        assert_eq!(EVM_PRIVATE_KEY_SIZE, 32);
        assert_eq!(SOLANA_PRIVATE_KEY_SIZE, 32);
        assert_eq!(EVM_ADDRESS_SIZE, 20);
        assert_eq!(SOLANA_PUBKEY_SIZE, 32);
    }
}
