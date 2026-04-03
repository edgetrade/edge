//! IPC domain messages
//!
//! Defines the command/query enums for IPC actor communication
//! and the external-facing IpcRequest/IpcResponse types.

use serde::{Deserialize, Serialize};

use crate::domains::ipc::errors::IpcError;
use crate::domains::ipc::state::{ConnectionId, IpcListener, IpcState};
use crate::event_bus::PoseidonRequest;

/// Messages sent to the IpcActor
///
/// These are internal messages used by the handle/actor pattern.
/// External clients use IpcRequest/IpcResponse for communication.
#[derive(Debug)]
pub enum IpcMessage {
    /// Start the IPC server
    Start {
        /// Listener configuration
        listener: IpcListener,
    },

    /// Stop the IPC server
    Stop,

    /// Route a request to the appropriate domain
    RouteRequest {
        /// Connection ID
        connection_id: ConnectionId,
        /// Request to route
        request: IpcRequest,
        /// Reply channel for response
        reply_to: tokio::sync::oneshot::Sender<IpcResponse>,
    },

    /// Handle client connection
    ClientConnected {
        /// Connection ID
        connection_id: ConnectionId,
        /// Connection kind
        kind: crate::domains::ipc::state::ConnectionKind,
        /// Sender for responses
        sender: tokio::sync::mpsc::Sender<IpcResponse>,
    },

    /// Handle client disconnection
    ClientDisconnected {
        /// Connection ID
        connection_id: ConnectionId,
    },

    /// Broadcast event to all connections
    BroadcastEvent {
        /// Event data
        event: serde_json::Value,
    },
}

/// Request type using PoseidonRequest pattern
///
/// IpcDomainRequest wraps IpcMessage with trace context and reply channel.
/// This enables request/response communication with telemetry support.
pub type IpcDomainRequest = PoseidonRequest<IpcMessage, IpcState, IpcError>;

/// IPC request from external clients
///
/// These are the operations that external clients (Tauri, CLI-to-daemon)
/// can request from the IPC domain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IpcRequest {
    // === Config operations ===
    /// Get a configuration value
    GetConfig {
        /// Configuration key
        key: String,
    },
    /// Update a configuration value
    UpdateConfig {
        /// Configuration key
        key: String,
        /// New value
        value: serde_json::Value,
    },

    // === Keystore operations ===
    /// Unlock the keystore
    UnlockKeystore {
        /// Password
        password: String,
    },
    /// Lock the keystore
    LockKeystore,

    // === Enclave/Wallet operations ===
    /// List all wallets
    ListWallets,
    /// Create a new wallet
    CreateWallet {
        /// Chain type
        chain: String,
        /// Wallet name
        name: String,
    },
    /// Import a wallet from private key
    ImportWallet {
        /// Chain type
        chain: String,
        /// Wallet name
        name: String,
        /// Private key (will be zeroized after use)
        private_key: String,
    },
    /// Delete a wallet
    DeleteWallet {
        /// Wallet address or name
        address: String,
    },

    // === Trade operations ===
    /// Create a trade intent
    CreateTradeIntent {
        /// Wallet to use
        wallet: String,
        /// Trade action
        action: TradeAction,
    },
    /// Confirm a trade intent
    ConfirmTradeIntent {
        /// Intent ID
        intent_id: u64,
    },
    /// Get trade status
    GetTradeStatus {
        /// Intent ID
        intent_id: u64,
    },

    // === MCP operations ===
    /// Get MCP server status
    GetMcpStatus,
    /// Start MCP server
    StartMcp {
        /// Transport type
        transport: String,
    },
    /// Stop MCP server
    StopMcp,

    // === Alert operations ===
    /// Subscribe to a procedure
    Subscribe {
        /// Procedure to subscribe to
        procedure: String,
    },
    /// Register a webhook for alerts
    RegisterWebhook {
        /// Webhook URL
        url: String,
        /// Optional HMAC secret
        secret: Option<String>,
    },

    // === Event subscription ===
    /// Subscribe to state events
    SubscribeEvents,
}

/// Trade action types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TradeAction {
    /// Swap tokens
    Swap {
        /// Source token
        from_token: String,
        /// Target token
        to_token: String,
        /// Amount
        amount: String,
    },
    /// Transfer tokens
    Transfer {
        /// Destination address
        to: String,
        /// Token to transfer
        token: String,
        /// Amount
        amount: String,
    },
}

/// IPC response to external clients
///
/// These are the responses sent back to external clients.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IpcResponse {
    /// Successful response with data
    Success {
        /// Request ID
        request_id: u64,
        /// Response data
        data: serde_json::Value,
    },
    /// Error response
    Error {
        /// Request ID
        request_id: u64,
        /// Error message
        error: String,
    },
    /// Event broadcast
    Event {
        /// Event data
        event: serde_json::Value,
    },
}

/// Events emitted by the IPC domain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IpcEvent {
    /// Client connected
    ClientConnected {
        /// Connection ID
        connection_id: String,
        /// Connection kind
        kind: String,
    },
    /// Client disconnected
    ClientDisconnected {
        /// Connection ID
        connection_id: String,
    },
    /// Request received
    RequestReceived {
        /// Request ID
        request_id: u64,
        /// Method called
        method: String,
    },
    /// Server started
    ServerStarted {
        /// Listener configuration
        listener: String,
    },
    /// Server stopped
    ServerStopped,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ipc_message_variants() {
        let start = IpcMessage::Start {
            listener: IpcListener::WebSocket {
                host: "127.0.0.1".to_string(),
                port: 8080,
            },
        };
        assert!(matches!(start, IpcMessage::Start { .. }));

        let stop = IpcMessage::Stop;
        assert!(matches!(stop, IpcMessage::Stop));
    }

    #[test]
    fn test_ipc_request_variants() {
        let get_config = IpcRequest::GetConfig {
            key: "api.url".to_string(),
        };
        assert!(matches!(get_config, IpcRequest::GetConfig { .. }));

        let create_wallet = IpcRequest::CreateWallet {
            chain: "EVM".to_string(),
            name: "test".to_string(),
        };
        assert!(matches!(create_wallet, IpcRequest::CreateWallet { .. }));

        let create_intent = IpcRequest::CreateTradeIntent {
            wallet: "0x1234".to_string(),
            action: TradeAction::Swap {
                from_token: "ETH".to_string(),
                to_token: "USDC".to_string(),
                amount: "1.0".to_string(),
            },
        };
        assert!(matches!(create_intent, IpcRequest::CreateTradeIntent { .. }));
    }

    #[test]
    fn test_trade_action_variants() {
        let swap = TradeAction::Swap {
            from_token: "ETH".to_string(),
            to_token: "USDC".to_string(),
            amount: "1.0".to_string(),
        };
        assert!(matches!(swap, TradeAction::Swap { .. }));

        let transfer = TradeAction::Transfer {
            to: "0x1234".to_string(),
            token: "ETH".to_string(),
            amount: "1.0".to_string(),
        };
        assert!(matches!(transfer, TradeAction::Transfer { .. }));
    }

    #[test]
    fn test_ipc_response_variants() {
        let success = IpcResponse::Success {
            request_id: 1,
            data: serde_json::json!({"status": "ok"}),
        };
        assert!(matches!(success, IpcResponse::Success { .. }));

        let error = IpcResponse::Error {
            request_id: 1,
            error: "not found".to_string(),
        };
        assert!(matches!(error, IpcResponse::Error { .. }));

        let event = IpcResponse::Event {
            event: serde_json::json!({"type": "update"}),
        };
        assert!(matches!(event, IpcResponse::Event { .. }));
    }

    #[test]
    fn test_ipc_event_variants() {
        let connected = IpcEvent::ClientConnected {
            connection_id: "conn-1".to_string(),
            kind: "tauri".to_string(),
        };
        assert!(matches!(connected, IpcEvent::ClientConnected { .. }));

        let request = IpcEvent::RequestReceived {
            request_id: 42,
            method: "get_config".to_string(),
        };
        assert!(matches!(request, IpcEvent::RequestReceived { .. }));
    }
}
