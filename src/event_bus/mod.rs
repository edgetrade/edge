//! EventBus module - Central communication highway
//!
//! All inter-domain communication flows through the EventBus.
//! Uses tokio::sync::broadcast for state change notifications.
//!
//! MIGRATED FROM:
//! - state/events.rs - StateEvent enum, broadcast channel, ServerFeature, StateEventEmitter trait
//! - error.rs - PoseidonError enum and all variants
//!
//! EXPANDED WITH:
//! - EventBus struct with publish/subscribe methods
//! - PoseidonMessage<T> base type for all messages
//! - PoseidonRequest<T, R, E> base type for request/response patterns
//! - TraceContext for distributed tracing support

use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::PathBuf;
use thiserror::Error;
use tokio::sync::broadcast;

/// Type aliases from state/events.rs - preserved
pub type ConfigKey = String;
pub type ConfigValue = serde_json::Value;

/// Features that can be enabled/disabled in server mode.
/// MIGRATED FROM: state/events.rs - ServerFeature enum (preserved exactly)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ServerFeature {
    /// MCP server functionality (stdio or HTTP transport).
    McpServer,
    /// Real-time subscription handling.
    Subscriptions,
    /// Alert delivery and management.
    Alerts,
}

impl fmt::Display for ServerFeature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServerFeature::McpServer => write!(f, "MCP Server"),
            ServerFeature::Subscriptions => write!(f, "Subscriptions"),
            ServerFeature::Alerts => write!(f, "Alerts"),
        }
    }
}

/// Trace context for distributed tracing across domains.
/// ADDED: New type for telemetry support in the actor/handler pattern.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TraceContext {
    /// Trace ID for distributed tracing
    pub trace_id: String,
    /// Span ID for the current operation
    pub span_id: String,
    /// Parent span ID if applicable
    pub parent_span_id: Option<String>,
    /// Baggage/contextual data
    pub baggage: ConfigValue,
}

impl TraceContext {
    /// Create a new trace context with generated IDs.
    pub fn new() -> Self {
        Self {
            trace_id: format!("{:x}", uuid::Uuid::new_v4().simple()),
            span_id: format!("{:x}", uuid::Uuid::new_v4().simple()),
            parent_span_id: None,
            baggage: serde_json::json!({}),
        }
    }

    /// Get the current trace context (for propagation).
    pub fn current() -> Self {
        Self::default()
    }

    /// Create a child span context.
    pub fn child(&self) -> Self {
        Self {
            trace_id: self.trace_id.clone(),
            span_id: format!("{:x}", uuid::Uuid::new_v4().simple()),
            parent_span_id: Some(self.span_id.clone()),
            baggage: self.baggage.clone(),
        }
    }
}

/// Events emitted when application state changes.
/// MIGRATED FROM: state/events.rs - StateEvent enum
/// EXPANDED: With all domain-specific event variants from the architecture plan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StateEvent {
    // === ORIGINAL VARIANTS (from state/events.rs) - PRESERVED ===
    /// Configuration value changed.
    ConfigChanged {
        /// The configuration key that changed.
        key: ConfigKey,
        /// The new configuration value.
        value: ConfigValue,
    },
    /// Session was unlocked (user authenticated).
    SessionUnlocked,
    /// Session was locked (user logged out).
    SessionLocked,
    /// Server feature configuration changed.
    ServerConfigChanged {
        /// The feature that was toggled.
        feature: ServerFeature,
        /// Whether the feature is now enabled.
        enabled: bool,
    },

    // === CONFIG DOMAIN EVENTS (NEW) ===
    /// Configuration loaded from disk.
    ConfigLoaded { path: PathBuf },

    // === KEYSTORE DOMAIN EVENTS (NEW) ===
    /// Keystore was unlocked.
    KeystoreUnlocked,
    /// Keystore was locked.
    KeystoreLocked,

    // === ENCLAVE DOMAIN EVENTS (NEW) ===
    /// Wallet was created.
    WalletCreated {
        /// Wallet name.
        name: String,
        /// Blockchain chain type.
        chain: String,
    },
    /// Wallet was imported.
    WalletImported { name: String },
    /// Wallet was deleted.
    WalletDeleted { name: String },
    /// Key material was zeroized from memory.
    KeyMaterialZeroized,

    // === CLIENT DOMAIN EVENTS (NEW) ===
    /// Client connected to Iris API.
    ClientConnected { url: String },
    /// Client disconnected from Iris API.
    ClientDisconnected,
    /// MCP manifest loaded.
    ManifestLoaded,
    /// MCP manifest updated.
    ManifestUpdated { version: String },

    // === MCP DOMAIN EVENTS (NEW) ===
    /// MCP server started.
    McpServerStarted { transport: String },
    /// MCP server stopped.
    McpServerStopped,

    // === ALERTS DOMAIN EVENTS (NEW) ===
    /// Subscription created.
    SubscriptionCreated {
        /// Subscription ID.
        id: u32,
        /// Procedure being subscribed to.
        procedure: String,
    },
    /// Subscription deleted.
    SubscriptionDeleted { id: u32 },
    /// Alert was delivered.
    AlertDelivered {
        /// Alert ID.
        alert_id: u64,
        /// Delivery target.
        target: String,
    },
    /// Alert delivery failed.
    AlertFailed {
        /// Alert ID.
        alert_id: u64,
        /// Error message.
        error: String,
    },

    // === TRADES DOMAIN EVENTS (NEW) ===
    /// Trade intent created.
    TradeIntentCreated {
        /// Intent ID.
        id: u64,
        /// Wallet address.
        wallet: String,
    },
    /// Trade intent confirmed.
    TradeIntentConfirmed { id: u64 },
    /// Trade submitted.
    TradeSubmitted {
        /// Intent ID.
        id: u64,
        /// Transaction hash if available.
        tx_hash: Option<String>,
    },
    /// Trade confirmed.
    TradeConfirmed { id: u64 },
    /// Trade failed.
    TradeFailed {
        /// Intent ID.
        id: u64,
        /// Error message.
        error: String,
    },
    /// Trade expired.
    TradeExpired { id: u64 },

    // === IPC DOMAIN EVENTS (NEW) ===
    /// IPC client connected.
    IpcClientConnected {
        /// Connection ID.
        connection_id: String,
        /// Connection kind.
        kind: String,
    },
    /// IPC client disconnected.
    IpcClientDisconnected { connection_id: String },
    /// IPC request received.
    IpcRequestReceived {
        /// Request ID.
        request_id: u64,
        /// Method being called.
        method: String,
    },
}

impl StateEvent {
    /// Returns a short display name for the event type.
    /// MIGRATED FROM: state/events.rs - preserved exactly
    pub fn event_name(&self) -> &'static str {
        match self {
            // Original events
            StateEvent::ConfigChanged { .. } => "config_changed",
            StateEvent::SessionUnlocked => "session_unlocked",
            StateEvent::SessionLocked => "session_locked",
            StateEvent::ServerConfigChanged { .. } => "server_config_changed",
            // Config events
            StateEvent::ConfigLoaded { .. } => "config_loaded",
            // Keystore events
            StateEvent::KeystoreUnlocked => "keystore_unlocked",
            StateEvent::KeystoreLocked => "keystore_locked",
            // Enclave events
            StateEvent::WalletCreated { .. } => "wallet_created",
            StateEvent::WalletImported { .. } => "wallet_imported",
            StateEvent::WalletDeleted { .. } => "wallet_deleted",
            StateEvent::KeyMaterialZeroized => "key_material_zeroized",
            // Client events
            StateEvent::ClientConnected { .. } => "client_connected",
            StateEvent::ClientDisconnected => "client_disconnected",
            StateEvent::ManifestLoaded => "manifest_loaded",
            StateEvent::ManifestUpdated { .. } => "manifest_updated",
            // MCP events
            StateEvent::McpServerStarted { .. } => "mcp_server_started",
            StateEvent::McpServerStopped => "mcp_server_stopped",
            // Alerts events
            StateEvent::SubscriptionCreated { .. } => "subscription_created",
            StateEvent::SubscriptionDeleted { .. } => "subscription_deleted",
            StateEvent::AlertDelivered { .. } => "alert_delivered",
            StateEvent::AlertFailed { .. } => "alert_failed",
            // Trades events
            StateEvent::TradeIntentCreated { .. } => "trade_intent_created",
            StateEvent::TradeIntentConfirmed { .. } => "trade_intent_confirmed",
            StateEvent::TradeSubmitted { .. } => "trade_submitted",
            StateEvent::TradeConfirmed { .. } => "trade_confirmed",
            StateEvent::TradeFailed { .. } => "trade_failed",
            StateEvent::TradeExpired { .. } => "trade_expired",
            // IPC events
            StateEvent::IpcClientConnected { .. } => "ipc_client_connected",
            StateEvent::IpcClientDisconnected { .. } => "ipc_client_disconnected",
            StateEvent::IpcRequestReceived { .. } => "ipc_request_received",
        }
    }
}

/// Base message envelope containing telemetry context.
/// NEW: Base type for all domain messages in the actor/handler pattern.
pub struct PoseidonMessage<T> {
    /// Domain-specific payload.
    pub payload: T,
    /// Telemetry trace context for distributed tracing.
    pub trace_ctx: TraceContext,
}

impl<T> PoseidonMessage<T> {
    /// Create a new message with the given payload.
    pub fn new(payload: T) -> Self {
        Self {
            payload,
            trace_ctx: TraceContext::current(),
        }
    }

    /// Create a new message with explicit trace context.
    pub fn with_trace(payload: T, trace_ctx: TraceContext) -> Self {
        Self { payload, trace_ctx }
    }
}

/// Request envelope with reply channel for request/response pattern.
/// NEW: Base type for domain requests with reply channel.
pub struct PoseidonRequest<T, R, E> {
    /// Domain-specific payload.
    pub payload: T,
    /// Telemetry trace context.
    pub trace_ctx: TraceContext,
    /// Reply channel for response.
    pub reply_to: tokio::sync::oneshot::Sender<std::result::Result<R, E>>,
}

impl<T, R, E> PoseidonRequest<T, R, E> {
    /// Create a new request with the given payload and reply channel.
    pub fn new(payload: T, reply_to: tokio::sync::oneshot::Sender<std::result::Result<R, E>>) -> Self {
        Self {
            payload,
            trace_ctx: TraceContext::current(),
            reply_to,
        }
    }

    /// Create a new request with explicit trace context.
    pub fn with_trace(
        payload: T,
        trace_ctx: TraceContext,
        reply_to: tokio::sync::oneshot::Sender<std::result::Result<R, E>>,
    ) -> Self {
        Self {
            payload,
            trace_ctx,
            reply_to,
        }
    }
}

/// EventBus - Central communication highway using tokio broadcast channel.
/// NEW: Wrapper around broadcast::Sender for StateEvent distribution.
#[derive(Clone)]
pub struct EventBus {
    /// The broadcast sender for state events.
    sender: broadcast::Sender<StateEvent>,
}

impl EventBus {
    /// Create a new EventBus with the specified channel capacity.
    ///
    /// # Arguments
    /// * `capacity` - The buffer size for the channel. When full, oldest
    ///   events are dropped for lagging receivers.
    pub fn new(capacity: usize) -> Self {
        let (sender, _) = broadcast::channel(capacity);
        Self { sender }
    }

    /// Publish an event to all subscribers.
    ///
    /// # Arguments
    /// * `event` - The event to publish
    ///
    /// # Returns
    /// - `Ok(usize)` - The number of receivers that got the event
    /// - `Err(_)` - If all receivers have been dropped
    pub fn publish(&self, event: StateEvent) -> std::result::Result<usize, broadcast::error::SendError<StateEvent>> {
        self.sender.send(event)
    }

    /// Subscribe to events from this EventBus.
    ///
    /// # Returns
    /// A receiver that will receive all future events. Receivers that
    /// lag behind may miss events if the channel buffer fills.
    pub fn subscribe(&self) -> broadcast::Receiver<StateEvent> {
        self.sender.subscribe()
    }

    /// Get the number of active subscribers.
    pub fn subscriber_count(&self) -> usize {
        self.sender.receiver_count()
    }
}

/// Trait for types that can emit state events.
/// MIGRATED FROM: state/events.rs - StateEventEmitter trait (preserved exactly)
pub trait StateEventEmitter {
    /// Emit a state event to all subscribers.
    ///
    /// # Arguments
    /// * `event` - The event to emit
    ///
    /// # Returns
    /// - `Ok(usize)` - The number of receivers that got the event
    /// - `Err(_)` - If all receivers have been dropped
    fn emit_event(&self, event: StateEvent) -> std::result::Result<usize, broadcast::error::SendError<StateEvent>>;
}

impl StateEventEmitter for broadcast::Sender<StateEvent> {
    fn emit_event(&self, event: StateEvent) -> std::result::Result<usize, broadcast::error::SendError<StateEvent>> {
        self.send(event)
    }
}

impl StateEventEmitter for EventBus {
    fn emit_event(&self, event: StateEvent) -> std::result::Result<usize, broadcast::error::SendError<StateEvent>> {
        self.publish(event)
    }
}

/// Type alias for state event sender (backward compatibility).
/// MIGRATED FROM: state/events.rs - StateEventSender
pub type StateEventSender = broadcast::Sender<StateEvent>;

/// Type alias for state event receiver (backward compatibility).
/// MIGRATED FROM: state/events.rs - StateEventReceiver
pub type StateEventReceiver = broadcast::Receiver<StateEvent>;

/// Create a new broadcast channel for state events (backward compatibility).
/// MIGRATED FROM: state/events.rs - create_state_event_channel function
pub fn create_state_event_channel(capacity: usize) -> (StateEventSender, StateEventReceiver) {
    broadcast::channel(capacity)
}

/// Unified error type for Poseidon operations.
/// MIGRATED FROM: error.rs - PoseidonError enum
/// PRESERVES: All existing error variants and From implementations.
#[derive(Debug, Error)]
pub enum PoseidonError {
    /// I/O errors.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Command execution errors.
    #[error("Command error: {0}")]
    Command(String),

    /// Authentication errors.
    #[error("Authentication failed: {0}")]
    Authentication(String),

    /// Crypto/encryption errors.
    #[error("Crypto error: {0}")]
    Crypto(String),

    /// Storage errors (file, keyring, etc.).
    #[error("Storage error: {0}")]
    Storage(String),

    /// Wallet operation errors.
    #[error("Wallet error: {0}")]
    Wallet(String),

    /// Invalid user input.
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    /// Resource not found.
    #[error("Not found: {0}")]
    NotFound(String),

    /// Resource already exists.
    #[error("Already exists: {0}")]
    AlreadyExists(String),

    /// Initialization errors.
    #[error("Initialization error: {0}")]
    Initialization(String),

    /// Lock acquisition errors.
    #[error("Lock error: {0}")]
    LockError(String),

    /// Serialization/deserialization errors.
    #[error("Serialization error: {0}")]
    Serialization(String),

    /// Manifest-related errors.
    #[error("Manifest error: {0}")]
    Manifest(String),

    /// Transport/transport key cache errors.
    #[error("Transport error: {0}")]
    Transport(String),

    /// Generic/other errors.
    #[error("{0}")]
    Other(String),

    /// Client/API communication errors.
    #[error("Client error: {0}")]
    Client(#[from] crate::messages::IrisClientError),

    // === NEW ERROR VARIANTS FOR ACTOR/HANDLER PATTERN ===
    /// Channel send error.
    #[error("Channel send error")]
    ChannelSend,

    /// Channel receive error.
    #[error("Channel receive error")]
    ChannelRecv,

    /// Oneshot reply error.
    #[error("Oneshot reply error")]
    OneshotReply,

    /// Domain error with message.
    #[error("Domain error: {0}")]
    Domain(String),
}

impl PoseidonError {
    /// Returns true if this error is authentication-related.
    /// MIGRATED FROM: error.rs - preserved exactly
    pub fn is_auth_error(&self) -> bool {
        matches!(
            self,
            PoseidonError::Authentication(_) | PoseidonError::Client(crate::messages::IrisClientError::Auth(_))
        )
    }

    /// Returns true if this error indicates a "not found" condition.
    /// MIGRATED FROM: error.rs - preserved exactly
    pub fn is_not_found(&self) -> bool {
        matches!(self, PoseidonError::NotFound(_))
    }

    /// Get a user-friendly error message.
    /// MIGRATED FROM: error.rs - preserved exactly
    pub fn user_message(&self) -> String {
        match self {
            PoseidonError::Authentication(msg) => {
                format!("Authentication failed: {}. Please check your API key.", msg)
            }
            PoseidonError::NotFound(msg) => {
                format!("Not found: {}. Please check the resource exists.", msg)
            }
            PoseidonError::AlreadyExists(msg) => {
                format!(
                    "Already exists: {}. Use a different name or delete the existing resource.",
                    msg
                )
            }
            PoseidonError::InvalidInput(msg) => {
                format!("Invalid input: {}. Please check your input and try again.", msg)
            }
            _ => self.to_string(),
        }
    }
}

impl From<crate::messages::CommandError> for PoseidonError {
    fn from(e: crate::messages::CommandError) -> Self {
        match e {
            crate::messages::CommandError::Authentication(msg) => PoseidonError::Authentication(msg),
            crate::messages::CommandError::Crypto(msg) => PoseidonError::Crypto(msg),
            crate::messages::CommandError::Storage(msg) => PoseidonError::Storage(msg),
            crate::messages::CommandError::Io(msg) => PoseidonError::Io(std::io::Error::other(msg)),
            crate::messages::CommandError::AlreadyExists => PoseidonError::AlreadyExists("Resource".to_string()),
            crate::messages::CommandError::NotFound => PoseidonError::NotFound("Resource".to_string()),
            crate::messages::CommandError::InvalidInput(msg) => PoseidonError::InvalidInput(msg),
            crate::messages::CommandError::Wallet(msg) => PoseidonError::Wallet(msg),
            crate::messages::CommandError::Session(_) => PoseidonError::Storage("Session error".to_string()),
        }
    }
}

impl From<toml::de::Error> for PoseidonError {
    fn from(e: toml::de::Error) -> Self {
        PoseidonError::Serialization(format!("TOML parse error: {}", e))
    }
}

impl From<toml::ser::Error> for PoseidonError {
    fn from(e: toml::ser::Error) -> Self {
        PoseidonError::Serialization(format!("TOML serialization error: {}", e))
    }
}

impl From<serde_json::Error> for PoseidonError {
    fn from(e: serde_json::Error) -> Self {
        PoseidonError::Serialization(format!("JSON error: {}", e))
    }
}

/// Type alias for Results using PoseidonError.
/// MIGRATED FROM: error.rs - Result<T>
pub type Result<T> = std::result::Result<T, PoseidonError>;

// Re-export dispatch module
pub mod dispatch;

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_server_feature_variants() {
        let features = [
            ServerFeature::McpServer,
            ServerFeature::Subscriptions,
            ServerFeature::Alerts,
        ];

        let cloned: Vec<ServerFeature> = features.to_vec();
        assert_eq!(cloned.len(), 3);
    }

    #[test]
    fn test_state_event_variants() {
        let events = [
            StateEvent::ConfigChanged {
                key: "test.key".to_string(),
                value: json!("test_value"),
            },
            StateEvent::SessionUnlocked,
            StateEvent::SessionLocked,
            StateEvent::ServerConfigChanged {
                feature: ServerFeature::McpServer,
                enabled: true,
            },
        ];

        let cloned: Vec<StateEvent> = events.to_vec();
        assert_eq!(cloned.len(), 4);
    }

    #[test]
    fn test_state_event_names() {
        assert_eq!(
            StateEvent::ConfigChanged {
                key: "x".to_string(),
                value: json!(1),
            }
            .event_name(),
            "config_changed"
        );
        assert_eq!(StateEvent::SessionUnlocked.event_name(), "session_unlocked");
        assert_eq!(StateEvent::SessionLocked.event_name(), "session_locked");
        assert_eq!(
            StateEvent::ServerConfigChanged {
                feature: ServerFeature::McpServer,
                enabled: true,
            }
            .event_name(),
            "server_config_changed"
        );
    }

    #[test]
    fn test_server_feature_serialization() {
        let feature = ServerFeature::Subscriptions;
        let json = serde_json::to_string(&feature).unwrap();
        assert_eq!(json, "\"Subscriptions\"");

        let deserialized: ServerFeature = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, ServerFeature::Subscriptions);
    }

    #[test]
    fn test_state_event_serialization() {
        let event = StateEvent::ServerConfigChanged {
            feature: ServerFeature::Alerts,
            enabled: false,
        };
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("Alerts"));
        assert!(json.contains("false"));

        let deserialized: StateEvent = serde_json::from_str(&json).unwrap();
        match deserialized {
            StateEvent::ServerConfigChanged { feature, enabled } => {
                assert_eq!(feature, ServerFeature::Alerts);
                assert!(!enabled);
            }
            _ => panic!("Deserialization produced wrong variant"),
        }
    }

    #[tokio::test]
    async fn test_event_bus_publish_subscribe() {
        let bus = EventBus::new(10);
        let mut rx = bus.subscribe();

        // Send an event
        let event = StateEvent::SessionUnlocked;
        let sent_count = bus.publish(event.clone()).unwrap();
        assert_eq!(sent_count, 1);

        // Receive the event
        let received = rx.recv().await.unwrap();
        assert_eq!(received.event_name(), "session_unlocked");
    }

    #[tokio::test]
    async fn test_event_bus_multiple_subscribers() {
        let bus = EventBus::new(10);
        let mut rx1 = bus.subscribe();
        let mut rx2 = bus.subscribe();

        // Send event
        let event = StateEvent::SessionLocked;
        let sent_count = bus.publish(event.clone()).unwrap();
        assert_eq!(sent_count, 2);

        // Both receivers should get the event
        let received1 = rx1.recv().await.unwrap();
        let received2 = rx2.recv().await.unwrap();
        assert_eq!(received1.event_name(), received2.event_name());
    }

    #[test]
    fn test_state_event_emitter_trait() {
        let bus = EventBus::new(10);

        // Subscribe to the bus so there will be at least one receiver
        let _rx = bus.subscribe();

        let event = StateEvent::ConfigChanged {
            key: "api.url".to_string(),
            value: json!("https://example.com"),
        };

        let result = bus.emit_event(event);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);
    }

    #[test]
    fn test_backward_compat_create_channel() {
        let (tx, _rx) = create_state_event_channel(10);

        let event = StateEvent::ConfigChanged {
            key: "x".to_string(),
            value: json!(1),
        };

        let result = tx.emit_event(event);
        assert!(result.is_ok());
    }

    #[test]
    fn test_error_display() {
        let err = PoseidonError::Authentication("test".to_string());
        assert_eq!(err.to_string(), "Authentication failed: test");

        let err = PoseidonError::NotFound("config".to_string());
        assert_eq!(err.to_string(), "Not found: config");
    }

    #[test]
    fn test_is_auth_error() {
        let err = PoseidonError::Authentication("test".to_string());
        assert!(err.is_auth_error());

        let err = PoseidonError::NotFound("test".to_string());
        assert!(!err.is_auth_error());
    }

    #[test]
    fn test_is_not_found() {
        let err = PoseidonError::NotFound("config".to_string());
        assert!(err.is_not_found());

        let err = PoseidonError::Authentication("test".to_string());
        assert!(!err.is_not_found());
    }

    #[test]
    fn test_user_message() {
        let err = PoseidonError::Authentication("invalid key".to_string());
        assert!(err.user_message().contains("Authentication failed"));

        let err = PoseidonError::NotFound("wallet".to_string());
        assert!(err.user_message().contains("Not found"));
    }

    #[test]
    fn test_poseidon_message_new() {
        let msg = PoseidonMessage::new("test_payload");
        assert_eq!(msg.payload, "test_payload");
    }

    #[test]
    fn test_trace_context_new() {
        let ctx = TraceContext::new();
        assert!(!ctx.trace_id.is_empty());
        assert!(!ctx.span_id.is_empty());
    }

    #[test]
    fn test_trace_context_child() {
        let parent = TraceContext::new();
        let parent_span_id = parent.span_id.clone();
        let parent_trace_id = parent.trace_id.clone();
        let child = parent.child();

        assert_eq!(child.trace_id, parent_trace_id);
        assert_eq!(child.parent_span_id, Some(parent_span_id));
        assert_ne!(child.span_id, parent.span_id);
    }

    // Verify Send + Sync bounds
    fn assert_send_sync<T: Send + Sync>() {}

    #[test]
    fn test_send_sync_bounds() {
        assert_send_sync::<StateEvent>();
        assert_send_sync::<ServerFeature>();
        assert_send_sync::<ConfigKey>();
        assert_send_sync::<ConfigValue>();
        assert_send_sync::<EventBus>();
        assert_send_sync::<TraceContext>();
        assert_send_sync::<PoseidonError>();
    }
}
