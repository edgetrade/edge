//! ipc domain - External entry point (Tauri, CLI-to-daemon)
//!
//! This domain manages external connections from Tauri, CLI-to-daemon,
//! and other clients. It provides the primary interface for external
//! clients to interact with the daemon.
//!
//! ## Architecture
//!
//! The IPC domain follows the actor/handler pattern:
//! - **handle.rs**: `IpcHandle` - thin gateway that sends messages to the actor
//! - **actor.rs**: `IpcActor` - state owner that processes messages and manages connections
//! - **state.rs**: `IpcState`, `IpcConnection`, `ConnectionKind`, `DomainGatewayRegistry` - state types
//! - **messages.rs**: `IpcMessage`, `IpcRequest`, `IpcResponse`, `IpcEvent` - message types
//! - **errors.rs**: `IpcError` - domain-specific errors
//! - **protocol.rs**: JSON-RPC protocol definitions
//!
//! ## Domain Gateway Registry
//!
//! The `DomainGatewayRegistry` contains mpsc senders for all domains that IPC
//! can route to. This enables direct async communication without going through
//! the EventBus for request/response patterns.

mod actor;
mod errors;
mod handle;
mod messages;
mod protocol;
mod state;

// Public exports - handle/actor pattern types
pub use actor::IpcActor;
pub use errors::{IpcError, IpcResult};
pub use handle::IpcHandle;
pub use messages::{IpcDomainRequest, IpcEvent, IpcMessage, IpcRequest, IpcResponse, TradeAction};
pub use protocol::json_rpc;
pub use state::{
    ConnectionId, ConnectionKind, DomainGatewayRegistry, IpcConnection, IpcListener, IpcResponse as IpcResponseState,
    IpcServer, IpcState,
};

// Re-export commonly used types for convenience
pub mod ipc_types {
    pub use super::messages::{IpcEvent, IpcMessage, IpcRequest, IpcResponse, TradeAction};
    pub use super::state::{ConnectionKind, IpcListener, IpcState};
}

/// Create a new IPC handle with domain gateway registry
///
/// Convenience function for creating the IPC domain handle and actor.
///
/// # Arguments
///
/// * `domain_gateways` - DomainGatewayRegistry containing senders for all domains
/// * `receiver` - The mpsc receiver channel for the actor
/// * `event_bus` - EventBus for publishing state events
///
/// # Returns
///
/// A tuple of `(IpcHandle, JoinHandle<()>)` that can be used to interact with the IPC domain
///
/// # Example
///
/// ```rust,no_run
/// use poseidon::domains::ipc::{DomainGatewayRegistry, create_ipc_handle, IpcListener};
/// use poseidon::event_bus::EventBus;
/// use tokio::sync::mpsc;
///
/// # async fn example() {
/// let event_bus = EventBus::new(128);
///
/// // Create domain gateway registry with senders from other domains
/// let (config_tx, _) = mpsc::channel(64);
/// let (keystore_tx, _) = mpsc::channel(64);
/// let (enclave_tx, _) = mpsc::channel(64);
/// let (client_tx, _) = mpsc::channel(64);
/// let (trades_tx, _) = mpsc::channel(64);
/// let (mcp_tx, _) = mpsc::channel(64);
/// let (alerts_tx, _) = mpsc::channel(64);
/// let (ipc_tx, _) = mpsc::channel(64);
///
/// let gateways = DomainGatewayRegistry {
///     config_tx,
///     keystore_tx,
///     enclave_tx,
///     client_tx,
///     trades_tx,
///     mcp_tx,
///     alerts_tx,
///     ipc_tx,
/// };
///
/// // IPC handle creates its own channel internally
/// let (ipc_handle, _join_handle) = create_ipc_handle(gateways, event_bus);
///
/// // Start the IPC server (IpcHandle::start returns a JoinHandle)
/// let _server_handle = ipc_handle.start(IpcListener::WebSocket {
///     host: "127.0.0.1".to_string(),
///     port: 8080,
/// }).await.unwrap();
/// # }
/// ```
pub fn create_ipc_handle(
    domain_gateways: DomainGatewayRegistry,
    event_bus: crate::event_bus::EventBus,
) -> (IpcHandle, tokio::task::JoinHandle<()>) {
    IpcHandle::new(domain_gateways, event_bus)
}
