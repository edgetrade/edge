use crate::client::IrisClient;
use rmcp::{Server, ToolError, tool, tool_handler};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
struct InspectInput {
    chain_id: String,
    address: String,
    view: String,
    #[serde(flatten)]
    extra: Value,
}

pub fn register(server: &Server, client: IrisClient) -> Result<(), Box<dyn std::error::Error>> {
    server.add_tool(
        tool!(
            name = "inspect",
            description = "Inspect tokens and pairs with multiple views (token_overview, token_holders, token_analytics, graduation, pair_overview, pair_metrics, pair_candles, pair_swaps). See: https://docs.edge.trade/agents/tools/inspect",
            input_schema = {
                "type": "object",
                "properties": {
                    "chain_id": {
                        "type": "string",
                        "description": "Chain ID (e.g., '8453' for Base)"
                    },
                    "address": {
                        "type": "string",
                        "description": "Token or pair address"
                    },
                    "view": {
                        "type": "string",
                        "enum": ["token_overview", "token_holders", "token_analytics", "graduation", "pair_overview", "pair_metrics", "pair_candles", "pair_swaps"],
                        "description": "View type to retrieve"
                    },
                    "interval": {
                        "type": "string",
                        "description": "For pair_candles: 1s, 10s, 1m, 5m, 15m, 1hr, 4hr, 6hr, 1day"
                    },
                    "price_type": {
                        "type": "string",
                        "description": "For pair_candles: token_usd, token_native, mcap_usd, mcap_native"
                    },
                    "from": {
                        "type": "integer",
                        "description": "For pair_candles: start timestamp"
                    },
                    "to": {
                        "type": "integer",
                        "description": "For pair_candles: end timestamp"
                    },
                    "count_back": {
                        "type": "integer",
                        "description": "For pair_candles: number of candles to retrieve"
                    },
                    "cursor": {
                        "type": "string",
                        "description": "For pair_swaps: pagination cursor"
                    },
                    "limit": {
                        "type": "integer",
                        "description": "For pair_swaps: max results"
                    }
                },
                "required": ["chain_id", "address", "view"]
            }
        ),
        tool_handler!(client, handle_inspect),
    )?;
    Ok(())
}

async fn handle_inspect(client: IrisClient, input: Value) -> Result<Value, ToolError> {
    let params: InspectInput = serde_json::from_value(input).map_err(|e| ToolError::InvalidInput(e.to_string()))?;

    match params.view.as_str() {
        "token_overview" => {
            let query_input = serde_json::json!({
                "chainId": params.chain_id,
                "tokenAddress": params.address,
            });
            client
                .query("market.getToken", query_input)
                .await
                .map_err(|e| ToolError::ExecutionError(e.to_string()))
        }
        "token_holders" => {
            let query_input = serde_json::json!({
                "chainId": params.chain_id,
                "tokenAddress": params.address,
            });
            client
                .query("portfolio.getTopHolders", query_input)
                .await
                .map_err(|e| ToolError::ExecutionError(e.to_string()))
        }
        "token_analytics" => {
            let query_input = serde_json::json!({
                "chainId": params.chain_id,
                "tokenAddress": params.address,
            });
            client
                .query("portfolio.getTopTraders", query_input)
                .await
                .map_err(|e| ToolError::ExecutionError(e.to_string()))
        }
        "graduation" => {
            let query_input = serde_json::json!({
                "chainId": params.chain_id,
                "pairAddress": params.address,
            });
            client
                .query("market.getPairGraduation", query_input)
                .await
                .map_err(|e| ToolError::ExecutionError(e.to_string()))
        }
        "pair_overview" => {
            let query_input = serde_json::json!({
                "chainId": params.chain_id,
                "pairAddress": params.address,
            });
            client
                .query("market.getPairDetailed", query_input)
                .await
                .map_err(|e| ToolError::ExecutionError(e.to_string()))
        }
        "pair_metrics" => {
            let query_input = serde_json::json!({
                "chainId": params.chain_id,
                "pairAddress": params.address,
            });
            client
                .query("market.getPairMetrics", query_input)
                .await
                .map_err(|e| ToolError::ExecutionError(e.to_string()))
        }
        "pair_candles" => {
            let mut query_input = serde_json::json!({
                "chainId": params.chain_id,
                "pairAddress": params.address,
            });

            if let Value::Object(ref extra_obj) = params.extra {
                if let Some(obj) = query_input.as_object_mut() {
                    for (k, v) in extra_obj {
                        obj.insert(k.clone(), v.clone());
                    }
                }
            }

            client
                .query("market.getPairCandles", query_input)
                .await
                .map_err(|e| ToolError::ExecutionError(e.to_string()))
        }
        "pair_swaps" => {
            let mut query_input = serde_json::json!({
                "chainId": params.chain_id,
                "pairAddress": params.address,
            });

            if let Value::Object(ref extra_obj) = params.extra {
                if let Some(obj) = query_input.as_object_mut() {
                    for (k, v) in extra_obj {
                        obj.insert(k.clone(), v.clone());
                    }
                }
            }

            client
                .query("market.getSwaps", query_input)
                .await
                .map_err(|e| ToolError::ExecutionError(e.to_string()))
        }
        _ => Err(ToolError::InvalidInput(format!(
            "Unknown view: {}. See: https://docs.edge.trade/agents/tools/inspect",
            params.view
        ))),
    }
}
