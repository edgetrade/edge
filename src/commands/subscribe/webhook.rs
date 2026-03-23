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
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .timeout(Duration::from_secs(10))
                .build()
                .unwrap(),
            registrations: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn register(&self, topic: &str, url: &str, secret: Option<&str>) {
        let mut regs = self.registrations.lock().await;
        regs.insert(topic.to_string(), (url.to_string(), secret.map(|s| s.to_string())));
    }

    pub async fn unregister(&self, topic: &str) {
        self.registrations.lock().await.remove(topic);
    }

    pub async fn get_webhook(&self, topic: &str) -> Option<(String, Option<String>)> {
        self.registrations.lock().await.get(topic).cloned()
    }

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
}

impl Default for WebhookDispatcher {
    fn default() -> Self {
        Self::new()
    }
}

fn sign_payload(body: &str, secret: &str) -> String {
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).unwrap();
    mac.update(body.as_bytes());
    hex::encode(mac.finalize().into_bytes())
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
