//! Enclave domain - Combined encryption (UEK) + wallet operations
//!
//! Provides secure wallet management with user encryption keys (UEK),
//! wallet creation, import, and secure trade signing.
//!
//! Security Model:
//! - NEVER keeps actual wallet material (private keys, UEK) in memory permanently
//! - UEK is only held temporarily during operations, then zeroized
//! - All sensitive data is encrypted and only decrypted temporarily
//! - Encrypted blobs are stored, never plaintext keys
//!
//! Architecture:
//! - `EnclaveHandle`: Public API for sending messages to the actor
//! - `EnclaveActor`: State owner that processes messages securely
//! - `EnclaveMessage`: Command/query enums for all operations
//! - `EnclaveState`: Current enclave state (wallets, temporary UEK)
//! - `EnclaveError`: Domain-specific error types

pub mod actor;
pub mod crypto;
pub mod errors;
pub mod handle;
pub mod messages;
pub mod wallet;

// Re-export the primary public API types
pub use actor::{EnclaveActor, run_enclave_actor};
pub use actor::{EnclaveState, TradeIntent, WalletInfo, WalletMetadata};
pub use crypto::{CryptoError, CryptoResult, EnclaveTransportKeys, KEY_SIZE, UsersEncryptionKeys};
pub use errors::{EnclaveError, EnclaveResult};
pub use handle::EnclaveHandle;
pub use messages::{EnclaveEvent, EnclaveMessage, EnclaveRequest, EnclaveResponse};

// Re-export wallet types for backward compatibility
pub use wallet::types::{EncryptedWalletBlob, Wallet, WalletError, WalletList, WalletResult};

// Re-export wallet operations - use the actual exported names
pub use wallet::{create_wallet, import_wallet};
