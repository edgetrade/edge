//! Client domain handle
//!
//! Thin gateway - public API that sends messages to the actor
//! Uses PoseidonRequest pattern for req/resp communication.
//!
//! Migrated from `client/mod.rs` and `client/trpc.rs`

use serde_json::Value;
use tokio::sync::mpsc::{self, UnboundedReceiver};

use crate::domains::client::actor::ClientActor;
use crate::domains::client::errors::ClientError;
use crate::domains::client::manifest::types::McpManifest;
use crate::domains::client::messages::{ClientMessage, ClientRequest, ClientResponse};
use crate::domains::enclave::wallet::types::Wallet;
use crate::domains::keystore::session::crypto::UsersEncryptionKeys;
use crate::event_bus::{EventBus, PoseidonRequest, TraceContext};

/// Public handle for the client domain
///
/// Provides a thin interface that sends messages to the ClientActor
/// and receives responses via oneshot channels using PoseidonRequest pattern.
#[derive(Debug, Clone)]
pub struct ClientHandle {
    sender: mpsc::Sender<ClientRequest>,
}

impl ClientHandle {
    /// Create a new client handle with EventBus.
    ///
    /// Spawns the actor internally and returns the handle.
    ///
    /// # Arguments
    /// * `event_bus` - EventBus for publishing state events
    pub fn new(event_bus: EventBus) -> Self {
        let sender = ClientActor::spawn(event_bus);
        Self { sender }
    }

    /// Create a client handle from an existing sender.
    ///
    /// Used internally when the actor is already spawned.
    pub fn from_sender(sender: mpsc::Sender<ClientRequest>) -> Self {
        Self { sender }
    }

    /// Send a request to the actor and wait for response
    ///
    /// Internal helper method that wraps the PoseidonRequest pattern.
    /// Creates a oneshot channel, sends the request with trace context,
    /// and awaits the response.
    async fn send_request(&self, payload: ClientMessage) -> Result<ClientResponse, ClientError> {
        let (reply_to, rx) = tokio::sync::oneshot::channel();

        let request = PoseidonRequest {
            payload,
            trace_ctx: TraceContext::current(),
            reply_to,
        };

        self.sender
            .send(request)
            .await
            .map_err(|_| ClientError::ChannelSend)?;

        rx.await.map_err(|_| ClientError::ChannelRecv)?
    }

    /// Connect to the Iris API
    ///
    /// # Arguments
    /// * `url` - Iris API URL (wss:// or ws://)
    /// * `api_key` - API key for authentication
    /// * `verbose` - Enable verbose logging
    pub async fn connect(&self, url: String, api_key: String, verbose: bool) -> Result<(), ClientError> {
        self.send_request(ClientMessage::Connect { url, api_key, verbose })
            .await
            .map(|_| ())
    }

    /// Disconnect from the Iris API
    pub async fn disconnect(&self) -> Result<(), ClientError> {
        self.send_request(ClientMessage::Disconnect)
            .await
            .map(|_| ())
    }

    /// Execute a query
    ///
    /// # Arguments
    /// * `path` - tRPC procedure path (e.g., "agent.listEncryptedWallets")
    /// * `input` - Query input parameters
    pub async fn query<T: serde::de::DeserializeOwned + serde::Serialize + Clone>(
        &self,
        path: &str,
        input: Value,
    ) -> Result<T, ClientError> {
        let response = self
            .send_request(ClientMessage::Query {
                path: path.to_string(),
                input,
            })
            .await?;

        match response {
            ClientResponse::QueryResult(value) => {
                serde_json::from_value(value).map_err(|e| ClientError::Deserialization(e.to_string()))
            }
            _ => Err(ClientError::InvalidResponse("Unexpected response type".to_string())),
        }
    }

    /// Execute a mutation
    ///
    /// # Arguments
    /// * `path` - tRPC procedure path
    /// * `input` - Mutation input parameters
    pub async fn mutation<T: serde::de::DeserializeOwned>(&self, path: &str, input: Value) -> Result<T, ClientError> {
        let response = self
            .send_request(ClientMessage::Mutation {
                path: path.to_string(),
                input,
            })
            .await?;

        match response {
            ClientResponse::MutationResult(value) => {
                serde_json::from_value(value).map_err(|e| ClientError::Deserialization(e.to_string()))
            }
            _ => Err(ClientError::InvalidResponse("Unexpected response type".to_string())),
        }
    }

    /// Subscribe to a real-time event stream
    ///
    /// # Arguments
    /// * `path` - tRPC subscription procedure path
    /// * `input` - Subscription input parameters
    ///
    /// # Returns
    /// Subscription ID and receiver channel for events
    pub async fn subscribe(&self, path: &str, input: Value) -> Result<(u32, UnboundedReceiver<Value>), ClientError> {
        let response = self
            .send_request(ClientMessage::Subscribe {
                path: path.to_string(),
                input,
            })
            .await?;

        match response {
            ClientResponse::Subscribed { id, receiver } => Ok((id, receiver)),
            _ => Err(ClientError::InvalidResponse("Unexpected response type".to_string())),
        }
    }

    /// Unsubscribe from an event stream
    ///
    /// # Arguments
    /// * `subscription_id` - ID of the subscription to cancel
    pub async fn unsubscribe(&self, subscription_id: u32) -> Result<(), ClientError> {
        self.send_request(ClientMessage::Unsubscribe { subscription_id })
            .await
            .map(|_| ())
    }

    /// Get the current MCP manifest
    ///
    /// # Returns
    /// Current MCP manifest containing tools, resources, prompts, and skills
    pub async fn get_manifest(&self) -> Result<McpManifest, ClientError> {
        let response = self.send_request(ClientMessage::GetManifest).await?;

        match response {
            ClientResponse::Manifest(manifest) => Ok(manifest),
            _ => Err(ClientError::InvalidResponse("Unexpected response type".to_string())),
        }
    }

    /// Refresh the manifest from server
    pub async fn refresh_manifest(&self) -> Result<(), ClientError> {
        self.send_request(ClientMessage::RefreshManifest)
            .await
            .map(|_| ())
    }

    /// Ping the server
    pub async fn ping(&self) -> Result<(), ClientError> {
        self.send_request(ClientMessage::Ping).await.map(|_| ())
    }

    /// Subscribe for dispatch (for alerts)
    ///
    /// # Arguments
    /// * `procedure` - tRPC procedure to subscribe to
    /// * `input` - Subscription parameters
    /// * `alert_id` - Alert identifier for dispatch routing
    /// * `alert_name` - Human-readable alert name
    ///
    /// # Returns
    /// Subscription ID for the dispatch subscription
    pub async fn subscribe_for_dispatch(
        &self,
        procedure: &str,
        input: Value,
        alert_id: u64,
        alert_name: String,
    ) -> Result<u32, ClientError> {
        let response = self
            .send_request(ClientMessage::SubscribeForDispatch {
                procedure: procedure.to_string(),
                input,
                alert_id,
                alert_name,
            })
            .await?;

        match response {
            ClientResponse::DispatchSubscribed { id } => Ok(id),
            _ => Err(ClientError::InvalidResponse("Unexpected response type".to_string())),
        }
    }

    /// Get the Iris client (for direct route execution)
    ///
    /// Returns a clone of the IrisClient if connected.
    pub async fn get_client(&self) -> Result<Option<crate::domains::client::trpc::IrisClient>, ClientError> {
        let response = self.send_request(ClientMessage::GetClient).await?;

        match response {
            ClientResponse::Client(client) => Ok(client),
            _ => Err(ClientError::InvalidResponse("Unexpected response type".to_string())),
        }
    }

    /// Get the sender channel for direct message sending
    ///
    /// Used by IPC domain for direct routing to client domain.
    pub fn sender(&self) -> &mpsc::Sender<ClientRequest> {
        &self.sender
    }

    // === Route-based operations (moved from routes.rs) ===

    /// Create or update an encrypted wallet.
    pub async fn upsert_wallet(&self, wallet: Wallet, user_key: &UsersEncryptionKeys) -> Result<(), ClientError> {
        let response = self
            .send_request(ClientMessage::UpsertWallet {
                wallet,
                user_key: user_key.clone(),
            })
            .await?;

        match response {
            ClientResponse::WalletUpdated => Ok(()),
            _ => Err(ClientError::InvalidResponse("Unexpected response type".to_string())),
        }
    }

    /// List all encrypted wallets for the current agent.
    pub async fn list_wallets(&self) -> Result<Vec<crate::domains::enclave::wallet::types::WalletList>, ClientError> {
        let response = self.send_request(ClientMessage::ListWallets).await?;

        match response {
            ClientResponse::WalletList(wallets) => Ok(wallets),
            _ => Err(ClientError::InvalidResponse("Unexpected response type".to_string())),
        }
    }

    /// Delete an encrypted wallet by address.
    pub async fn delete_wallet(&self, address: String) -> Result<(), ClientError> {
        let response = self
            .send_request(ClientMessage::DeleteWallet { address })
            .await?;

        match response {
            ClientResponse::WalletDeleted => Ok(()),
            _ => Err(ClientError::InvalidResponse("Unexpected response type".to_string())),
        }
    }

    /// Rotate the user encryption key.
    pub async fn rotate_user_encryption_key(
        &self,
        new_key: &UsersEncryptionKeys,
        old_key: &UsersEncryptionKeys,
    ) -> Result<(), ClientError> {
        let response = self
            .send_request(ClientMessage::RotateUserEncryptionKey {
                new_key: new_key.clone(),
                old_key: old_key.clone(),
            })
            .await?;

        match response {
            ClientResponse::KeyRotated => Ok(()),
            _ => Err(ClientError::InvalidResponse("Unexpected response type".to_string())),
        }
    }

    /// Conduct the proof game.
    pub async fn proof_game(
        &self,
        request: &crate::domains::client::generated::routes::requests::agent_proof_game::ProofGameRequest,
    ) -> Result<crate::domains::client::generated::routes::requests::agent_proof_game::ProofGameResponse, ClientError>
    {
        let response = self
            .send_request(ClientMessage::ProofGame {
                request: request.clone(),
            })
            .await?;

        match response {
            ClientResponse::ProofGameResult(result) => Ok(result),
            _ => Err(ClientError::InvalidResponse("Unexpected response type".to_string())),
        }
    }
}

impl Default for ClientHandle {
    fn default() -> Self {
        // Create a dummy sender for default implementation
        let (sender, _rx) = mpsc::channel(1);
        Self::from_sender(sender)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_client_handle_new() {
        let event_bus = EventBus::new(128);
        let _handle = ClientHandle::new(event_bus);
        // Just verify it creates without error
    }

    #[test]
    fn test_client_handle_default() {
        let _handle: ClientHandle = Default::default();
        // Just verify Default impl works
    }
}
