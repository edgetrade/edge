//! Alerts actor - state owner for the alerts domain
//!
//! This actor manages:
//! - Alert registrations with delivery configurations
//! - Active subscriptions and their event buffers
//! - Event dispatch to webhook, Redis, and Telegram targets

use std::collections::HashMap;

use serde_json::Value;
use tokio::sync::mpsc::{self};

use crate::domains::alerts::buffer::SubscriptionManager;
use crate::domains::alerts::errors::AlertsError;
use crate::domains::alerts::messages::{AlertRegistration, AlertsMessage, AlertsRequest, AlertsResponse};

use crate::domains::client::ClientHandle;
use crate::event_bus::{EventBus, StateEvent};

/// Internal state for the AlertsActor
pub struct AlertsState {
    /// Active subscriptions by subscription ID
    pub subscriptions: HashMap<u32, SubscriptionInfo>,
    /// Alert delivery configurations by delivery ID
    pub delivery: HashMap<u64, DeliveryConfig>,
    /// Next subscription ID counter
    next_subscription_id: u32,
    /// Next delivery ID counter
    next_delivery_id: u64,
}

impl AlertsState {
    /// Create new empty alerts state
    pub fn new() -> Self {
        Self {
            subscriptions: HashMap::new(),
            delivery: HashMap::new(),
            next_subscription_id: 1,
            next_delivery_id: 1,
        }
    }

    /// Generate next subscription ID
    pub fn next_subscription_id(&mut self) -> u32 {
        let id = self.next_subscription_id;
        self.next_subscription_id += 1;
        id
    }

    /// Generate next delivery ID
    pub fn next_delivery_id(&mut self) -> u64 {
        let id = self.next_delivery_id;
        self.next_delivery_id += 1;
        id
    }
}

impl Default for AlertsState {
    fn default() -> Self {
        Self::new()
    }
}

/// Subscription information for active subscriptions
#[derive(Debug)]
pub struct SubscriptionInfo {
    /// Subscription ID
    pub id: u32,
    /// Procedure being subscribed to
    pub procedure: String,
    /// Event receiver channel
    pub event_receiver: tokio::sync::mpsc::UnboundedReceiver<Value>,
}

/// Delivery configuration for alert destinations
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum DeliveryConfig {
    /// Webhook delivery with optional HMAC signature
    Webhook {
        /// Target webhook URL
        url: String,
        /// Secret for HMAC-SHA256 signature
        secret: Option<String>,
    },
    /// Redis stream delivery
    Redis {
        /// Redis connection URL
        url: String,
        /// Redis stream channel name
        channel: String,
    },
    /// Telegram bot delivery
    Telegram {
        /// Bot token for Telegram API
        bot_token: String,
        /// Target chat ID
        chat_id: String,
    },
}

impl DeliveryConfig {
    /// Get a summary string for the delivery config
    pub fn summary(&self) -> String {
        match self {
            DeliveryConfig::Webhook { url, .. } => format!("webhook: {}", url),
            DeliveryConfig::Redis { channel, .. } => format!("redis: {}", channel),
            DeliveryConfig::Telegram { .. } => "telegram".to_string(),
        }
    }
}

/// Alerts actor that owns all alert-related state and business logic
pub struct AlertsActor {
    /// Actor state containing subscriptions and delivery configs
    state: AlertsState,
    /// EventBus for publishing state events
    event_bus: EventBus,
    /// Client handle for subscribing to Iris events
    client: ClientHandle,
    /// Subscription manager for event buffering
    subscription_manager: SubscriptionManager,
    /// HTTP client for deliveries
    http_client: reqwest::Client,
    /// Active alert registrations
    alert_registrations: HashMap<u64, AlertRegistration>,
}

impl AlertsActor {
    /// Create a new alerts actor
    pub fn new(client: ClientHandle, event_bus: EventBus) -> Self {
        Self {
            state: AlertsState::new(),
            event_bus,
            client,
            subscription_manager: SubscriptionManager::new(),
            http_client: reqwest::Client::new(),
            alert_registrations: HashMap::new(),
        }
    }

    /// Spawn the actor with an existing receiver and return a sender
    pub async fn spawn_with_receiver(
        client: ClientHandle,
        receiver: mpsc::Receiver<AlertsRequest>,
        event_bus: EventBus,
    ) -> mpsc::Sender<AlertsRequest> {
        let (tx, _rx) = mpsc::channel::<AlertsRequest>(64);
        let actor = Self::new(client, event_bus);

        tokio::spawn(async move {
            actor.run(receiver).await;
        });

        tx
    }

    /// Run the actor message loop
    pub async fn run(mut self, mut receiver: mpsc::Receiver<AlertsRequest>) {
        while let Some(req) = receiver.recv().await {
            let reply = match req.payload {
                AlertsMessage::Subscribe { procedure, input } => self.subscribe(procedure, input).await,
                AlertsMessage::Unsubscribe { id } => self.unsubscribe(id).await,
                AlertsMessage::RegisterDelivery { config } => self.register_delivery(config).await,
                AlertsMessage::PollEvents { subscription_id, limit } => self.poll_events(subscription_id, limit).await,
                AlertsMessage::RegisterAlert {
                    alert_name,
                    procedure,
                    delivery,
                    input,
                } => {
                    self.register_alert(alert_name, procedure, delivery, input)
                        .await
                }
                AlertsMessage::UnregisterAlert { alert_id } => self.unregister_alert(alert_id).await,
                AlertsMessage::ListAlerts => self.list_alerts().await,
                AlertsMessage::DispatchEvent {
                    alert_id,
                    alert_name,
                    delivery,
                    event,
                } => {
                    self.dispatch_event(alert_id, alert_name, delivery, event)
                        .await
                }
                AlertsMessage::Shutdown => self.shutdown().await,
            };
            let _ = req.reply_to.send(reply);
        }
    }

    /// Subscribe to a procedure
    async fn subscribe(&mut self, procedure: String, input: Value) -> Result<AlertsResponse, AlertsError> {
        let (sub_id, receiver) = self
            .client
            .subscribe(&procedure, input)
            .await
            .map_err(|e| AlertsError::SubscribeFailed {
                procedure: procedure.clone(),
                reason: e.to_string(),
            })?;

        let sub_info = SubscriptionInfo {
            id: sub_id,
            procedure: procedure.clone(),
            event_receiver: receiver,
        };
        self.state.subscriptions.insert(sub_id, sub_info);

        self.subscription_manager
            .create_subscription(sub_id.to_string())
            .await;

        self.emit_state_event(StateEvent::SubscriptionCreated {
            id: sub_id,
            procedure: procedure.clone(),
        });

        Ok(AlertsResponse::Subscribed {
            id: sub_id,
            message: format!("Subscribed to {}", procedure),
        })
    }

    /// Unsubscribe from a procedure
    async fn unsubscribe(&mut self, id: u32) -> Result<AlertsResponse, AlertsError> {
        self.client
            .unsubscribe(id)
            .await
            .map_err(|e| AlertsError::UnsubscribeFailed(e.to_string()))?;

        if let Some(sub) = self.state.subscriptions.remove(&id) {
            self.subscription_manager
                .remove_subscription(&id.to_string())
                .await;

            self.emit_state_event(StateEvent::SubscriptionDeleted { id });

            Ok(AlertsResponse::Unsubscribed {
                procedure: sub.procedure,
            })
        } else {
            Err(AlertsError::SubscriptionNotFound(id))
        }
    }

    /// Register a delivery configuration
    async fn register_delivery(&mut self, config: DeliveryConfig) -> Result<AlertsResponse, AlertsError> {
        let delivery_id = self.state.next_delivery_id();
        let summary = config.summary();

        self.state.delivery.insert(delivery_id, config);

        Ok(AlertsResponse::DeliveryRegistered {
            id: delivery_id,
            summary,
        })
    }

    /// Poll events from a subscription buffer
    async fn poll_events(&mut self, subscription_id: u32, limit: usize) -> Result<AlertsResponse, AlertsError> {
        let events = self
            .subscription_manager
            .poll_events(&subscription_id.to_string(), limit)
            .await;

        let count = events.len();
        Ok(AlertsResponse::EventsPolled { events, count })
    }

    /// Register a new alert
    async fn register_alert(
        &mut self,
        alert_name: String,
        _procedure: String,
        delivery: DeliveryConfig,
        _input: Value,
    ) -> Result<AlertsResponse, AlertsError> {
        let alert_id = self.state.next_delivery_id();
        let sub_id = self.state.next_subscription_id();

        self.emit_state_event(StateEvent::SubscriptionCreated {
            id: sub_id,
            procedure: _procedure.clone(),
        });

        self.alert_registrations.insert(
            alert_id,
            AlertRegistration {
                alert_name,
                subscription_id: sub_id,
                delivery,
            },
        );

        Ok(AlertsResponse::AlertRegistered {
            alert_id,
            subscription_id: sub_id,
        })
    }

    /// Unregister an alert by ID
    async fn unregister_alert(&mut self, alert_id: u64) -> Result<AlertsResponse, AlertsError> {
        if let Some(reg) = self.alert_registrations.remove(&alert_id) {
            self.emit_state_event(StateEvent::SubscriptionDeleted {
                id: reg.subscription_id,
            });
            Ok(AlertsResponse::AlertUnregistered {
                alert_name: reg.alert_name,
            })
        } else {
            Err(AlertsError::AlertNotFound(alert_id))
        }
    }

    /// List all registered alerts
    async fn list_alerts(&self) -> Result<AlertsResponse, AlertsError> {
        let alerts: Vec<Value> = self
            .alert_registrations
            .iter()
            .map(|(id, reg)| {
                serde_json::json!({
                    "id": id,
                    "name": reg.alert_name,
                    "subscription_id": reg.subscription_id,
                    "delivery": reg.delivery.summary()
                })
            })
            .collect();

        Ok(AlertsResponse::AlertList(alerts))
    }

    /// Dispatch an event to a delivery target
    async fn dispatch_event(
        &mut self,
        alert_id: u64,
        alert_name: String,
        delivery: DeliveryConfig,
        event: Value,
    ) -> Result<AlertsResponse, AlertsError> {
        let result = dispatch_event(&delivery, &alert_name, event, &self.http_client).await;

        match result {
            Ok(_) => {
                let target = match &delivery {
                    DeliveryConfig::Webhook { url, .. } => url.clone(),
                    DeliveryConfig::Redis { channel, .. } => channel.clone(),
                    DeliveryConfig::Telegram { chat_id, .. } => format!("telegram:{}", chat_id),
                };
                self.emit_state_event(StateEvent::AlertDelivered { alert_id, target });
                Ok(AlertsResponse::EventDispatched { alert_id })
            }
            Err(e) => {
                self.emit_state_event(StateEvent::AlertFailed {
                    alert_id,
                    error: e.clone(),
                });
                Err(AlertsError::DeliveryFailed(e))
            }
        }
    }

    /// Shutdown the alerts domain
    async fn shutdown(&mut self) -> Result<AlertsResponse, AlertsError> {
        let ids: Vec<u32> = self.state.subscriptions.keys().copied().collect();
        for id in ids {
            let _ = self.client.unsubscribe(id).await;
            self.subscription_manager
                .remove_subscription(&id.to_string())
                .await;
        }
        self.state.subscriptions.clear();
        self.alert_registrations.clear();

        Ok(AlertsResponse::Success)
    }

    /// Emit a StateEvent to the EventBus
    fn emit_state_event(&self, event: StateEvent) {
        if let Err(_e) = self.event_bus.publish(event) {
            // EventBus publish error is non-critical
        }
    }
}

/// Dispatch an event to the configured delivery target
pub async fn dispatch_event(
    delivery: &DeliveryConfig,
    alert_name: &str,
    event: Value,
    http_client: &reqwest::Client,
) -> Result<(), String> {
    match delivery {
        DeliveryConfig::Webhook { url, secret } => {
            dispatch_webhook(url, secret.as_deref(), alert_name, event, http_client).await
        }
        DeliveryConfig::Redis { url, channel } => dispatch_redis(url, channel, alert_name, event).await,
        DeliveryConfig::Telegram { bot_token, chat_id } => {
            dispatch_telegram(bot_token, chat_id, alert_name, event, http_client).await
        }
    }
}

/// Dispatch to webhook with optional HMAC signature
async fn dispatch_webhook(
    url: &str,
    secret: Option<&str>,
    alert_name: &str,
    event: Value,
    client: &reqwest::Client,
) -> Result<(), String> {
    use hmac::{Hmac, Mac};
    use sha2::Sha256;
    type HmacSha256 = Hmac<Sha256>;

    let ts = chrono::Utc::now().to_rfc3339();
    let payload = serde_json::json!({ "alert_name": alert_name, "event": event, "ts": ts });
    let body = serde_json::to_string(&payload).map_err(|e| e.to_string())?;

    let mut request = client.post(url).header("Content-Type", "application/json");

    if let Some(secret) = secret {
        let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).map_err(|e| e.to_string())?;
        mac.update(body.as_bytes());
        let sig: String = mac
            .finalize()
            .into_bytes()
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect();
        request = request.header("X-Edge-Signature", format!("sha256={}", sig));
    }

    for attempt in 0..3u32 {
        match request.try_clone().unwrap().body(body.clone()).send().await {
            Ok(r) if r.status().is_success() => return Ok(()),
            Ok(r) if attempt == 2 => return Err(format!("HTTP {}", r.status())),
            Err(e) if attempt == 2 => return Err(e.to_string()),
            _ => {}
        }
        tokio::time::sleep(std::time::Duration::from_secs(1 << attempt)).await;
    }

    Err("Max retries exceeded".to_string())
}

/// Dispatch to Redis stream
async fn dispatch_redis(url: &str, channel: &str, alert_name: &str, event: Value) -> Result<(), String> {
    let client = redis::Client::open(url).map_err(|e| e.to_string())?;
    let mut conn = client
        .get_multiplexed_async_connection()
        .await
        .map_err(|e| e.to_string())?;
    let ts = chrono::Utc::now().to_rfc3339();
    let event_json = serde_json::to_string(&event).map_err(|e| e.to_string())?;
    let _: String = redis::cmd("XADD")
        .arg(channel)
        .arg("*")
        .arg("alert_name")
        .arg(alert_name)
        .arg("event")
        .arg(&event_json)
        .arg("ts")
        .arg(&ts)
        .query_async(&mut conn)
        .await
        .map_err(|e: redis::RedisError| e.to_string())?;
    Ok(())
}

/// Dispatch to Telegram
async fn dispatch_telegram(
    bot_token: &str,
    chat_id: &str,
    alert_name: &str,
    event: Value,
    client: &reqwest::Client,
) -> Result<(), String> {
    let ts = chrono::Utc::now().to_rfc3339();
    let event_json = serde_json::to_string_pretty(&event).unwrap_or_default();
    let text = format!(
        "<b>Alert: {}</b>\n<pre>{}</pre>\n<i>{}</i>",
        html_escape::encode_text(alert_name),
        html_escape::encode_text(&event_json),
        html_escape::encode_text(&ts),
    );
    let url = format!("https://api.telegram.org/bot{}/sendMessage", bot_token);
    let payload = serde_json::json!({ "chat_id": chat_id, "text": text, "parse_mode": "HTML" });
    client
        .post(&url)
        .json(&payload)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delivery_config_summary() {
        let webhook = DeliveryConfig::Webhook {
            url: "https://example.com".to_string(),
            secret: None,
        };
        assert_eq!(webhook.summary(), "webhook: https://example.com");
    }

    #[test]
    fn test_alerts_state_new() {
        let state = AlertsState::new();
        assert!(state.subscriptions.is_empty());
        assert!(state.delivery.is_empty());
        assert_eq!(state.next_subscription_id, 1);
        assert_eq!(state.next_delivery_id, 1);
    }

    #[test]
    fn test_next_ids() {
        let mut state = AlertsState::new();
        assert_eq!(state.next_subscription_id(), 1);
        assert_eq!(state.next_subscription_id(), 2);
        assert_eq!(state.next_delivery_id(), 1);
        assert_eq!(state.next_delivery_id(), 2);
    }
}
