//! keystore domain - Key storage backends (keyring + filestore)
//!
//! This domain manages secure key storage using either:
//! - Filestore (password-based encryption with filesystem storage)
//! - Keyring (OS-native keyring storage)
//!
//! The domain follows the actor/handler pattern:
//! - `KeystoreHandle`: Public API for sending messages to the actor
//! - `KeystoreActor`: State owner that processes messages
//! - `KeystoreMessage`: Command/query enums
//! - `KeystoreStatus`: Current keystore status (backend + status)
//! - `KeystoreError`: Domain-specific error types

pub mod actor;
pub mod auth;
pub mod crypto;
pub mod errors;
pub mod filestore;
pub mod handle;
pub mod keyring;
pub mod messages;
pub mod session;

// Re-export main types
pub use actor::{KeystoreActor, KeystoreStatus};
pub use errors::KeystoreError;
pub use handle::KeystoreHandle;
pub use messages::{
    BackendType, KeystoreEvent, KeystoreMessage, KeystoreRequest, KeystoreResponse, KeystoreStatusInfo,
};

/// Re-export crypto modules for backward compatibility
/// Migrated from commands/key/filestore/crypto
pub use crypto::{derivation, encryption, storage, types as crypto_types};

/// Re-export auth modules for backward compatibility
/// Migrated from commands/key/filestore/auth
pub use auth::{passkey, password, types as auth_types};

/// Re-export session modules for backward compatibility
/// Migrated from session/
pub use session::{crypto as session_crypto, filestore as session_filestore, keyring as session_keyring};

/// Re-export filestore operations
/// Migrated from commands/key/filestore
pub use filestore::{
    key_create, key_create_with_context, key_delete, key_lock, key_unlock, key_unlock_with_context, key_update,
};

/// Re-export keyring operations
/// Migrated from commands/key/keyring
pub use keyring::{
    keyring_create, keyring_create_with_context, keyring_delete, keyring_lock, keyring_unlock, keyring_update,
};

// Re-export crypto types with their original names for compatibility
pub use crypto::types::{
    CryptoError, CryptoResult, EncryptedData, MasterKey, NONCE_SIZE, PBKDF2_ITERATIONS, SALT_SIZE, TAG_SIZE,
};

// Re-export auth types for compatibility
pub use auth::types::{AuthError, AuthResult, AuthenticationMethod, AuthenticationResult, Authenticator};

// Re-export session types for compatibility
pub use session::crypto::{CryptoError as SessionCryptoError, EnclaveTransportKeys, KEY_SIZE, UsersEncryptionKeys};

/// Re-export the unified Session enum
pub use session::{Session, SessionError};

pub use session::filestore::Session as FileStoreSession;
pub use session::filestore::SessionError as FileStoreSessionError;

pub use session::keyring::{
    Entry as KeyringEntry, KEYRING_SERVICE, KEYRING_USERNAME, Session as KeyringSession,
    SessionError as KeyringSessionError,
};

/// Check if the OS keyring is available.
///
/// Attempts to access the system keyring to determine if it's
/// available for secure key storage.
///
/// # Returns
/// `true` if the keyring is available, `false` otherwise.
pub fn keyring_available() -> bool {
    use crate::domains::keystore::session_keyring::Entry;
    // Try to access the keyring with a test entry
    Entry::new("edge_test", "capability_check")
        .and_then(|e| e.get_password())
        .is_ok()
        || Entry::new("edge_test", "capability_check")
            .and_then(|e| e.set_password("test"))
            .is_ok()
}
