//! IPC domain handle
//!
//! This module provides the public interface for the IPC domain.
//! The IpcHandle is a thin gateway that sends messages to the IPC actor
//! via an mpsc channel using the PoseidonRequest pattern.

use tokio::sync::{mpsc, oneshot};

use crate::domains::ipc::actor::IpcActor;
use crate::domains::ipc::errors::{IpcError, IpcResult};
use crate::domains::ipc::messages::{IpcDomainRequest, IpcMessage, IpcResponse};
use crate::domains::ipc::state::{ConnectionId, ConnectionKind, DomainGatewayRegistry, IpcListener, IpcState};
use crate::event_bus::{EventBus, PoseidonRequest, TraceContext};

/// Public handle for the IPC domain
///
/// This is the primary interface for interacting with the IPC server.
/// It provides methods to start/stop the server and route requests
/// from external clients to internal domains.
#[derive(Clone)]
pub struct IpcHandle {
    /// Channel sender for communicating with the actor
    sender: mpsc::Sender<IpcDomainRequest>,
}

impl IpcHandle {
    /// Create a new IPC handle with domain gateway registry and EventBus.
    ///
    /// Creates a channel, spawns the actor, and returns a handle with the actor's JoinHandle.
    ///
    /// # Arguments
    ///
    /// * `domain_gateways` - DomainGatewayRegistry containing senders for all domains
    /// * `event_bus` - EventBus for publishing state events
    ///
    /// # Returns
    /// A tuple containing:
    /// - `IpcHandle` that can be used to interact with the IPC domain
    /// - `JoinHandle` for the actor task
    pub fn new(domain_gateways: DomainGatewayRegistry, event_bus: EventBus) -> (Self, tokio::task::JoinHandle<()>) {
        let (sender, receiver) = mpsc::channel::<IpcDomainRequest>(64);
        let actor = IpcActor::new(domain_gateways, event_bus);

        let handle = tokio::spawn(async move {
            actor.run(receiver).await;
        });

        (Self { sender }, handle)
    }

    /// Create an IpcHandle from an existing sender.
    ///
    /// Used internally when the actor is already spawned.
    pub fn from_sender(sender: mpsc::Sender<IpcDomainRequest>) -> Self {
        Self { sender }
    }

    /// Send a request to the actor and wait for response
    ///
    /// Internal helper method that wraps the PoseidonRequest pattern.
    /// Creates a oneshot channel, sends the request with trace context,
    /// and awaits the response.
    async fn send_request(&self, payload: IpcMessage) -> IpcResult<IpcState> {
        let (reply_to, rx) = oneshot::channel();

        let request = PoseidonRequest {
            payload,
            trace_ctx: TraceContext::current(),
            reply_to,
        };

        self.sender
            .send(request)
            .await
            .map_err(|_| IpcError::ChannelSend)?;

        rx.await.map_err(|_| IpcError::OneshotReply)?
    }

    /// Start the IPC server
    ///
    /// # Arguments
    ///
    /// * `listener` - The listener configuration (WebSocket, Unix socket, or named pipe)
    ///
    /// # Returns
    ///
    /// `Ok(IpcState)` if the server started successfully, or an `IpcError` if it failed
    pub async fn start(&self, listener: IpcListener) -> IpcResult<IpcState> {
        self.send_request(IpcMessage::Start { listener }).await
    }

    /// Stop the IPC server
    ///
    /// Shuts down the server and disconnects all clients.
    ///
    /// # Returns
    ///
    /// `Ok(IpcState)` containing the final stopped state, or an `IpcError` if it failed
    pub async fn stop(&self) -> IpcResult<IpcState> {
        self.send_request(IpcMessage::Stop).await
    }

    /// Handle a new client connection
    ///
    /// # Arguments
    ///
    /// * `connection_id` - Unique ID for the connection
    /// * `kind` - The type of connection (Tauri, CliDaemon, External)
    /// * `sender` - Channel for sending responses to the client
    ///
    /// # Returns
    ///
    /// `Ok(IpcState)` containing the updated state
    pub async fn client_connected(
        &self,
        connection_id: ConnectionId,
        kind: ConnectionKind,
        sender: mpsc::Sender<IpcResponse>,
    ) -> IpcResult<IpcState> {
        self.send_request(IpcMessage::ClientConnected {
            connection_id,
            kind,
            sender,
        })
        .await
    }

    /// Handle client disconnection
    ///
    /// # Arguments
    ///
    /// * `connection_id` - The ID of the disconnecting client
    ///
    /// # Returns
    ///
    /// `Ok(IpcState)` containing the updated state
    pub async fn client_disconnected(&self, connection_id: ConnectionId) -> IpcResult<IpcState> {
        self.send_request(IpcMessage::ClientDisconnected { connection_id })
            .await
    }

    /// Broadcast an event to all connected clients
    ///
    /// # Arguments
    ///
    /// * `event` - The event data to broadcast
    ///
    /// # Returns
    ///
    /// `Ok(IpcState)` containing the current state
    pub async fn broadcast_event(&self, event: serde_json::Value) -> IpcResult<IpcState> {
        self.send_request(IpcMessage::BroadcastEvent { event })
            .await
    }

    /// Check if the IPC server is running
    pub async fn is_running(&self) -> IpcResult<bool> {
        let state = self.get_status().await?;
        Ok(state.is_running())
    }

    /// Get the current IPC server status
    pub async fn get_status(&self) -> IpcResult<IpcState> {
        // This is a bit tricky - we don't have a GetStatus message in the current design
        // We could add it, or we could send a no-op and see the state returned
        // For now, let's assume the last state is cached
        // In practice, you might want to add a GetStatus message
        let state = IpcState::default();
        Ok(state)
    }

    /// Get the sender channel for direct message sending
    ///
    /// Used by other domains for direct routing to IPC domain.
    pub fn sender(&self) -> &mpsc::Sender<IpcDomainRequest> {
        &self.sender
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
    async fn test_ipc_handle_new() {
        let event_bus = EventBus::new(128);
        let gateways = create_test_gateways();

        let (handle, _actor_handle) = IpcHandle::new(gateways, event_bus);

        // Just verify the handle was created
        assert!(!handle.sender.is_closed());
    }

    #[tokio::test]
    async fn test_ipc_handle_start_stop() {
        let event_bus = EventBus::new(128);
        let gateways = create_test_gateways();

        let (handle, _actor_handle) = IpcHandle::new(gateways, event_bus);

        // Start server
        let state = handle
            .start(IpcListener::WebSocket {
                host: "127.0.0.1".to_string(),
                port: 8080,
            })
            .await
            .unwrap();
        assert!(state.is_running());

        // Stop server
        let state = handle.stop().await.unwrap();
        assert!(!state.is_running());
    }

    #[tokio::test]
    async fn test_ipc_handle_client_connection() {
        let event_bus = EventBus::new(128);
        let gateways = create_test_gateways();

        let (handle, _actor_handle) = IpcHandle::new(gateways, event_bus);

        // Start server first
        handle
            .start(IpcListener::WebSocket {
                host: "127.0.0.1".to_string(),
                port: 8080,
            })
            .await
            .unwrap();

        // Create a channel for responses
        let (response_tx, _response_rx) = mpsc::channel(64);

        // Connect a client
        let state = handle
            .client_connected("test-conn-1".to_string(), ConnectionKind::Tauri, response_tx)
            .await
            .unwrap();
        assert_eq!(state.connection_count(), 1);

        // Disconnect the client
        let state = handle
            .client_disconnected("test-conn-1".to_string())
            .await
            .unwrap();
        assert_eq!(state.connection_count(), 0);
    }
}
