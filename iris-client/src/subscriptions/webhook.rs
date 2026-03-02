use hmac::{Hmac, Mac};
use reqwest::Client;
use serde_json::Value;
use sha2::Sha256;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

use crate::types::events::WebhookRegistration;

type HmacSha256 = Hmac<Sha256>;

#[derive(Clone)]
pub struct WebhookDispatcher {
    client: Client,
    registrations: Arc<Mutex<HashMap<String, WebhookRegistration>>>,
}

impl WebhookDispatcher {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .timeout(Duration::from_secs(10))
                .build()
                .unwrap(),
            registrations: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn register(&self, topic: &str, url: &str, secret: &str, filters: Option<Value>) {
        let mut regs = self.registrations.lock().await;
        regs.insert(
            topic.to_string(),
            WebhookRegistration {
                alert_type: topic.to_string(),
                webhook_url: url.to_string(),
                webhook_secret: Some(secret.to_string()),
                chain_id: None,
                address: None,
                wallet_address: None,
                wallet_addresses: None,
                interval: None,
                threshold: None,
                direction: None,
                filters,
            },
        );
    }

    pub async fn unregister(&self, topic: &str) {
        let mut regs = self.registrations.lock().await;
        regs.remove(topic);
    }

    pub async fn get_webhook(&self, topic: &str) -> Option<(String, String)> {
        let regs = self.registrations.lock().await;
        regs.get(topic)
            .map(|reg| (reg.webhook_url.clone(), reg.webhook_secret.clone().unwrap_or_default()))
    }

    pub async fn dispatch(&self, url: &str, secret: Option<&str>, payload: Value) -> Result<(), String> {
        let body = serde_json::to_string(&payload).map_err(|e| e.to_string())?;

        let mut request = self
            .client
            .post(url)
            .header("Content-Type", "application/json");

        if let Some(secret) = secret {
            let signature = self.sign_payload(&body, secret);
            request = request.header("X-Edge-Signature", format!("sha256={}", signature));
        }

        for attempt in 0..3 {
            match request.try_clone().unwrap().body(body.clone()).send().await {
                Ok(response) if response.status().is_success() => {
                    return Ok(());
                }
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

            let backoff = Duration::from_secs(1 << attempt);
            tokio::time::sleep(backoff).await;
        }

        Err("Max retries exceeded".to_string())
    }

    fn sign_payload(&self, body: &str, secret: &str) -> String {
        let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).unwrap();
        mac.update(body.as_bytes());
        hex::encode(mac.finalize().into_bytes())
    }
}

impl Default for WebhookDispatcher {
    fn default() -> Self {
        Self::new()
    }
}

mod hex {
    pub fn encode(bytes: impl AsRef<[u8]>) -> String {
        bytes
            .as_ref()
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect()
    }
}
