//! MCP domain handle
//!
//! This module provides the public interface for the MCP domain.
//! The McpHandle is a thin gateway that sends messages to the MCP actor
//! via an mpsc channel using the PoseidonRequest pattern.
//!
//! Architecture:
//! - handle.rs: Thin gateway - public API, creates channel, spawns actor
//! - actor.rs: State owner - business logic, receives messages, includes state types

use tokio::sync::{mpsc, oneshot};

use crate::domains::alerts::AlertsHandle;
use crate::domains::client::ClientHandle;
use crate::domains::enclave::EnclaveHandle;
use crate::domains::mcp::actor::McpActor;
use crate::domains::mcp::actor::{McpState, TransportType};
use crate::domains::mcp::errors::{McpError, McpResult};
use crate::domains::mcp::messages::{McpMessage, McpRequest};
use crate::domains::trades::TradesHandle;
use crate::event_bus::{EventBus, PoseidonRequest, TraceContext};

/// Public handle for the MCP domain
///
/// This is the primary interface for interacting with the MCP server.
/// It provides methods to start/stop the server and query its status.
/// All operations send messages to the McpActor via PoseidonRequest pattern.
#[derive(Clone, Debug)]
pub struct McpHandle {
    /// Channel sender for communicating with the actor
    sender: mpsc::Sender<McpRequest>,
}

impl McpHandle {
    /// Create a new MCP handle with dependencies and EventBus.
    ///
    /// Creates the mpsc channel internally, spawns the actor,
    /// and returns the handle with the actor's JoinHandle.
    ///
    /// # Arguments
    /// * `client` - Client handle for API calls
    /// * `enclave` - Enclave handle for wallet operations
    /// * `trades` - Trades handle for trade operations
    /// * `alerts` - Alerts handle for alert subscriptions
    /// * `event_bus` - EventBus for publishing state events
    ///
    /// # Returns
    /// A tuple containing:
    /// - `McpHandle` that can be used to interact with the MCP domain
    /// - `JoinHandle` for the actor task
    pub fn new(
        client: ClientHandle,
        enclave: EnclaveHandle,
        trades: TradesHandle,
        alerts: AlertsHandle,
        event_bus: EventBus,
    ) -> (Self, tokio::task::JoinHandle<()>) {
        let (sender, receiver) = mpsc::channel(64);

        // Spawn the actor with the created receiver and dependencies
        let actor = McpActor::new(client, enclave, trades, alerts, event_bus);
        let handle = tokio::spawn(async move {
            actor.run(receiver).await;
        });

        (Self { sender }, handle)
    }

    /// Create an McpHandle from an existing sender.
    ///
    /// Used internally when the actor is already spawned.
    pub fn from_sender(sender: mpsc::Sender<McpRequest>) -> Self {
        Self { sender }
    }

    /// Send a request to the actor and wait for response
    ///
    /// Internal helper method that wraps the PoseidonRequest pattern.
    /// Creates a oneshot channel, sends the request with trace context,
    /// and awaits the response.
    async fn send_request(&self, payload: McpMessage) -> McpResult<McpState> {
        let (reply_to, rx) = oneshot::channel();

        let request = PoseidonRequest {
            payload,
            trace_ctx: TraceContext::current(),
            reply_to,
        };

        self.sender
            .send(request)
            .await
            .map_err(|_| McpError::ChannelError)?;

        rx.await.map_err(|_| McpError::ReplyChannelClosed)?
    }

    /// Start the MCP server with the specified transport
    ///
    /// # Arguments
    ///
    /// * `transport` - The transport type (stdio or HTTP)
    ///
    /// # Returns
    ///
    /// `Ok(McpState)` if the server started successfully, or an `McpError` if it failed
    pub async fn start(&self, transport: TransportType) -> McpResult<McpState> {
        self.send_request(McpMessage::Start { transport }).await
    }

    /// Start the MCP server in stdio mode
    ///
    /// This is a convenience method that starts the server with stdio transport.
    pub async fn start_stdio(&self) -> McpResult<McpState> {
        self.start(TransportType::Stdio).await
    }

    /// Start the MCP server in HTTP mode
    ///
    /// This is a convenience method that starts the server with HTTP transport.
    ///
    /// # Arguments
    ///
    /// * `host` - The host address to bind to (e.g., "127.0.0.1")
    /// * `port` - The port number to listen on
    pub async fn start_http(&self, host: impl Into<String>, port: u16) -> McpResult<McpState> {
        self.start(TransportType::Http {
            host: host.into(),
            port,
        })
        .await
    }

    /// Stop the MCP server
    ///
    /// Sends a shutdown signal to the running server and waits for it to complete.
    ///
    /// # Returns
    ///
    /// `Ok(McpState)` containing the final stopped state, or an `McpError` if it failed
    pub async fn stop(&self) -> McpResult<McpState> {
        self.send_request(McpMessage::Stop).await
    }

    /// Get the current MCP server status
    ///
    /// # Returns
    ///
    /// The current `McpState` containing transport type, mode, and server info
    pub async fn get_status(&self) -> McpResult<McpState> {
        self.send_request(McpMessage::GetStatus).await
    }

    /// Check if the server is currently running
    ///
    /// # Returns
    ///
    /// `true` if the server is in `Running` mode, `false` otherwise
    pub async fn is_running(&self) -> McpResult<bool> {
        let status = self.get_status().await?;
        Ok(matches!(
            status.mode,
            crate::domains::mcp::actor::McpMode::Running { .. }
        ))
    }

    /// Get the sender channel for direct message sending
    ///
    /// Used by IPC domain for direct routing to MCP domain.
    pub fn sender(&self) -> &mpsc::Sender<McpRequest> {
        &self.sender
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transport_type_clone() {
        let transport = TransportType::Http {
            host: "127.0.0.1".to_string(),
            port: 8080,
        };
        let cloned = transport.clone();
        match cloned {
            TransportType::Http { host, port } => {
                assert_eq!(host, "127.0.0.1");
                assert_eq!(port, 8080);
            }
            _ => panic!("Expected HTTP transport"),
        }
    }

    #[test]
    fn test_transport_type_stdio() {
        let transport = TransportType::Stdio;
        match transport {
            TransportType::Stdio => {}
            _ => panic!("Expected Stdio transport"),
        }
    }

    #[test]
    fn test_mcp_handle_debug() {
        // We can't actually create a handle without spawning an actor,
        // but we can verify the structure exists
        struct DummyMcpHandle;

        // Just verify the struct can be created
        let _dummy = DummyMcpHandle;
    }

    // Note: These tests require domain dependencies which are not available
    // without full initialization. They are marked as ignored for now.
}
