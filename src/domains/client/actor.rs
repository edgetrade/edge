//! Client domain actor
//!
//! State owner for the client domain - manages Iris API connection,
//! manifest fetching/caching, and background refresh.
//! Uses PoseidonRequest pattern for message handling.

use std::sync::Arc;

use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::Value;
use tokio::sync::RwLock;
use tokio::sync::mpsc;
use tokio::time::{Duration, interval};

use tyche_enclave::envelopes::transport::TransportEnvelope;
use tyche_enclave::types::chain_type::ChainType;

use crate::domains::client::errors::ClientError;
use crate::domains::client::generated::routes::requests::agent_rotate_user_encryption_key;
use crate::domains::client::manifest::ManifestManager;
use crate::domains::client::messages::{ClientMessage, ClientRequest, ClientResponse};
use crate::domains::client::state::ClientState;
use crate::domains::client::trpc::{Route, RouteType};
use crate::event_bus::{EventBus, StateEvent};

/// Client actor state owner
///
/// Migrated from `client/mod.rs`, `client/trpc.rs`, `manifest/manager.rs`
/// Handles all client domain operations and emits StateEvents.
pub struct ClientActor {
    /// Client state (iris_client, manifest, etc.)
    state: ClientState,
    /// EventBus for publishing state events
    event_bus: EventBus,
}

impl ClientActor {
    /// Create a new client actor
    ///
    /// # Arguments
    /// * `event_bus` - EventBus for publishing state events
    pub fn new(event_bus: EventBus) -> Self {
        Self {
            state: ClientState::new(),
            event_bus,
        }
    }

    /// Run the actor message loop
    ///
    /// Processes incoming ClientRequest messages and sends responses
    /// via the oneshot reply channel.
    ///
    /// # Arguments
    /// * `receiver` - Channel receiver for ClientRequest messages
    pub async fn run(mut self, mut receiver: mpsc::Receiver<ClientRequest>) {
        while let Some(req) = receiver.recv().await {
            let reply = self.handle_message(req.payload).await;
            let _ = req.reply_to.send(reply);
        }
    }

    /// Spawn the actor and return the sender channel
    ///
    /// This is the primary way to start the client actor.
    ///
    /// # Arguments
    /// * `event_bus` - EventBus for publishing state events
    ///
    /// # Returns
    /// The sender half of the channel for sending ClientRequest messages
    pub fn spawn(event_bus: EventBus) -> mpsc::Sender<ClientRequest> {
        let (sender, receiver) = mpsc::channel::<ClientRequest>(64);

        let actor = Self::new(event_bus);
        tokio::spawn(async move {
            actor.run(receiver).await;
        });

        sender
    }

    /// Emit a StateEvent to the EventBus.
    fn emit_state_event(&self, event: StateEvent) {
        if let Err(_e) = self.event_bus.publish(event) {
            // EventBus publish error is non-critical
        }
    }

    /// Handle incoming messages
    async fn handle_message(&mut self, payload: ClientMessage) -> Result<ClientResponse, ClientError> {
        match payload {
            ClientMessage::Connect { url, api_key, verbose } => self.connect(url, api_key, verbose).await,
            ClientMessage::Disconnect => self.disconnect().await,
            ClientMessage::Query { path, input } => self.query(path, input).await,
            ClientMessage::Mutation { path, input } => self.mutation(path, input).await,
            ClientMessage::Subscribe { path, input } => self.subscribe(path, input).await,
            ClientMessage::SubscribeForDispatch {
                procedure,
                input,
                alert_id,
                alert_name,
            } => {
                self.subscribe_for_dispatch(procedure, input, alert_id, alert_name)
                    .await
            }
            ClientMessage::Unsubscribe { subscription_id } => self.unsubscribe(subscription_id).await,
            ClientMessage::GetManifest => self.get_manifest().await,
            ClientMessage::RefreshManifest => self.refresh_manifest().await,
            ClientMessage::Ping => self.ping().await,
            ClientMessage::GetClient => self.get_client(),
            // Route messages
            ClientMessage::UpsertWallet { wallet, user_key } => self.upsert_wallet(wallet, user_key).await,
            ClientMessage::ListWallets => self.list_wallets().await,
            ClientMessage::DeleteWallet { address } => self.delete_wallet(address).await,
            ClientMessage::RotateUserEncryptionKey { new_key, old_key } => {
                self.rotate_user_encryption_key(new_key, old_key).await
            }
            ClientMessage::ProofGame { request } => self.proof_game(request).await,
            ClientMessage::ExecuteRoute { path, input } => {
                // Use query as the default for generic route execution
                self.query(path, input).await
            }
        }
    }

    /// Connect to Iris API
    ///
    /// # Arguments
    /// * `url` - Iris API URL
    /// * `api_key` - API key for authentication
    /// * `verbose` - Enable verbose logging
    ///
    /// # Emits
    /// * `StateEvent::ClientConnected` - On successful connection
    async fn connect(&mut self, url: String, api_key: String, verbose: bool) -> Result<ClientResponse, ClientError> {
        // Connect using trpc module's IrisClient
        let client = crate::domains::client::trpc::IrisClient::connect(&url, &api_key, verbose)
            .await
            .map_err(ClientError::from_iris_error)?;

        self.state.iris_client = Some(client);
        self.state.url = Some(url.clone());
        self.state.api_key = Some(api_key.clone());
        self.state.verbose = verbose;

        // Initialize manifest manager
        let manifest_url = url.replace("wss://", "https://").replace("ws://", "http:");
        match ManifestManager::new(format!("{}/manifest", manifest_url), api_key, true).await {
            Ok(manager) => {
                self.state.manifest_manager = Some(Arc::new(RwLock::new(manager)));
            }
            Err(e) => {
                return Err(ClientError::Manifest(format!("{}", e)));
            }
        }

        // Start background manifest refresh (60s interval)
        self.start_manifest_refresh().await;

        // Emit ClientConnected event
        self.emit_state_event(StateEvent::ClientConnected { url });

        Ok(ClientResponse::Connected)
    }

    /// Disconnect from Iris API
    ///
    /// Cancels manifest refresh task and clears state.
    ///
    /// # Emits
    /// * `StateEvent::ClientDisconnected` - On successful disconnect
    async fn disconnect(&mut self) -> Result<ClientResponse, ClientError> {
        // Cancel manifest refresh task
        if let Some(handle) = self.state.manifest_refresh.take() {
            handle.abort();
        }

        self.state.iris_client = None;
        self.state.manifest_manager = None;
        self.state.manifest = None;

        // Emit ClientDisconnected event
        self.emit_state_event(StateEvent::ClientDisconnected);

        Ok(ClientResponse::Disconnected)
    }

    /// Execute a query
    async fn query(&self, path: String, input: Value) -> Result<ClientResponse, ClientError> {
        self.execute_raw_query(&path, input).await
    }

    /// Execute a mutation
    async fn mutation(&self, path: String, input: Value) -> Result<ClientResponse, ClientError> {
        self.execute_raw_mutation(&path, input).await
    }

    /// Internal helper to execute a raw query using the internal trpc functions
    async fn execute_raw_query(&self, path: &str, input: Value) -> Result<ClientResponse, ClientError> {
        use crate::domains::client::trpc::call;

        let client = self
            .state
            .iris_client
            .as_ref()
            .ok_or_else(|| ClientError::Connection("Not connected".to_string()))?;

        let result: Value = call(&client.inner, path, input)
            .await
            .map_err(ClientError::from_iris_error)?;

        Ok(ClientResponse::QueryResult(result))
    }

    /// Internal helper to execute a raw mutation using the internal trpc functions
    async fn execute_raw_mutation(&self, path: &str, input: Value) -> Result<ClientResponse, ClientError> {
        use crate::domains::client::trpc::call;

        let client = self
            .state
            .iris_client
            .as_ref()
            .ok_or_else(|| ClientError::Connection("Not connected".to_string()))?;

        let result: Value = call(&client.inner, path, input)
            .await
            .map_err(ClientError::from_iris_error)?;

        Ok(ClientResponse::MutationResult(result))
    }

    /// Subscribe to a real-time event stream
    async fn subscribe(&self, path: String, input: Value) -> Result<ClientResponse, ClientError> {
        let client = self
            .state
            .iris_client
            .as_ref()
            .ok_or_else(|| ClientError::Connection("Not connected".to_string()))?;

        let (id, receiver) = client
            .subscribe(&path, input)
            .await
            .map_err(ClientError::from_iris_error)?;

        Ok(ClientResponse::Subscribed { id, receiver })
    }

    /// Subscribe for dispatch (alert delivery)
    async fn subscribe_for_dispatch(
        &self,
        procedure: String,
        input: Value,
        _alert_id: u64,
        _alert_name: String,
    ) -> Result<ClientResponse, ClientError> {
        let client = self
            .state
            .iris_client
            .as_ref()
            .ok_or_else(|| ClientError::Connection("Not connected".to_string()))?;

        let (id, _receiver) = client
            .subscribe(&procedure, input)
            .await
            .map_err(ClientError::from_iris_error)?;

        Ok(ClientResponse::DispatchSubscribed { id })
    }

    /// Unsubscribe from an event stream
    async fn unsubscribe(&self, subscription_id: u32) -> Result<ClientResponse, ClientError> {
        let client = self
            .state
            .iris_client
            .as_ref()
            .ok_or_else(|| ClientError::Connection("Not connected".to_string()))?;

        client
            .unsubscribe(subscription_id)
            .await
            .map_err(ClientError::from_iris_error)?;

        Ok(ClientResponse::Unsubscribed)
    }

    /// Get the current manifest
    async fn get_manifest(&self) -> Result<ClientResponse, ClientError> {
        let manager = self
            .state
            .manifest_manager
            .as_ref()
            .ok_or_else(|| ClientError::Manifest("Manifest manager not initialized".to_string()))?;

        let manifest = manager.read().await.manifest().read().await.clone();
        Ok(ClientResponse::Manifest(manifest))
    }

    /// Refresh the manifest from server
    ///
    /// # Emits
    /// * `StateEvent::ManifestUpdated` - On successful refresh
    async fn refresh_manifest(&self) -> Result<ClientResponse, ClientError> {
        let manager = self
            .state
            .manifest_manager
            .as_ref()
            .ok_or_else(|| ClientError::Manifest("Manifest manager not initialized".to_string()))?;

        let guard = manager.read().await;
        guard
            .refresh()
            .await
            .map_err(|e| ClientError::Manifest(format!("{}", e)))?;

        // Emit ManifestUpdated event
        self.emit_state_event(StateEvent::ManifestUpdated {
            version: "unknown".to_string(),
        });

        Ok(ClientResponse::ManifestRefreshed)
    }

    /// Ping the server
    async fn ping(&self) -> Result<ClientResponse, ClientError> {
        let client = self
            .state
            .iris_client
            .as_ref()
            .ok_or_else(|| ClientError::Connection("Not connected".to_string()))?;

        client.ping().await.map_err(ClientError::from_iris_error)?;

        Ok(ClientResponse::Pong)
    }

    /// Get the Iris client
    fn get_client(&self) -> Result<ClientResponse, ClientError> {
        let client = self.state.iris_client.clone();
        Ok(ClientResponse::Client(client))
    }

    /// Start background manifest refresh task
    ///
    /// Runs every 60 seconds to keep manifest fresh.
    async fn start_manifest_refresh(&mut self) {
        let manager = match &self.state.manifest_manager {
            Some(m) => m.clone(),
            None => return,
        };

        let event_bus = self.event_bus.clone();

        let handle = tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(60));

            loop {
                interval.tick().await;

                let guard = manager.read().await;
                if let Err(_e) = guard.refresh().await {
                    // Log error but continue
                } else {
                    // Emit ManifestUpdated event
                    let _ = event_bus.publish(StateEvent::ManifestUpdated {
                        version: "unknown".to_string(),
                    });
                }
            }
        });

        self.state.manifest_refresh = Some(handle);
    }

    /// Execute a typed route with the given input and return the typed response.
    ///
    /// This is the core route execution logic moved from executor.rs
    async fn execute_route<R, S>(&self, route: &Route<R, S>, input: &R) -> Result<S, ClientError>
    where
        R: Serialize,
        S: DeserializeOwned + Serialize + Clone,
    {
        let input_value = serde_json::to_value(input).map_err(|e| ClientError::Serialization(e.to_string()))?;

        match route.route_type {
            RouteType::Query => self.execute_query(route.procedure, input_value).await,
            RouteType::Mutation => self.execute_mutation(route.procedure, input_value).await,
            RouteType::Subscription => Err(ClientError::NotImplemented(
                "Subscriptions not supported via route execution".to_string(),
            )),
        }
    }

    /// Execute a query call with the given path and input.
    async fn execute_query<T: DeserializeOwned + Serialize + Clone>(
        &self,
        path: &str,
        input: Value,
    ) -> Result<T, ClientError> {
        use crate::domains::client::trpc::call;

        let client = self
            .state
            .iris_client
            .as_ref()
            .ok_or_else(|| ClientError::Connection("Not connected".to_string()))?;

        call::<T>(&client.inner, path, input)
            .await
            .map_err(ClientError::from_iris_error)
    }

    /// Execute a mutation call with the given path and input.
    async fn execute_mutation<T: DeserializeOwned>(&self, path: &str, input: Value) -> Result<T, ClientError> {
        use crate::domains::client::trpc::call;

        let client = self
            .state
            .iris_client
            .as_ref()
            .ok_or_else(|| ClientError::Connection("Not connected".to_string()))?;

        call::<T>(&client.inner, path, input)
            .await
            .map_err(ClientError::from_iris_error)
    }

    /// Create or update an encrypted wallet.
    ///
    /// Moved from routes.rs
    async fn upsert_wallet(
        &self,
        wallet: crate::domains::enclave::wallet::types::Wallet,
        user_key: crate::domains::keystore::session::crypto::UsersEncryptionKeys,
    ) -> Result<ClientResponse, ClientError> {
        use base64::Engine;
        use base64::engine::general_purpose::STANDARD;
        use tyche_enclave::envelopes::transport::{
            RotateUserKeyPayload, TransportEnvelope, TransportEnvelopeKey, WalletUpsert,
        };

        // Get transport key
        let enclave_keys = self.get_transport_key().await?;
        let key = TransportEnvelopeKey::Unsealing(enclave_keys.deterministic);

        let encrypted_blob = WalletUpsert::new(wallet.encrypted_private_key.clone())
            .seal(&key)
            .map_err(|e| ClientError::Wallet(format!("Failed to seal wallet: {}", e)))?;

        let envelope = RotateUserKeyPayload::new(user_key.storage, None)
            .seal(&key)
            .map_err(|e| ClientError::Wallet(format!("Failed to seal envelope: {}", e)))?;

        let request = crate::domains::client::generated::routes::requests::agent_create_encrypted_wallet::CreateEncryptedWalletRequest {
            name: wallet.name.clone(),
            address: wallet.address.clone(),
            blob: STANDARD.encode(&encrypted_blob),
            envelope: STANDARD.encode(&envelope),
        };

        self.execute_route(
            &crate::domains::client::generated::routes::requests::agent_create_encrypted_wallet::ROUTE,
            &request,
        )
        .await?;

        Ok(ClientResponse::WalletUpdated)
    }

    /// List all encrypted wallets for the current agent.
    ///
    /// Moved from routes.rs
    async fn list_wallets(&self) -> Result<ClientResponse, ClientError> {
        use crate::domains::client::generated::routes::requests::agent_list_encrypted_wallets;
        use crate::domains::enclave::wallet::types::WalletList;

        let wallets: Vec<WalletList> = self
            .execute_route(&agent_list_encrypted_wallets::ROUTE, &())
            .await?
            .into_iter()
            .map(|w| {
                ChainType::parse(w.chain_type.as_str())
                    .map(|chain_type| WalletList {
                        chain_type,
                        name: w.name.unwrap_or_default(),
                        address: w.address,
                    })
                    .map_err(|e| ClientError::Wallet(e.to_string()))
            })
            .collect::<Result<Vec<_>, ClientError>>()?;

        Ok(ClientResponse::WalletList(wallets))
    }

    /// Delete an encrypted wallet by address.
    ///
    /// Moved from routes.rs
    async fn delete_wallet(&self, address: String) -> Result<ClientResponse, ClientError> {
        use crate::domains::client::generated::routes::requests::agent_delete_encrypted_wallet;

        let request = agent_delete_encrypted_wallet::DeleteEncryptedWalletRequest {
            wallet_address: address,
        };

        self.execute_route(&agent_delete_encrypted_wallet::ROUTE, &request)
            .await?;

        Ok(ClientResponse::WalletDeleted)
    }

    /// Rotate the user encryption key.
    ///
    /// Moved from routes.rs
    async fn rotate_user_encryption_key(
        &self,
        new_key: crate::domains::keystore::session::crypto::UsersEncryptionKeys,
        old_key: crate::domains::keystore::session::crypto::UsersEncryptionKeys,
    ) -> Result<ClientResponse, ClientError> {
        use base64::Engine;
        use base64::engine::general_purpose::STANDARD;
        use tyche_enclave::envelopes::transport::{RotateUserKeyPayload, TransportEnvelopeKey};

        let enclave_keys = self.get_transport_key().await?;
        let key = TransportEnvelopeKey::Unsealing(enclave_keys.deterministic);

        let envelope = RotateUserKeyPayload::new(new_key.storage, Some(old_key.storage))
            .seal(&key)
            .map_err(|e| ClientError::Wallet(format!("Failed to seal envelope: {}", e)))?;

        let request = crate::domains::client::generated::routes::requests::agent_rotate_user_encryption_key::RotateUserEncryptionKeyRequest {
            envelope: STANDARD.encode(&envelope),
        };

        self.execute_route(&agent_rotate_user_encryption_key::ROUTE, &request)
            .await?;

        Ok(ClientResponse::KeyRotated)
    }

    /// Conduct the proof game.
    ///
    /// Moved from routes.rs
    async fn proof_game(
        &self,
        request: crate::domains::client::generated::routes::requests::agent_proof_game::ProofGameRequest,
    ) -> Result<ClientResponse, ClientError> {
        use crate::domains::client::generated::routes::requests::agent_proof_game;

        let response: agent_proof_game::ProofGameResponse = self
            .execute_route(&agent_proof_game::ROUTE, &request)
            .await?;

        Ok(ClientResponse::ProofGameResult(response))
    }

    /// Get transport key (internal helper).
    ///
    /// Moved from transport.rs - kept as internal helper
    async fn get_transport_key(&self) -> Result<tyche_enclave::shared::attestation::TransportKeyReceiver, ClientError> {
        use crate::domains::client::generated::routes::requests::agent_get_transport_key;

        // For now, execute the route directly
        // In a full implementation, this would use the transport cache
        let response: agent_get_transport_key::GetTransportKeyResponse = self
            .execute_route(&agent_get_transport_key::ROUTE, &())
            .await?;

        // Parse the response and return transport keys
        // This is a simplified implementation
        use base64::Engine;
        use base64::engine::general_purpose::STANDARD;

        let ephemeral_bytes = STANDARD
            .decode(&response.ephemeral)
            .map_err(|e| ClientError::Transport(format!("Failed to decode ephemeral key: {}", e)))?;
        let deterministic_bytes = STANDARD
            .decode(&response.deterministic)
            .map_err(|e| ClientError::Transport(format!("Failed to decode deterministic key: {}", e)))?;

        let ephemeral = ed25519_dalek::VerifyingKey::from_bytes(
            &ephemeral_bytes
                .try_into()
                .map_err(|_| ClientError::Transport("Invalid ephemeral key length".to_string()))?,
        )
        .map_err(|e| ClientError::Transport(format!("Invalid ephemeral key: {}", e)))?;

        let deterministic = ed25519_dalek::VerifyingKey::from_bytes(
            &deterministic_bytes
                .try_into()
                .map_err(|_| ClientError::Transport("Invalid deterministic key length".to_string()))?,
        )
        .map_err(|e| ClientError::Transport(format!("Invalid deterministic key: {}", e)))?;

        Ok(tyche_enclave::shared::attestation::TransportKeyReceiver {
            ephemeral,
            deterministic,
            attestation: STANDARD.decode(&response.attestation).unwrap_or_default(),
        })
    }
}

// RouteExecutor trait is defined in trpc.rs and implemented for IrisClient there.
// The actor doesn't directly implement RouteExecutor - instead,
// the execute_route method provides the same functionality internally
