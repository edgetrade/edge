//! UEK encryption/decryption for Edge CLI.
//!
//! Re-exports crypto functionality from session/crypto.rs.

pub use crate::domains::keystore::session::crypto::{
    CryptoError, CryptoResult, EnclaveTransportKeys, KEY_SIZE, UsersEncryptionKeys,
};

/// Size of an EVM private key in bytes (32 bytes).
pub const EVM_PRIVATE_KEY_SIZE: usize = KEY_SIZE;

/// Size of a Solana private key seed in bytes (32 bytes).
pub const SOLANA_PRIVATE_KEY_SIZE: usize = KEY_SIZE;

/// Size of an EVM address in bytes (20 bytes).
pub const EVM_ADDRESS_SIZE: usize = 20;

/// Size of a Solana public key in bytes (32 bytes).
pub const SOLANA_PUBKEY_SIZE: usize = KEY_SIZE;
