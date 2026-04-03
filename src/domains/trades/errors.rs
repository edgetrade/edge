//! Trades domain error types
//!
//! Defines all error variants for trade operations including
//! intent management, signing orchestration, and submission to Iris API.

use thiserror::Error;

/// Result type for trades operations.
pub type TradesResult<T> = Result<T, TradesError>;

/// Errors that can occur in the trades domain.
#[derive(Debug, Error, Clone)]
pub enum TradesError {
    /// Intent has expired.
    #[error("Intent expired: {0}")]
    IntentExpired(u64),

    /// Intent not found.
    #[error("Intent not found: {0}")]
    IntentNotFound(u64),

    /// Signing failed in enclave.
    #[error("Signing failed: {0}")]
    SigningFailed(String),

    /// Submission to Iris API failed.
    #[error("Submission failed: {0}")]
    SubmissionFailed(String),

    /// Wallet not found in enclave.
    #[error("Wallet not found: {0}")]
    WalletNotFound(String),

    /// Invalid trade action.
    #[error("Invalid trade action")]
    InvalidAction,

    /// Invalid chain type.
    #[error("Invalid chain type: {0}")]
    InvalidChain(String),

    /// Channel communication error.
    #[error("Channel error")]
    ChannelError,

    /// Channel send error.
    #[error("Channel send error")]
    ChannelSend,

    /// Channel receive error.
    #[error("Channel receive error")]
    ChannelRecv,

    /// Oneshot reply error.
    #[error("Oneshot reply error")]
    OneshotReply,

    /// Enclave operation error.
    #[error("Enclave error: {0}")]
    Enclave(String),

    /// Client operation error.
    #[error("Client error: {0}")]
    Client(String),

    /// Trade already active.
    #[error("Trade already active: {0}")]
    AlreadyActive(u64),

    /// Trade not in correct state for operation.
    #[error("Invalid state for operation: expected {expected}, got {actual}")]
    InvalidState {
        /// Expected state
        expected: String,
        /// Actual state
        actual: String,
    },

    /// Timeout waiting for operation.
    #[error("Operation timeout: {0}")]
    Timeout(String),

    /// Configuration error.
    #[error("Configuration error: {0}")]
    Config(String),

    /// Serialization error.
    #[error("Serialization error: {0}")]
    Serialization(String),

    /// Invalid input.
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    /// Invalid response from actor.
    #[error("Invalid response: {0}")]
    InvalidResponse(String),

    /// Execution failed (e.g., spot order execution).
    #[error("Execution failed: {0}")]
    ExecutionFailed(String),
}

impl TradesError {
    /// Returns true if this error indicates an intent was not found.
    pub fn is_not_found(&self) -> bool {
        matches!(self, TradesError::IntentNotFound(_))
    }

    /// Returns true if this error indicates an intent has expired.
    pub fn is_expired(&self) -> bool {
        matches!(self, TradesError::IntentExpired(_))
    }

    /// Returns true if this error indicates a signing failure.
    pub fn is_signing_failed(&self) -> bool {
        matches!(self, TradesError::SigningFailed(_))
    }

    /// Returns true if this error indicates a submission failure.
    pub fn is_submission_failed(&self) -> bool {
        matches!(self, TradesError::SubmissionFailed(_))
    }

    /// Returns true if this error indicates the wallet was not found.
    pub fn is_wallet_not_found(&self) -> bool {
        matches!(self, TradesError::WalletNotFound(_))
    }

    /// Returns true if this error indicates a channel error.
    pub fn is_channel_error(&self) -> bool {
        matches!(
            self,
            TradesError::ChannelError | TradesError::ChannelSend | TradesError::ChannelRecv | TradesError::OneshotReply
        )
    }

    /// Get the intent ID if this error relates to a specific intent.
    pub fn intent_id(&self) -> Option<u64> {
        match self {
            TradesError::IntentExpired(id) => Some(*id),
            TradesError::IntentNotFound(id) => Some(*id),
            TradesError::AlreadyActive(id) => Some(*id),
            _ => None,
        }
    }
}

impl From<std::io::Error> for TradesError {
    fn from(e: std::io::Error) -> Self {
        TradesError::Client(format!("IO error: {}", e))
    }
}

impl From<serde_json::Error> for TradesError {
    fn from(e: serde_json::Error) -> Self {
        TradesError::Serialization(e.to_string())
    }
}

impl From<std::str::Utf8Error> for TradesError {
    fn from(e: std::str::Utf8Error) -> Self {
        TradesError::InvalidInput(format!("UTF-8 error: {}", e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trades_error_display() {
        let err = TradesError::IntentExpired(42);
        assert!(err.to_string().contains("42"));

        let err = TradesError::WalletNotFound("test_wallet".to_string());
        assert!(err.to_string().contains("test_wallet"));

        let err = TradesError::SigningFailed("key unavailable".to_string());
        assert!(err.to_string().contains("key unavailable"));
    }

    #[test]
    fn test_is_not_found() {
        assert!(TradesError::IntentNotFound(1).is_not_found());
        assert!(!TradesError::IntentExpired(1).is_not_found());
        assert!(!TradesError::SigningFailed("test".to_string()).is_not_found());
    }

    #[test]
    fn test_is_expired() {
        assert!(TradesError::IntentExpired(1).is_expired());
        assert!(!TradesError::IntentNotFound(1).is_expired());
    }

    #[test]
    fn test_is_signing_failed() {
        assert!(TradesError::SigningFailed("test".to_string()).is_signing_failed());
        assert!(!TradesError::IntentExpired(1).is_signing_failed());
    }

    #[test]
    fn test_is_submission_failed() {
        assert!(TradesError::SubmissionFailed("test".to_string()).is_submission_failed());
        assert!(!TradesError::IntentExpired(1).is_submission_failed());
    }

    #[test]
    fn test_is_wallet_not_found() {
        assert!(TradesError::WalletNotFound("test".to_string()).is_wallet_not_found());
        assert!(!TradesError::IntentExpired(1).is_wallet_not_found());
    }

    #[test]
    fn test_is_channel_error() {
        assert!(TradesError::ChannelError.is_channel_error());
        assert!(TradesError::ChannelSend.is_channel_error());
        assert!(TradesError::ChannelRecv.is_channel_error());
        assert!(TradesError::OneshotReply.is_channel_error());
        assert!(!TradesError::IntentExpired(1).is_channel_error());
    }

    #[test]
    fn test_intent_id() {
        assert_eq!(TradesError::IntentExpired(42).intent_id(), Some(42));
        assert_eq!(TradesError::IntentNotFound(42).intent_id(), Some(42));
        assert_eq!(TradesError::AlreadyActive(42).intent_id(), Some(42));
        assert_eq!(TradesError::SigningFailed("test".to_string()).intent_id(), None);
    }

    #[test]
    fn test_from_io_error() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let err: TradesError = io_err.into();
        assert!(matches!(err, TradesError::Client(_)));
    }

    #[test]
    fn test_from_serde_error() {
        let json = "{ invalid json";
        let result: Result<serde_json::Value, _> = serde_json::from_str(json);
        let err: TradesError = result.unwrap_err().into();
        assert!(matches!(err, TradesError::Serialization(_)));
    }
}
