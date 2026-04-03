//! alerts domain messages
//!
//! Defines the command/query enums for alerts actor communication.
//! Uses PoseidonRequest pattern for req/resp communication.
//!
//! REFACTORED FOR: PoseidonRequest pattern with trace context
//! PRESERVED: All message variants from original implementation

use crate::domains::alerts::actor::DeliveryConfig;
use crate::domains::alerts::errors::AlertsError;
use crate::event_bus::PoseidonRequest;
use serde_json::Value;

/// Messages sent to the AlertsActor
///
/// PRESERVED FROM: original messages.rs
/// REFACTORED: Uses PoseidonRequest pattern
#[derive(Debug)]
pub enum AlertsMessage {
    /// Subscribe to a procedure
    ///
    /// Creates a new subscription via the client domain
    /// and stores subscription info in the actor state.
    Subscribe {
        /// Procedure being subscribed to (e.g., "price.updates")
        procedure: String,
        /// Subscription input parameters
        input: Value,
    },

    /// Unsubscribe from a procedure
    ///
    /// Removes subscription via client domain and
    /// cleans up subscription info.
    Unsubscribe {
        /// Subscription ID to remove
        id: u32,
    },

    /// Register a delivery configuration
    ///
    /// Adds a new delivery target for alert dispatch.
    RegisterDelivery {
        /// Delivery configuration (webhook, redis, telegram)
        config: DeliveryConfig,
    },

    /// Poll events from a subscription buffer
    ///
    /// Retrieves events from the subscription manager buffer.
    PollEvents {
        /// Subscription ID to poll
        subscription_id: u32,
        /// Maximum number of events to retrieve
        limit: usize,
    },

    /// Register a new alert (legacy)
    ///
    /// PRESERVED: Original register alert functionality
    RegisterAlert {
        /// Alert name (from manifest)
        alert_name: String,
        /// Procedure being subscribed to
        procedure: String,
        /// Delivery configuration
        delivery: DeliveryConfig,
        /// Alert input parameters
        input: Value,
    },

    /// Unregister an alert by ID (legacy)
    ///
    /// PRESERVED: Original unregister functionality
    UnregisterAlert {
        /// Alert ID to remove
        alert_id: u64,
    },

    /// List all registered alerts
    ListAlerts,

    /// Dispatch an event to a delivery target
    ///
    /// Sends an event to the configured delivery destination.
    DispatchEvent {
        /// Alert ID for tracking
        alert_id: u64,
        /// Alert name
        alert_name: String,
        /// Delivery configuration
        delivery: DeliveryConfig,
        /// Event payload
        event: Value,
    },

    /// Shutdown the alerts domain
    Shutdown,
}

/// Response types for alerts operations
///
/// PRESERVED FROM: original messages.rs - AlertsResponse enum
/// Contains all response variants for alerts domain operations.
#[derive(Clone, Debug)]
pub enum AlertsResponse {
    /// Subscription created successfully
    ///
    /// Contains the subscription ID and event receiver channel.
    Subscribed {
        /// Subscription ID
        id: u32,
        /// Human-readable message
        message: String,
    },

    /// Unsubscribed successfully
    Unsubscribed {
        /// Procedure that was unsubscribed
        procedure: String,
    },

    /// Delivery configuration registered
    DeliveryRegistered {
        /// Delivery ID
        id: u64,
        /// Summary of delivery configuration
        summary: String,
    },

    /// Events polled from buffer
    EventsPolled {
        /// Retrieved events
        events: Vec<Value>,
        /// Number of events retrieved
        count: usize,
    },

    /// Alert registered successfully
    AlertRegistered {
        /// Alert ID
        alert_id: u64,
        /// Subscription ID
        subscription_id: u32,
    },

    /// Alert unregistered
    AlertUnregistered {
        /// Alert name that was removed
        alert_name: String,
    },

    /// List of registered alerts
    AlertList(Vec<Value>),

    /// Event dispatched successfully
    EventDispatched {
        /// Alert ID
        alert_id: u64,
    },

    /// Generic success
    Success,

    /// Operation failed with error message
    Error(String),
}

/// Alert registration information
///
/// PRESERVED FROM: original messages.rs - AlertRegistration struct
#[derive(Clone, Debug)]
pub struct AlertRegistration {
    /// Name of the alert (from manifest)
    pub alert_name: String,
    /// Subscription ID from the client
    pub subscription_id: u32,
    /// Delivery configuration
    pub delivery: DeliveryConfig,
}

/// Request type using PoseidonRequest pattern
///
/// AlertsRequest wraps AlertsMessage with trace context and reply channel.
/// This enables request/response communication with telemetry support.
///
/// REFACTORED: Now uses PoseidonRequest from event_bus module
pub type AlertsRequest = PoseidonRequest<AlertsMessage, AlertsResponse, AlertsError>;

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_alerts_message_variants() {
        let msg = AlertsMessage::Subscribe {
            procedure: "price.updates".to_string(),
            input: json!({"symbol": "BTC"}),
        };
        assert!(matches!(msg, AlertsMessage::Subscribe { .. }));

        let msg = AlertsMessage::Unsubscribe { id: 42 };
        assert!(matches!(msg, AlertsMessage::Unsubscribe { id: 42 }));

        let msg = AlertsMessage::PollEvents {
            subscription_id: 1,
            limit: 100,
        };
        assert!(matches!(msg, AlertsMessage::PollEvents { .. }));
    }

    #[test]
    fn test_alerts_response_variants() {
        let resp = AlertsResponse::Subscribed {
            id: 1,
            message: "Subscribed".to_string(),
        };
        assert!(matches!(resp, AlertsResponse::Subscribed { .. }));

        let resp = AlertsResponse::EventsPolled {
            events: vec![json!({"test": true})],
            count: 1,
        };
        assert!(matches!(resp, AlertsResponse::EventsPolled { .. }));
    }
}
