//! Keystore domain handle.
//!
//! Provides the public API for interacting with the keystore domain.
//! Sends messages to the actor and receives responses.

use tokio::sync::mpsc;

use crate::domains::config::Config;
use crate::event_bus::EventBus;

use super::actor::{KeystoreActor, KeystoreStatus, run_keystore_actor};
use super::errors::{KeystoreError, KeystoreResult};
use super::messages::{BackendType, KeystoreMessage, KeystoreRequest};

/// Handle for interacting with the keystore domain.
#[derive(Debug, Clone)]
pub struct KeystoreHandle {
    /// Sender for requests to the actor.
    sender: mpsc::Sender<KeystoreRequest>,
}

impl KeystoreHandle {
    /// Create a new keystore handle with the given config, backend, receiver, and EventBus.
    ///
    /// Spawns the actor internally and returns the handle.
    ///
    /// # Arguments
    /// * `config` - Configuration data
    /// * `backend` - Keyring or Filestore backend type
    /// * `receiver` - The mpsc receiver channel for the actor
    /// * `event_bus` - EventBus for publishing state events
    pub async fn new(
        config: Config,
        backend: BackendType,
        receiver: mpsc::Receiver<KeystoreRequest>,
        event_bus: EventBus,
    ) -> KeystoreResult<Self> {
        let (sender, _rx) = mpsc::channel::<KeystoreRequest>(64);
        let tx = sender.clone();

        // Create the actor using the simple constructor
        let actor = KeystoreActor::new(config, backend, event_bus);

        // Spawn the actor task
        tokio::spawn(async move {
            run_keystore_actor(actor, receiver).await;
        });

        Ok(Self { sender: tx })
    }

    /// Create a KeystoreHandle from an existing sender.
    ///
    /// Used internally when the actor is already spawned.
    pub fn from_sender(sender: mpsc::Sender<KeystoreRequest>) -> Self {
        Self { sender }
    }

    /// Send a message and await a response.
    async fn send_message(&self, message: KeystoreMessage) -> KeystoreResult<KeystoreStatus> {
        let (reply_to, rx) = tokio::sync::oneshot::channel();
        let request = KeystoreRequest {
            payload: message,
            reply_to,
            trace_ctx: crate::event_bus::TraceContext::new(),
        };

        self.sender
            .send(request)
            .await
            .map_err(|_| KeystoreError::ChannelError)?;

        rx.await.map_err(|_| KeystoreError::ChannelError)?
    }

    /// Unlock the keystore with password.
    pub async fn unlock(&self, password: String) -> KeystoreResult<KeystoreStatus> {
        self.send_message(KeystoreMessage::Unlock { password })
            .await
    }

    /// Lock the keystore.
    pub async fn lock(&self) -> KeystoreResult<KeystoreStatus> {
        self.send_message(KeystoreMessage::Lock).await
    }

    /// Change the password.
    pub async fn change_password(&self, old_password: String, new_password: String) -> KeystoreResult<KeystoreStatus> {
        self.send_message(KeystoreMessage::ChangePassword {
            old_password,
            new_password,
        })
        .await
    }

    /// Get the current keystore status.
    pub async fn get_status(&self) -> KeystoreResult<KeystoreStatus> {
        self.send_message(KeystoreMessage::GetStatus).await
    }
}
