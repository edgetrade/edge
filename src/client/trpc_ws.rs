use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::{Mutex, mpsc, oneshot};
use tokio_tungstenite::tungstenite::Message;

#[derive(Error, Debug)]
pub enum IrisClientError {
    #[error("WebSocket error: {0}")]
    WebSocket(String),

    #[error("Connection error: {0}. See: https://docs.edge.trade/agents/authentication")]
    Connection(String),

    #[error("Authentication failed: {0}. See: https://docs.edge.trade/agents/authentication")]
    Auth(String),

    #[error("Request timeout. See: https://docs.edge.trade/agents/errors")]
    Timeout,

    #[error("Invalid response: {0}. See: https://docs.edge.trade/agents/errors")]
    InvalidResponse(String),

    #[error("RPC error: {0}. See: https://docs.edge.trade/agents/errors")]
    Rpc(String),

    #[error("Not implemented: {0}. See: https://docs.edge.trade/agents/tools/trade#execution")]
    NotImplemented(String),
}

impl IrisClientError {
    pub const CONNECTION_DOCS: &'static str = "https://docs.edge.trade/agents/authentication";
    pub const AUTH_DOCS: &'static str = "https://docs.edge.trade/agents/authentication";
    pub const ERRORS_DOCS: &'static str = "https://docs.edge.trade/agents/errors";
    pub const EXECUTION_DOCS: &'static str = "https://docs.edge.trade/agents/tools/trade#execution";

    pub fn docs_url(&self) -> &'static str {
        match self {
            Self::WebSocket(_) => Self::ERRORS_DOCS,
            Self::Connection(_) => Self::CONNECTION_DOCS,
            Self::Auth(_) => Self::AUTH_DOCS,
            Self::Timeout => Self::ERRORS_DOCS,
            Self::InvalidResponse(_) => Self::ERRORS_DOCS,
            Self::Rpc(_) => Self::ERRORS_DOCS,
            Self::NotImplemented(_) => Self::EXECUTION_DOCS,
        }
    }
}

#[derive(Serialize)]
struct TrpcRequest {
    id: u32,
    method: String,
    params: TrpcParams,
}

#[derive(Serialize)]
struct TrpcParams {
    path: String,
    input: TrpcInput,
}

#[derive(Serialize)]
struct TrpcInput {
    json: Value,
}

#[derive(Deserialize)]
struct TrpcResponse {
    id: u32,
    result: Option<TrpcResult>,
    error: Option<TrpcError>,
}

#[derive(Deserialize)]
struct TrpcResult {
    #[serde(rename = "type")]
    _result_type: String,
    data: TrpcData,
}

#[derive(Deserialize)]
struct TrpcData {
    json: Value,
}

#[derive(Deserialize)]
struct TrpcError {
    message: String,
    code: Option<String>,
}

type RequestSender = oneshot::Sender<Result<Value, IrisClientError>>;
type SubscriptionSender = mpsc::UnboundedSender<Value>;

#[derive(Clone)]
pub struct IrisClient {
    inner: Arc<IrisClientInner>,
}

struct IrisClientInner {
    tx: mpsc::UnboundedSender<Message>,
    pending_requests: Arc<Mutex<HashMap<u32, RequestSender>>>,
    active_subscriptions: Arc<Mutex<HashMap<u32, SubscriptionSender>>>,
    next_id: Arc<Mutex<u32>>,
    verbose: bool,
}

impl IrisClient {
    pub async fn connect(url: &str, api_key: &str, verbose: bool) -> Result<Self, IrisClientError> {
        let connection_params = serde_json::json!({
            "apiKey": api_key
        });
        let encoded_params = base64::encode(connection_params.to_string());

        let ws_url = format!("{}?connectionParams={}", url, encoded_params);

        if verbose {
            eprintln!("[edge] connecting to {}", url);
            eprintln!(
                "[edge] api key: {}...{}",
                &api_key[..4.min(api_key.len())],
                &api_key[api_key.len().saturating_sub(4)..]
            );
        }

        let (ws_stream, response) = tokio_tungstenite::connect_async(&ws_url)
            .await
            .map_err(|e| IrisClientError::WebSocket(format!("WebSocket connection failed: {}", e)))?;

        if verbose {
            eprintln!("[edge] handshake status: {}", response.status());
            for (k, v) in response.headers() {
                eprintln!("[edge]   {}: {}", k, v.to_str().unwrap_or("?"));
            }
        }

        if response.status() == 401 {
            return Err(IrisClientError::Auth("Invalid API key".to_string()));
        } else if !response.status().is_success() {
            return Err(IrisClientError::Connection(format!("HTTP {}", response.status())));
        }

        if verbose {
            eprintln!("[edge] connected");
        }

        let (mut write, mut read) = ws_stream.split();
        let (tx, mut rx) = mpsc::unbounded_channel::<Message>();

        let pending_requests: Arc<Mutex<HashMap<u32, RequestSender>>> = Arc::new(Mutex::new(HashMap::new()));
        let active_subscriptions: Arc<Mutex<HashMap<u32, SubscriptionSender>>> = Arc::new(Mutex::new(HashMap::new()));
        let next_id = Arc::new(Mutex::new(1));

        let pending_requests_clone = pending_requests.clone();
        let active_subscriptions_clone = active_subscriptions.clone();

        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                if write.send(msg).await.is_err() {
                    break;
                }
            }
        });

        tokio::spawn(async move {
            while let Some(msg) = read.next().await {
                match msg {
                    Ok(Message::Text(text)) => match serde_json::from_str::<TrpcResponse>(&text) {
                        Ok(response) => {
                            if let Some(error) = response.error {
                                let err = if error.code.as_deref() == Some("UNAUTHORIZED") {
                                    IrisClientError::Auth(error.message)
                                } else if error.code.as_deref() == Some("NOT_IMPLEMENTED") {
                                    IrisClientError::NotImplemented(error.message)
                                } else {
                                    IrisClientError::Rpc(error.message)
                                };

                                let mut requests = pending_requests_clone.lock().await;
                                if let Some(sender) = requests.remove(&response.id) {
                                    let _ = sender.send(Err(err));
                                }
                            } else if let Some(result) = response.result {
                                let mut requests = pending_requests_clone.lock().await;
                                if let Some(sender) = requests.remove(&response.id) {
                                    let _ = sender.send(Ok(result.data.json));
                                } else {
                                    drop(requests);
                                    let subscriptions = active_subscriptions_clone.lock().await;
                                    if let Some(sub_sender) = subscriptions.get(&response.id) {
                                        let _ = sub_sender.send(result.data.json);
                                    }
                                }
                            } else {
                                let err = IrisClientError::InvalidResponse(format!(
                                    "Response missing both result and error for id {}",
                                    response.id
                                ));
                                let mut requests = pending_requests_clone.lock().await;
                                if let Some(sender) = requests.remove(&response.id) {
                                    let _ = sender.send(Err(err));
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("Failed to parse tRPC response: {}", e);
                        }
                    },
                    Ok(Message::Close(_)) => break,
                    Err(e) => {
                        eprintln!("WebSocket error: {}", e);
                        break;
                    }
                    _ => {}
                }
            }
        });

        Ok(Self {
            inner: Arc::new(IrisClientInner {
                tx,
                pending_requests,
                active_subscriptions,
                next_id,
                verbose,
            }),
        })
    }

    pub async fn query(&self, path: &str, input: Value) -> Result<Value, IrisClientError> {
        self.call("query", path, input).await
    }

    pub async fn mutation(&self, path: &str, input: Value) -> Result<Value, IrisClientError> {
        self.call("mutation", path, input).await
    }

    async fn call(&self, method: &str, path: &str, input: Value) -> Result<Value, IrisClientError> {
        let mut next_id = self.inner.next_id.lock().await;
        let id = *next_id;
        *next_id += 1;
        drop(next_id);

        if self.inner.verbose {
            eprintln!("[edge] → {} {} (id={}): {}", method, path, id, input);
        }

        let (tx, rx) = oneshot::channel();
        self.inner.pending_requests.lock().await.insert(id, tx);

        let request = TrpcRequest {
            id,
            method: method.to_string(),
            params: TrpcParams {
                path: path.to_string(),
                input: TrpcInput { json: input },
            },
        };

        let msg = Message::Text(serde_json::to_string(&request).unwrap().into());
        self.inner
            .tx
            .send(msg)
            .map_err(|_| IrisClientError::WebSocket("Connection closed".to_string()))?;

        let result = rx.await.map_err(|_| IrisClientError::Timeout)?;

        if self.inner.verbose {
            match &result {
                Ok(v) => eprintln!("[edge] ← {} {} (id={}): {}", method, path, id, v),
                Err(e) => eprintln!("[edge] ✗ {} {} (id={}): {}", method, path, id, e),
            }
        }

        result
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
        self.inner.active_subscriptions.lock().await.insert(id, tx);

        let request = TrpcRequest {
            id,
            method: "subscription".to_string(),
            params: TrpcParams {
                path: path.to_string(),
                input: TrpcInput { json: input },
            },
        };

        let msg = Message::Text(serde_json::to_string(&request).unwrap().into());
        self.inner
            .tx
            .send(msg)
            .map_err(|_| IrisClientError::WebSocket("Connection closed".to_string()))?;

        if self.inner.verbose {
            eprintln!("[edge] ← subscribe {} (id={}) registered", path, id);
        }

        Ok((id, rx))
    }

    pub async fn unsubscribe(&self, id: u32) -> Result<(), IrisClientError> {
        if self.inner.verbose {
            eprintln!("[edge] → subscription.stop (id={})", id);
        }

        self.inner.active_subscriptions.lock().await.remove(&id);

        let request = serde_json::json!({
            "id": id,
            "method": "subscription.stop"
        });

        let msg = Message::Text(serde_json::to_string(&request).unwrap().into());
        self.inner
            .tx
            .send(msg)
            .map_err(|_| IrisClientError::WebSocket("Connection closed".to_string()))?;

        Ok(())
    }
}

mod base64 {
    use std::fmt::Write;

    pub fn encode(input: String) -> String {
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
}
