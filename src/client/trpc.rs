use std::marker::PhantomData;
use std::sync::Arc;

use serde_json::Value;

use super::executor::ping;
use super::subscribe::{DispatchParams, IrisClientInner, subscribe, subscribe_for_dispatch, unsubscribe};

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
