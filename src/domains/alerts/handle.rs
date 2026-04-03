//! Alerts handle - public API for the alerts domain
//!
//! This is the thin gateway that provides a public interface to the alerts domain.
//! All operations are sent as messages to the AlertsActor using PoseidonRequest pattern.

use tokio::sync::mpsc;

use crate::domains::alerts::actor::{AlertsActor, DeliveryConfig};
use crate::domains::alerts::errors::AlertsError;
use crate::domains::alerts::messages::{AlertsMessage, AlertsRequest, AlertsResponse};
use crate::domains::client::ClientHandle;
use crate::event_bus::EventBus;

/// Public handle for the alerts domain
#[derive(Clone, Debug)]
pub struct AlertsHandle {
    sender: mpsc::Sender<AlertsRequest>,
}

impl AlertsHandle {
    /// Create a new alerts handle with client dependency, receiver, and EventBus.
    ///
    /// Used by the orchestrator to wire domains together.
    pub async fn new(
        client: &ClientHandle,
        receiver: mpsc::Receiver<AlertsRequest>,
        event_bus: EventBus,
    ) -> Result<Self, AlertsError> {
        // Create a client handle from the existing client
        // ClientHandle::from_sender preserves the sender for later use
        let client_handle = ClientHandle::from_sender(client.sender().clone());

        // Spawn the actor with the provided receiver
        let sender = AlertsActor::spawn_with_receiver(client_handle, receiver, event_bus).await;

        Ok(Self { sender })
    }

    /// Create an AlertsHandle from an existing sender.
    pub fn from_sender(sender: mpsc::Sender<AlertsRequest>) -> Self {
        Self { sender }
    }

    /// Send a request to the actor and wait for response
    async fn send_request(&self, payload: AlertsMessage) -> Result<AlertsResponse, AlertsError> {
        let (reply_to, rx) = tokio::sync::oneshot::channel();

        let request = crate::event_bus::PoseidonRequest {
            payload,
            trace_ctx: crate::event_bus::TraceContext::current(),
            reply_to,
        };

        self.sender
            .send(request)
            .await
            .map_err(|_| AlertsError::ChannelSend)?;

        rx.await.map_err(|_| AlertsError::ChannelRecv)?
    }

    /// Subscribe to a procedure
    pub async fn subscribe(&self, procedure: String, input: serde_json::Value) -> Result<AlertsResponse, AlertsError> {
        self.send_request(AlertsMessage::Subscribe { procedure, input })
            .await
    }

    /// Unsubscribe from a procedure
    pub async fn unsubscribe(&self, id: u32) -> Result<AlertsResponse, AlertsError> {
        self.send_request(AlertsMessage::Unsubscribe { id }).await
    }

    /// Register a delivery configuration
    pub async fn register_delivery(&self, config: DeliveryConfig) -> Result<AlertsResponse, AlertsError> {
        self.send_request(AlertsMessage::RegisterDelivery { config })
            .await
    }

    /// Poll events from a subscription buffer
    pub async fn poll_events(&self, subscription_id: u32, limit: usize) -> Result<AlertsResponse, AlertsError> {
        self.send_request(AlertsMessage::PollEvents { subscription_id, limit })
            .await
    }

    /// Register a new alert
    pub async fn register_alert(
        &self,
        alert_name: String,
        procedure: String,
        delivery: DeliveryConfig,
        input: serde_json::Value,
    ) -> Result<AlertsResponse, AlertsError> {
        self.send_request(AlertsMessage::RegisterAlert {
            alert_name,
            procedure,
            delivery,
            input,
        })
        .await
    }

    /// Unregister an alert by ID
    pub async fn unregister_alert(&self, alert_id: u64) -> Result<AlertsResponse, AlertsError> {
        self.send_request(AlertsMessage::UnregisterAlert { alert_id })
            .await
    }

    /// List all registered alerts
    pub async fn list_alerts(&self) -> Result<AlertsResponse, AlertsError> {
        self.send_request(AlertsMessage::ListAlerts).await
    }

    /// Dispatch an event to a delivery target
    pub async fn dispatch_event(
        &self,
        alert_id: u64,
        alert_name: String,
        delivery: DeliveryConfig,
        event: serde_json::Value,
    ) -> Result<AlertsResponse, AlertsError> {
        self.send_request(AlertsMessage::DispatchEvent {
            alert_id,
            alert_name,
            delivery,
            event,
        })
        .await
    }

    /// Shutdown the alerts domain
    pub async fn shutdown(&self) -> Result<AlertsResponse, AlertsError> {
        self.send_request(AlertsMessage::Shutdown).await
    }

    /// Get the sender channel for direct message sending
    pub fn sender(&self) -> &mpsc::Sender<AlertsRequest> {
        &self.sender
    }
}

impl Default for AlertsHandle {
    fn default() -> Self {
        // Create a dummy sender - used for testing or when handle is needed before spawn
        let (sender, _rx) = mpsc::channel(1);
        Self { sender }
    }
}

/// Generate next alert ID using atomic counter
pub fn next_alert_id() -> u64 {
    use std::sync::atomic::{AtomicU64, Ordering};
    static NEXT_ALERT_ID: AtomicU64 = AtomicU64::new(1);
    NEXT_ALERT_ID.fetch_add(1, Ordering::Relaxed)
}

/// Get delivery summary string for a delivery config
pub fn delivery_summary(delivery: &DeliveryConfig) -> String {
    delivery.summary()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_alert_id() {
        let id1 = next_alert_id();
        let id2 = next_alert_id();
        assert_ne!(id1, id2);
        assert!(id2 > id1);
    }

    #[test]
    fn test_delivery_summary() {
        let webhook = DeliveryConfig::Webhook {
            url: "https://example.com/webhook".to_string(),
            secret: None,
        };
        assert_eq!(delivery_summary(&webhook), "webhook: https://example.com/webhook");

        let redis = DeliveryConfig::Redis {
            url: "redis://localhost".to_string(),
            channel: "alerts".to_string(),
        };
        assert_eq!(delivery_summary(&redis), "redis: alerts");

        let telegram = DeliveryConfig::Telegram {
            bot_token: "token".to_string(),
            chat_id: "123".to_string(),
        };
        assert_eq!(delivery_summary(&telegram), "telegram");
    }
}
