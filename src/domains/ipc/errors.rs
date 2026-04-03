//! IPC domain errors
//!
//! Defines domain-specific error types for the IPC domain,
//! including connection errors, routing errors, and server lifecycle errors.

use thiserror::Error;

/// Errors that can occur in the IPC domain
#[derive(Error, Debug, Clone)]
pub enum IpcError {
    /// Connection failed
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),

    /// Invalid request
    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    /// Unauthorized operation
    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    /// Connection not found
    #[error("Connection not found: {0}")]
    ConnectionNotFound(String),

    /// Domain routing failed
    #[error("Domain routing failed: {0}")]
    RoutingFailed(String),

    /// Server already running
    #[error("Server already running")]
    AlreadyRunning,

    /// Server not running
    #[error("Server not running")]
    NotRunning,

    /// Channel error
    #[error("Channel error")]
    ChannelError,

    /// Channel send error
    #[error("Channel send error")]
    ChannelSend,

    /// Channel receive error
    #[error("Channel receive error")]
    ChannelRecv,

    /// Oneshot reply error
    #[error("Oneshot reply error")]
    OneshotReply,

    /// Config domain error
    #[error("Config error: {0}")]
    ConfigError(String),

    /// Keystore domain error
    #[error("Keystore error: {0}")]
    KeystoreError(String),

    /// Enclave domain error
    #[error("Enclave error: {0}")]
    EnclaveError(String),

    /// Client domain error
    #[error("Client error: {0}")]
    ClientError(String),

    /// Trades domain error
    #[error("Trades error: {0}")]
    TradesError(String),

    /// MCP domain error
    #[error("MCP error: {0}")]
    McpError(String),

    /// Alerts domain error
    #[error("Alerts error: {0}")]
    AlertsError(String),
}

/// Result type for IPC operations
pub type IpcResult<T> = Result<T, IpcError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ipc_error_display() {
        let err = IpcError::ConnectionFailed("timeout".to_string());
        assert!(err.to_string().contains("Connection failed"));

        let err = IpcError::InvalidRequest("bad format".to_string());
        assert!(err.to_string().contains("Invalid request"));

        let err = IpcError::Unauthorized("missing token".to_string());
        assert!(err.to_string().contains("Unauthorized"));

        let err = IpcError::AlreadyRunning;
        assert_eq!(err.to_string(), "Server already running");

        let err = IpcError::NotRunning;
        assert_eq!(err.to_string(), "Server not running");

        let err = IpcError::ChannelError;
        assert_eq!(err.to_string(), "Channel error");
    }

    #[test]
    fn test_ipc_error_clone() {
        let err = IpcError::RoutingFailed("config".to_string());
        let cloned = err.clone();
        assert_eq!(err.to_string(), cloned.to_string());
    }

    #[test]
    fn test_domain_errors() {
        let err = IpcError::ConfigError("key not found".to_string());
        assert!(err.to_string().contains("Config error"));

        let err = IpcError::KeystoreError("locked".to_string());
        assert!(err.to_string().contains("Keystore error"));

        let err = IpcError::EnclaveError("no wallet".to_string());
        assert!(err.to_string().contains("Enclave error"));
    }
}
