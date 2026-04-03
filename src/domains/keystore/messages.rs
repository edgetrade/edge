//! Keystore domain messages.
//!
//! Defines all command and query messages for the keystore domain,
//! used for communication between handle and actor.

use serde::{Deserialize, Serialize};

use crate::domains::keystore::session::crypto::UsersEncryptionKeys;
use crate::event_bus::PoseidonRequest;

use super::actor::KeystoreStatus;
use super::errors::KeystoreError;

/// Messages sent to the keystore actor.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KeystoreMessage {
    /// Unlock the keystore with password.
    Unlock { password: String },

    /// Lock the keystore.
    Lock,

    /// Change password (re-wrap UEK with new KWK).
    ChangePassword { old_password: String, new_password: String },

    /// Get the current keystore status.
    GetStatus,
}

/// Request type alias using PoseidonRequest pattern.
pub type KeystoreRequest = PoseidonRequest<KeystoreMessage, KeystoreStatus, KeystoreError>;

/// Response types for keystore operations.
#[derive(Debug, Clone)]
#[allow(clippy::large_enum_variant)]
pub enum KeystoreResponse {
    /// Operation succeeded with no data.
    Success,

    /// Operation succeeded with status.
    Status(KeystoreStatus),

    /// Operation succeeded with UEK.
    UserEncryptionKey(UsersEncryptionKeys),

    /// Operation succeeded with unlock state.
    Unlocked(bool),

    /// Operation failed.
    Error(KeystoreError),
}

/// Keystore status information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeystoreStatusInfo {
    /// Whether the keystore is unlocked.
    pub is_unlocked: bool,
    /// The backend type (filestore or keyring).
    pub backend: BackendType,
    /// Whether keys exist.
    pub keys_exist: bool,
}

/// Backend type for keystore storage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BackendType {
    /// File-based storage with password encryption.
    Filestore,
    /// OS keyring storage.
    Keyring,
}

impl std::fmt::Display for BackendType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BackendType::Filestore => write!(f, "filestore"),
            BackendType::Keyring => write!(f, "keyring"),
        }
    }
}

/// Events emitted by the keystore domain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KeystoreEvent {
    /// Keystore was unlocked.
    Unlocked,
    /// Keystore was locked.
    Locked,
    /// New key was created.
    KeyCreated,
    /// Key was deleted.
    KeyDeleted,
    /// Key was updated.
    KeyUpdated,
    /// Authentication failed.
    AuthenticationFailed { reason: String },
}
