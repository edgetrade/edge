//! Alerts domain errors
//!
//! Errors that can occur in the alerts domain.
//! REFACTORED FOR: PoseidonRequest pattern integration

use thiserror::Error;

/// Errors that can occur in the alerts domain
///
/// PRESERVED: All existing error variants from original implementation
/// ADDED: ChannelSend, ChannelRecv, OneshotReply variants for PoseidonRequest pattern
#[derive(Error, Debug, Clone)]
pub enum AlertsError {
    /// Failed to send message to actor
    #[error("Channel send error")]
    ChannelSend,

    /// Failed to receive response from actor
    #[error("Channel receive error")]
    ChannelRecv,

    /// Failed to get reply from oneshot channel
    #[error("Oneshot reply error")]
    OneshotReply,

    /// Subscription failed
    #[error("Subscription failed: {0}")]
    SubscriptionFailed(String),

    /// Delivery failed
    #[error("Delivery failed: {0}")]
    DeliveryFailed(String),

    /// Subscription not found
    #[error("Subscription not found: {0}")]
    SubscriptionNotFound(u32),

    /// Delivery target not found
    #[error("Delivery target not found: {0}")]
    DeliveryNotFound(u64),

    /// Webhook error
    #[error("Webhook error: {0}")]
    WebhookError(String),

    /// Redis error
    #[error("Redis error: {0}")]
    RedisError(String),

    /// Telegram error
    #[error("Telegram error: {0}")]
    TelegramError(String),

    /// Webhook delivery failed
    #[error("Webhook delivery failed: {0}")]
    WebhookDeliveryFailed(String),

    /// Redis delivery failed
    #[error("Redis delivery failed: {0}")]
    RedisDeliveryFailed(String),

    /// Telegram delivery failed
    #[error("Telegram delivery failed: {0}")]
    TelegramDeliveryFailed(String),

    /// Alert registration not found
    #[error("Alert registration not found: {0}")]
    AlertNotFound(u64),

    /// Failed to subscribe to procedure
    #[error("Failed to subscribe to {procedure}: {reason}")]
    SubscribeFailed { procedure: String, reason: String },

    /// Failed to unsubscribe
    #[error("Failed to unsubscribe: {0}")]
    UnsubscribeFailed(String),

    /// Invalid delivery configuration
    #[error("Invalid delivery configuration: {0}")]
    InvalidDeliveryConfig(String),

    /// Unknown alert type
    #[error("Unknown alert type: {0}")]
    UnknownAlertType(String),

    /// Missing required field
    #[error("Missing required field: {0}")]
    MissingField(String),

    /// Invalid alert ID
    #[error("Invalid alert ID: {0}")]
    InvalidAlertId(String),

    /// Buffer error
    #[error("Buffer error: {0}")]
    BufferError(String),

    /// HTTP client error
    #[error("HTTP error: {0}")]
    HttpError(String),

    /// Serialization error
    #[error("Serialization error: {0}")]
    Serialization(String),

    /// Generic domain error
    #[error("{0}")]
    Other(String),
}

impl From<std::io::Error> for AlertsError {
    fn from(e: std::io::Error) -> Self {
        AlertsError::Other(e.to_string())
    }
}

impl From<serde_json::Error> for AlertsError {
    fn from(e: serde_json::Error) -> Self {
        AlertsError::Serialization(e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = AlertsError::ChannelSend;
        assert_eq!(err.to_string(), "Channel send error");

        let err = AlertsError::SubscriptionNotFound(42);
        assert_eq!(err.to_string(), "Subscription not found: 42");

        let err = AlertsError::WebhookError("timeout".to_string());
        assert_eq!(err.to_string(), "Webhook error: timeout");
    }

    #[test]
    fn test_from_io_error() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let err: AlertsError = io_err.into();
        assert!(matches!(err, AlertsError::Other(_)));
    }

    #[test]
    fn test_from_json_error() {
        let json_err = serde_json::from_str::<serde_json::Value>("invalid").unwrap_err();
        let err: AlertsError = json_err.into();
        assert!(matches!(err, AlertsError::Serialization(_)));
    }
}
