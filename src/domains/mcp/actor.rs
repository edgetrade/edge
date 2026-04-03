//! MCP domain actor
//!
//! This module contains the McpActor which owns the MCP server state and lifecycle.
//! The actor receives messages via an mpsc channel and manages the server lifecycle.
//!
//! Architecture:
//! - McpActor: State owner that receives PoseidonRequest messages and manages server lifecycle
//! - Integrates with client, enclave, trades, and alerts domains via handles

use std::sync::Arc;

use tokio::sync::{RwLock, mpsc};
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

use crate::domains::alerts::AlertsHandle;
use crate::domains::client::ClientHandle;
use crate::domains::enclave::EnclaveHandle;
use crate::domains::mcp::errors::McpError;
use crate::domains::mcp::messages::{McpMessage, McpRequest};
use crate::domains::mcp::server::EdgeServer;
use crate::domains::trades::TradesHandle;
use crate::event_bus::{EventBus, StateEvent};

/// MCP state exposed by the actor
#[derive(Debug, Clone)]
pub struct McpState {
    /// Current server instance (if running)
    pub server: Option<EdgeServerHandle>,
    /// Current server mode
    pub mode: McpMode,
    /// Current transport configuration
    pub transport: TransportType,
}

/// Server mode state
#[derive(Debug, Clone)]
pub enum McpMode {
    /// Server is stopped
    Stopped,
    /// Server is running with shutdown token
    Running {
        /// Token for graceful shutdown
        shutdown: CancellationToken,
    },
}

/// Transport type for MCP server
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum TransportType {
    /// Standard I/O transport (for AI agents)
    Stdio,
    /// HTTP transport with host, port, and path
    Http {
        /// Host address (e.g., "127.0.0.1")
        host: String,
        /// Port number
        port: u16,
    },
}

/// Handle to a running MCP server
#[derive(Debug)]
pub struct EdgeServerHandle {
    /// Handle to the server task
    pub task_handle: JoinHandle<()>,
    /// Token for graceful shutdown
    pub shutdown: CancellationToken,
}

// Manual Clone implementation since JoinHandle doesn't implement Clone
impl Clone for EdgeServerHandle {
    fn clone(&self) -> Self {
        // We can't clone JoinHandle, but for state cloning purposes
        // we just create a dummy handle that will never be used
        Self {
            task_handle: tokio::spawn(async {}),
            shutdown: self.shutdown.clone(),
        }
    }
}

impl McpState {
    /// Create initial stopped state
    pub fn new() -> Self {
        Self {
            server: None,
            mode: McpMode::Stopped,
            transport: TransportType::Stdio,
        }
    }

    /// Create initial stopped state with specific transport
    pub fn with_transport(transport: TransportType) -> Self {
        Self {
            server: None,
            mode: McpMode::Stopped,
            transport,
        }
    }
}

impl Default for McpState {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for TransportType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransportType::Stdio => write!(f, "stdio"),
            TransportType::Http { host, port } => {
                write!(f, "http://{}:{}", host, port)
            }
        }
    }
}

/// Actor that manages MCP server lifecycle and state
pub struct McpActor {
    /// Current state
    state: McpState,
    /// EventBus for publishing state events
    event_bus: EventBus,
    /// Client handle for API calls
    client: ClientHandle,
    /// Enclave handle for wallet operations
    #[allow(unused)]
    enclave: EnclaveHandle,
    /// Trades handle for trade operations
    #[allow(unused)]
    trades: TradesHandle,
    /// Alerts handle for alert subscriptions
    #[allow(unused)]
    alerts: AlertsHandle,
}

impl McpActor {
    /// Create a new MCP actor with dependencies
    pub fn new(
        client: ClientHandle,
        enclave: EnclaveHandle,
        trades: TradesHandle,
        alerts: AlertsHandle,
        event_bus: EventBus,
    ) -> Self {
        Self {
            state: McpState::new(),
            event_bus,
            client,
            enclave,
            trades,
            alerts,
        }
    }

    /// Run the actor, processing McpRequest messages until the channel closes
    pub async fn run(mut self, mut receiver: mpsc::Receiver<McpRequest>) {
        while let Some(req) = receiver.recv().await {
            let reply = match req.payload {
                McpMessage::Start { transport } => self.start_server(transport).await,
                McpMessage::Stop => self.stop_server().await,
                McpMessage::GetStatus => Ok(self.state.clone()),
            };
            let _ = req.reply_to.send(reply);
        }
    }

    /// Emit a StateEvent to the EventBus.
    fn emit_state_event(&self, event: StateEvent) {
        let _ = self.event_bus.publish(event);
    }

    /// Start the MCP server with the specified transport
    async fn start_server(&mut self, transport: TransportType) -> Result<McpState, McpError> {
        // Check if already running
        if matches!(self.state.mode, McpMode::Running { .. }) {
            return Err(McpError::AlreadyRunning);
        }

        self.state.transport = transport.clone();

        // Get manifest from client
        let manifest = match self.client.get_manifest().await {
            Ok(m) => m,
            Err(_e) => {
                return Err(McpError::ManifestUnavailable);
            }
        };
        let manifest = Arc::new(RwLock::new(manifest));

        // Get IrisClient from client handle
        let client = match self.client.get_client().await {
            Ok(Some(client)) => client,
            Ok(None) => {
                return Err(McpError::ServerStart("Client not connected".to_string()));
            }
            Err(e) => {
                return Err(McpError::ServerStart(format!("Failed to get client: {}", e)));
            }
        };

        // Create EdgeServer instance
        let edge_server = match EdgeServer::new(client, manifest).await {
            Ok(server) => server,
            Err(e) => {
                return Err(McpError::ServerStart(format!("Failed to create EdgeServer: {}", e)));
            }
        };

        // Create shutdown token
        let shutdown = CancellationToken::new();
        let shutdown_clone = shutdown.clone();

        // Spawn server task based on transport type
        let task_handle = match transport {
            TransportType::Stdio => {
                tokio::spawn(async move {
                    // Clone for the async block
                    let server = edge_server.clone();
                    if let Err(e) = server.serve_stdio().await {
                        eprintln!("MCP server error: {}", e);
                    }
                })
            }
            TransportType::Http { ref host, port } => {
                let host = host.clone();
                tokio::spawn(async move {
                    let server = edge_server.clone();
                    if let Err(e) = server.serve_http(&host, port, "/mcp").await {
                        eprintln!("MCP server error: {}", e);
                    }
                })
            }
        };

        self.state.server = Some(EdgeServerHandle {
            task_handle,
            shutdown: shutdown_clone.clone(),
        });
        self.state.mode = McpMode::Running {
            shutdown: shutdown_clone,
        };

        // Emit event
        self.emit_state_event(StateEvent::McpServerStarted {
            transport: transport.to_string(),
        });

        Ok(self.state.clone())
    }

    /// Stop the MCP server
    async fn stop_server(&mut self) -> Result<McpState, McpError> {
        // Check if running
        if !matches!(self.state.mode, McpMode::Running { .. }) {
            return Err(McpError::NotRunning);
        }

        // Cancel the server task and wait for it to complete
        if let Some(server) = self.state.server.take() {
            server.shutdown.cancel();

            // Wait for task to complete with timeout
            let timeout = tokio::time::Duration::from_secs(5);
            match tokio::time::timeout(timeout, server.task_handle).await {
                Ok(Ok(())) => {
                    // Task completed successfully
                }
                Ok(Err(e)) => {
                    eprintln!("MCP server task panicked: {}", e);
                }
                Err(_) => {
                    eprintln!("MCP server shutdown timed out");
                }
            }
        }

        self.state.mode = McpMode::Stopped;
        self.state.server = None;

        // Emit McpServerStopped event
        self.emit_state_event(StateEvent::McpServerStopped);

        Ok(self.state.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: These tests require domain dependencies. They are marked as ignored
    // since we can't easily create mock handles in unit tests.

    #[test]
    fn test_mcp_state_new() {
        let state = McpState::new();
        assert!(matches!(state.mode, McpMode::Stopped));
        assert!(state.server.is_none());
    }

    #[test]
    fn test_mcp_state_with_transport() {
        let transport = TransportType::Http {
            host: "127.0.0.1".to_string(),
            port: 8080,
        };
        let state = McpState::with_transport(transport);
        assert!(matches!(state.mode, McpMode::Stopped));
        assert!(state.server.is_none());
        assert!(matches!(state.transport, TransportType::Http { .. }));
    }

    #[test]
    fn test_transport_type_display() {
        let stdio = TransportType::Stdio;
        assert_eq!(stdio.to_string(), "stdio");

        let http = TransportType::Http {
            host: "127.0.0.1".to_string(),
            port: 8080,
        };
        assert_eq!(http.to_string(), "http://127.0.0.1:8080");
    }

    #[test]
    fn test_mcp_mode_variants() {
        let stopped = McpMode::Stopped;
        assert!(matches!(stopped, McpMode::Stopped));

        let token = CancellationToken::new();
        let running = McpMode::Running {
            shutdown: token.clone(),
        };
        assert!(matches!(running, McpMode::Running { .. }));
    }
}
