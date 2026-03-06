use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::mpsc;

use crate::types::urls::DOCS_BASE_URL;

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
}

impl IrisClientError {
    pub fn docs_url(&self) -> String {
        match self {
            Self::Http(_) | Self::Timeout | Self::InvalidResponse(_) | Self::Rpc(_) => {
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

impl IrisClient {
    pub async fn connect(url: &str, api_key: &str, verbose: bool) -> Result<Self, IrisClientError> {
        let base_url = url
            .replace("wss://", "https://")
            .replace("ws://", "http://");

        if verbose {
            eprintln!("[edge] connecting to {}", base_url);
            eprintln!(
                "[edge] api key: {}...{}",
                &api_key[..4.min(api_key.len())],
                &api_key[api_key.len().saturating_sub(4)..]
            );
        }

        let http = reqwest::Client::new();

        if verbose {
            eprintln!("[edge] connected");
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

    pub async fn query(&self, path: &str, input: Value) -> Result<Value, IrisClientError> {
        self.call(path, input).await
    }

    pub async fn mutation(&self, path: &str, input: Value) -> Result<Value, IrisClientError> {
        self.call(path, input).await
    }

    async fn call(&self, path: &str, input: Value) -> Result<Value, IrisClientError> {
        if self.inner.verbose {
            eprintln!("[edge] → {} (query/mutation): {}", path, input);
        }

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
                eprintln!("[edge] ✗ {} (query/mutation): {}", path, err);
            }

            return Err(err);
        }

        let data = api_response
            .data
            .ok_or_else(|| IrisClientError::InvalidResponse("Missing data in response".to_string()))?;

        if self.inner.verbose {
            eprintln!("[edge] ← {} (query/mutation): {}", path, data);
        }

        Ok(data)
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
            eprintln!("[edge] → subscribe {} (id={}): {}", path, id, input);
        }

        let (tx, rx) = mpsc::unbounded_channel();
        self.inner.subscriptions.lock().await.insert(id, tx.clone());

        let inner = self.inner.clone();
        let path_clone = path.to_string();

        tokio::spawn(async move {
            if let Err(e) = inner
                .start_subscription(&path_clone, input.clone(), id, tx)
                .await
                && inner.verbose
            {
                eprintln!("[edge] ✗ subscribe {} (id={}): {}", path_clone, id, e);
            }
        });

        if self.inner.verbose {
            eprintln!("[edge] ← subscribe {} (id={}) registered", path, id);
        }

        Ok((id, rx))
    }

    pub async fn unsubscribe(&self, id: u32) -> Result<(), IrisClientError> {
        if self.inner.verbose {
            eprintln!("[edge] → subscription.stop (id={})", id);
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
        let encoded_input = base64_url_encode(&input_json);

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
                            eprintln!("[edge] Failed to parse SSE event data: {}", data_str);
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

fn base64_url_encode(input: &str) -> String {
    use std::fmt::Write;

    let bytes = input.as_bytes();
    let mut result = String::new();

    for chunk in bytes.chunks(3) {
        let mut buf = [0u8; 3];
        for (i, &b) in chunk.iter().enumerate() {
            buf[i] = b;
        }

        let b1 = (buf[0] >> 2) & 0x3F;
        let b2 = ((buf[0] & 0x03) << 4) | ((buf[1] >> 4) & 0x0F);
        let b3 = ((buf[1] & 0x0F) << 2) | ((buf[2] >> 6) & 0x03);
        let b4 = buf[2] & 0x3F;

        write!(&mut result, "{}", encode_char(b1)).unwrap();
        write!(&mut result, "{}", encode_char(b2)).unwrap();

        if chunk.len() > 1 {
            write!(&mut result, "{}", encode_char(b3)).unwrap();
        }

        if chunk.len() > 2 {
            write!(&mut result, "{}", encode_char(b4)).unwrap();
        }
    }

    result
}

fn encode_char(b: u8) -> char {
    match b {
        0..=25 => (b'A' + b) as char,
        26..=51 => (b'a' + (b - 26)) as char,
        52..=61 => (b'0' + (b - 52)) as char,
        62 => '-',
        63 => '_',
        _ => unreachable!(),
    }
}
