use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::sync::Mutex;

static NEXT_ALERT_ID: AtomicU64 = AtomicU64::new(1);

pub fn next_alert_id() -> u64 {
    NEXT_ALERT_ID.fetch_add(1, Ordering::Relaxed)
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AlertDelivery {
    Webhook { url: String, secret: Option<String> },
    Redis { url: String, channel: String },
    Telegram { bot_token: String, chat_id: String },
}

#[derive(Clone)]
pub struct AlertRegistration {
    pub alert_name: String,
    pub subscription_id: u32,
    pub delivery: AlertDelivery,
}

pub type AlertRegistry = Arc<Mutex<HashMap<u64, AlertRegistration>>>;

pub fn new_alert_registry() -> AlertRegistry {
    Arc::new(Mutex::new(HashMap::new()))
}

/// Dispatches a single SSE event to the configured delivery target.
pub async fn dispatch_event(
    delivery: &AlertDelivery,
    alert_name: &str,
    event: Value,
    http_client: &reqwest::Client,
) -> Result<(), String> {
    match delivery {
        AlertDelivery::Webhook { url, secret } => {
            dispatch_webhook(url, secret.as_deref(), alert_name, event, http_client).await
        }
        AlertDelivery::Redis { url, channel } => dispatch_redis(url, channel, alert_name, event).await,
        AlertDelivery::Telegram { bot_token, chat_id } => {
            dispatch_telegram(bot_token, chat_id, alert_name, event, http_client).await
        }
    }
}

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
        request = request.header("X-Edge-Signature", format!("sha256={sig}"));
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

async fn dispatch_telegram(
    bot_token: &str,
    chat_id: &str,
    alert_name: &str,
    event: Value,
    client: &reqwest::Client,
) -> Result<(), String> {
    let ts = chrono::Utc::now().to_rfc3339();
    let event_json = serde_json::to_string_pretty(&event).unwrap_or_default();
    // HTML mode: escape dynamic content so special chars in event JSON don't break formatting.
    let text = format!(
        "<b>Alert: {}</b>\n<pre>{}</pre>\n<i>{}</i>",
        html_escape::encode_text(alert_name),
        html_escape::encode_text(&event_json),
        html_escape::encode_text(&ts),
    );
    let url = format!("https://api.telegram.org/bot{bot_token}/sendMessage");
    let payload = serde_json::json!({ "chat_id": chat_id, "text": text, "parse_mode": "HTML" });
    client
        .post(&url)
        .json(&payload)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}
