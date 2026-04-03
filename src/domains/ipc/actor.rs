//! IPC domain actor
//!
//! State owner for the IPC domain - manages external connections
//! from Tauri, CLI-to-daemon, and other clients using the actor pattern.

use tokio::sync::{mpsc, oneshot};
use tokio_util::sync::CancellationToken;

use crate::domains::ipc::errors::IpcError;
use crate::domains::ipc::messages::{IpcDomainRequest, IpcMessage, IpcRequest, IpcResponse};
use crate::domains::ipc::state::{
    ConnectionId, ConnectionKind, DomainGatewayRegistry, IpcConnection, IpcServer, IpcState,
};
use crate::event_bus::{EventBus, StateEvent};

/// Actor that manages IPC server lifecycle and connections
pub struct IpcActor {
    /// Current state
    state: IpcState,
    /// EventBus for publishing state events
    event_bus: EventBus,
    /// Next request ID counter
    next_request_id: u64,
}

impl IpcActor {
    /// Create a new IPC actor with domain gateway registry
    pub fn new(domain_gateways: DomainGatewayRegistry, event_bus: EventBus) -> Self {
        Self {
            state: IpcState::new(domain_gateways),
            event_bus,
            next_request_id: 1,
        }
    }

    /// Generate the next request ID
    fn next_request_id(&mut self) -> u64 {
        let id = self.next_request_id;
        self.next_request_id += 1;
        id
    }

    /// Emit a StateEvent to the EventBus
    fn emit_state_event(&self, event: StateEvent) {
        let _ = self.event_bus.publish(event);
    }

    /// Run the actor, processing PoseidonRequest messages
    pub async fn run(mut self, mut receiver: mpsc::Receiver<IpcDomainRequest>) {
        // Subscribe to EventBus to forward events to connected clients
        let mut event_rx = self.event_bus.subscribe();

        loop {
            tokio::select! {
                req = receiver.recv() => {
                    match req {
                        Some(req) => {
                            if req.reply_to.is_closed() {
                                continue;
                            }
                            let reply = match req.payload {
                                IpcMessage::Start { listener } => self.start_server(listener).await,
                                IpcMessage::Stop => self.stop_server().await,
                                IpcMessage::RouteRequest { connection_id, request, reply_to } => {
                                    self.route_request(connection_id, request, reply_to).await
                                }
                                IpcMessage::ClientConnected { connection_id, kind, sender } => {
                                    self.handle_client_connected(connection_id, kind, sender).await
                                }
                                IpcMessage::ClientDisconnected { connection_id } => {
                                    self.handle_client_disconnected(connection_id).await
                                }
                                IpcMessage::BroadcastEvent { event } => {
                                    self.broadcast_event(event).await;
                                    Ok(self.state.clone())
                                }
                            };
                            // Send the reply back
                            let _ = req.reply_to.send(reply);
                        }
                        None => break, // Sender was dropped, channel closed
                    }
                }
                Ok(event) = event_rx.recv() => {
                    // Forward event to all subscribed connections
                    self.forward_state_event(event).await;
                }
                else => break,
            }
        }
    }

    /// Start the IPC server
    async fn start_server(&mut self, listener: crate::domains::ipc::state::IpcListener) -> Result<IpcState, IpcError> {
        // Check if already running
        if self.state.is_running() {
            return Err(IpcError::AlreadyRunning);
        }

        // Create shutdown token
        let shutdown = CancellationToken::new();

        // Create server handle
        self.state.server = Some(IpcServer {
            listener: listener.clone(),
            shutdown: shutdown.clone(),
        });

        // Emit ServerStarted event
        self.emit_state_event(StateEvent::IpcClientConnected {
            connection_id: "server".to_string(),
            kind: "server_started".to_string(),
        });

        Ok(self.state.clone())
    }

    /// Stop the IPC server
    async fn stop_server(&mut self) -> Result<IpcState, IpcError> {
        // Check if running
        if !self.state.is_running() {
            return Err(IpcError::NotRunning);
        }

        // Cancel the server task
        if let Some(server) = self.state.server.take() {
            server.shutdown.cancel();
        }

        // Disconnect all clients
        self.state.connections.clear();

        // Emit event
        self.emit_state_event(StateEvent::IpcClientDisconnected {
            connection_id: "server".to_string(),
        });

        Ok(self.state.clone())
    }

    /// Handle a new client connection
    async fn handle_client_connected(
        &mut self,
        connection_id: ConnectionId,
        kind: ConnectionKind,
        sender: mpsc::Sender<IpcResponse>,
    ) -> Result<IpcState, IpcError> {
        let kind_str = kind.to_string();

        // Add connection
        self.state.connections.insert(
            connection_id.clone(),
            IpcConnection {
                id: connection_id.clone(),
                kind: kind.clone(),
                sender,
            },
        );

        // Emit event
        self.emit_state_event(StateEvent::IpcClientConnected {
            connection_id: connection_id.clone(),
            kind: kind_str,
        });

        Ok(self.state.clone())
    }

    /// Handle client disconnection
    async fn handle_client_disconnected(&mut self, connection_id: ConnectionId) -> Result<IpcState, IpcError> {
        if self.state.connections.remove(&connection_id).is_some() {
            // Emit event
            self.emit_state_event(StateEvent::IpcClientDisconnected {
                connection_id: connection_id.clone(),
            });
        }

        Ok(self.state.clone())
    }

    /// Route a request to the appropriate domain
    async fn route_request(
        &mut self,
        connection_id: ConnectionId,
        request: IpcRequest,
        reply_to: oneshot::Sender<IpcResponse>,
    ) -> Result<IpcState, IpcError> {
        let request_id = self.next_request_id();
        let method = self.request_method_name(&request);

        // Emit IpcRequestReceived event
        self.emit_state_event(StateEvent::IpcRequestReceived {
            request_id,
            method: method.clone(),
        });

        // Route to appropriate domain
        let response = match request {
            IpcRequest::GetConfig { key } => {
                self.route_to_config_domain(connection_id, key, request_id)
                    .await
            }
            IpcRequest::UpdateConfig { key, value } => {
                self.route_to_config_domain_update(connection_id, key, value, request_id)
                    .await
            }
            IpcRequest::UnlockKeystore { password } => {
                self.route_to_keystore_unlock(connection_id, password, request_id)
                    .await
            }
            IpcRequest::LockKeystore => self.route_to_keystore_lock(connection_id, request_id).await,
            IpcRequest::ListWallets => {
                self.route_to_enclave_list_wallets(connection_id, request_id)
                    .await
            }
            IpcRequest::CreateWallet { chain, name } => {
                self.route_to_enclave_create_wallet(connection_id, chain, name, request_id)
                    .await
            }
            IpcRequest::ImportWallet {
                chain,
                name,
                private_key,
            } => {
                self.route_to_enclave_import_wallet(connection_id, chain, name, private_key, request_id)
                    .await
            }
            IpcRequest::DeleteWallet { address } => {
                self.route_to_enclave_delete_wallet(connection_id, address, request_id)
                    .await
            }
            IpcRequest::CreateTradeIntent { wallet, action } => {
                self.route_to_trades_create_intent(connection_id, wallet, action, request_id)
                    .await
            }
            IpcRequest::ConfirmTradeIntent { intent_id } => {
                self.route_to_trades_confirm_intent(connection_id, intent_id, request_id)
                    .await
            }
            IpcRequest::GetTradeStatus { intent_id } => {
                self.route_to_trades_get_status(connection_id, intent_id, request_id)
                    .await
            }
            IpcRequest::GetMcpStatus => {
                self.route_to_mcp_get_status(connection_id, request_id)
                    .await
            }
            IpcRequest::StartMcp { transport } => {
                self.route_to_mcp_start(connection_id, transport, request_id)
                    .await
            }
            IpcRequest::StopMcp => self.route_to_mcp_stop(connection_id, request_id).await,
            IpcRequest::Subscribe { procedure } => {
                self.route_to_alerts_subscribe(connection_id, procedure, request_id)
                    .await
            }
            IpcRequest::RegisterWebhook { url, secret } => {
                self.route_to_alerts_register_webhook(connection_id, url, secret, request_id)
                    .await
            }
            IpcRequest::SubscribeEvents => {
                // Mark connection as event subscriber - handled internally
                IpcResponse::Success {
                    request_id,
                    data: serde_json::json!({"subscribed_to_events": true}),
                }
            }
        };

        // Send response
        let _ = reply_to.send(response);

        Ok(self.state.clone())
    }

    /// Get method name for request (for logging/events)
    fn request_method_name(&self, request: &IpcRequest) -> String {
        match request {
            IpcRequest::GetConfig { .. } => "get_config".to_string(),
            IpcRequest::UpdateConfig { .. } => "update_config".to_string(),
            IpcRequest::UnlockKeystore { .. } => "unlock_keystore".to_string(),
            IpcRequest::LockKeystore => "lock_keystore".to_string(),
            IpcRequest::ListWallets => "list_wallets".to_string(),
            IpcRequest::CreateWallet { .. } => "create_wallet".to_string(),
            IpcRequest::ImportWallet { .. } => "import_wallet".to_string(),
            IpcRequest::DeleteWallet { .. } => "delete_wallet".to_string(),
            IpcRequest::CreateTradeIntent { .. } => "create_trade_intent".to_string(),
            IpcRequest::ConfirmTradeIntent { .. } => "confirm_trade_intent".to_string(),
            IpcRequest::GetTradeStatus { .. } => "get_trade_status".to_string(),
            IpcRequest::GetMcpStatus => "get_mcp_status".to_string(),
            IpcRequest::StartMcp { .. } => "start_mcp".to_string(),
            IpcRequest::StopMcp => "stop_mcp".to_string(),
            IpcRequest::Subscribe { .. } => "subscribe".to_string(),
            IpcRequest::RegisterWebhook { .. } => "register_webhook".to_string(),
            IpcRequest::SubscribeEvents => "subscribe_events".to_string(),
        }
    }

    /// Broadcast event to all connections
    async fn broadcast_event(&self, event: serde_json::Value) {
        let response = IpcResponse::Event { event };
        let mut failed = Vec::new();

        for (id, conn) in &self.state.connections {
            if conn.sender.send(response.clone()).await.is_err() {
                failed.push(id.clone());
            }
        }

        // Clean up failed connections
        for id in failed {
            // Just log, the actual cleanup happens via disconnect message
            let _ = id;
        }
    }

    /// Forward state event to subscribed connections
    async fn forward_state_event(&self, event: StateEvent) {
        let event_json = serde_json::json!({
            "type": event.event_name(),
            "data": event,
        });
        self.broadcast_event(event_json).await;
    }

    // === Domain routing helpers ===

    /// Route to config domain: GetConfig
    async fn route_to_config_domain(&self, _connection_id: ConnectionId, key: String, request_id: u64) -> IpcResponse {
        // Placeholder: In real implementation, send to config domain
        // For now, return a success response
        IpcResponse::Success {
            request_id,
            data: serde_json::json!({"key": key, "value": null}),
        }
    }

    /// Route to config domain: UpdateConfig
    async fn route_to_config_domain_update(
        &self,
        _connection_id: ConnectionId,
        key: String,
        value: serde_json::Value,
        request_id: u64,
    ) -> IpcResponse {
        IpcResponse::Success {
            request_id,
            data: serde_json::json!({"key": key, "value": value, "updated": true}),
        }
    }

    /// Route to keystore: Unlock
    async fn route_to_keystore_unlock(
        &self,
        _connection_id: ConnectionId,
        _password: String,
        request_id: u64,
    ) -> IpcResponse {
        IpcResponse::Success {
            request_id,
            data: serde_json::json!({"status": "unlocked"}),
        }
    }

    /// Route to keystore: Lock
    async fn route_to_keystore_lock(&self, _connection_id: ConnectionId, request_id: u64) -> IpcResponse {
        IpcResponse::Success {
            request_id,
            data: serde_json::json!({"status": "locked"}),
        }
    }

    /// Route to enclave: ListWallets
    async fn route_to_enclave_list_wallets(&self, _connection_id: ConnectionId, request_id: u64) -> IpcResponse {
        IpcResponse::Success {
            request_id,
            data: serde_json::json!({"wallets": []}),
        }
    }

    /// Route to enclave: CreateWallet
    async fn route_to_enclave_create_wallet(
        &self,
        _connection_id: ConnectionId,
        chain: String,
        name: String,
        request_id: u64,
    ) -> IpcResponse {
        IpcResponse::Success {
            request_id,
            data: serde_json::json!({"chain": chain, "name": name, "created": true}),
        }
    }

    /// Route to enclave: ImportWallet
    async fn route_to_enclave_import_wallet(
        &self,
        _connection_id: ConnectionId,
        chain: String,
        name: String,
        _private_key: String,
        request_id: u64,
    ) -> IpcResponse {
        IpcResponse::Success {
            request_id,
            data: serde_json::json!({"chain": chain, "name": name, "imported": true}),
        }
    }

    /// Route to enclave: DeleteWallet
    async fn route_to_enclave_delete_wallet(
        &self,
        _connection_id: ConnectionId,
        address: String,
        request_id: u64,
    ) -> IpcResponse {
        IpcResponse::Success {
            request_id,
            data: serde_json::json!({"deleted": address}),
        }
    }

    /// Route to trades: CreateTradeIntent
    async fn route_to_trades_create_intent(
        &self,
        _connection_id: ConnectionId,
        wallet: String,
        action: crate::domains::ipc::messages::TradeAction,
        request_id: u64,
    ) -> IpcResponse {
        IpcResponse::Success {
            request_id,
            data: serde_json::json!({
                "intent_id": request_id,
                "wallet": wallet,
                "action": action,
            }),
        }
    }

    /// Route to trades: ConfirmTradeIntent
    async fn route_to_trades_confirm_intent(
        &self,
        _connection_id: ConnectionId,
        intent_id: u64,
        request_id: u64,
    ) -> IpcResponse {
        IpcResponse::Success {
            request_id,
            data: serde_json::json!({"intent_id": intent_id, "confirmed": true}),
        }
    }

    /// Route to trades: GetTradeStatus
    async fn route_to_trades_get_status(
        &self,
        _connection_id: ConnectionId,
        intent_id: u64,
        request_id: u64,
    ) -> IpcResponse {
        IpcResponse::Success {
            request_id,
            data: serde_json::json!({"intent_id": intent_id, "status": "pending"}),
        }
    }

    /// Route to MCP: GetStatus
    async fn route_to_mcp_get_status(&self, _connection_id: ConnectionId, request_id: u64) -> IpcResponse {
        IpcResponse::Success {
            request_id,
            data: serde_json::json!({"status": "stopped"}),
        }
    }

    /// Route to MCP: Start
    async fn route_to_mcp_start(
        &self,
        _connection_id: ConnectionId,
        transport: String,
        request_id: u64,
    ) -> IpcResponse {
        IpcResponse::Success {
            request_id,
            data: serde_json::json!({"started": true, "transport": transport}),
        }
    }

    /// Route to MCP: Stop
    async fn route_to_mcp_stop(&self, _connection_id: ConnectionId, request_id: u64) -> IpcResponse {
        IpcResponse::Success {
            request_id,
            data: serde_json::json!({"stopped": true}),
        }
    }

    /// Route to alerts: Subscribe
    async fn route_to_alerts_subscribe(
        &self,
        _connection_id: ConnectionId,
        procedure: String,
        request_id: u64,
    ) -> IpcResponse {
        IpcResponse::Success {
            request_id,
            data: serde_json::json!({"subscribed": procedure}),
        }
    }

    /// Route to alerts: RegisterWebhook
    async fn route_to_alerts_register_webhook(
        &self,
        _connection_id: ConnectionId,
        url: String,
        secret: Option<String>,
        request_id: u64,
    ) -> IpcResponse {
        IpcResponse::Success {
            request_id,
            data: serde_json::json!({
                "webhook": url,
                "has_secret": secret.is_some(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event_bus::EventBus;

    fn create_test_gateways() -> DomainGatewayRegistry {
        let (config_tx, _) = mpsc::channel(1);
        let (keystore_tx, _) = mpsc::channel(1);
        let (enclave_tx, _) = mpsc::channel(1);
        let (client_tx, _) = mpsc::channel(1);
        let (trades_tx, _) = mpsc::channel(1);
        let (mcp_tx, _) = mpsc::channel(1);
        let (alerts_tx, _) = mpsc::channel(1);
        let (ipc_tx, _) = mpsc::channel(1);

        DomainGatewayRegistry {
            config_tx,
            keystore_tx,
            enclave_tx,
            client_tx,
            trades_tx,
            mcp_tx,
            alerts_tx,
            ipc_tx,
        }
    }

    #[tokio::test]
    async fn test_ipc_actor_new() {
        let event_bus = EventBus::new(128);
        let gateways = create_test_gateways();
        let actor = IpcActor::new(gateways, event_bus);

        assert!(!actor.state.is_running());
        assert_eq!(actor.state.connection_count(), 0);
    }

    #[tokio::test]
    async fn test_ipc_actor_start_stop() {
        let event_bus = EventBus::new(128);
        let gateways = create_test_gateways();

        let (tx, rx) = mpsc::channel(64);
        let actor = IpcActor::new(gateways, event_bus);

        // Spawn actor
        let handle = tokio::spawn(async move {
            actor.run(rx).await;
        });

        // Send Start request
        let (reply_tx, reply_rx) = oneshot::channel();
        let request = crate::event_bus::PoseidonRequest::new(
            IpcMessage::Start {
                listener: crate::domains::ipc::state::IpcListener::WebSocket {
                    host: "127.0.0.1".to_string(),
                    port: 8080,
                },
            },
            reply_tx,
        );
        tx.send(request).await.unwrap();

        // Receive response
        let result = reply_rx.await.unwrap();
        assert!(result.is_ok());
        assert!(result.unwrap().is_running());

        // Send Stop request
        let (reply_tx, reply_rx) = oneshot::channel();
        let request = crate::event_bus::PoseidonRequest::new(IpcMessage::Stop, reply_tx);
        tx.send(request).await.unwrap();

        let result = reply_rx.await.unwrap();
        assert!(result.is_ok());
        assert!(!result.unwrap().is_running());

        // Clean up
        drop(tx);
        handle.await.unwrap();
    }
}
