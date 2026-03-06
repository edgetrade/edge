use std::sync::Arc;

use rmcp::{
    ErrorData as McpError, ServerHandler, ServiceExt,
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
use tokio::sync::Mutex;

use crate::client::IrisClient;
use crate::manifest::McpManifest;
use crate::subscriptions::{SubscriptionManager, WebhookDispatcher};

/// Maps procedure → subscription id for active SSE subscriptions.
type ActiveSubscriptions = Arc<Mutex<std::collections::HashMap<String, u32>>>;

#[derive(Clone)]
pub struct EdgeServer {
    client: IrisClient,
    manifest: Arc<McpManifest>,
    subscription_manager: SubscriptionManager,
    webhook_dispatcher: WebhookDispatcher,
    active_subscriptions: ActiveSubscriptions,
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

    fn list_tools(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> impl Future<Output = Result<ListToolsResult, McpError>> + Send + '_ {
        let tools = self
            .manifest
            .tools
            .iter()
            .map(|def| {
                let schema = match serde_json::from_value::<serde_json::Map<String, Value>>(def.input_schema.clone()) {
                    Ok(m) => m,
                    Err(_) => serde_json::Map::new(),
                };
                Tool::new(def.name.clone(), def.description.clone(), Arc::new(schema))
            })
            .collect::<Vec<_>>();
        std::future::ready(Ok(ListToolsResult::with_all_items(tools)))
    }

    fn call_tool(
        &self,
        request: CallToolRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> impl Future<Output = Result<CallToolResult, McpError>> + Send + '_ {
        let name = request.name.to_string();
        let args = request.arguments.map(Value::Object).unwrap_or_default();
        let tool = self.manifest.tools.iter().find(|t| t.name == name).cloned();
        let client = self.client.clone();
        let sub_manager = self.subscription_manager.clone();
        let webhook_dispatcher = self.webhook_dispatcher.clone();
        let active_subscriptions = self.active_subscriptions.clone();

        async move {
            let tool = match tool {
                Some(t) => t,
                None => {
                    return Ok(CallToolResult::error(vec![Content::text(format!(
                        "Unknown tool: {name}"
                    ))]));
                }
            };

            if tool.kind == "subscription" {
                return handle_subscription(
                    args,
                    &tool.procedure,
                    client,
                    sub_manager,
                    webhook_dispatcher,
                    active_subscriptions,
                )
                .await;
            }

            match client.query(&tool.procedure, args).await {
                Ok(result) => Ok(CallToolResult::success(vec![Content::text(result.to_string())])),
                Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
            }
        }
    }

    fn list_resources(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> impl Future<Output = Result<ListResourcesResult, McpError>> + Send + '_ {
        let resources = self
            .manifest
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
        std::future::ready(Ok(ListResourcesResult::with_all_items(resources)))
    }

    fn read_resource(
        &self,
        request: ReadResourceRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> impl Future<Output = Result<ReadResourceResult, McpError>> + Send + '_ {
        let uri = request.uri;
        let resource = self
            .manifest
            .resources
            .iter()
            .find(|r| r.uri == uri)
            .cloned();
        async move {
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
                None => Err(McpError::resource_not_found(format!("Resource not found: {uri}"), None)),
            }
        }
    }

    fn list_prompts(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> impl Future<Output = Result<ListPromptsResult, McpError>> + Send + '_ {
        let prompts = self
            .manifest
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
        std::future::ready(Ok(ListPromptsResult::with_all_items(prompts)))
    }

    fn get_prompt(
        &self,
        request: GetPromptRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> impl Future<Output = Result<GetPromptResult, McpError>> + Send + '_ {
        let name = request.name;
        let prompt = self
            .manifest
            .prompts
            .iter()
            .find(|p| p.name == name)
            .cloned();
        async move {
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
                None => Err(McpError::invalid_params(format!("Prompt not found: {name}"), None)),
            }
        }
    }
}

impl EdgeServer {
    pub async fn new(
        url: &str,
        api_key: &str,
        manifest: McpManifest,
        verbose: bool,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let client = IrisClient::connect(url, api_key, verbose).await?;
        Ok(Self {
            client,
            manifest: Arc::new(manifest),
            subscription_manager: SubscriptionManager::new(),
            webhook_dispatcher: WebhookDispatcher::new(),
            active_subscriptions: Arc::new(Mutex::new(std::collections::HashMap::new())),
        })
    }

    pub async fn serve_stdio(self) -> Result<(), Box<dyn std::error::Error>> {
        let service = self.serve(stdio()).await?;
        service.waiting().await?;
        Ok(())
    }

    pub async fn serve_http(self, host: &str, port: u16) -> Result<(), Box<dyn std::error::Error>> {
        use rmcp::transport::streamable_http_server::session::local::LocalSessionManager;
        use rmcp::transport::streamable_http_server::{StreamableHttpServerConfig, StreamableHttpService};

        let addr = format!("{}:{}", host, port);
        eprintln!("Starting HTTP server on http://{}/mcp", addr);

        let config = StreamableHttpServerConfig {
            stateful_mode: false,
            json_response: true,
            ..Default::default()
        };
        let session_manager = Arc::new(LocalSessionManager::default());
        let service = StreamableHttpService::new(move || Ok(self.clone()), session_manager, config);
        let router = axum::Router::new().nest_service("/mcp", service);

        let listener = tokio::net::TcpListener::bind(&addr).await?;
        axum::serve(listener, router).await?;
        Ok(())
    }
}

/// Handles a tool call for a subscription-kind tool.
///
/// Subscription tools support three actions via the reserved `_action` argument:
///
/// - `subscribe` (default): Starts a new SSE subscription to `procedure`. All other
///   arguments (minus `_action`, `_webhook_url`, `_webhook_secret`) are forwarded as
///   the procedure input. Returns `{ subscription_id, message }`.
///
/// - `poll`: Drains buffered events. Requires `subscription_id` (returned from subscribe).
///   Accepts an optional `limit` (default 10).
///
/// - `stop`: Cancels an active subscription. Requires `subscription_id`.
async fn handle_subscription(
    args: Value,
    procedure: &str,
    client: IrisClient,
    sub_manager: SubscriptionManager,
    webhook_dispatcher: WebhookDispatcher,
    active_subscriptions: ActiveSubscriptions,
) -> Result<CallToolResult, McpError> {
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
