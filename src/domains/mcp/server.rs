//! MCP server implementation
//!
//! This module contains the EdgeServer which implements the rmcp ServerHandler trait.
//! It provides MCP protocol support including tools, resources, and prompts.
//!
//! MIGRATED FROM: pkg/poseidon/src/commands/serve/mcp.rs
//! - Ported working implementation from main branch
//! - Adapted to use new domain structure

use std::collections::HashMap;
use std::future::Future;
use std::sync::Arc;

use rmcp::{
    ErrorData as McpErrorData, ServerHandler, ServiceExt,
    model::{
        CallToolRequestParams, CallToolResult, Content, GetPromptRequestParams, GetPromptResult, Implementation,
        ListPromptsResult, ListResourcesResult, ListToolsResult, PaginatedRequestParams, Prompt, PromptArgument,
        PromptMessage, PromptMessageRole, RawResource, ReadResourceRequestParams, ReadResourceResult, Resource,
        ResourceContents, ServerCapabilities, ServerInfo, Tool,
    },
    service::{RequestContext, RoleServer},
    transport::io::stdio,
};
use serde_json::Value;
use tokio::sync::{Mutex, RwLock};

use crate::domains::alerts::{AlertRegistration, DeliveryConfig, SubscriptionManager};
use crate::domains::client::manifest::inject::{inject_local_agent_actions, inject_local_resources};
use crate::domains::client::manifest::types::McpManifest;
use crate::domains::client::{IrisClient, IrisClientError, generated::validation::find_route};
use crate::messages;

/// Type alias for active subscriptions map
pub type ActiveSubscriptions = std::sync::Arc<tokio::sync::Mutex<std::collections::HashMap<String, u32>>>;

/// Type alias for alert registry
pub use crate::domains::alerts::AlertRegistry;

/// Generate next alert ID using atomic counter
pub use crate::domains::alerts::next_alert_id;
#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AlertDelivery {
    /// Webhook delivery with optional HMAC signature
    Webhook {
        /// Target webhook URL
        url: String,
        /// Secret for HMAC-SHA256 signature
        secret: Option<String>,
    },
    /// Redis stream delivery
    Redis {
        /// Redis connection URL
        url: String,
        /// Redis stream channel name
        channel: String,
    },
    /// Telegram bot delivery
    Telegram {
        /// Bot token for Telegram API
        bot_token: String,
        /// Target chat ID
        chat_id: String,
    },
}

/// Webhook dispatcher for subscription events
#[derive(Clone)]
pub struct WebhookDispatcher {
    inner: Arc<Mutex<HashMap<String, WebhookConfig>>>,
}

#[derive(Clone)]
struct WebhookConfig {
    url: String,
    secret: Option<String>,
}

impl WebhookDispatcher {
    /// Create a new webhook dispatcher
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Register a webhook for a procedure
    pub async fn register(&self, procedure: &str, url: &str, secret: Option<&str>) {
        let mut inner = self.inner.lock().await;
        inner.insert(
            procedure.to_string(),
            WebhookConfig {
                url: url.to_string(),
                secret: secret.map(|s| s.to_string()),
            },
        );
    }

    /// Unregister a webhook for a procedure
    pub async fn unregister(&self, procedure: &str) {
        let mut inner = self.inner.lock().await;
        inner.remove(procedure);
    }

    /// Get webhook config for a procedure
    pub async fn get_webhook(&self, procedure: &str) -> Option<(String, Option<String>)> {
        let inner = self.inner.lock().await;
        inner
            .get(procedure)
            .map(|c| (c.url.clone(), c.secret.clone()))
    }

    /// Dispatch an event to a webhook
    pub async fn dispatch(&self, url: &str, secret: Option<&str>, event: Value) -> Result<(), String> {
        use hmac::{Hmac, Mac};
        use sha2::Sha256;
        type HmacSha256 = Hmac<Sha256>;

        let ts = chrono::Utc::now().to_rfc3339();
        let payload = serde_json::json!({ "ts": ts, "event": event });
        let body = serde_json::to_string(&payload).map_err(|e| e.to_string())?;

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .map_err(|e| e.to_string())?;

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
            request = request.header("X-Edge-Signature", format!("sha256={}", sig));
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
}

impl Default for WebhookDispatcher {
    fn default() -> Self {
        Self::new()
    }
}

/// EdgeServer - MCP server implementation for Poseidon
///
/// Provides MCP protocol support including:
/// - Tools from manifest (namespace-based with discriminated union actions)
/// - Resources from manifest
/// - Prompts from manifest
/// - Local actions (ping, list_alerts, register_alert, unregister_alert)
/// - Subscription-based tools (subscribe, poll, stop actions)
#[derive(Clone)]
pub struct EdgeServer {
    /// Iris client for API calls
    pub client: IrisClient,
    /// MCP manifest containing tools, resources, prompts
    pub manifest: Arc<RwLock<McpManifest>>,
    /// Subscription manager for buffered events
    pub subscription_manager: SubscriptionManager,
    /// Webhook dispatcher for subscription notifications
    pub webhook_dispatcher: WebhookDispatcher,
    /// Active subscriptions map: procedure -> subscription_id
    pub active_subscriptions: ActiveSubscriptions,
    /// Alert registry: alert_id -> AlertRegistration
    pub alert_registry: AlertRegistry,
    /// HTTP client for webhook/HTTP calls
    pub http_client: reqwest::Client,
}

impl ServerHandler for EdgeServer {
    fn get_info(&self) -> ServerInfo {
        let mut info = ServerInfo::new(
            ServerCapabilities::builder()
                .enable_tools()
                .enable_resources()
                .enable_prompts()
                .build(),
        );
        info.server_info = Implementation::new("edge", env!("CARGO_PKG_VERSION"));
        info
    }

    async fn list_tools(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListToolsResult, McpErrorData> {
        let manifest = self.manifest.read().await;
        let tools = manifest
            .tools
            .iter()
            .map(|def| {
                // Build a discriminated union for `data`. Each branch is titled with the
                // action name so the LLM knows: pick action X → use the branch titled X for data.
                let one_of: Vec<Value> = def
                    .actions
                    .iter()
                    .map(|action| {
                        let mut branch = action.input_schema.clone();
                        if let Value::Object(ref mut map) = branch {
                            map.insert("title".to_string(), Value::String(action.name.clone()));
                            map.entry("type".to_string())
                                .or_insert_with(|| Value::String("object".to_string()));
                        }
                        branch
                    })
                    .collect();

                let data_schema = match one_of.len() {
                    0 => serde_json::json!({ "type": "object" }),
                    1 => one_of.into_iter().next().unwrap(),
                    _ => serde_json::json!({ "oneOf": one_of }),
                };

                let mut schema = serde_json::Map::new();
                schema.insert("type".to_string(), Value::String("object".to_string()));
                schema.insert(
                    "properties".to_string(),
                    serde_json::json!({
                        "action": {
                            "type": "string",
                            "enum": def.actions.iter().map(|a| a.name.as_str()).collect::<Vec<_>>()
                        },
                        "data": data_schema
                    }),
                );
                schema.insert("required".to_string(), serde_json::json!(["action"]));

                Tool::new(def.name.clone(), def.description.clone(), Arc::new(schema))
            })
            .collect::<Vec<_>>();
        Ok(ListToolsResult::with_all_items(tools))
    }

    fn call_tool(
        &self,
        request: CallToolRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> impl Future<Output = Result<CallToolResult, McpErrorData>> + Send + '_ {
        let name = request.name.to_string();
        let args = request.arguments.map(Value::Object).unwrap_or_default();
        let client = self.client.clone();
        let sub_manager = self.subscription_manager.clone();
        let webhook_dispatcher = self.webhook_dispatcher.clone();
        let active_subscriptions = self.active_subscriptions.clone();
        let manifest = self.manifest.clone();
        let alert_registry = self.alert_registry.clone();
        let http_client = self.http_client.clone();

        async move {
            // Find the namespace tool, then resolve the action within it.
            let tool = manifest
                .read()
                .await
                .tools
                .iter()
                .find(|t| t.name == name)
                .cloned();
            let tool = match tool {
                Some(t) => t,
                None => {
                    return Ok(CallToolResult::error(vec![Content::text(format!(
                        "Unknown tool: {name}"
                    ))]));
                }
            };

            let action_name = match args.get("action").and_then(|v| v.as_str()) {
                Some(a) => a.to_string(),
                None => {
                    let available = tool
                        .actions
                        .iter()
                        .map(|a| a.name.as_str())
                        .collect::<Vec<_>>()
                        .join(", ");
                    return Ok(CallToolResult::error(vec![Content::text(format!(
                        "Missing required field: action. Available: {available}"
                    ))]));
                }
            };

            let action_def = match tool.actions.iter().find(|a| a.name == action_name) {
                Some(a) => a.clone(),
                None => {
                    let available = tool
                        .actions
                        .iter()
                        .map(|a| a.name.as_str())
                        .collect::<Vec<_>>()
                        .join(", ");
                    return Ok(CallToolResult::error(vec![Content::text(format!(
                        "Unknown action '{action_name}'. Available: {available}"
                    ))]));
                }
            };

            let data = args
                .get("data")
                .cloned()
                .unwrap_or(Value::Object(Default::default()));

            if action_def.kind == "local" {
                return handle_local_action(&action_name, data, client, manifest, alert_registry, http_client).await;
            }

            if action_def.kind == "subscription" {
                return handle_subscription(
                    data,
                    &action_def.procedure,
                    client,
                    sub_manager,
                    webhook_dispatcher,
                    active_subscriptions,
                )
                .await;
            }

            let route = find_route(&action_def.procedure).ok_or_else(|| {
                McpErrorData::invalid_params(format!("Action not found: {}", action_def.procedure), None)
            })?;

            match route.execute(&client, data).await {
                Ok(result) => Ok(CallToolResult::success(vec![Content::text(result.to_string())])),
                Err(IrisClientError::Deserialization(_)) => Ok(CallToolResult::error(vec![Content::text(format!(
                    "Input validation failed for '{}'. Expected input schema: {}",
                    action_def.procedure,
                    serde_json::to_string_pretty(&action_def.input_schema).unwrap_or_default()
                ))])),
                Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
            }
        }
    }

    async fn list_resources(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListResourcesResult, McpErrorData> {
        let manifest = self.manifest.read().await;
        let resources = manifest
            .resources
            .iter()
            .map(|def| Resource {
                raw: RawResource {
                    uri: def.uri.clone(),
                    name: def.name.clone(),
                    title: None,
                    description: Some(def.description.clone()),
                    mime_type: Some(def.mime_type.clone()),
                    size: None,
                    icons: None,
                    meta: None,
                },
                annotations: None,
            })
            .collect::<Vec<_>>();
        Ok(ListResourcesResult::with_all_items(resources))
    }

    fn read_resource(
        &self,
        request: ReadResourceRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> impl Future<Output = Result<ReadResourceResult, McpErrorData>> + Send + '_ {
        let uri = request.uri;
        let manifest = self.manifest.clone();
        async move {
            let resource = manifest
                .read()
                .await
                .resources
                .iter()
                .find(|r| r.uri == uri)
                .cloned();
            match resource {
                Some(def) => {
                    let text = serde_json::to_string_pretty(&def.content).unwrap_or_default();
                    Ok(ReadResourceResult::new(vec![ResourceContents::TextResourceContents {
                        uri: def.uri,
                        mime_type: Some(def.mime_type),
                        text,
                        meta: None,
                    }]))
                }
                None => Err(McpErrorData::resource_not_found(
                    format!("Resource not found: {uri}"),
                    None,
                )),
            }
        }
    }

    async fn list_prompts(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListPromptsResult, McpErrorData> {
        let manifest = self.manifest.read().await;
        let prompts = manifest
            .prompts
            .iter()
            .map(|def| {
                let args: Vec<PromptArgument> = def
                    .arguments
                    .iter()
                    .map(|a| {
                        PromptArgument::new(a.name.clone())
                            .with_description(a.description.clone())
                            .with_required(a.required)
                    })
                    .collect();
                Prompt::new(def.name.clone(), Some(def.description.clone()), Some(args))
            })
            .collect::<Vec<_>>();
        Ok(ListPromptsResult::with_all_items(prompts))
    }

    fn get_prompt(
        &self,
        request: GetPromptRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> impl Future<Output = Result<GetPromptResult, McpErrorData>> + Send + '_ {
        let name = request.name;
        let manifest = self.manifest.clone();
        async move {
            let prompt = manifest
                .read()
                .await
                .prompts
                .iter()
                .find(|p| p.name == name)
                .cloned();
            match prompt {
                Some(def) => {
                    let messages: Vec<PromptMessage> = def
                        .messages
                        .iter()
                        .filter_map(|msg| {
                            let role_str = msg.get("role").and_then(|r| r.as_str())?;
                            let role = match role_str {
                                "assistant" => PromptMessageRole::Assistant,
                                _ => PromptMessageRole::User,
                            };
                            let text = msg
                                .get("content")
                                .and_then(|c| c.get("text"))
                                .and_then(|t| t.as_str())
                                .unwrap_or_default();
                            Some(PromptMessage::new_text(role, text))
                        })
                        .collect();
                    Ok(GetPromptResult::new(messages).with_description(def.description))
                }
                None => Err(McpErrorData::invalid_params(format!("Prompt not found: {name}"), None)),
            }
        }
    }
}

impl EdgeServer {
    /// Create a new EdgeServer instance
    ///
    /// # Arguments
    /// * `client` - Iris client for API calls
    /// * `manifest` - MCP manifest containing tools, resources, prompts
    ///
    /// # Returns
    /// A new EdgeServer instance with all dependencies initialized
    pub async fn new(
        client: IrisClient,
        manifest: Arc<RwLock<McpManifest>>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        // Inject local actions and resources into manifest
        {
            let mut m = manifest.write().await;
            inject_local_agent_actions(&mut m);
            inject_local_resources(&mut m);
        }

        Ok(Self {
            client,
            manifest,
            subscription_manager: SubscriptionManager::new(),
            webhook_dispatcher: WebhookDispatcher::new(),
            active_subscriptions: Arc::new(Mutex::new(HashMap::new())),
            alert_registry: Arc::new(Mutex::new(HashMap::new())),
            http_client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(10))
                .build()
                .unwrap(),
        })
    }

    /// Serve MCP over stdio transport
    ///
    /// This is the primary transport for AI agents and MCP clients.
    /// Uses stdin/stdout for JSON-RPC communication.
    pub async fn serve_stdio(self) -> crate::domains::mcp::errors::McpResult<()> {
        let service = self
            .serve(stdio())
            .await
            .map_err(|e| crate::domains::mcp::errors::McpError::Service(e.to_string()))?;
        service
            .waiting()
            .await
            .map_err(|e| crate::domains::mcp::errors::McpError::Service(e.to_string()))?;
        Ok(())
    }

    /// Serve MCP over HTTP transport
    ///
    /// This provides an HTTP endpoint for web-based MCP clients.
    /// Uses axum for HTTP server and rmcp's streamable HTTP transport.
    ///
    /// # Arguments
    /// * `host` - Host address to bind to (e.g., "127.0.0.1")
    /// * `port` - Port number to listen on
    /// * `path` - URL path for the MCP endpoint (e.g., "/mcp")
    pub async fn serve_http(self, host: &str, port: u16, path: &str) -> crate::domains::mcp::errors::McpResult<()> {
        use rmcp::transport::streamable_http_server::session::local::LocalSessionManager;
        use rmcp::transport::streamable_http_server::{StreamableHttpServerConfig, StreamableHttpService};

        let addr = format!("{}:{}", host, port);
        let path = if path.starts_with('/') {
            path.to_string()
        } else {
            format!("/{}", path)
        };
        messages::error::http_server_starting(&addr, &path);

        let session_manager = Arc::new(LocalSessionManager::default());
        let service = StreamableHttpService::new(
            move || Ok(self.clone()),
            session_manager,
            StreamableHttpServerConfig::default(),
        );
        let router = axum::Router::new().nest_service(&path, service);

        let listener = tokio::net::TcpListener::bind(&addr)
            .await
            .map_err(|e| crate::domains::mcp::errors::McpError::TransportError(e.to_string()))?;
        axum::serve(listener, router)
            .await
            .map_err(|e| crate::domains::mcp::errors::McpError::TransportError(e.to_string()))?;
        Ok(())
    }
}

/// Handle local actions (ping, list_alerts, register_alert, unregister_alert)
///
/// These are actions that don't require external API calls and are handled
/// directly by the edge server.
async fn handle_local_action(
    action_name: &str,
    data: Value,
    client: IrisClient,
    manifest: Arc<RwLock<McpManifest>>,
    alert_registry: AlertRegistry,
    http_client: reqwest::Client,
) -> Result<CallToolResult, McpErrorData> {
    match action_name {
        "ping" => match client.ping().await {
            Ok(()) => Ok(CallToolResult::success(vec![Content::text(r#"{"message":"pong"}"#)])),
            Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
        },
        "list_alerts" => handle_list_alerts(alert_registry).await,
        "register_alert" => handle_register_alert(data, client, manifest, alert_registry, http_client).await,
        "unregister_alert" => handle_unregister_alert(data, client, alert_registry).await,
        _ => Ok(CallToolResult::error(vec![Content::text(format!(
            "Unknown local action: {action_name}"
        ))])),
    }
}

/// Handle list_alerts action
async fn handle_list_alerts(alert_registry: AlertRegistry) -> Result<CallToolResult, McpErrorData> {
    let registry = alert_registry.lock().await;
    let mut alerts: Vec<_> = registry
        .iter()
        .map(|(id, reg)| {
            serde_json::json!({
                "alert_id": id.to_string(),
                "alert_name": reg.alert_name,
                "destination": delivery_summary(&reg.delivery),
            })
        })
        .collect();
    alerts.sort_by_key(|a| {
        a["alert_id"]
            .as_str()
            .unwrap_or("")
            .parse::<u64>()
            .unwrap_or(0)
    });
    let resp = serde_json::json!({ "alerts": alerts, "count": alerts.len() });
    Ok(CallToolResult::success(vec![Content::text(resp.to_string())]))
}

/// Get delivery summary string
fn delivery_summary(delivery: &DeliveryConfig) -> String {
    match delivery {
        DeliveryConfig::Webhook { url, .. } => {
            format!("webhook: {}", url)
        }
        DeliveryConfig::Redis { channel, .. } => {
            format!("redis: {}", channel)
        }
        DeliveryConfig::Telegram { .. } => "telegram".to_string(),
    }
}

/// Convert DeliveryConfig to AlertDelivery for dispatch params
fn convert_delivery_config(config: &DeliveryConfig) -> crate::domains::alerts::AlertDelivery {
    match config {
        DeliveryConfig::Webhook { url, secret } => crate::domains::alerts::AlertDelivery::Webhook {
            url: url.clone(),
            secret: secret.clone(),
        },
        DeliveryConfig::Redis { url, channel } => crate::domains::alerts::AlertDelivery::Redis {
            url: url.clone(),
            channel: channel.clone(),
        },
        DeliveryConfig::Telegram { bot_token, chat_id } => crate::domains::alerts::AlertDelivery::Telegram {
            bot_token: bot_token.clone(),
            chat_id: chat_id.clone(),
        },
    }
}

/// Handle register_alert action
async fn handle_register_alert(
    data: Value,
    client: IrisClient,
    manifest: Arc<RwLock<McpManifest>>,
    alert_registry: AlertRegistry,
    http_client: reqwest::Client,
) -> Result<CallToolResult, McpErrorData> {
    let alert_name = match data.get("alert_name").and_then(|v| v.as_str()) {
        Some(n) => n.to_string(),
        None => {
            return Ok(CallToolResult::error(vec![Content::text(
                "Missing required field: alert_name. Read edge://alerts to see available alert types.",
            )]));
        }
    };

    let input = data
        .get("input")
        .cloned()
        .unwrap_or(Value::Object(Default::default()));

    let delivery_value = data.get("delivery").cloned().unwrap_or(Value::Null);
    let delivery: DeliveryConfig = match serde_json::from_value(delivery_value) {
        Ok(d) => d,
        Err(e) => {
            return Ok(CallToolResult::error(vec![Content::text(format!(
                "Invalid delivery config: {e}. Use type=webhook|redis|telegram."
            ))]));
        }
    };

    // Resolve procedure from the edge://alerts resource content in the manifest.
    let procedure = {
        let m = manifest.read().await;
        m.resources
            .iter()
            .find(|r| r.uri == "edge://alerts")
            .and_then(|r| r.content.as_array())
            .and_then(|arr| {
                arr.iter()
                    .find(|item| item.get("name").and_then(|n| n.as_str()) == Some(alert_name.as_str()))
            })
            .and_then(|item| item.get("procedure").and_then(|p| p.as_str()))
            .map(|s| s.to_string())
    };

    let procedure = match procedure {
        Some(p) => p,
        None => {
            return Ok(CallToolResult::error(vec![Content::text(format!(
                "Unknown alert_name: '{alert_name}'. Read edge://alerts to see available types."
            ))]));
        }
    };

    let alert_id = next_alert_id();

    // Convert DeliveryConfig to AlertDelivery for dispatch params
    let alert_delivery = convert_delivery_config(&delivery);

    // Subscribe using the client's subscribe_for_dispatch method with proper dispatch params
    let dispatch_params = crate::domains::client::DispatchParams {
        alert_id,
        alert_name: alert_name.clone(),
        delivery: alert_delivery,
        alert_registry: alert_registry.clone(),
        http_client,
    };

    let sub_id = match client
        .subscribe_for_dispatch(&procedure, input, dispatch_params)
        .await
    {
        Ok(id) => id,
        Err(e) => {
            return Ok(CallToolResult::error(vec![Content::text(format!(
                "Failed to subscribe to {procedure}: {e}"
            ))]));
        }
    };

    messages::error::alert_registered(&alert_name, alert_id, &delivery_summary(&delivery));

    alert_registry.lock().await.insert(
        alert_id,
        AlertRegistration {
            alert_name,
            subscription_id: sub_id,
            delivery,
        },
    );

    let resp = serde_json::json!({ "alert_id": alert_id.to_string() });
    Ok(CallToolResult::success(vec![Content::text(resp.to_string())]))
}

/// Handle unregister_alert action
async fn handle_unregister_alert(
    data: Value,
    client: IrisClient,
    alert_registry: AlertRegistry,
) -> Result<CallToolResult, McpErrorData> {
    let alert_id_str = match data.get("alert_id").and_then(|v| v.as_str()) {
        Some(s) => s.to_string(),
        None => {
            return Ok(CallToolResult::error(vec![Content::text(
                "Missing required field: alert_id",
            )]));
        }
    };

    let alert_id: u64 = match alert_id_str.parse() {
        Ok(id) => id,
        Err(_) => {
            return Ok(CallToolResult::error(vec![Content::text(format!(
                "Invalid alert_id: '{alert_id_str}'"
            ))]));
        }
    };

    let registration = alert_registry.lock().await.remove(&alert_id);
    match registration {
        Some(reg) => {
            let _ = client.unsubscribe(reg.subscription_id).await;
            let resp = serde_json::json!({
                "message": format!("Alert '{}' (id={}) unregistered", reg.alert_name, alert_id)
            });
            Ok(CallToolResult::success(vec![Content::text(resp.to_string())]))
        }
        None => Ok(CallToolResult::error(vec![Content::text(format!(
            "No active alert with id={alert_id}"
        ))])),
    }
}

/// Handle subscription actions (subscribe, poll, stop)
///
/// Subscription tools support three actions via the reserved `_action` argument:
/// - `subscribe` (default): Starts a new SSE subscription to `procedure`
/// - `poll`: Drains buffered events
/// - `stop`: Cancels an active subscription
async fn handle_subscription(
    args: Value,
    procedure: &str,
    client: IrisClient,
    sub_manager: SubscriptionManager,
    webhook_dispatcher: WebhookDispatcher,
    active_subscriptions: ActiveSubscriptions,
) -> Result<CallToolResult, McpErrorData> {
    let action = args
        .get("_action")
        .and_then(|v| v.as_str())
        .unwrap_or("subscribe");

    match action {
        "subscribe" => {
            let webhook_url = args
                .get("_webhook_url")
                .and_then(|v| v.as_str())
                .map(String::from);
            let webhook_secret = args
                .get("_webhook_secret")
                .and_then(|v| v.as_str())
                .map(String::from);

            // Strip meta fields before forwarding to the server.
            let procedure_input = match args.clone() {
                Value::Object(mut map) => {
                    map.remove("_action");
                    map.remove("_webhook_url");
                    map.remove("_webhook_secret");
                    Value::Object(map)
                }
                other => other,
            };

            match client.subscribe(procedure, procedure_input).await {
                Ok((sub_id, mut rx)) => {
                    active_subscriptions
                        .lock()
                        .await
                        .insert(procedure.to_string(), sub_id);

                    if let Some(url) = &webhook_url {
                        webhook_dispatcher
                            .register(procedure, url, webhook_secret.as_deref())
                            .await;
                    }

                    let sub_id_str = sub_id.to_string();
                    sub_manager.create_subscription(sub_id_str.clone()).await;

                    let sub_manager_bg = sub_manager.clone();
                    let webhook_dispatcher_bg = webhook_dispatcher.clone();
                    let procedure_owned = procedure.to_string();

                    tokio::spawn(async move {
                        while let Some(event) = rx.recv().await {
                            sub_manager_bg.push_event(&sub_id_str, event.clone()).await;
                            if let Some((url, secret)) = webhook_dispatcher_bg.get_webhook(&procedure_owned).await {
                                let _ = webhook_dispatcher_bg
                                    .dispatch(&url, secret.as_deref(), event)
                                    .await;
                            }
                        }
                    });

                    let resp = serde_json::json!({
                        "subscription_id": sub_id,
                        "message": format!(
                            "Subscribed to {}. Call again with _action=poll and subscription_id={} to receive buffered events.",
                            procedure, sub_id
                        ),
                    });
                    Ok(CallToolResult::success(vec![Content::text(resp.to_string())]))
                }
                Err(e) => Ok(CallToolResult::error(vec![Content::text(format!(
                    "Error subscribing to {procedure}: {e}"
                ))])),
            }
        }

        "poll" => {
            let sub_id = args
                .get("subscription_id")
                .and_then(|v| v.as_u64())
                .map(|n| n as u32);
            let limit = args.get("limit").and_then(|v| v.as_u64()).unwrap_or(10) as usize;

            match sub_id {
                Some(sub_id) => {
                    let events = sub_manager.poll_events(&sub_id.to_string(), limit).await;
                    let resp = serde_json::json!({ "events": events, "count": events.len() });
                    Ok(CallToolResult::success(vec![Content::text(resp.to_string())]))
                }
                None => Ok(CallToolResult::error(vec![Content::text(
                    "subscription_id is required for _action=poll",
                )])),
            }
        }

        "stop" => {
            let sub_id = args
                .get("subscription_id")
                .and_then(|v| v.as_u64())
                .map(|n| n as u32);

            match sub_id {
                Some(sub_id) => match client.unsubscribe(sub_id).await {
                    Ok(_) => {
                        active_subscriptions.lock().await.remove(procedure);
                        sub_manager.remove_subscription(&sub_id.to_string()).await;
                        webhook_dispatcher.unregister(procedure).await;
                        let resp = serde_json::json!({ "message": format!("Unsubscribed from {procedure}") });
                        Ok(CallToolResult::success(vec![Content::text(resp.to_string())]))
                    }
                    Err(e) => Ok(CallToolResult::error(vec![Content::text(format!(
                        "Error unsubscribing: {e}"
                    ))])),
                },
                None => Ok(CallToolResult::error(vec![Content::text(
                    "subscription_id is required for _action=stop",
                )])),
            }
        }

        _ => Ok(CallToolResult::error(vec![Content::text(format!(
            "Unknown _action: '{action}'. Valid values: subscribe (default), poll, stop"
        ))])),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_alert_id() {
        let id1 = next_alert_id();
        let id2 = next_alert_id();
        assert_ne!(id1, id2);
        assert!(id2 > id1);
    }

    #[test]
    fn test_webhook_dispatcher_new() {
        let dispatcher = WebhookDispatcher::new();
        // Just verify it creates
        let _ = dispatcher;
    }

    #[test]
    fn test_alert_delivery_deserialize() {
        let webhook_json = r#"{"type":"webhook","url":"https://example.com","secret":"abc"}"#;
        let delivery: AlertDelivery = serde_json::from_str(webhook_json).unwrap();
        match delivery {
            AlertDelivery::Webhook { url, secret } => {
                assert_eq!(url, "https://example.com");
                assert_eq!(secret, Some("abc".to_string()));
            }
            _ => panic!("Expected Webhook variant"),
        }
    }
}
