use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::mpsc;

use crate::domains::alerts::{AlertDelivery, AlertRegistry};
use crate::messages;

use super::IrisClientError;

type SubscriptionSender = mpsc::UnboundedSender<Value>;

pub struct DispatchParams {
    pub alert_id: u64,
    pub alert_name: String,
    pub delivery: AlertDelivery,
    pub alert_registry: AlertRegistry,
    pub http_client: reqwest::Client,
}

pub struct IrisClientInner {
    pub base_url: String,
    pub api_key: String,
    pub http: reqwest::Client,
    pub verbose: bool,
    pub subscriptions: Arc<tokio::sync::Mutex<HashMap<u32, SubscriptionSender>>>,
    pub next_id: Arc<tokio::sync::Mutex<u32>>,
}

impl IrisClientInner {
    pub async fn start_subscription(
        &self,
        path: &str,
        input: Value,
        _id: u32,
        tx: SubscriptionSender,
    ) -> Result<(), IrisClientError> {
        let input_json = serde_json::to_string(&input)
            .map_err(|e| IrisClientError::InvalidResponse(format!("Failed to serialize input: {}", e)))?;
        use base64::Engine as _;
        let encoded_input = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(&input_json);
        let url = format!("{}/v1/subscribe/{}?input={}", self.base_url, path, encoded_input);

        let response = self
            .http
            .get(&url)
            .bearer_auth(&self.api_key)
            .header("Accept", "text/event-stream")
            .send()
            .await
            .map_err(|e| IrisClientError::Http(format!("Subscription request failed: {}", e)))?;

        if response.status().as_u16() == 401 {
            return Err(IrisClientError::Auth("Invalid API key".to_string()));
        }

        let mut stream = response.bytes_stream();
        let mut buffer = String::new();
        use futures::stream::StreamExt;

        while let Some(chunk_result) = stream.next().await {
            let chunk = chunk_result.map_err(|e| IrisClientError::Http(format!("Stream error: {}", e)))?;
            buffer.push_str(&String::from_utf8_lossy(&chunk));

            let lines: Vec<&str> = buffer.lines().collect();
            for (i, line) in lines.iter().enumerate() {
                if i == lines.len() - 1 && !buffer.ends_with('\n') {
                    buffer = line.to_string();
                    break;
                }
                if line.starts_with("data: ") {
                    if let Some(data_str) = line.strip_prefix("data: ") {
                        if let Ok(data) = serde_json::from_str::<Value>(data_str) {
                            if tx.send(data).is_err() {
                                return Ok(());
                            }
                        } else if self.verbose {
                            messages::error::sse_parse_error(data_str);
                        }
                    }
                } else if line.starts_with("event: error") {
                    return Err(IrisClientError::Rpc("Server error in subscription".to_string()));
                }
            }
            if buffer.is_empty() || buffer.ends_with('\n') {
                buffer.clear();
            }
        }
        Ok(())
    }

    async fn run_subscription_loop(&self, id: u32, path: &str, input: Value, tx: SubscriptionSender) {
        let mut error_deadline: Option<tokio::time::Instant> = None;
        let mut backoff = std::time::Duration::from_secs(1);

        loop {
            if !self.subscriptions.lock().await.contains_key(&id) {
                break;
            }
            match self
                .start_subscription(path, input.clone(), id, tx.clone())
                .await
            {
                Ok(()) => {
                    if tx.is_closed() || !self.subscriptions.lock().await.contains_key(&id) {
                        break;
                    }
                    if self.verbose {
                        messages::success::subscribe_reconnect(path, id);
                    }
                    error_deadline = None;
                    backoff = std::time::Duration::from_secs(1);
                    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                }
                Err(IrisClientError::Auth(e)) => {
                    messages::error::auth_error(path, &id.to_string(), &e.to_string());
                    break;
                }
                Err(e) => {
                    if self.verbose {
                        messages::error::subscription_error(path, &id.to_string(), &e.to_string());
                    }
                    let deadline = error_deadline
                        .get_or_insert_with(|| tokio::time::Instant::now() + std::time::Duration::from_secs(300));
                    if tokio::time::Instant::now() >= *deadline {
                        messages::error::reconnect_failed(path, &id.to_string());
                        break;
                    }
                    tokio::time::sleep(backoff).await;
                    backoff = (backoff * 2).min(std::time::Duration::from_secs(30));
                }
            }
        }
        self.subscriptions.lock().await.remove(&id);
    }
}

pub async fn subscribe(
    inner: Arc<IrisClientInner>,
    path: &str,
    input: Value,
) -> Result<(u32, mpsc::UnboundedReceiver<Value>), IrisClientError> {
    let id = {
        let mut next_id = inner.next_id.lock().await;
        let id = *next_id;
        *next_id += 1;
        id
    };

    if inner.verbose {
        messages::success::subscribe_request(path, id, &input.to_string());
    }

    let (tx, rx) = mpsc::unbounded_channel();
    inner.subscriptions.lock().await.insert(id, tx.clone());

    let inner_clone = inner.clone();
    let path_owned = path.to_string();
    tokio::spawn(async move {
        inner_clone
            .run_subscription_loop(id, &path_owned, input, tx)
            .await;
    });

    if inner.verbose {
        messages::success::subscribe_registered(path, id);
    }
    Ok((id, rx))
}

pub async fn subscribe_for_dispatch(
    inner: Arc<IrisClientInner>,
    procedure: &str,
    input: Value,
    params: DispatchParams,
) -> Result<u32, IrisClientError> {
    let (sub_id, mut rx) = subscribe(inner.clone(), procedure, input).await?;
    let DispatchParams {
        alert_id,
        alert_name,
        delivery,
        alert_registry,
        http_client,
    } = params;

    tokio::spawn(async move {
        while let Some(event) = rx.recv().await {
            if !alert_registry.lock().await.contains_key(&alert_id) {
                break;
            }
            let _ = crate::domains::alerts::dispatch_event(&delivery, &alert_name, event, &http_client).await;
        }
    });
    Ok(sub_id)
}

pub async fn unsubscribe(inner: Arc<IrisClientInner>, id: u32) -> Result<(), IrisClientError> {
    if inner.verbose {
        messages::success::subscription_stop(id);
    }
    inner.subscriptions.lock().await.remove(&id);
    Ok(())
}
