//! Enclave domain error types.
//!
//! Defines all error variants for enclave operations including
//! wallet management, encryption/decryption, and secure operations.

use thiserror::Error;

/// Result type for enclave operations.
pub type EnclaveResult<T> = Result<T, EnclaveError>;

/// Errors that can occur in the enclave domain.
#[derive(Debug, Error, Clone)]
pub enum EnclaveError {
    /// No UEK available.
    #[error("No UEK available")]
    NoUek,

    /// Wallet not found.
    #[error("Wallet not found: {0}")]
    WalletNotFound(String),

    /// Wallet already exists.
    #[error("Wallet already exists: {0}")]
    WalletAlreadyExists(String),

    /// Invalid address format.
    #[error("Invalid address: {0}")]
    InvalidAddress(String),

    /// Invalid private key format.
    #[error("Invalid private key: {0}")]
    InvalidPrivateKey(String),

    /// Decryption failed.
    #[error("Decryption failed: {0}")]
    DecryptionFailed(String),

    /// Encryption failed.
    #[error("Encryption failed: {0}")]
    EncryptionFailed(String),

    /// Signing failed.
    #[error("Signing failed: {0}")]
    SigningFailed(String),

    /// Keystore is locked.
    #[error("Keystore locked")]
    KeystoreLocked,

    /// Channel communication error.
    #[error("Channel error")]
    ChannelError,

    /// Channel send error.
    #[error("Channel send error")]
    ChannelSend,

    /// Channel receive error.
    #[error("Channel receive error")]
    ChannelRecv,

    /// Oneshot reply error.
    #[error("Oneshot reply error")]
    OneshotReply,

    /// Wallet operation error.
    #[error("Wallet error: {0}")]
    Wallet(String),

    /// Crypto operation error.
    #[error("Crypto error: {0}")]
    Crypto(String),

    /// Invalid input.
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    /// IO error.
    #[error("IO error: {0}")]
    Io(String),

    /// Configuration error.
    #[error("Configuration error: {0}")]
    Config(String),

    /// Transport error.
    #[error("Transport error: {0}")]
    Transport(String),

    /// Storage error.
    #[error("Storage error: {0}")]
    Storage(String),

    /// Agent ID not found.
    #[error("Agent ID not found")]
    AgentIdNotFound,

    /// Game state error.
    #[error("Game state error: {0}")]
    GameState(String),

    /// Key derivation failed.
    #[error("Key derivation failed: {0}")]
    KeyDerivationFailed(String),

    /// Trade intent error.
    #[error("Trade intent error: {0}")]
    TradeIntent(String),

    /// Key material zeroization error.
    #[error("Key zeroization error: {0}")]
    Zeroization(String),
}

impl EnclaveError {
    /// Returns true if this error indicates the keystore is locked.
    pub fn is_keystore_locked(&self) -> bool {
        matches!(self, EnclaveError::KeystoreLocked | EnclaveError::NoUek)
    }

    /// Returns true if this is a "not found" error.
    pub fn is_not_found(&self) -> bool {
        matches!(self, EnclaveError::WalletNotFound(_))
    }

    /// Returns true if this is an "already exists" error.
    pub fn is_already_exists(&self) -> bool {
        matches!(self, EnclaveError::WalletAlreadyExists(_))
    }

    /// Returns true if this is a crypto/decryption error.
    pub fn is_crypto_error(&self) -> bool {
        matches!(
            self,
            EnclaveError::DecryptionFailed(_)
                | EnclaveError::EncryptionFailed(_)
                | EnclaveError::Crypto(_)
                | EnclaveError::SigningFailed(_)
        )
    }
}

impl From<std::io::Error> for EnclaveError {
    fn from(e: std::io::Error) -> Self {
        EnclaveError::Io(e.to_string())
    }
}

impl From<crate::error::PoseidonError> for EnclaveError {
    fn from(e: crate::error::PoseidonError) -> Self {
        EnclaveError::Wallet(e.to_string())
    }
}

impl From<crate::domains::enclave::crypto::CryptoError> for EnclaveError {
    fn from(e: crate::domains::enclave::crypto::CryptoError) -> Self {
        EnclaveError::Crypto(e.to_string())
    }
}

impl From<crate::domains::enclave::wallet::types::WalletError> for EnclaveError {
    fn from(e: crate::domains::enclave::wallet::types::WalletError) -> Self {
        match e {
            crate::domains::enclave::wallet::types::WalletError::InvalidChain(msg) => {
                EnclaveError::InvalidInput(format!("Invalid chain: {}", msg))
            }
            crate::domains::enclave::wallet::types::WalletError::InvalidPrivateKey(msg) => {
                EnclaveError::InvalidPrivateKey(msg)
            }
            crate::domains::enclave::wallet::types::WalletError::InvalidAddress(msg) => {
                EnclaveError::InvalidAddress(msg)
            }
            crate::domains::enclave::wallet::types::WalletError::Crypto(msg) => EnclaveError::Crypto(msg),
            crate::domains::enclave::wallet::types::WalletError::EncryptionFailed(msg) => {
                EnclaveError::EncryptionFailed(msg)
            }
            crate::domains::enclave::wallet::types::WalletError::DecryptionFailed(msg) => {
                EnclaveError::DecryptionFailed(msg)
            }
            crate::domains::enclave::wallet::types::WalletError::Serialization(msg) => {
                EnclaveError::InvalidInput(format!("Serialization error: {}", msg))
            }
            crate::domains::enclave::wallet::types::WalletError::KeyGenerationFailed(msg) => {
                EnclaveError::Crypto(format!("Key generation failed: {}", msg))
            }
            crate::domains::enclave::wallet::types::WalletError::AddressDerivationFailed(msg) => {
                EnclaveError::Crypto(format!("Address derivation failed: {}", msg))
            }
            crate::domains::enclave::wallet::types::WalletError::WalletAlreadyExists(msg) => {
                EnclaveError::WalletAlreadyExists(msg)
            }
            crate::domains::enclave::wallet::types::WalletError::WalletNotFound(msg) => {
                EnclaveError::WalletNotFound(msg)
            }
            crate::domains::enclave::wallet::types::WalletError::StorageFailed(msg) => EnclaveError::Storage(msg),
            crate::domains::enclave::wallet::types::WalletError::ParsingWalletList => {
                EnclaveError::InvalidInput("Failed to parse wallet list".to_string())
            }
            crate::domains::enclave::wallet::types::WalletError::TransportCache(msg) => EnclaveError::Transport(msg),
        }
    }
}

impl From<crate::messages::CommandError> for EnclaveError {
    fn from(e: crate::messages::CommandError) -> Self {
        EnclaveError::InvalidInput(e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enclave_error_variants() {
        let err = EnclaveError::NoUek;
        assert_eq!(err.to_string(), "No UEK available");

        let err = EnclaveError::WalletNotFound("test".to_string());
        assert!(err.to_string().contains("test"));

        let err = EnclaveError::InvalidAddress("0xbad".to_string());
        assert!(err.to_string().contains("0xbad"));
    }

    #[test]
    fn test_is_keystore_locked() {
        assert!(EnclaveError::NoUek.is_keystore_locked());
        assert!(EnclaveError::KeystoreLocked.is_keystore_locked());
        assert!(!EnclaveError::WalletNotFound("test".to_string()).is_keystore_locked());
    }

    #[test]
    fn test_is_not_found() {
        assert!(EnclaveError::WalletNotFound("test".to_string()).is_not_found());
        assert!(!EnclaveError::NoUek.is_not_found());
    }

    #[test]
    fn test_is_already_exists() {
        assert!(EnclaveError::WalletAlreadyExists("test".to_string()).is_already_exists());
        assert!(!EnclaveError::NoUek.is_already_exists());
    }

    #[test]
    fn test_is_crypto_error() {
        assert!(EnclaveError::DecryptionFailed("test".to_string()).is_crypto_error());
        assert!(EnclaveError::EncryptionFailed("test".to_string()).is_crypto_error());
        assert!(EnclaveError::SigningFailed("test".to_string()).is_crypto_error());
        assert!(EnclaveError::Crypto("test".to_string()).is_crypto_error());
        assert!(!EnclaveError::NoUek.is_crypto_error());
    }

    #[test]
    fn test_from_io_error() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let err: EnclaveError = io_err.into();
        assert!(matches!(err, EnclaveError::Io(_)));
    }
}
