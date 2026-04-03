//! Client domain errors
//!
//! Migrated from `pkg/poseidon/src/messages/error.rs` - IrisClientError

use thiserror::Error;

/// Domain-specific errors for the client domain
#[derive(Error, Debug, Clone)]
pub enum ClientError {
    /// HTTP error
    #[error("HTTP error: {0}")]
    Http(String),

    /// Connection error
    #[error("Connection error: {0}")]
    Connection(String),

    /// Authentication failed
    #[error("Authentication failed: {0}")]
    Auth(String),

    /// Request timeout
    #[error("Request timeout")]
    Timeout,

    /// Invalid response
    #[error("Invalid response: {0}")]
    InvalidResponse(String),

    /// RPC error
    #[error("RPC error: {0}")]
    Rpc(String),

    /// Not implemented
    #[error("Not implemented: {0}")]
    NotImplemented(String),

    /// Deserialization error
    #[error("Deserialization error: {0}")]
    Deserialization(String),

    /// Serialization error
    #[error("Serialization error: {0}")]
    Serialization(String),

    /// Missing data in response
    #[error("Missing data in response")]
    MissingData,

    /// Channel send error
    #[error("Failed to send message to client actor")]
    ChannelSend,

    /// Channel receive error
    #[error("Failed to receive response from client actor")]
    ChannelRecv,

    /// Manifest error
    #[error("Manifest error: {0}")]
    Manifest(String),

    /// Transport error
    #[error("Transport error: {0}")]
    Transport(String),

    /// Wallet operation error
    #[error("Wallet operation error: {0}")]
    Wallet(String),
}

impl ClientError {
    /// Convert from IrisClientError (from messages module) to ClientError
    pub fn from_iris_error(err: crate::messages::IrisClientError) -> Self {
        use crate::messages::IrisClientError;
        match err {
            IrisClientError::Http(s) => ClientError::Http(s),
            IrisClientError::Connection(s) => ClientError::Connection(s),
            IrisClientError::Auth(s) => ClientError::Auth(s),
            IrisClientError::Timeout => ClientError::Timeout,
            IrisClientError::InvalidResponse(s) => ClientError::InvalidResponse(s),
            IrisClientError::Rpc(s) => ClientError::Rpc(s),
            IrisClientError::NotImplemented(s) => ClientError::NotImplemented(s),
            IrisClientError::Deserialization(s) => ClientError::Deserialization(s),
            IrisClientError::Serialization(s) => ClientError::Serialization(s),
            IrisClientError::MissingData => ClientError::MissingData,
        }
    }
}

impl From<ClientError> for crate::messages::IrisClientError {
    fn from(err: ClientError) -> Self {
        match err {
            ClientError::Http(s) => crate::messages::IrisClientError::Http(s),
            ClientError::Connection(s) => crate::messages::IrisClientError::Connection(s),
            ClientError::Auth(s) => crate::messages::IrisClientError::Auth(s),
            ClientError::Timeout => crate::messages::IrisClientError::Timeout,
            ClientError::InvalidResponse(s) => crate::messages::IrisClientError::InvalidResponse(s),
            ClientError::Rpc(s) => crate::messages::IrisClientError::Rpc(s),
            ClientError::NotImplemented(s) => crate::messages::IrisClientError::NotImplemented(s),
            ClientError::Deserialization(s) => crate::messages::IrisClientError::Deserialization(s),
            ClientError::Serialization(s) => crate::messages::IrisClientError::Serialization(s),
            ClientError::MissingData => crate::messages::IrisClientError::MissingData,
            _ => crate::messages::IrisClientError::Http(err.to_string()),
        }
    }
}
