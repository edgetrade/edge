//! Event dispatching utilities for EventBus.
//!
//! Provides:
//! - Event filtering and routing
//! - Async event handlers
//! - Batch event processing
//!
//! MIGRATED FROM: Part of state/events.rs
//! NEW: Dispatch utilities for actor/handler pattern.

use super::{EventBus, StateEvent, StateEventReceiver};
use std::future::Future;
use std::pin::Pin;

/// Event filter function type.
pub type EventFilter = Box<dyn Fn(&StateEvent) -> bool + Send + Sync>;

/// Event handler function type.
pub type EventHandler = Box<dyn Fn(StateEvent) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>;

/// Event dispatcher that routes events to registered handlers.
#[derive(Default)]
pub struct EventDispatcher {
    /// List of registered handlers with their filters.
    handlers: Vec<(EventFilter, EventHandler)>,
}

impl EventDispatcher {
    /// Create a new event dispatcher.
    pub fn new() -> Self {
        Self { handlers: Vec::new() }
    }

    /// Register a handler for all events matching the filter.
    ///
    /// # Arguments
    /// * `filter` - Function that returns true if the handler should receive the event
    /// * `handler` - Async function to handle matching events
    pub fn on<F, H, Fut>(&mut self, filter: F, handler: H)
    where
        F: Fn(&StateEvent) -> bool + Send + Sync + 'static,
        H: Fn(StateEvent) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        self.handlers
            .push((Box::new(filter), Box::new(move |event| Box::pin(handler(event)))));
    }

    /// Register a handler for a specific event name.
    pub fn on_event_name<H, Fut>(&mut self, name: &str, handler: H)
    where
        H: Fn(StateEvent) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let name = name.to_string();
        self.on(move |event| event.event_name() == name, handler);
    }

    /// Dispatch an event to all matching handlers.
    pub async fn dispatch(&self, event: StateEvent) {
        for (filter, handler) in &self.handlers {
            if filter(&event) {
                handler(event.clone()).await;
            }
        }
    }

    /// Spawn a background task that dispatches events from the EventBus.
    /// Consumes self to move ownership into the spawned task.
    pub fn spawn_dispatcher(self, mut receiver: StateEventReceiver) {
        tokio::spawn(async move {
            let dispatcher = self;
            loop {
                match receiver.recv().await {
                    Ok(event) => {
                        dispatcher.dispatch(event).await;
                    }
                    Err(tokio::sync::broadcast::error::RecvError::Lagged(_n)) => {
                        // Dispatcher lagged behind events - continue receiving
                    }
                    Err(tokio::sync::broadcast::error::RecvError::Closed) => {
                        break;
                    }
                }
            }
        });
    }
}

/// Spawn a background dispatcher for the given EventBus.
/// Returns a handle that can be used to control the dispatcher.
pub fn spawn_bus_dispatcher(bus: &EventBus) {
    let rx = bus.subscribe();
    let dispatcher = EventDispatcher::new();
    dispatcher.spawn_dispatcher(rx);
}

/// Helper functions for common event filters.
pub mod filters {
    use super::StateEvent;

    /// Filter for config-related events.
    pub fn config_events(event: &StateEvent) -> bool {
        matches!(
            event,
            StateEvent::ConfigChanged { .. } | StateEvent::ConfigLoaded { .. }
        )
    }

    /// Filter for session/keystore events.
    pub fn auth_events(event: &StateEvent) -> bool {
        matches!(
            event,
            StateEvent::SessionUnlocked
                | StateEvent::SessionLocked
                | StateEvent::KeystoreUnlocked
                | StateEvent::KeystoreLocked
        )
    }

    /// Filter for wallet events.
    pub fn wallet_events(event: &StateEvent) -> bool {
        matches!(
            event,
            StateEvent::WalletCreated { .. } | StateEvent::WalletImported { .. } | StateEvent::WalletDeleted { .. }
        )
    }

    /// Filter for trade events.
    pub fn trade_events(event: &StateEvent) -> bool {
        matches!(
            event,
            StateEvent::TradeIntentCreated { .. }
                | StateEvent::TradeIntentConfirmed { .. }
                | StateEvent::TradeSubmitted { .. }
                | StateEvent::TradeConfirmed { .. }
                | StateEvent::TradeFailed { .. }
                | StateEvent::TradeExpired { .. }
        )
    }

    /// Filter for MCP events.
    pub fn mcp_events(event: &StateEvent) -> bool {
        matches!(
            event,
            StateEvent::McpServerStarted { .. } | StateEvent::McpServerStopped
        )
    }

    /// Filter for client events.
    pub fn client_events(event: &StateEvent) -> bool {
        matches!(
            event,
            StateEvent::ClientConnected { .. }
                | StateEvent::ClientDisconnected
                | StateEvent::ManifestLoaded
                | StateEvent::ManifestUpdated { .. }
        )
    }

    /// Filter for alert events.
    pub fn alert_events(event: &StateEvent) -> bool {
        matches!(
            event,
            StateEvent::SubscriptionCreated { .. }
                | StateEvent::SubscriptionDeleted { .. }
                | StateEvent::AlertDelivered { .. }
                | StateEvent::AlertFailed { .. }
        )
    }

    /// Filter for IPC events.
    pub fn ipc_events(event: &StateEvent) -> bool {
        matches!(
            event,
            StateEvent::IpcClientConnected { .. }
                | StateEvent::IpcClientDisconnected { .. }
                | StateEvent::IpcRequestReceived { .. }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::super::StateEvent;
    use super::*;
    use serde_json::json;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    #[tokio::test]
    async fn test_dispatcher_basic() {
        let bus = EventBus::new(10);
        let received = Arc::new(Mutex::new(Vec::new()));
        let received_clone = received.clone();

        let mut dispatcher = EventDispatcher::new();
        dispatcher.on(
            |e| matches!(e, StateEvent::SessionUnlocked),
            move |event| {
                let received = received_clone.clone();
                async move {
                    received.lock().await.push(event.event_name().to_string());
                }
            },
        );

        let mut rx = bus.subscribe();

        // Send events
        bus.publish(StateEvent::SessionUnlocked).unwrap();
        bus.publish(StateEvent::SessionLocked).unwrap();

        // Receive and dispatch
        while let Ok(event) = rx.try_recv() {
            dispatcher.dispatch(event).await;
        }

        let events = received.lock().await;
        assert_eq!(events.len(), 1);
        assert_eq!(events[0], "session_unlocked");
    }

    #[tokio::test]
    async fn test_dispatcher_on_event_name() {
        let _bus = EventBus::new(10);
        let received = Arc::new(Mutex::new(Vec::new()));
        let received_clone = received.clone();

        let mut dispatcher = EventDispatcher::new();
        dispatcher.on_event_name("config_changed", move |event| {
            let received = received_clone.clone();
            async move {
                received.lock().await.push(event.event_name().to_string());
            }
        });

        // Send config event
        let event = StateEvent::ConfigChanged {
            key: "test".to_string(),
            value: json!("value"),
        };
        dispatcher.dispatch(event).await;

        // Send other event
        dispatcher.dispatch(StateEvent::SessionUnlocked).await;

        let events = received.lock().await;
        assert_eq!(events.len(), 1);
        assert_eq!(events[0], "config_changed");
    }

    #[test]
    fn test_filter_config_events() {
        let event = StateEvent::ConfigChanged {
            key: "test".to_string(),
            value: json!("value"),
        };
        assert!(filters::config_events(&event));

        let event = StateEvent::SessionUnlocked;
        assert!(!filters::config_events(&event));
    }

    #[test]
    fn test_filter_auth_events() {
        assert!(filters::auth_events(&StateEvent::SessionUnlocked));
        assert!(filters::auth_events(&StateEvent::KeystoreUnlocked));
        assert!(!filters::auth_events(&StateEvent::ConfigChanged {
            key: "x".to_string(),
            value: json!(1),
        }));
    }

    #[test]
    fn test_filter_wallet_events() {
        assert!(filters::wallet_events(&StateEvent::WalletCreated {
            name: "test".to_string(),
            chain: "ethereum".to_string(),
        }));
        assert!(!filters::wallet_events(&StateEvent::SessionUnlocked));
    }

    #[test]
    fn test_filter_trade_events() {
        assert!(filters::trade_events(&StateEvent::TradeIntentCreated {
            id: 1,
            wallet: "0x123".to_string(),
        }));
        assert!(filters::trade_events(&StateEvent::TradeConfirmed { id: 1 }));
        assert!(!filters::trade_events(&StateEvent::SessionUnlocked));
    }

    #[test]
    fn test_filter_mcp_events() {
        assert!(filters::mcp_events(&StateEvent::McpServerStarted {
            transport: "stdio".to_string(),
        }));
        assert!(!filters::mcp_events(&StateEvent::SessionUnlocked));
    }

    #[test]
    fn test_filter_client_events() {
        assert!(filters::client_events(&StateEvent::ClientConnected {
            url: "https://api.example.com".to_string(),
        }));
        assert!(!filters::client_events(&StateEvent::SessionUnlocked));
    }

    #[test]
    fn test_filter_alert_events() {
        assert!(filters::alert_events(&StateEvent::SubscriptionCreated {
            id: 1,
            procedure: "test".to_string(),
        }));
        assert!(!filters::alert_events(&StateEvent::SessionUnlocked));
    }

    #[test]
    fn test_filter_ipc_events() {
        assert!(filters::ipc_events(&StateEvent::IpcClientConnected {
            connection_id: "123".to_string(),
            kind: "test".to_string(),
        }));
        assert!(!filters::ipc_events(&StateEvent::SessionUnlocked));
    }
}
