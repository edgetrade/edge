//! IPC domain state
//!
//! Defines the state types for the IPC domain including connections,
//! server state, and the domain gateway registry for cross-domain routing.

use std::collections::HashMap;
use tokio::sync::mpsc;

/// Re-export IpcResponse from messages module
pub use crate::domains::ipc::messages::IpcResponse;

/// IPC state exposed by the actor
#[derive(Debug, Clone)]
pub struct IpcState {
    /// Current server instance (if running)
    pub server: Option<IpcServer>,
    /// Active connections
    pub connections: HashMap<ConnectionId, IpcConnection>,
    /// Domain gateway registry for routing to other domains
    pub domain_gateways: DomainGatewayRegistry,
}

/// IPC server state
#[derive(Debug, Clone)]
pub struct IpcServer {
    /// Server listener configuration
    pub listener: IpcListener,
    /// Shutdown token for graceful shutdown
    pub shutdown: tokio_util::sync::CancellationToken,
}

/// IPC connection information
#[derive(Debug, Clone)]
pub struct IpcConnection {
    /// Connection ID
    pub id: ConnectionId,
    /// Connection kind
    pub kind: ConnectionKind,
    /// Sender for responses to this connection
    pub sender: mpsc::Sender<IpcResponse>,
}

/// Connection ID type
pub type ConnectionId = String;

/// Connection kind for IPC clients
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ConnectionKind {
    /// Desktop app (Tauri)
    Tauri,
    /// CLI talking to running daemon
    CliDaemon,
    /// Other integrations
    External,
}

impl std::fmt::Display for ConnectionKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConnectionKind::Tauri => write!(f, "tauri"),
            ConnectionKind::CliDaemon => write!(f, "cli_daemon"),
            ConnectionKind::External => write!(f, "external"),
        }
    }
}

/// Domain gateway registry for routing to other domains
///
/// Contains mpsc senders for all domains that IPC can route to.
/// This enables direct async communication without going through
/// the EventBus for request/response patterns.
#[derive(Debug, Clone)]
pub struct DomainGatewayRegistry {
    /// Config domain sender
    pub config_tx: mpsc::Sender<crate::domains::config::ConfigRequest>,
    /// Keystore domain sender
    pub keystore_tx: mpsc::Sender<crate::domains::keystore::KeystoreRequest>,
    /// Enclave domain sender
    pub enclave_tx: mpsc::Sender<crate::domains::enclave::EnclaveRequest>,
    /// Client domain sender
    pub client_tx: mpsc::Sender<crate::domains::client::ClientRequest>,
    /// Trades domain sender
    pub trades_tx: mpsc::Sender<crate::domains::trades::TradesRequest>,
    /// MCP domain sender
    pub mcp_tx: mpsc::Sender<crate::domains::mcp::McpRequest>,
    /// Alerts domain sender
    pub alerts_tx: mpsc::Sender<crate::domains::alerts::AlertsRequest>,
    /// IPC domain sender (for self-routing)
    pub ipc_tx: mpsc::Sender<crate::domains::ipc::IpcDomainRequest>,
}

/// IPC listener configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum IpcListener {
    /// WebSocket transport
    WebSocket {
        /// Host address (e.g., "127.0.0.1")
        host: String,
        /// Port number
        port: u16,
    },
    /// Unix socket transport
    UnixSocket {
        /// Socket path
        path: String,
    },
    /// Named pipe transport (Windows)
    NamedPipe {
        /// Pipe name
        name: String,
    },
}

impl IpcState {
    /// Create new IPC state with domain gateway registry
    pub fn new(domain_gateways: DomainGatewayRegistry) -> Self {
        Self {
            server: None,
            connections: HashMap::new(),
            domain_gateways,
        }
    }

    /// Check if the IPC server is running
    pub fn is_running(&self) -> bool {
        self.server.is_some()
    }

    /// Get the number of active connections
    pub fn connection_count(&self) -> usize {
        self.connections.len()
    }
}

impl Default for IpcState {
    fn default() -> Self {
        // Create a dummy registry for default state
        // This is mainly for testing purposes
        let (config_tx, _) = mpsc::channel(1);
        let (keystore_tx, _) = mpsc::channel(1);
        let (enclave_tx, _) = mpsc::channel(1);
        let (client_tx, _) = mpsc::channel(1);
        let (trades_tx, _) = mpsc::channel(1);
        let (mcp_tx, _) = mpsc::channel(1);
        let (alerts_tx, _) = mpsc::channel(1);
        let (ipc_tx, _) = mpsc::channel(1);

        Self {
            server: None,
            connections: HashMap::new(),
            domain_gateways: DomainGatewayRegistry {
                config_tx,
                keystore_tx,
                enclave_tx,
                client_tx,
                trades_tx,
                mcp_tx,
                alerts_tx,
                ipc_tx,
            },
        }
    }
}

impl std::fmt::Display for IpcListener {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IpcListener::WebSocket { host, port } => {
                write!(f, "ws://{}:{}", host, port)
            }
            IpcListener::UnixSocket { path } => {
                write!(f, "unix:{}", path)
            }
            IpcListener::NamedPipe { name } => {
                write!(f, "pipe:{}", name)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ipc_state_default() {
        let state = IpcState::default();
        assert!(!state.is_running());
        assert_eq!(state.connection_count(), 0);
    }

    #[test]
    fn test_connection_kind_display() {
        assert_eq!(ConnectionKind::Tauri.to_string(), "tauri");
        assert_eq!(ConnectionKind::CliDaemon.to_string(), "cli_daemon");
        assert_eq!(ConnectionKind::External.to_string(), "external");
    }

    #[test]
    fn test_ipc_listener_display() {
        let ws = IpcListener::WebSocket {
            host: "127.0.0.1".to_string(),
            port: 8080,
        };
        assert_eq!(ws.to_string(), "ws://127.0.0.1:8080");

        let unix = IpcListener::UnixSocket {
            path: "/tmp/poseidon.sock".to_string(),
        };
        assert_eq!(unix.to_string(), "unix:/tmp/poseidon.sock");

        let pipe = IpcListener::NamedPipe {
            name: "poseidon".to_string(),
        };
        assert_eq!(pipe.to_string(), "pipe:poseidon");
    }
}
