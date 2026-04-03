//! MCP domain - MCP server lifecycle (stdio/HTTP)
//!
//! This domain manages the Model Context Protocol (MCP) server lifecycle,
//! supporting both stdio (for AI agents) and HTTP (for web clients) transports.
//!
//! MIGRATED FROM: commands/serve/mcp.rs
//! - All MCP server code migrated to actor/handler pattern
//! - Server lifecycle management in actor.rs
//! - Public interface in handle.rs
//! - Messages and events in messages.rs
//! - Error types in errors.rs
//!
//! ## Architecture
//!
//! The MCP domain follows the Demeter pattern (Actor::new + run_actor):
//! - **handle.rs**: `McpHandle` - creates channel, spawns actor, returns handle
//! - **actor.rs**: `McpActor` - state owner + state types, runs message loop
//! - **messages.rs**: `McpMessage`, `McpRequest`, `McpEvent` - message types
//! - **errors.rs**: `McpError` - domain-specific errors
//!
//! ## Usage
//!
//! ```rust,ignore
//! use poseidon::domains::mcp::McpHandle;
//! use poseidon::domains::client::ClientHandle;
//! use poseidon::domains::enclave::EnclaveHandle;
//! use poseidon::domains::trades::TradesHandle;
//! use poseidon::domains::alerts::AlertsHandle;
//! use poseidon::event_bus::EventBus;
//!
//! // After setting up required handles from other domains:
//! // let (mcp_handle, actor_task) = McpHandle::new(client, enclave, trades, alerts, event_bus);
//! //
//! // Start stdio server:
//! // mcp_handle.start_stdio().await?;
//! //
//! // Or start HTTP server:
//! // mcp_handle.start_http("127.0.0.1", 8080).await?;
//! //
//! // Check status:
//! // let status = mcp_handle.get_status().await?;
//! ```

mod actor;
mod errors;
mod handle;
mod messages;
mod server;

#[cfg(test)]
mod tests;

// Public exports - handle/actor pattern types
pub use actor::{EdgeServerHandle, McpActor, McpMode, McpState, TransportType};
pub use errors::{McpError, McpResult};
pub use handle::McpHandle;
pub use messages::{McpEvent, McpMessage, McpRequest};

// Re-export EdgeServer and related types from server module
//
// MIGRATED FROM: commands/serve/mcp.rs
// These types provide the actual MCP server implementation with:
// - ServerHandler trait implementation for MCP protocol
// - Tool/resource/prompt handlers
// - Local action handlers (ping, list_alerts, etc.)
// - Subscription handlers (subscribe, poll, stop)
pub use server::{ActiveSubscriptions, AlertDelivery, AlertRegistry, EdgeServer, WebhookDispatcher, next_alert_id};

// Re-export message types that are commonly used together
pub mod mcp_types {
    pub use super::actor::{McpMode, McpState, TransportType};
    pub use super::messages::{McpEvent, McpMessage};
    pub use super::server::{AlertDelivery, EdgeServer};
}
