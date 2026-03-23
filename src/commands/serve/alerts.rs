use std::sync::Arc;

use rmcp::{
    ErrorData as McpError,
    model::{CallToolResult, Content},
};
use serde_json::Value;
use tokio::sync::{Mutex, RwLock};

use crate::client::IrisClient;
use crate::commands::subscribe::{SubscriptionManager, WebhookDispatcher};
use crate::messages;

use crate::commands::subscribe::alerts::{AlertDelivery, AlertRegistration, AlertRegistry, next_alert_id};
use crate::manifest::McpManifest;

/// Maps procedure → subscription id for active SSE subscriptions.
pub type ActiveSubscriptions = Arc<Mutex<std::collections::HashMap<String, u32>>>;

pub async fn handle_register_alert(
    data: Value,
    client: IrisClient,
    manifest: Arc<RwLock<McpManifest>>,
    alert_registry: AlertRegistry,
    http_client: reqwest::Client,
) -> Result<CallToolResult, McpError> {
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
    let delivery: AlertDelivery = match serde_json::from_value(delivery_value) {
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

    let sub_id = match client
        .subscribe_for_dispatch(
            &procedure,
            input.clone(),
            crate::client::DispatchParams {
                alert_id,
                alert_name: alert_name.clone(),
                delivery: delivery.clone(),
                alert_registry: alert_registry.clone(),
                http_client,
            },
        )
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

pub async fn handle_list_alerts(alert_registry: AlertRegistry) -> Result<CallToolResult, McpError> {
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

fn delivery_summary(delivery: &AlertDelivery) -> String {
    match delivery {
        AlertDelivery::Webhook { url, .. } => format!("webhook: {}", url),
        AlertDelivery::Redis { channel, .. } => format!("redis: {}", channel),
        AlertDelivery::Telegram { .. } => "telegram".to_string(),
    }
}

pub async fn handle_unregister_alert(
    data: Value,
    client: IrisClient,
    alert_registry: AlertRegistry,
) -> Result<CallToolResult, McpError> {
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
pub async fn handle_subscription(
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
