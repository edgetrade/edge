//! Client domain - Iris API client + manifest + generated route handlers
//!
//! Migrated from:
//! - `pkg/poseidon/src/client/` (trpc.rs, executor.rs, routes.rs, subscribe.rs)
//! - `pkg/poseidon/src/manifest/` (cache.rs, fetch.rs, inject.rs, manager.rs, types.rs)
//! - `pkg/poseidon/src/generated/` (routes/requests/*.rs, routes/subscriptions/*.rs)
//! - `pkg/poseidon/src/session/transport.rs` (transport.rs)
//!
//! Architecture:
//! - `ClientActor`: State owner running in background task
//! - `ClientHandle`: Thin gateway for sending messages
//! - `ClientMessage`: Command/query enum
//! - `ClientState`: Domain state (iris_client, manifest, etc.)
//! - `ClientError`: Domain-specific error types
//!
//! # Example
//! ```rust,no_run
//! use poseidon::domains::client::ClientHandle;
//! use poseidon::event_bus::EventBus;
//!
//! # async fn example() {
//! let event_bus = EventBus::new(128);
//! let client = ClientHandle::new(event_bus);
//!
//! // Connect to Iris API
//! client.connect(
//!     "wss://api.example.com".to_string(),
//!     "api-key".to_string(),
//!     false
//! ).await.unwrap();
//!
//! // Get manifest
//! let manifest = client.get_manifest().await.unwrap();
//! # }
//! ```

// === Core domain modules (actor/handler pattern) ===

mod state;
pub use state::{ActionDefinition, ActionKind, ClientState, ResourceDefinition, ToolDefinition};

mod messages;
pub use messages::{ClientMessage, ClientRequest, ClientResponse};

mod actor;
pub use actor::ClientActor;

mod handle;
pub use handle::ClientHandle;

mod errors;
pub use errors::ClientError;

// === Legacy modules (kept for backward compatibility) ===

/// Legacy tRPC client module
mod trpc;
pub use trpc::{IrisClient, Route, RouteExecutor, RouteType, new_client};

/// Legacy subscribe module
mod subscribe;
pub use subscribe::{DispatchParams, IrisClientInner, subscribe, subscribe_for_dispatch, unsubscribe};

/// Legacy routes module (deprecated - use ClientHandle methods instead)
mod routes;
pub use routes::{
    delete_wallet, list_wallets, place_spot_order, proof_game, rotate_user_encryption_key, upsert_wallet,
};

/// Transport key cache module
mod transport;
pub use transport::{
    CachedTransportKeys, DEFAULT_TRANSPORT_KEY_TTL_MINUTES, TRANSPORT_KEY_FILENAME, TransportCacheError,
    delete_cached_transport_keys, get_transport_key, is_cache_fresh, load_cached_transport_keys, save_transport_keys,
    transport_key_path, verify_attestation_document,
};

// === Submodules ===

/// Manifest submodules
pub mod manifest;

/// Generated route handlers
pub mod generated;

// Re-export IrisClientError from messages module for backward compatibility
pub use crate::messages::IrisClientError;

/// Trading-related routes that delegate to trades domain
///
/// These routes are extracted from the client domain and should be
/// accessed through the trades domain for trading operations.
pub mod trading_routes {
    pub use crate::domains::client::generated::routes::requests::{
        orders_apply_entry_strategy, orders_apply_exit_strategy, orders_place_spot_order,
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event_bus::EventBus;

    #[test]
    fn test_module_exports() {
        // Verify that all public types are exported
        let _: fn() -> ClientState = ClientState::new;
        let _: fn(EventBus) -> ClientHandle = ClientHandle::new;
    }

    #[test]
    fn test_client_error_display() {
        let err = ClientError::Connection("test".to_string());
        assert!(err.to_string().contains("Connection"));
    }
}
