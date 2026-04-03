//! Client domain state
//!
//! Defines the state structures for the client domain actor.
//! Migrated from `client/trpc.rs` and `manifest/manager.rs`

use std::sync::Arc;
use tokio::sync::RwLock;

use crate::domains::client::manifest::ManifestManager;
use crate::domains::client::manifest::types::McpManifest;
use crate::domains::client::trpc::IrisClient;

/// Client state containing Iris connection and manifest
///
/// This is the authoritative state owned by the ClientActor.
/// External domains access state through the ClientHandle.
#[derive(Debug)]
pub struct ClientState {
    /// Iris API client connection (from trpc module)
    pub iris_client: Option<IrisClient>,
    /// MCP manifest from Iris API
    pub manifest: Option<McpManifest>,
    /// Manifest manager for background refresh
    pub manifest_manager: Option<Arc<RwLock<ManifestManager>>>,
    /// Background refresh task handle
    pub manifest_refresh: Option<tokio::task::JoinHandle<()>>,
    /// Connection URL
    pub url: Option<String>,
    /// API key
    pub api_key: Option<String>,
    /// Verbose mode
    pub verbose: bool,
}

impl ClientState {
    /// Create a new empty client state
    pub fn new() -> Self {
        Self {
            iris_client: None,
            manifest: None,
            manifest_manager: None,
            manifest_refresh: None,
            url: None,
            api_key: None,
            verbose: false,
        }
    }

    /// Check if connected to Iris API
    pub fn is_connected(&self) -> bool {
        self.iris_client.is_some()
    }

    /// Get connection URL if connected
    pub fn url(&self) -> Option<&str> {
        self.url.as_deref()
    }
}

impl Default for ClientState {
    fn default() -> Self {
        Self::new()
    }
}

/// Tool definition from MCP manifest
///
/// Migrated from `manifest/types.rs` - `ToolDef`
#[derive(Debug, Clone)]
pub struct ToolDefinition {
    /// Tool name
    pub name: String,
    /// Tool description
    pub description: String,
    /// Available actions
    pub actions: Vec<ActionDefinition>,
}

/// Action definition within a tool
///
/// Migrated from `manifest/types.rs` - `ActionDef`
#[derive(Debug, Clone)]
pub struct ActionDefinition {
    /// Action name
    pub name: String,
    /// Action description
    pub description: String,
    /// tRPC procedure path
    pub procedure: String,
    /// Action kind: query, mutation, or subscription
    pub kind: ActionKind,
}

/// Action kind enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionKind {
    /// Read-only query
    Query,
    /// Write mutation
    Mutation,
    /// Real-time subscription
    Subscription,
}

/// Resource definition from MCP manifest
///
/// Migrated from `manifest/types.rs` - `ResourceDef`
#[derive(Debug, Clone)]
pub struct ResourceDefinition {
    /// Resource URI
    pub uri: String,
    /// Resource name
    pub name: String,
    /// Resource description
    pub description: String,
    /// MIME type
    pub mime_type: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_state_new() {
        let state = ClientState::new();
        assert!(!state.is_connected());
        assert!(state.url().is_none());
        assert!(state.iris_client.is_none());
        assert!(state.manifest.is_none());
    }

    #[test]
    fn test_action_kind_variants() {
        assert_eq!(ActionKind::Query, ActionKind::Query);
        assert_ne!(ActionKind::Query, ActionKind::Mutation);
    }

    #[test]
    fn test_tool_definition_creation() {
        let tool = ToolDefinition {
            name: "test_tool".to_string(),
            description: "A test tool".to_string(),
            actions: vec![],
        };
        assert_eq!(tool.name, "test_tool");
    }

    #[test]
    fn test_resource_definition_creation() {
        let resource = ResourceDefinition {
            uri: "test://resource".to_string(),
            name: "test_resource".to_string(),
            description: "A test resource".to_string(),
            mime_type: "application/json".to_string(),
        };
        assert_eq!(resource.uri, "test://resource");
    }
}
