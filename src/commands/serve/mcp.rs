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
use tokio::sync::{Mutex, RwLock};

use crate::client::IrisClient;
use crate::commands::subscribe::alerts::{AlertRegistry, new_alert_registry};
use crate::commands::subscribe::{SubscriptionManager, WebhookDispatcher};
use crate::manifest::{McpManifest, inject_local_agent_actions, inject_local_resources};
use crate::messages;

use super::alerts::ActiveSubscriptions;
use super::alerts::handle_subscription;
use super::local::handle_local_action;

#[derive(Clone)]
pub struct EdgeServer {
    client: IrisClient,
    manifest: Arc<RwLock<McpManifest>>,
    subscription_manager: SubscriptionManager,
    webhook_dispatcher: WebhookDispatcher,
    active_subscriptions: ActiveSubscriptions,
    alert_registry: AlertRegistry,
    http_client: reqwest::Client,
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
    ) -> Result<ListToolsResult, McpError> {
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
    ) -> impl Future<Output = Result<CallToolResult, McpError>> + Send + '_ {
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

            match client
                .query::<Value>(&action_def.procedure, data.clone())
                .await
            {
                Ok(result) => Ok(CallToolResult::success(vec![Content::text(result.to_string())])),
                Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
            }
        }
    }

    async fn list_resources(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListResourcesResult, McpError> {
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
    ) -> impl Future<Output = Result<ReadResourceResult, McpError>> + Send + '_ {
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
                None => Err(McpError::resource_not_found(format!("Resource not found: {uri}"), None)),
            }
        }
    }

    async fn list_prompts(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListPromptsResult, McpError> {
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
    ) -> impl Future<Output = Result<GetPromptResult, McpError>> + Send + '_ {
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
                None => Err(McpError::invalid_params(format!("Prompt not found: {name}"), None)),
            }
        }
    }
}

impl EdgeServer {
    pub async fn new(
        client: IrisClient,
        manifest: Arc<RwLock<McpManifest>>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
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
            active_subscriptions: Arc::new(Mutex::new(std::collections::HashMap::new())),
            alert_registry: new_alert_registry(),
            http_client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(10))
                .build()
                .unwrap(),
        })
    }

    pub async fn serve_stdio(self) -> Result<(), Box<dyn std::error::Error>> {
        let service = self.serve(stdio()).await?;
        service.waiting().await?;
        Ok(())
    }

    pub async fn serve_http(self, host: &str, port: u16, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        use rmcp::transport::streamable_http_server::session::local::LocalSessionManager;
        use rmcp::transport::streamable_http_server::{StreamableHttpServerConfig, StreamableHttpService};

        let addr = format!("{}:{}", host, port);
        let path = if path.starts_with('/') {
            path.to_string()
        } else {
            format!("/{}", path)
        };
        messages::error::http_server_starting(&addr, &path);

        let config = StreamableHttpServerConfig {
            stateful_mode: false,
            ..Default::default()
        };
        let session_manager = Arc::new(LocalSessionManager::default());
        let service = StreamableHttpService::new(move || Ok(self.clone()), session_manager, config);
        let router = axum::Router::new().nest_service(&path, service);

        let listener = tokio::net::TcpListener::bind(&addr).await?;
        axum::serve(listener, router).await?;
        Ok(())
    }
}
