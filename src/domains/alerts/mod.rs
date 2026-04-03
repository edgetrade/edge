//! alerts domain - Real-time subscriptions + delivery (webhook/Redis/Telegram)
//!
//! This domain handles:
//! - Alert registrations with delivery configurations
//! - Real-time subscriptions to Iris API procedures
//! - Event dispatch to webhook, Redis, and Telegram targets
//! - Event buffering for poll-based consumption

// Domain modules
pub mod actor;
pub mod buffer;
pub mod errors;
pub mod handle;
pub mod messages;
pub mod webhook;

// Re-export main types for convenience
// Actor and state types (now combined in actor.rs)
pub use actor::{AlertsActor, AlertsState, DeliveryConfig, SubscriptionInfo, dispatch_event};

// Handle types
pub use handle::{AlertsHandle, delivery_summary, next_alert_id};

// Message types
pub use messages::{AlertRegistration, AlertsMessage, AlertsRequest, AlertsResponse};

// Error types
pub use errors::AlertsError;

// Buffer types
pub use buffer::SubscriptionManager;

// Webhook types
pub use webhook::WebhookDispatcher;

// BACKWARD COMPATIBILITY: Alias DeliveryConfig as AlertDelivery
/// Backward compatibility alias for DeliveryConfig
pub use actor::DeliveryConfig as AlertDelivery;

/// Convenience type alias for alert registry
pub type AlertRegistry = std::sync::Arc<tokio::sync::Mutex<std::collections::HashMap<u64, AlertRegistration>>>;

/// Create a new alert registry
pub fn new_alert_registry() -> AlertRegistry {
    std::sync::Arc::new(tokio::sync::Mutex::new(std::collections::HashMap::new()))
}
