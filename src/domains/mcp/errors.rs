//! MCP domain errors
//!
//! Defines domain-specific error types for the MCP (Model Context Protocol)
//! server operations, including transport errors, lifecycle errors, and configuration errors.

use thiserror::Error;

/// Errors that can occur in the MCP domain
#[derive(Debug, Clone, Error)]
pub enum McpError {
    /// Server is already running
    #[error("Server already running")]
    AlreadyRunning,

    /// Server is not running
    #[error("Server not running")]
    NotRunning,

    /// Port is already in use
    #[error("Port in use: {0}")]
    PortInUse(u16),

    /// Manifest unavailable
    #[error("Manifest unavailable")]
    ManifestUnavailable,

    /// Transport error (stdio, HTTP)
    #[error("Transport error: {0}")]
    TransportError(String),

    /// Channel send error (actor communication)
    #[error("Channel error")]
    ChannelError,

    /// Server failed to start
    #[error("Server failed to start: {0}")]
    ServerStart(String),

    /// Server failed to stop
    #[error("Server failed to stop: {0}")]
    ServerStop(String),

    /// Invalid transport configuration
    #[error("Invalid transport: {0}")]
    InvalidTransport(String),

    /// Service error from underlying MCP library
    #[error("Service error: {0}")]
    Service(String),

    /// Reply channel closed
    #[error("Reply channel closed")]
    ReplyChannelClosed,

    /// Channel receive error
    #[error("Channel receive error")]
    ChannelRecv,

    /// Channel send error (alternative variant)
    #[error("Channel send error")]
    ChannelSend,
}

/// Result type for MCP domain operations
pub type McpResult<T> = std::result::Result<T, McpError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mcp_error_display() {
        let err = McpError::AlreadyRunning;
        assert_eq!(err.to_string(), "Server already running");

        let err = McpError::NotRunning;
        assert_eq!(err.to_string(), "Server not running");

        let err = McpError::PortInUse(8080);
        assert_eq!(err.to_string(), "Port in use: 8080");

        let err = McpError::ManifestUnavailable;
        assert_eq!(err.to_string(), "Manifest unavailable");

        let err = McpError::TransportError("stdio failed".to_string());
        assert_eq!(err.to_string(), "Transport error: stdio failed");
    }

    #[test]
    fn test_mcp_error_clone() {
        let err = McpError::PortInUse(8080);
        let cloned = err.clone();
        assert_eq!(err.to_string(), cloned.to_string());
    }
}
