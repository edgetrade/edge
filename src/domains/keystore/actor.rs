//! Keystore domain actor.
//!
//! Owns the keystore state and processes messages from the handle.
//! Supports both filestore (password-based) and keyring backends.

use tokio::sync::mpsc;

use crate::domains::config::Config;
use crate::domains::keystore::auth::password::verify_password;
use crate::domains::keystore::crypto::storage::{default_blind_user_key_path, default_salt_path, load_salt};
use crate::domains::keystore::filestore::{key_lock, key_unlock};
use crate::domains::keystore::keyring::{keyring_lock, keyring_unlock};
use crate::event_bus::{EventBus, StateEvent};

use super::errors::{KeystoreError, KeystoreResult};
use super::messages::{BackendType, KeystoreMessage, KeystoreRequest};

/// The keystore actor that owns keystore state.
pub struct KeystoreActor {
    /// The current config.
    config: Config,
    /// The backend type (filestore or keyring).
    backend: BackendType,
    /// EventBus for publishing state events.
    event_bus: EventBus,
    /// The current keystore state.
    status: KeystoreStatus,
}

impl KeystoreActor {
    /// Create a new keystore actor.
    pub fn new(config: Config, backend: BackendType, event_bus: EventBus) -> Self {
        Self {
            config,
            backend,
            event_bus,
            status: KeystoreStatus::Locked,
        }
    }

    /// Run the actor loop.
    pub async fn run(mut self, mut receiver: mpsc::Receiver<KeystoreRequest>) {
        while let Some(req) = receiver.recv().await {
            let reply = match req.payload {
                KeystoreMessage::Unlock { password } => self.unlock(password).await,
                KeystoreMessage::Lock => self.lock().await,
                KeystoreMessage::ChangePassword {
                    old_password,
                    new_password,
                } => self.change_password(old_password, new_password).await,
                KeystoreMessage::GetStatus => Ok(self.get_status()),
            };

            let _ = req.reply_to.send(reply);
        }
    }

    /// Emit a StateEvent to the EventBus.
    fn emit_state_event(&self, event: StateEvent) {
        let _ = self.event_bus.publish(event);
    }

    /// Unlock the keystore.
    async fn unlock(&mut self, password: String) -> KeystoreResult<KeystoreStatus> {
        match self.backend {
            BackendType::Filestore => {
                // Get paths
                let salt_path = default_salt_path()
                    .ok_or_else(|| KeystoreError::Storage("Could not determine salt path".to_string()))?;
                let blind_key_path = default_blind_user_key_path()
                    .ok_or_else(|| KeystoreError::Storage("Could not determine key path".to_string()))?;

                // Load salt and verify password
                let salt_bytes = load_salt(&salt_path).map_err(|e| KeystoreError::Storage(e.to_string()))?;

                if salt_bytes.len() != 16 {
                    return Err(KeystoreError::Crypto("Invalid salt size".to_string()));
                }

                let mut salt = [0u8; 16];
                salt.copy_from_slice(&salt_bytes);

                // Verify password
                verify_password(&password, &salt).map_err(|_e| KeystoreError::WrongPassword)?;

                // Actually unlock
                key_unlock(self.config.clone()).map_err(|e| KeystoreError::Storage(e.to_string()))?;

                // Load encrypted key for state
                let encrypted_key = tokio::fs::read(&blind_key_path)
                    .await
                    .map_err(|e| KeystoreError::Storage(e.to_string()))?;

                // Update state
                self.status = KeystoreStatus::Unlocked {
                    salt: salt_bytes,
                    encrypted_key,
                };

                // Emit KeystoreUnlocked event after success
                self.emit_state_event(StateEvent::KeystoreUnlocked);

                Ok(self.status.clone())
            }
            BackendType::Keyring => {
                // Keyring doesn't need password to unlock
                keyring_unlock().map_err(|e| KeystoreError::Keyring(e.to_string()))?;

                // Keyring is "unlocked" when we can access the key
                self.status = KeystoreStatus::Unlocked {
                    salt: vec![],          // Keyring doesn't use salt in the same way
                    encrypted_key: vec![], // Key material stays in keyring
                };

                // Emit KeystoreUnlocked event after success
                self.emit_state_event(StateEvent::KeystoreUnlocked);

                Ok(self.status.clone())
            }
        }
    }

    /// Lock the keystore.
    async fn lock(&mut self) -> KeystoreResult<KeystoreStatus> {
        match self.backend {
            BackendType::Filestore => {
                key_lock(self.config.clone()).map_err(|e| KeystoreError::Storage(e.to_string()))?;
            }
            BackendType::Keyring => {
                keyring_lock().map_err(|e| KeystoreError::Keyring(e.to_string()))?;
            }
        }

        // Update state
        self.status = KeystoreStatus::Locked;

        // Emit KeystoreLocked event after success
        self.emit_state_event(StateEvent::KeystoreLocked);

        Ok(self.status.clone())
    }

    /// Change the password.
    async fn change_password(&mut self, old_password: String, _new_password: String) -> KeystoreResult<KeystoreStatus> {
        // First verify old password by unlocking
        self.unlock(old_password).await?;

        // Password change is backend-specific
        match self.backend {
            BackendType::Filestore => {
                // For filestore, we need to re-wrap the UEK with the new password
                // This requires a client for key rotation
                // TODO: Implement password change with key rotation
                // For now, just return the current state
                Ok(self.status.clone())
            }
            BackendType::Keyring => {
                // For keyring, password doesn't apply
                // The keyring entry is managed separately
                Ok(self.status.clone())
            }
        }
    }

    /// Get the current status.
    fn get_status(&self) -> KeystoreStatus {
        self.status.clone()
    }
}

/// Start the keystore actor.
pub async fn run_keystore_actor(actor: KeystoreActor, receiver: mpsc::Receiver<KeystoreRequest>) {
    actor.run(receiver).await;
}

// ============================================================================
// State Types
// ============================================================================

/// The lock status of the keystore.
#[derive(Debug, Clone, Default)]
pub enum KeystoreStatus {
    /// Keystore is locked, no key material in memory.
    #[default]
    Locked,
    /// Keystore is unlocked, key material available.
    Unlocked {
        /// The salt used for key derivation.
        salt: Vec<u8>,
        /// The encrypted master key.
        encrypted_key: Vec<u8>,
    },
}

impl KeystoreStatus {
    /// Check if the keystore is unlocked.
    pub fn is_unlocked(&self) -> bool {
        matches!(self, KeystoreStatus::Unlocked { .. })
    }

    /// Check if the keystore is locked.
    pub fn is_locked(&self) -> bool {
        matches!(self, KeystoreStatus::Locked)
    }

    /// Get the salt if unlocked.
    pub fn salt(&self) -> Option<&[u8]> {
        match self {
            KeystoreStatus::Unlocked { salt, .. } => Some(salt),
            KeystoreStatus::Locked => None,
        }
    }

    /// Get the encrypted key if unlocked.
    pub fn encrypted_key(&self) -> Option<&[u8]> {
        match self {
            KeystoreStatus::Unlocked { encrypted_key, .. } => Some(encrypted_key),
            KeystoreStatus::Locked => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keystore_status_default() {
        let status = KeystoreStatus::default();
        assert!(status.is_locked());
        assert!(!status.is_unlocked());
    }

    #[test]
    fn test_keystore_status_unlocked() {
        let status = KeystoreStatus::Unlocked {
            salt: vec![1, 2, 3],
            encrypted_key: vec![4, 5, 6],
        };
        assert!(!status.is_locked());
        assert!(status.is_unlocked());
        assert_eq!(status.salt(), Some(&[1, 2, 3][..]));
        assert_eq!(status.encrypted_key(), Some(&[4, 5, 6][..]));
    }

    #[test]
    fn test_keystore_status_locked_accessors() {
        let status = KeystoreStatus::Locked;
        assert_eq!(status.salt(), None);
        assert_eq!(status.encrypted_key(), None);
    }
}
