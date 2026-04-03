use std::marker::PhantomData;
use std::sync::Arc;

use serde_json::Value;

use super::subscribe::{DispatchParams, IrisClientInner, subscribe, subscribe_for_dispatch, unsubscribe};
use crate::domains::client::errors::ClientError;
use crate::messages::{self, IrisClientError};

#[derive(Debug, Clone, Copy)]
pub enum RouteType {
    Query,
    Mutation,
    Subscription,
}

#[derive(Debug, Clone, Copy)]
pub struct Route<R, S> {
    pub procedure: &'static str,
    pub route_type: RouteType,
    pub input_schema: PhantomData<fn() -> R>,
    pub output_schema: PhantomData<fn() -> S>,
}

#[derive(Clone)]
pub struct IrisClient {
    pub(crate) inner: Arc<IrisClientInner>,
}

impl std::fmt::Debug for IrisClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IrisClient")
            .field("base_url", &self.inner.base_url)
            .field("verbose", &self.inner.verbose)
            .finish()
    }
}

impl IrisClient {
    pub async fn connect(url: &str, api_key: &str, verbose: bool) -> Result<Self, IrisClientError> {
        let base_url = url
            .replace("wss://", "https://")
            .replace("ws://", "http://");

        let http = reqwest::Client::new();
        if verbose {
            messages::success::connecting_to_url(&base_url);
        }

        Ok(Self {
            inner: Arc::new(IrisClientInner {
                base_url,
                api_key: api_key.to_string(),
                http,
                verbose,
                subscriptions: Arc::new(tokio::sync::Mutex::new(std::collections::HashMap::new())),
                next_id: Arc::new(tokio::sync::Mutex::new(1)),
            }),
        })
    }

    pub async fn ping(&self) -> Result<(), IrisClientError> {
        let url = format!("{}/ping", self.inner.base_url);
        ping(&self.inner, &url).await
    }

    pub async fn subscribe(
        &self,
        path: &str,
        input: Value,
    ) -> Result<(u32, tokio::sync::mpsc::UnboundedReceiver<Value>), IrisClientError> {
        subscribe(self.inner.clone(), path, input).await
    }

    pub async fn subscribe_for_dispatch(
        &self,
        procedure: &str,
        input: Value,
        params: DispatchParams,
    ) -> Result<u32, IrisClientError> {
        subscribe_for_dispatch(self.inner.clone(), procedure, input, params).await
    }

    pub async fn unsubscribe(&self, id: u32) -> Result<(), IrisClientError> {
        unsubscribe(self.inner.clone(), id).await
    }
}

pub async fn new_client(url: String, api_key: String, verbose: bool) -> Result<IrisClient, IrisClientError> {
    IrisClient::connect(&url, &api_key, verbose).await
}

// === Internal route execution functions (moved from executor.rs) ===

/// Execute a query call with the given path and input.
async fn query<T: serde::de::DeserializeOwned + serde::Serialize + Clone>(
    inner: &IrisClientInner,
    path: &str,
    input: Value,
) -> Result<T, IrisClientError> {
    let result = call::<T>(inner, path, input).await;
    if inner.verbose
        && let Ok(rez) = result.as_ref()
    {
        messages::success::query_response(path, &serde_json::to_string(rez).unwrap());
    }
    result
}

/// Execute a mutation call with the given path and input.
async fn mutation<T: serde::de::DeserializeOwned>(
    inner: &IrisClientInner,
    path: &str,
    input: Value,
) -> Result<T, IrisClientError> {
    let result = call::<T>(inner, path, input).await;
    if inner.verbose && result.is_ok() {
        messages::success::mutation_response(path);
    }
    result
}

fn map_request_error(e: &reqwest::Error) -> IrisClientError {
    if e.is_timeout() {
        IrisClientError::Timeout
    } else if e.status().is_some_and(|s| s.as_u16() == 401) {
        IrisClientError::Auth("Invalid API key".to_string())
    } else {
        IrisClientError::Http(format!("Request failed: {}", e))
    }
}

/// Internal function to make an HTTP call to the API.
pub async fn call<T: serde::de::DeserializeOwned>(
    inner: &IrisClientInner,
    path: &str,
    input: Value,
) -> Result<T, IrisClientError> {
    let url = format!("{}/v1/call", inner.base_url);
    let request_body = serde_json::json!({ "path": path, "input": input });

    if inner.verbose {
        messages::success::query_request(path, &serde_json::to_string(&request_body).unwrap());
    }

    let response = inner
        .http
        .post(&url)
        .bearer_auth(&inner.api_key)
        .json(&request_body)
        .send()
        .await
        .map_err(|e| map_request_error(&e))?;

    let api_response: Value = response
        .json()
        .await
        .map_err(|e| IrisClientError::InvalidResponse(e.to_string()))?;

    if let Some(error) = api_response.get("error").and_then(|e| e.as_object()) {
        let code = error
            .get("code")
            .and_then(|c| c.as_str())
            .unwrap_or("UNKNOWN");
        let message = error
            .get("message")
            .and_then(|m| m.as_str())
            .unwrap_or("Unknown error");
        let err = match code {
            "UNAUTHORIZED" => IrisClientError::Auth(message.to_string()),
            "NOT_IMPLEMENTED" => IrisClientError::NotImplemented(message.to_string()),
            _ => IrisClientError::Rpc(message.to_string()),
        };
        if inner.verbose {
            messages::error::query_error(path, &err.to_string());
        }
        return Err(err);
    }

    let data = api_response
        .get("data")
        .cloned()
        .ok_or(IrisClientError::MissingData)?;

    if inner.verbose {
        let mut s = serde_json::to_string(&data).unwrap();
        s.truncate(2084);
        messages::success::query_response(path, &s);
    }

    serde_json::from_value::<T>(data).map_err(|e| IrisClientError::Deserialization(e.to_string()))
}

/// Execute a ping call with the given path and input.
pub async fn ping(inner: &IrisClientInner, path: &str) -> Result<(), IrisClientError> {
    let response = inner
        .http
        .get(path)
        .send()
        .await
        .map_err(|e| IrisClientError::Http(format!("Ping failed: {}", e)))?;

    if response.status().is_success() {
        Ok(())
    } else {
        Err(IrisClientError::Http(format!("Ping failed: {}", response.status())))
    }
}

/// Trait for executing typed routes via the HTTP transport.
#[allow(async_fn_in_trait)]
pub trait RouteExecutor {
    /// Execute a route with the given input and return the typed response.
    async fn execute<R, S>(&self, route: &Route<R, S>, input: &R) -> Result<S, ClientError>
    where
        R: serde::Serialize,
        S: serde::de::DeserializeOwned + serde::Serialize + Clone;
}

impl RouteExecutor for IrisClient {
    async fn execute<R, S>(&self, route: &Route<R, S>, input: &R) -> Result<S, ClientError>
    where
        R: serde::Serialize,
        S: serde::de::DeserializeOwned + serde::Serialize + Clone,
    {
        let input_value = serde_json::to_value(input).map_err(|e| ClientError::Serialization(e.to_string()))?;

        match route.route_type {
            RouteType::Query => {
                let result = query::<S>(&self.inner, route.procedure, input_value)
                    .await
                    .map_err(ClientError::from_iris_error)?;
                Ok(result)
            }
            RouteType::Mutation => {
                let result = mutation::<S>(&self.inner, route.procedure, input_value)
                    .await
                    .map_err(ClientError::from_iris_error)?;
                Ok(result)
            }
            RouteType::Subscription => Err(ClientError::NotImplemented(
                "Subscriptions not supported via RouteExecutor".to_string(),
            )),
        }
    }
}
