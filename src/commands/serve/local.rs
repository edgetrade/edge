use std::sync::Arc;

use rmcp::{
    ErrorData as McpError,
    model::{CallToolResult, Content},
};
use serde_json::Value;
use tokio::sync::RwLock;

use crate::client::IrisClient;

use crate::commands::subscribe::alerts::AlertRegistry;
use crate::manifest::McpManifest;

use super::alerts::{handle_list_alerts, handle_register_alert, handle_unregister_alert};

/// Routes locally-handled `agent` actions before they reach the TypeScript server.
pub async fn handle_local_action(
    action_name: &str,
    data: Value,
    client: IrisClient,
    manifest: Arc<RwLock<McpManifest>>,
    alert_registry: AlertRegistry,
    http_client: reqwest::Client,
) -> Result<CallToolResult, McpError> {
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
