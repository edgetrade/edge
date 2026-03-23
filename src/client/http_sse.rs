use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::mpsc;

use crate::messages;
use crate::utils::urls::DOCS_BASE_URL;

#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum IrisClientError {
    #[error("HTTP error: {0}")]
    Http(String),

    #[error("Connection error: {0}. See: {DOCS_BASE_URL}/authentication")]
    Connection(String),

    #[error("Authentication failed: {0}. See: {DOCS_BASE_URL}/authentication")]
    Auth(String),

    #[error("Request timeout. See: {DOCS_BASE_URL}/errors")]
    Timeout,

    #[error("Invalid response: {0}. See: {DOCS_BASE_URL}/errors")]
    InvalidResponse(String),

    #[error("RPC error: {0}. See: {DOCS_BASE_URL}/errors")]
    Rpc(String),

    #[error("Not implemented: {0}. See: {DOCS_BASE_URL}/tools/trade#execution")]
    NotImplemented(String),

    #[error("Deserialization error: {0}")]
    Deserialization(String),
}

impl IrisClientError {
    #[allow(dead_code)]
    pub fn docs_url(&self) -> String {
        match self {
            Self::Http(_) | Self::Timeout | Self::InvalidResponse(_) | Self::Rpc(_) | Self::Deserialization(_) => {
                format!("{}/errors", DOCS_BASE_URL)
            }
            Self::Connection(_) | Self::Auth(_) => format!("{}/authentication", DOCS_BASE_URL),
            Self::NotImplemented(_) => format!("{}/tools/trade#execution", DOCS_BASE_URL),
        }
    }
}

#[derive(Serialize)]
struct ApiCallRequest {
    path: String,
    input: Value,
}

#[derive(Deserialize)]
struct ApiCallResponse {
    data: Option<Value>,
    error: Option<ApiError>,
}

#[derive(Deserialize)]
struct ApiError {
    code: String,
    message: String,
}

type SubscriptionSender = mpsc::UnboundedSender<Value>;

#[derive(Clone)]
pub struct IrisClient {
    inner: Arc<IrisClientInner>,
}

struct IrisClientInner {
    base_url: String,
    api_key: String,
    http: reqwest::Client,
    verbose: bool,
    subscriptions: Arc<tokio::sync::Mutex<HashMap<u32, SubscriptionSender>>>,
    next_id: Arc<tokio::sync::Mutex<u32>>,
}

pub struct DispatchParams {
    pub alert_id: u64,
    pub alert_name: String,
    pub delivery: crate::commands::subscribe::alerts::AlertDelivery,
    pub alert_registry: crate::commands::subscribe::alerts::AlertRegistry,
    pub http_client: reqwest::Client,
}

impl IrisClient {
    pub async fn connect(url: &str, api_key: &str, verbose: bool) -> Result<Self, IrisClientError> {
        let base_url = url
            .replace("wss://", "https://")
            .replace("ws://", "http://");

        if verbose {
            messages::error::connection_failed_url_key(&base_url, api_key);
        }

        let http = reqwest::Client::new();

        if verbose {
            messages::success::connection_succeeded();
        }

        Ok(Self {
            inner: Arc::new(IrisClientInner {
                base_url,
                api_key: api_key.to_string(),
                http,
                verbose,
                subscriptions: Arc::new(tokio::sync::Mutex::new(HashMap::new())),
                next_id: Arc::new(tokio::sync::Mutex::new(1)),
            }),
        })
    }

    pub async fn query<T: serde::de::DeserializeOwned + Serialize + Clone>(
        &self,
        path: &str,
        input: Value,
    ) -> Result<T, IrisClientError> {
        if self.inner.verbose {
            messages::success::query_request(path, &input.to_string());
        }
        let result = self.call::<T>(path, input).await?;
        let rez = result.clone();
        if self.inner.verbose {
            messages::success::query_response(path, &serde_json::to_string(&rez).unwrap());
        }
        Ok(result)
    }

    pub async fn mutation<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        input: Value,
    ) -> Result<T, IrisClientError> {
        if self.inner.verbose {
            messages::success::mutation_request(path, &input.to_string());
        }
        let result = self.call::<T>(path, input).await;
        if self.inner.verbose {
            messages::success::mutation_response(path);
        }
        result
    }

    async fn call<T: serde::de::DeserializeOwned>(&self, path: &str, input: Value) -> Result<T, IrisClientError> {
        let url = format!("{}/v1/call", self.inner.base_url);
        let request_body = ApiCallRequest {
            path: path.to_string(),
            input,
        };

        let response = self
            .inner
            .http
            .post(&url)
            .bearer_auth(&self.inner.api_key)
            .json(&request_body)
            .send()
            .await
            .map_err(|e| {
                if e.is_timeout() {
                    IrisClientError::Timeout
                } else if e.status().is_some_and(|s| s.as_u16() == 401) {
                    IrisClientError::Auth("Invalid API key".to_string())
                } else {
                    IrisClientError::Http(format!("Request failed: {}", e))
                }
            })?;

        let status = response.status();
        if status.as_u16() == 401 {
            return Err(IrisClientError::Auth("Invalid API key".to_string()));
        }

        let api_response: ApiCallResponse = response
            .json()
            .await
            .map_err(|e| IrisClientError::InvalidResponse(format!("Failed to parse response: {}", e)))?;

        if let Some(error) = api_response.error {
            let err = match error.code.as_str() {
                "UNAUTHORIZED" => IrisClientError::Auth(error.message),
                "NOT_IMPLEMENTED" => IrisClientError::NotImplemented(error.message),
                _ => IrisClientError::Rpc(error.message),
            };

            if self.inner.verbose {
                messages::error::query_error(path, &err.to_string());
            }

            return Err(err);
        }

        let data = api_response
            .data
            .ok_or_else(|| IrisClientError::InvalidResponse("Missing data in response".to_string()))?;

        let deserialized = serde_json::from_value::<T>(data)
            .map_err(|e| IrisClientError::Deserialization(format!("Failed to deserialize response: {}", e)))?;

        Ok(deserialized)
    }

    pub async fn subscribe(
        &self,
        path: &str,
        input: Value,
    ) -> Result<(u32, mpsc::UnboundedReceiver<Value>), IrisClientError> {
        let mut next_id = self.inner.next_id.lock().await;
        let id = *next_id;
        *next_id += 1;
        drop(next_id);

        if self.inner.verbose {
            messages::success::subscribe_request(path, id, &input.to_string());
        }

        let (tx, rx) = mpsc::unbounded_channel();
        self.inner.subscriptions.lock().await.insert(id, tx.clone());

        let inner = self.inner.clone();
        let path_owned = path.to_string();

        tokio::spawn(async move {
            // 5-minute window: set on first error, reset on any successful connection.
            let mut error_deadline: Option<tokio::time::Instant> = None;
            let mut backoff = std::time::Duration::from_secs(1);

            loop {
                // Honour explicit unsubscribe() calls.
                if !inner.subscriptions.lock().await.contains_key(&id) {
                    break;
                }

                match inner
                    .start_subscription(&path_owned, input.clone(), id, tx.clone())
                    .await
                {
                    Ok(()) => {
                        // tx.is_closed() is true when the receiver (server.rs task) was
                        // dropped intentionally — treat as a clean stop.
                        if tx.is_closed() {
                            break;
                        }
                        // Also exit cleanly if unsubscribe() was called while the stream
                        // was active — no point reconnecting a cancelled subscription.
                        if !inner.subscriptions.lock().await.contains_key(&id) {
                            break;
                        }
                        // Server closed the stream (e.g. reboot/deploy). Reset state and
                        // reconnect after a brief pause.
                        if inner.verbose {
                            messages::success::subscribe_reconnect(&path_owned, id);
                        }
                        error_deadline = None;
                        backoff = std::time::Duration::from_secs(1);
                        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                    }
                    Err(IrisClientError::Auth(e)) => {
                        // Auth errors are permanent — no point retrying.
                        messages::error::auth_error(&path_owned, &id.to_string(), &e.to_string());
                        break;
                    }
                    Err(e) => {
                        if inner.verbose {
                            messages::error::subscription_error(&path_owned, &id.to_string(), &e.to_string());
                        }
                        // Start the 5-minute clock on the first error.
                        let deadline = error_deadline
                            .get_or_insert_with(|| tokio::time::Instant::now() + std::time::Duration::from_secs(300));
                        if tokio::time::Instant::now() >= *deadline {
                            messages::error::reconnect_failed(&path_owned, &id.to_string());
                            break;
                        }
                        tokio::time::sleep(backoff).await;
                        backoff = (backoff * 2).min(std::time::Duration::from_secs(30));
                    }
                }
            }

            // Clean up so callers see the channel close.
            inner.subscriptions.lock().await.remove(&id);
        });

        if self.inner.verbose {
            messages::success::subscribe_registered(path, id);
        }

        Ok((id, rx))
    }

    /// Subscribes to a procedure and spawns a background task that dispatches
    /// each incoming SSE event to the configured alert delivery target.
    ///
    /// The dispatch loop runs until the SSE stream ends or the alert is removed
    /// from `alert_registry` (i.e. `unregister_alert` was called).
    pub async fn subscribe_for_dispatch(
        &self,
        procedure: &str,
        input: Value,
        params: DispatchParams,
    ) -> Result<u32, IrisClientError> {
        let (sub_id, mut rx) = self.subscribe(procedure, input).await?;
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
                let _ = crate::commands::subscribe::alerts::dispatch_event(&delivery, &alert_name, event, &http_client)
                    .await;
            }
        });

        Ok(sub_id)
    }

    pub async fn ping(&self) -> Result<(), IrisClientError> {
        let url = format!("{}/ping", self.inner.base_url);
        let response = self
            .inner
            .http
            .get(&url)
            .send()
            .await
            .map_err(|e| IrisClientError::Http(format!("Ping failed: {}", e)))?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(IrisClientError::Http(format!(
                "Ping returned status: {}",
                response.status()
            )))
        }
    }

    pub async fn unsubscribe(&self, id: u32) -> Result<(), IrisClientError> {
        if self.inner.verbose {
            messages::success::subscription_stop(id);
        }

        self.inner.subscriptions.lock().await.remove(&id);
        Ok(())
    }
}

impl IrisClientInner {
    async fn start_subscription(
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
}

pub async fn new_client(url: String, api_key: String, verbose: bool) -> Result<IrisClient, IrisClientError> {
    IrisClient::connect(&url, &api_key, verbose).await
}
