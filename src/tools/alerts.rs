use crate::client::IrisClient;
use crate::subscriptions::{SubscriptionManager, WebhookDispatcher};
use rmcp::{Server, ToolError, tool, tool_handler};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Serialize, Deserialize)]
struct AlertsInput {
    action: String,
    #[serde(flatten)]
    extra: Value,
}

struct AlertsState {
    client: IrisClient,
    subscription_manager: SubscriptionManager,
    webhook_dispatcher: WebhookDispatcher,
    active_subscriptions: Arc<Mutex<std::collections::HashMap<String, u32>>>,
}

pub fn register(server: &Server, client: IrisClient) -> Result<(), Box<dyn std::error::Error>> {
    let state = Arc::new(AlertsState {
        client: client.clone(),
        subscription_manager: SubscriptionManager::new(),
        webhook_dispatcher: WebhookDispatcher::new(),
        active_subscriptions: Arc::new(Mutex::new(std::collections::HashMap::new())),
    });

    server.add_tool(
        tool!(
            name = "alerts",
            description = "Subscribe to real-time alerts (price, order_fill, graduation, memescope, pair_metrics, wallet_swaps, native_price, portfolio_updates). Actions: subscribe, poll, unsubscribe. See: https://docs.edge.trade/agents/tools/alerts",
            input_schema = {
                "type": "object",
                "properties": {
                    "action": {
                        "type": "string",
                        "enum": ["subscribe", "poll", "unsubscribe"],
                        "description": "Action to perform"
                    },
                    "alert_type": {
                        "type": "string",
                        "enum": ["price", "order_fill", "graduation", "memescope", "pair_metrics", "wallet_swaps", "native_price", "portfolio_updates"],
                        "description": "Type of alert (required for subscribe)"
                    },
                    "subscription_id": {
                        "type": "string",
                        "description": "Subscription ID (required for poll/unsubscribe)"
                    },
                    "chain_id": {
                        "type": "string",
                        "description": "Chain ID"
                    },
                    "address": {
                        "type": "string",
                        "description": "Token or pair address"
                    },
                    "wallet": {
                        "type": "string",
                        "description": "Wallet address"
                    },
                    "wallet_addresses": {
                        "type": "array",
                        "items": {"type": "string"},
                        "description": "Multiple wallet addresses"
                    },
                    "threshold": {
                        "type": "number",
                        "description": "Price threshold"
                    },
                    "direction": {
                        "type": "string",
                        "enum": ["above", "below"],
                        "description": "Price direction"
                    },
                    "interval": {
                        "type": "string",
                        "description": "Interval for pair_metrics"
                    },
                    "filters": {
                        "type": "object",
                        "description": "Filters for memescope"
                    },
                    "webhook_url": {
                        "type": "string",
                        "description": "Webhook URL for HTTP delivery"
                    },
                    "webhook_secret": {
                        "type": "string",
                        "description": "Webhook secret for HMAC signature"
                    },
                    "limit": {
                        "type": "integer",
                        "description": "Max events to poll (default 20)"
                    }
                },
                "required": ["action"]
            }
        ),
        tool_handler!(state, handle_alerts),
    )?;
    Ok(())
}

async fn handle_alerts(state: Arc<AlertsState>, input: Value) -> Result<Value, ToolError> {
    let params: AlertsInput = serde_json::from_value(input).map_err(|e| ToolError::InvalidInput(e.to_string()))?;

    match params.action.as_str() {
        "subscribe" => handle_subscribe(state, params.extra).await,
        "poll" => handle_poll(state, params.extra).await,
        "unsubscribe" => handle_unsubscribe(state, params.extra).await,
        _ => Err(ToolError::InvalidInput(format!(
            "Unknown action: {}. See: https://docs.edge.trade/agents/tools/alerts",
            params.action
        ))),
    }
}

async fn handle_subscribe(state: Arc<AlertsState>, extra: Value) -> Result<Value, ToolError> {
    let alert_type = extra
        .get("alert_type")
        .and_then(|v| v.as_str())
        .ok_or_else(|| ToolError::InvalidInput("alert_type required for subscribe".to_string()))?;

    let subscription_path = match alert_type {
        "price" => "alerts.onPairState",
        "order_fill" => "alerts.onOrderUpdates",
        "graduation" => "alerts.onPairState",
        "memescope" => "alerts.onMemescope",
        "pair_metrics" => "alerts.onPairMetrics",
        "wallet_swaps" => "alerts.onWalletSwaps",
        "native_price" => "alerts.onNativePrice",
        "portfolio_updates" => "alerts.onPortfolioUpdates",
        _ => return Err(ToolError::InvalidInput(format!("Unknown alert_type: {}", alert_type))),
    };

    let mut subscription_input = serde_json::json!({});
    if let Value::Object(ref extra_obj) = extra {
        if let Some(obj) = subscription_input.as_object_mut() {
            for (k, v) in extra_obj {
                if k != "action" && k != "alert_type" && k != "webhook_url" && k != "webhook_secret" {
                    let key = match k.as_str() {
                        "chain_id" => "chainId",
                        "wallet_addresses" => "walletAddresses",
                        _ => k.as_str(),
                    };
                    obj.insert(key.to_string(), v.clone());
                }
            }
        }
    }

    let (ws_id, mut rx) = state
        .client
        .subscribe(subscription_path, subscription_input)
        .await
        .map_err(|e| ToolError::ExecutionError(e.to_string()))?;

    let subscription_id = format!("sub_{}", ws_id);
    state
        .subscription_manager
        .create_subscription(subscription_id.clone())
        .await;
    state
        .active_subscriptions
        .lock()
        .await
        .insert(subscription_id.clone(), ws_id);

    let webhook_url = extra
        .get("webhook_url")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    let webhook_secret = extra
        .get("webhook_secret")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    let manager = state.subscription_manager.clone();
    let dispatcher = state.webhook_dispatcher.clone();
    let sub_id = subscription_id.clone();

    tokio::spawn(async move {
        while let Some(event) = rx.recv().await {
            manager.push_event(&sub_id, event.clone()).await;

            if let Some(ref url) = webhook_url {
                let payload = serde_json::json!({
                    "subscription_id": sub_id,
                    "alert_type": alert_type,
                    "event": event,
                    "timestamp": chrono::Utc::now().timestamp(),
                });

                let url = url.clone();
                let secret = webhook_secret.clone();
                let dispatcher = dispatcher.clone();

                tokio::spawn(async move {
                    let _ = dispatcher.dispatch(&url, secret.as_deref(), payload).await;
                });
            }
        }
    });

    Ok(serde_json::json!({
        "subscription_id": subscription_id
    }))
}

async fn handle_poll(state: Arc<AlertsState>, extra: Value) -> Result<Value, ToolError> {
    let subscription_id = extra
        .get("subscription_id")
        .and_then(|v| v.as_str())
        .ok_or_else(|| ToolError::InvalidInput("subscription_id required for poll".to_string()))?;

    let limit = extra.get("limit").and_then(|v| v.as_u64()).unwrap_or(20) as usize;

    let events = state
        .subscription_manager
        .poll_events(subscription_id, limit)
        .await;

    Ok(serde_json::json!({
        "events": events
    }))
}

async fn handle_unsubscribe(state: Arc<AlertsState>, extra: Value) -> Result<Value, ToolError> {
    let subscription_id = extra
        .get("subscription_id")
        .and_then(|v| v.as_str())
        .ok_or_else(|| ToolError::InvalidInput("subscription_id required for unsubscribe".to_string()))?;

    let mut subs = state.active_subscriptions.lock().await;
    if let Some(ws_id) = subs.remove(subscription_id) {
        state
            .client
            .unsubscribe(ws_id)
            .await
            .map_err(|e| ToolError::ExecutionError(e.to_string()))?;
        state
            .subscription_manager
            .remove_subscription(subscription_id)
            .await;
    }

    Ok(serde_json::json!({
        "success": true
    }))
}
