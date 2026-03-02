use std::sync::Arc;

use rmcp::{
    ServerHandler, ServiceExt,
    handler::server::tool::{Parameters, ToolRouter},
    handler::server::wrapper::Json,
    tool, tool_handler, tool_router,
    transport::io::stdio,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::sync::Mutex;

use crate::client::{IrisClient, IrisClientError};
use crate::subscriptions::{SubscriptionManager, WebhookDispatcher};
use crate::types::{
    events::AlertsResponse,
    market::{InspectResponse, ScreenResponse, SearchResponse},
    orders::TradeResponse,
    portfolio::PortfolioResponse,
};

fn format_error(e: &IrisClientError) -> String {
    format!("{}. See: {}", e, e.docs_url())
}

#[derive(Clone)]
pub struct EdgeServer {
    client: IrisClient,
    subscription_manager: SubscriptionManager,
    webhook_dispatcher: WebhookDispatcher,
    active_subscriptions: Arc<Mutex<std::collections::HashMap<String, u32>>>,
    tool_router: ToolRouter<Self>,
}

#[tool_handler(router = self.tool_router)]
impl ServerHandler for EdgeServer {}

impl EdgeServer {
    pub async fn new(url: &str, api_key: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let client = IrisClient::connect(url, api_key).await?;
        Ok(Self {
            client,
            subscription_manager: SubscriptionManager::new(),
            webhook_dispatcher: WebhookDispatcher::new(),
            active_subscriptions: Arc::new(Mutex::new(std::collections::HashMap::new())),
            tool_router: Self::tool_router(),
        })
    }

    pub async fn serve_stdio(self) -> Result<(), Box<dyn std::error::Error>> {
        let service = self.serve(stdio()).await?;
        service.waiting().await?;
        Ok(())
    }

    pub async fn serve_sse(self, host: &str, port: u16) -> Result<(), Box<dyn std::error::Error>> {
        use rmcp::transport::sse_server::{SseServer, SseServerConfig};
        use std::net::SocketAddr;
        use tokio_util::sync::CancellationToken;

        let addr: SocketAddr = format!("{}:{}", host, port).parse()?;
        eprintln!("Starting SSE server on http://{}/sse", addr);

        let config = SseServerConfig {
            bind: addr,
            sse_path: "/sse".to_string(),
            post_path: "/message".to_string(),
            ct: CancellationToken::new(),
            sse_keep_alive: None,
        };

        let mut sse_server = SseServer::serve_with_config(config).await?;

        while let Some(transport) = sse_server.next_transport().await {
            let service = self.clone();
            tokio::spawn(async move {
                if let Err(e) = service.serve(transport).await {
                    eprintln!("SSE transport error: {}", e);
                }
            });
        }

        Ok(())
    }

    pub async fn serve_http(self, host: &str, port: u16) -> Result<(), Box<dyn std::error::Error>> {
        use rmcp::transport::streamable_http_server::session::local::LocalSessionManager;
        use rmcp::transport::streamable_http_server::{StreamableHttpServerConfig, StreamableHttpService};
        use std::sync::Arc;

        let addr = format!("{}:{}", host, port);
        eprintln!("Starting HTTP server on http://{}/mcp", addr);

        let config = StreamableHttpServerConfig::default();
        let session_manager = Arc::new(LocalSessionManager::default());

        let service = StreamableHttpService::new(move || Ok(self.clone()), session_manager, config);
        let router = axum::Router::new().nest_service("/mcp", service);

        let listener = tokio::net::TcpListener::bind(&addr).await?;
        axum::serve(listener, router).await?;

        Ok(())
    }
}

// Input schemas
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SearchInput {
    pub query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct InspectInput {
    pub chain_id: String,
    pub address: String,
    pub view: String,
    #[serde(flatten)]
    pub extra: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ScreenInput {
    pub chain_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_mcap: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_mcap: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_liquidity: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_volume_24h: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_sniper_pct: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_insider_pct: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_top10_pct: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_twitter: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_telegram: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dexscreener_paid: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graduation_min_pct: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PortfolioInput {
    pub address: String,
    pub view: String,
    #[serde(flatten)]
    pub extra: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TradeInput {
    pub action: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet: Option<String>,
    #[serde(flatten)]
    pub extra: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct AlertsInput {
    pub action: String,
    #[serde(flatten)]
    pub extra: Option<Value>,
}

#[tool_router(router = tool_router)]
impl EdgeServer {
    /// Search tokens by name or address. See: https://docs.edge.trade/agents/tools/search
    #[tool(name = "search")]
    pub async fn search(&self, params: Parameters<SearchInput>) -> Json<SearchResponse> {
        let input = serde_json::json!({
            "query": params.0.query,
            "chainId": params.0.chain_id,
        });

        match self.client.query("market.searchTokens", input).await {
            Ok(result) => match serde_json::from_value(result) {
                Ok(tokens) => Json(SearchResponse::Tokens(tokens)),
                Err(e) => Json(SearchResponse::Error {
                    error: format!("Deserialization error: {}", e),
                }),
            },
            Err(e) => Json(SearchResponse::Error {
                error: format_error(&e),
            }),
        }
    }

    /// Inspect tokens and pairs with multiple views. See: https://docs.edge.trade/agents/tools/inspect
    #[tool(name = "inspect")]
    pub async fn inspect(&self, params: Parameters<InspectInput>) -> Json<InspectResponse> {
        let params = params.0;
        let procedure = match params.view.as_str() {
            "token_overview" => "market.getToken",
            "token_holders" => "portfolio.getTopHolders",
            "token_analytics" => "portfolio.getTopTraders",
            "graduation" => "market.getPairGraduation",
            "pair_overview" => "market.getPairDetailed",
            "pair_metrics" => "market.getPairMetrics",
            "pair_candles" => "market.getPairCandles",
            "pair_swaps" => "market.getSwaps",
            _ => {
                return Json(InspectResponse::Error {
                    error: format!(
                        "Unknown view: {}. See: https://docs.edge.trade/agents/tools/inspect",
                        params.view
                    ),
                });
            }
        };

        let mut input = serde_json::json!({
            "chainId": params.chain_id,
        });

        if params.view == "token_overview" || params.view == "token_holders" || params.view == "token_analytics" {
            input["tokenAddress"] = serde_json::json!(params.address);
        } else {
            input["pairAddress"] = serde_json::json!(params.address);
        }

        if let Some(extra) = params.extra
            && let Some(obj) = input.as_object_mut()
            && let Some(extra_obj) = extra.as_object()
        {
            for (k, v) in extra_obj {
                obj.insert(k.clone(), v.clone());
            }
        }

        match self.client.query(procedure, input).await {
            Ok(result) => Json(InspectResponse::TokenOverview(result)),
            Err(e) => Json(InspectResponse::Error {
                error: format_error(&e),
            }),
        }
    }

    /// Screen tokens by market cap, liquidity, and holder metrics. See: https://docs.edge.trade/agents/tools/screen
    #[tool(name = "screen")]
    pub async fn screen(&self, params: Parameters<ScreenInput>) -> Json<ScreenResponse> {
        let params = params.0;
        let mut input = serde_json::json!({
            "chainId": params.chain_id,
        });

        let obj = input.as_object_mut().unwrap();
        if let Some(v) = params.min_mcap {
            obj.insert("minMcap".to_string(), serde_json::json!(v));
        }
        if let Some(v) = params.max_mcap {
            obj.insert("maxMcap".to_string(), serde_json::json!(v));
        }
        if let Some(v) = params.min_liquidity {
            obj.insert("minLiquidity".to_string(), serde_json::json!(v));
        }
        if let Some(v) = params.min_volume_24h {
            obj.insert("minVolume24h".to_string(), serde_json::json!(v));
        }
        if let Some(v) = params.max_sniper_pct {
            obj.insert("maxSniperPct".to_string(), serde_json::json!(v));
        }
        if let Some(v) = params.max_insider_pct {
            obj.insert("maxInsiderPct".to_string(), serde_json::json!(v));
        }
        if let Some(v) = params.max_top10_pct {
            obj.insert("maxTop10Pct".to_string(), serde_json::json!(v));
        }
        if let Some(v) = params.has_twitter {
            obj.insert("hasTwitter".to_string(), serde_json::json!(v));
        }
        if let Some(v) = params.has_telegram {
            obj.insert("hasTelegram".to_string(), serde_json::json!(v));
        }
        if let Some(v) = params.dexscreener_paid {
            obj.insert("dexscreenerPaid".to_string(), serde_json::json!(v));
        }
        if let Some(v) = params.graduation_min_pct {
            obj.insert("graduationMinPct".to_string(), serde_json::json!(v));
        }
        if let Some(v) = params.sort_by {
            obj.insert("sortBy".to_string(), serde_json::json!(v));
        }
        if let Some(v) = params.limit {
            obj.insert("limit".to_string(), serde_json::json!(v));
        }

        match self.client.query("market.screenTokens", input).await {
            Ok(result) => match serde_json::from_value(result) {
                Ok(tokens) => Json(ScreenResponse::Tokens(tokens)),
                Err(e) => Json(ScreenResponse::Error {
                    error: format!("Deserialization error: {}", e),
                }),
            },
            Err(e) => Json(ScreenResponse::Error {
                error: format_error(&e),
            }),
        }
    }

    /// View wallet holdings, history, and transactions. See: https://docs.edge.trade/agents/tools/portfolio
    #[tool(name = "portfolio")]
    pub async fn portfolio(&self, params: Parameters<PortfolioInput>) -> Json<PortfolioResponse> {
        let params = params.0;
        let procedure = match params.view.as_str() {
            "holdings" => "portfolio.getHoldings",
            "summary" => "portfolio.getSummary",
            "history" => "portfolio.getHoldingHistory",
            "balances" => "portfolio.getNativeBalances",
            "scan" => "portfolio.scanWallet",
            "swaps" => "portfolio.getWalletSwaps",
            "tx" => "intelligence.getSwapByTxHash",
            _ => {
                return Json(PortfolioResponse::Error {
                    error: format!(
                        "Unknown view: {}. See: https://docs.edge.trade/agents/tools/portfolio",
                        params.view
                    ),
                });
            }
        };

        let mut input = serde_json::json!({
            "walletAddress": params.address,
        });

        if let Some(extra) = params.extra
            && let Some(obj) = input.as_object_mut()
            && let Some(extra_obj) = extra.as_object()
        {
            for (k, v) in extra_obj {
                obj.insert(k.clone(), v.clone());
            }
        }

        match self.client.query(procedure, input).await {
            Ok(result) => Json(PortfolioResponse::ScanResult(result)),
            Err(e) => Json(PortfolioResponse::Error {
                error: format_error(&e),
            }),
        }
    }

    /// Place limit orders, manage strategies, estimate impact. See: https://docs.edge.trade/agents/tools/trade
    #[tool(name = "trade")]
    pub async fn trade(&self, params: Parameters<TradeInput>) -> Json<TradeResponse> {
        let params = params.0;
        match params.action.as_str() {
            "build" | "submit" => {
                return Json(TradeResponse::Error {
                    error: "Execution (build/submit) coming soon. See: https://docs.edge.trade/agents/tools/trade#execution".to_string()
                });
            }
            _ => {}
        }

        let procedure = match params.action.as_str() {
            "place" => "orders.place",
            "list" => "orders.list",
            "get" => "orders.get",
            "cancel" => "orders.cancel",
            "cancel_all" => "orders.cancelAll",
            "extend" => "orders.extend",
            "create_entry_strategy" => "orders.createEntryStrategy",
            "create_exit_strategy" => "orders.createExitStrategy",
            "list_strategies" => "orders.listEntryStrategies",
            "apply_strategy" => "orders.applyEntryStrategy",
            "update_strategy" => "orders.updateEntryStrategy",
            "delete_strategy" => "orders.removeEntryStrategy",
            "impact" => "intelligence.estimateImpact",
            _ => {
                return Json(TradeResponse::Error {
                    error: format!(
                        "Unknown action: {}. See: https://docs.edge.trade/agents/tools/trade",
                        params.action
                    ),
                });
            }
        };

        let mut input = serde_json::json!({});
        if let Some(wallet) = params.wallet {
            input["walletAddress"] = serde_json::json!(wallet);
        }

        if let Some(extra) = params.extra
            && let Some(obj) = input.as_object_mut()
            && let Some(extra_obj) = extra.as_object()
        {
            for (k, v) in extra_obj {
                obj.insert(k.clone(), v.clone());
            }
        }

        let is_mutation = matches!(
            params.action.as_str(),
            "place"
                | "cancel"
                | "cancel_all"
                | "extend"
                | "create_entry_strategy"
                | "create_exit_strategy"
                | "apply_strategy"
                | "update_strategy"
                | "delete_strategy"
        );

        let result = if is_mutation {
            self.client.mutation(procedure, input).await
        } else {
            self.client.query(procedure, input).await
        };

        match result {
            Ok(result) => Json(TradeResponse::BuildResult(result)),
            Err(e) => Json(TradeResponse::Error {
                error: format_error(&e),
            }),
        }
    }

    /// Subscribe to price alerts and order updates. See: https://docs.edge.trade/agents/tools/alerts
    #[tool(name = "alerts")]
    pub async fn alerts(&self, params: Parameters<AlertsInput>) -> Json<AlertsResponse> {
        let params = params.0;
        let extra = params.extra.unwrap_or_default();

        match params.action.as_str() {
            "subscribe" => {
                let topic = extra
                    .get("topic")
                    .and_then(|v| v.as_str())
                    .unwrap_or("all")
                    .to_string();
                let filters = extra.get("filters").cloned().unwrap_or_default();
                let filters_for_webhook = filters.clone();

                let procedure = format!("alerts.subscribe.{}", topic);
                match self.client.subscribe(&procedure, filters).await {
                    Ok((sub_id, mut rx)) => {
                        let mut subs = self.active_subscriptions.lock().await;
                        subs.insert(topic.clone(), sub_id);
                        drop(subs);

                        let webhook_url = extra
                            .get("webhook_url")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string());
                        let webhook_secret = extra
                            .get("webhook_secret")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string());

                        if let (Some(url), Some(secret)) = (&webhook_url, &webhook_secret) {
                            self.webhook_dispatcher
                                .register(&topic, url, secret, Some(filters_for_webhook))
                                .await;
                        }

                        let sub_manager = self.subscription_manager.clone();
                        let webhook_dispatcher = self.webhook_dispatcher.clone();
                        let topic_clone = topic.clone();
                        let sub_id_str = sub_id.to_string();

                        sub_manager.create_subscription(sub_id_str.clone()).await;

                        tokio::spawn(async move {
                            while let Some(event) = rx.recv().await {
                                sub_manager.push_event(&sub_id_str, event.clone()).await;

                                if let Some((url, secret)) = webhook_dispatcher.get_webhook(&topic_clone).await {
                                    let _ = webhook_dispatcher
                                        .dispatch(&url, Some(&secret), event)
                                        .await;
                                }
                            }
                        });

                        Json(AlertsResponse::Subscription {
                            message: format!("Subscribed to {} alerts", topic),
                            subscription_id: sub_id,
                        })
                    }
                    Err(e) => Json(AlertsResponse::Error {
                        error: format!("Error subscribing: {}", e),
                    }),
                }
            }
            "poll" => {
                let topic = extra
                    .get("topic")
                    .and_then(|v| v.as_str())
                    .unwrap_or("all")
                    .to_string();
                let limit = extra.get("limit").and_then(|v| v.as_u64()).unwrap_or(10) as usize;

                let subs = self.active_subscriptions.lock().await;
                if let Some(sub_id) = subs.get(&topic) {
                    let events = self
                        .subscription_manager
                        .poll_events(&sub_id.to_string(), limit)
                        .await;

                    let alert_events: Vec<_> = events
                        .into_iter()
                        .filter_map(|e| serde_json::from_value(e).ok())
                        .collect();

                    Json(AlertsResponse::Events(alert_events))
                } else {
                    Json(AlertsResponse::Error {
                        error: format!("No active subscription for topic: {}", topic),
                    })
                }
            }
            "unsubscribe" => {
                let topic = extra
                    .get("topic")
                    .and_then(|v| v.as_str())
                    .unwrap_or("all")
                    .to_string();

                let mut subs = self.active_subscriptions.lock().await;
                if let Some(sub_id) = subs.remove(&topic) {
                    match self.client.unsubscribe(sub_id).await {
                        Ok(_) => {
                            self.subscription_manager
                                .remove_subscription(&sub_id.to_string())
                                .await;
                            self.webhook_dispatcher.unregister(&topic).await;
                            Json(AlertsResponse::Success {
                                message: format!("Unsubscribed from {} alerts", topic),
                            })
                        }
                        Err(e) => Json(AlertsResponse::Error {
                            error: format!("Error unsubscribing: {}", e),
                        }),
                    }
                } else {
                    Json(AlertsResponse::Error {
                        error: format!("No active subscription for topic: {}", topic),
                    })
                }
            }
            _ => Json(AlertsResponse::Error {
                error: format!(
                    "Unknown action: {}. See: https://docs.edge.trade/agents/tools/alerts",
                    params.action
                ),
            }),
        }
    }
}
