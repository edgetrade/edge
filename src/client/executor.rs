use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value;

use crate::messages::{self, IrisClientError};

use super::subscribe::IrisClientInner;
use super::trpc::{IrisClient, Route, RouteType};

/// Trait for executing typed routes via the HTTP transport.
#[allow(async_fn_in_trait)]
pub trait RouteExecutor {
    /// Execute a route with the given input and return the typed response.
    async fn execute<R, S>(&self, route: &Route<R, S>, input: &R) -> Result<S, IrisClientError>
    where
        R: Serialize,
        S: DeserializeOwned + Serialize + Clone;
}

impl RouteExecutor for IrisClient {
    async fn execute<R, S>(&self, route: &Route<R, S>, input: &R) -> Result<S, IrisClientError>
    where
        R: Serialize,
        S: DeserializeOwned + Serialize + Clone,
    {
        let input_value = serde_json::to_value(input).map_err(|e| IrisClientError::Serialization(e.to_string()))?;

        match route.route_type {
            RouteType::Query => query(&self.inner, route.procedure, input_value).await,
            RouteType::Mutation => mutation(&self.inner, route.procedure, input_value).await,
            RouteType::Subscription => unimplemented!("Subscriptions not supported via RouteExecutor"),
        }
    }
}

/// Execute a query call with the given path and input.
pub async fn query<T: DeserializeOwned + Serialize + Clone>(
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
pub async fn mutation<T: DeserializeOwned>(
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
async fn call<T: DeserializeOwned>(inner: &IrisClientInner, path: &str, input: Value) -> Result<T, IrisClientError> {
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
