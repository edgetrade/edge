//! Webhook delivery implementation
//!
//! MIGRATED FROM: pkg/poseidon/src/commands/subscribe/webhook.rs
//! Original implementation preserved with adaptations for actor pattern

use hmac::{Hmac, Mac};
use reqwest::Client;
use serde_json::Value;
use sha2::Sha256;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

type HmacSha256 = Hmac<Sha256>;
type Registrations = Arc<Mutex<HashMap<String, (String, Option<String>)>>>;

/// Dispatches buffered subscription events to a registered HTTP webhook.
/// Knows nothing about the shape of the events — those are defined by iris.
#[derive(Clone)]
pub struct WebhookDispatcher {
    client: Client,
    registrations: Registrations,
}

impl WebhookDispatcher {
    /// Create a new webhook dispatcher with default timeout
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .timeout(Duration::from_secs(10))
                .build()
                .expect("Failed to build HTTP client"),
            registrations: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Register a webhook URL for a topic/procedure
    pub async fn register(&self, topic: &str, url: &str, secret: Option<&str>) {
        let mut regs = self.registrations.lock().await;
        regs.insert(topic.to_string(), (url.to_string(), secret.map(|s| s.to_string())));
    }

    /// Unregister a webhook for a topic
    pub async fn unregister(&self, topic: &str) {
        self.registrations.lock().await.remove(topic);
    }

    /// Get webhook registration for a topic
    pub async fn get_webhook(&self, topic: &str) -> Option<(String, Option<String>)> {
        self.registrations.lock().await.get(topic).cloned()
    }

    /// Dispatch an event to a webhook URL with optional HMAC signature
    pub async fn dispatch(&self, url: &str, secret: Option<&str>, payload: Value) -> Result<(), String> {
        let body = serde_json::to_string(&payload).map_err(|e| e.to_string())?;

        let mut request = self
            .client
            .post(url)
            .header("Content-Type", "application/json");

        if let Some(secret) = secret {
            let signature = sign_payload(&body, secret);
            request = request.header("X-Edge-Signature", format!("sha256={}", signature));
        }

        // Retry with exponential backoff
        for attempt in 0..3u32 {
            match request.try_clone().unwrap().body(body.clone()).send().await {
                Ok(response) if response.status().is_success() => return Ok(()),
                Ok(response) => {
                    if attempt == 2 {
                        return Err(format!("HTTP {}", response.status()));
                    }
                }
                Err(e) => {
                    if attempt == 2 {
                        return Err(e.to_string());
                    }
                }
            }
            tokio::time::sleep(Duration::from_secs(1 << attempt)).await;
        }

        Err("Max retries exceeded".to_string())
    }

    /// Dispatch an alert event with structured payload
    pub async fn dispatch_alert(
        &self,
        url: &str,
        secret: Option<&str>,
        alert_name: &str,
        event: Value,
    ) -> Result<(), String> {
        let ts = chrono::Utc::now().to_rfc3339();
        let payload = serde_json::json!({ "alert_name": alert_name, "event": event, "ts": ts });
        self.dispatch(url, secret, payload).await
    }
}

impl Default for WebhookDispatcher {
    fn default() -> Self {
        Self::new()
    }
}

/// Sign a payload with HMAC-SHA256
fn sign_payload(body: &str, secret: &str) -> String {
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).expect("HMAC can take key of any size");
    mac.update(body.as_bytes());
    hex::encode(mac.finalize().into_bytes())
}

/// Hex encoding utilities
mod hex {
    /// Encode bytes as hex string
    pub fn encode(bytes: impl AsRef<[u8]>) -> String {
        bytes
            .as_ref()
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect()
    }
}
