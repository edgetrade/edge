//! MCP domain messages
//!
//! Defines the command/query enums for MCP operations using PoseidonRequest pattern.

use crate::domains::mcp::actor::{McpState, TransportType};
use crate::domains::mcp::errors::McpError;
use crate::event_bus::PoseidonRequest;

/// Messages that can be sent to the MCP actor
#[derive(Debug)]
pub enum McpMessage {
    /// Start the MCP server
    Start {
        /// Transport configuration
        transport: TransportType,
    },
    /// Stop the MCP server
    Stop,
    /// Get current server status
    GetStatus,
}

/// Request type using PoseidonRequest pattern
pub type McpRequest = PoseidonRequest<McpMessage, McpState, McpError>;

/// Events emitted by the MCP domain
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum McpEvent {
    /// Server started successfully
    ServerStarted {
        /// Transport type used
        transport: String,
    },
    /// Server stopped
    ServerStopped,
    /// Server failed to start
    ServerFailed {
        /// Error message
        error: String,
    },
}

/// Server capabilities
#[allow(dead_code)]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct ServerCapabilities {
    /// Tools are supported
    pub tools: bool,
    /// Resources are supported
    pub resources: bool,
    /// Prompts are supported
    pub prompts: bool,
}

/// Server information
#[allow(dead_code)]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ServerInfo {
    /// Server name
    pub name: String,
    /// Server version
    pub version: String,
    /// Server capabilities
    pub capabilities: ServerCapabilities,
}

/// Server mode state (legacy, for compatibility)
#[allow(dead_code)]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ServerMode {
    /// Server is stopped
    Stopped,
    /// Server is running
    Running {
        /// Whether server can be gracefully shut down
        can_stop: bool,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mcp_message_variants() {
        let start_msg = McpMessage::Start {
            transport: TransportType::Stdio,
        };
        assert!(matches!(start_msg, McpMessage::Start { .. }));

        let stop_msg = McpMessage::Stop;
        assert!(matches!(stop_msg, McpMessage::Stop));

        let status_msg = McpMessage::GetStatus;
        assert!(matches!(status_msg, McpMessage::GetStatus));
    }

    #[test]
    fn test_mcp_event_variants() {
        let started = McpEvent::ServerStarted {
            transport: "stdio".to_string(),
        };
        assert!(matches!(started, McpEvent::ServerStarted { .. }));

        let stopped = McpEvent::ServerStopped;
        assert!(matches!(stopped, McpEvent::ServerStopped));

        let failed = McpEvent::ServerFailed {
            error: "test error".to_string(),
        };
        assert!(matches!(failed, McpEvent::ServerFailed { .. }));
    }

    #[test]
    fn test_server_capabilities_default() {
        let caps = ServerCapabilities::default();
        assert!(!caps.tools);
        assert!(!caps.resources);
        assert!(!caps.prompts);
    }

    #[test]
    fn test_server_info_creation() {
        let info = ServerInfo {
            name: "edge".to_string(),
            version: "1.0.0".to_string(),
            capabilities: ServerCapabilities {
                tools: true,
                resources: true,
                prompts: true,
            },
        };
        assert_eq!(info.name, "edge");
        assert_eq!(info.version, "1.0.0");
        assert!(info.capabilities.tools);
    }
}
