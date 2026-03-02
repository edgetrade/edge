use crate::client::IrisClient;
use rmcp::{Server, ToolError, tool, tool_handler};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
struct TradeInput {
    action: String,
    wallet: Option<String>,
    #[serde(flatten)]
    extra: Value,
}

pub fn register(server: &Server, client: IrisClient) -> Result<(), Box<dyn std::error::Error>> {
    server.add_tool(
        tool!(
            name = "trade",
            description = "Place limit orders, manage entry/exit strategies, estimate price impact, and execute swaps. Actions: place, list, get, cancel, cancel_all, extend, create_entry_strategy, create_exit_strategy, list_strategies, apply_strategy, update_strategy, delete_strategy, impact, build, submit. See: https://docs.edge.trade/agents/tools/trade",
            input_schema = {
                "type": "object",
                "properties": {
                    "action": {
                        "type": "string",
                        "enum": ["place", "list", "get", "cancel", "cancel_all", "extend", "create_entry_strategy", "create_exit_strategy", "list_strategies", "apply_strategy", "update_strategy", "delete_strategy", "impact", "build", "submit"],
                        "description": "Action to perform"
                    },
                    "wallet": {
                        "type": "string",
                        "description": "Wallet address (required for most actions)"
                    },
                    "chain_id": {
                        "type": "string",
                        "description": "Chain ID"
                    },
                    "pair_address": {
                        "type": "string",
                        "description": "Pair address for orders"
                    },
                    "direction": {
                        "type": "string",
                        "enum": ["buy", "sell"],
                        "description": "Order direction"
                    },
                    "trigger_price": {
                        "type": "number",
                        "description": "Trigger price for limit order"
                    },
                    "amount_native": {
                        "type": "number",
                        "description": "Amount in native token"
                    },
                    "slippage": {
                        "type": "number",
                        "description": "Slippage tolerance (0-100)"
                    },
                    "order_id": {
                        "type": "string",
                        "description": "Order ID for get/cancel/extend"
                    },
                    "strategy_id": {
                        "type": "string",
                        "description": "Strategy ID"
                    },
                    "name": {
                        "type": "string",
                        "description": "Strategy name"
                    },
                    "steps": {
                        "type": "array",
                        "description": "Strategy steps"
                    },
                    "token_address": {
                        "type": "string",
                        "description": "Token address"
                    }
                },
                "required": ["action"]
            }
        ),
        tool_handler!(client, handle_trade),
    )?;
    Ok(())
}

async fn handle_trade(client: IrisClient, input: Value) -> Result<Value, ToolError> {
    let params: TradeInput = serde_json::from_value(input).map_err(|e| ToolError::InvalidInput(e.to_string()))?;

    match params.action.as_str() {
        "place" => {
            let wallet = params
                .wallet
                .ok_or_else(|| ToolError::InvalidInput("wallet required for place action".to_string()))?;

            let mut query_input = serde_json::json!({
                "walletAddress": wallet,
            });

            if let Value::Object(ref extra_obj) = params.extra {
                if let Some(obj) = query_input.as_object_mut() {
                    for (k, v) in extra_obj {
                        let key = match k.as_str() {
                            "chain_id" => "chainId",
                            "pair_address" => "pairAddress",
                            "trigger_price" => "triggerPrice",
                            "amount_native" => "amountNative",
                            _ => k.as_str(),
                        };
                        obj.insert(key.to_string(), v.clone());
                    }
                }
            }

            client
                .mutation("orders.place", query_input)
                .await
                .map_err(|e| ToolError::ExecutionError(e.to_string()))
        }
        "list" => {
            let wallet = params
                .wallet
                .ok_or_else(|| ToolError::InvalidInput("wallet required for list action".to_string()))?;

            let query_input = serde_json::json!({
                "walletAddress": wallet,
            });

            client
                .query("orders.list", query_input)
                .await
                .map_err(|e| ToolError::ExecutionError(e.to_string()))
        }
        "get" => {
            let order_id = if let Value::Object(ref extra_obj) = params.extra {
                extra_obj
                    .get("order_id")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| ToolError::InvalidInput("order_id required".to_string()))?
            } else {
                return Err(ToolError::InvalidInput("order_id required".to_string()));
            };

            let query_input = serde_json::json!({
                "orderId": order_id,
            });

            client
                .query("orders.get", query_input)
                .await
                .map_err(|e| ToolError::ExecutionError(e.to_string()))
        }
        "cancel" => {
            let order_id = if let Value::Object(ref extra_obj) = params.extra {
                extra_obj
                    .get("order_id")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| ToolError::InvalidInput("order_id required".to_string()))?
            } else {
                return Err(ToolError::InvalidInput("order_id required".to_string()));
            };

            let query_input = serde_json::json!({
                "orderId": order_id,
            });

            client
                .mutation("orders.cancel", query_input)
                .await
                .map_err(|e| ToolError::ExecutionError(e.to_string()))
        }
        "cancel_all" => {
            let wallet = params
                .wallet
                .ok_or_else(|| ToolError::InvalidInput("wallet required for cancel_all action".to_string()))?;

            let query_input = serde_json::json!({
                "walletAddress": wallet,
            });

            client
                .mutation("orders.cancelAll", query_input)
                .await
                .map_err(|e| ToolError::ExecutionError(e.to_string()))
        }
        "extend" => {
            let mut query_input = serde_json::json!({});

            if let Value::Object(ref extra_obj) = params.extra {
                if let Some(obj) = query_input.as_object_mut() {
                    for (k, v) in extra_obj {
                        let key = match k.as_str() {
                            "order_id" => "orderId",
                            "expires_at" => "expiresAt",
                            _ => k.as_str(),
                        };
                        obj.insert(key.to_string(), v.clone());
                    }
                }
            }

            client
                .mutation("orders.extend", query_input)
                .await
                .map_err(|e| ToolError::ExecutionError(e.to_string()))
        }
        "create_entry_strategy" => {
            let mut query_input = serde_json::json!({});

            if let Value::Object(ref extra_obj) = params.extra {
                if let Some(obj) = query_input.as_object_mut() {
                    for (k, v) in extra_obj {
                        obj.insert(k.clone(), v.clone());
                    }
                }
            }

            client
                .mutation("orders.createEntryStrategy", query_input)
                .await
                .map_err(|e| ToolError::ExecutionError(e.to_string()))
        }
        "create_exit_strategy" => {
            let mut query_input = serde_json::json!({});

            if let Value::Object(ref extra_obj) = params.extra {
                if let Some(obj) = query_input.as_object_mut() {
                    for (k, v) in extra_obj {
                        obj.insert(k.clone(), v.clone());
                    }
                }
            }

            client
                .mutation("orders.createExitStrategy", query_input)
                .await
                .map_err(|e| ToolError::ExecutionError(e.to_string()))
        }
        "list_strategies" => {
            let wallet = params
                .wallet
                .ok_or_else(|| ToolError::InvalidInput("wallet required for list_strategies action".to_string()))?;

            let query_input = serde_json::json!({
                "walletAddress": wallet,
            });

            client
                .query("orders.listEntryStrategies", query_input)
                .await
                .map_err(|e| ToolError::ExecutionError(e.to_string()))
        }
        "apply_strategy" => {
            let mut query_input = serde_json::json!({});

            if let Value::Object(ref extra_obj) = params.extra {
                if let Some(obj) = query_input.as_object_mut() {
                    for (k, v) in extra_obj {
                        let key = match k.as_str() {
                            "strategy_id" => "strategyId",
                            "token_address" => "tokenAddress",
                            "chain_id" => "chainId",
                            _ => k.as_str(),
                        };
                        obj.insert(key.to_string(), v.clone());
                    }
                }
            }

            client
                .mutation("orders.applyEntryStrategy", query_input)
                .await
                .map_err(|e| ToolError::ExecutionError(e.to_string()))
        }
        "update_strategy" => {
            let mut query_input = serde_json::json!({});

            if let Value::Object(ref extra_obj) = params.extra {
                if let Some(obj) = query_input.as_object_mut() {
                    for (k, v) in extra_obj {
                        let key = match k.as_str() {
                            "strategy_id" => "strategyId",
                            _ => k.as_str(),
                        };
                        obj.insert(key.to_string(), v.clone());
                    }
                }
            }

            client
                .mutation("orders.updateEntryStrategy", query_input)
                .await
                .map_err(|e| ToolError::ExecutionError(e.to_string()))
        }
        "delete_strategy" => {
            let strategy_id = if let Value::Object(ref extra_obj) = params.extra {
                extra_obj
                    .get("strategy_id")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| ToolError::InvalidInput("strategy_id required".to_string()))?
            } else {
                return Err(ToolError::InvalidInput("strategy_id required".to_string()));
            };

            let query_input = serde_json::json!({
                "strategyId": strategy_id,
            });

            client
                .mutation("orders.removeEntryStrategy", query_input)
                .await
                .map_err(|e| ToolError::ExecutionError(e.to_string()))
        }
        "impact" => {
            let mut query_input = serde_json::json!({});

            if let Value::Object(ref extra_obj) = params.extra {
                if let Some(obj) = query_input.as_object_mut() {
                    for (k, v) in extra_obj {
                        let key = match k.as_str() {
                            "chain_id" => "chainId",
                            "token_address" => "tokenAddress",
                            "amount_native" => "amountNative",
                            _ => k.as_str(),
                        };
                        obj.insert(key.to_string(), v.clone());
                    }
                }
            }

            client
                .query("intelligence.estimateImpact", query_input)
                .await
                .map_err(|e| ToolError::ExecutionError(e.to_string()))
        }
        "build" => Err(ToolError::ExecutionError(
            "Execution (build/submit) coming soon. See: https://docs.edge.trade/agents/tools/trade#execution"
                .to_string(),
        )),
        "submit" => Err(ToolError::ExecutionError(
            "Execution (build/submit) coming soon. See: https://docs.edge.trade/agents/tools/trade#execution"
                .to_string(),
        )),
        _ => Err(ToolError::InvalidInput(format!(
            "Unknown action: {}. See: https://docs.edge.trade/agents/tools/trade",
            params.action
        ))),
    }
}
