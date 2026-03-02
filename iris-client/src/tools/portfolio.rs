use crate::client::IrisClient;
use rmcp::{Server, ToolError, tool, tool_handler};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
struct PortfolioInput {
    address: String,
    view: String,
    #[serde(flatten)]
    extra: Value,
}

pub fn register(server: &Server, client: IrisClient) -> Result<(), Box<dyn std::error::Error>> {
    server.add_tool(
        tool!(
            name = "portfolio",
            description = "View wallet holdings, history, transactions, and analytics (holdings, summary, history, balances, scan, swaps, tx). See: https://docs.edge.trade/agents/tools/portfolio",
            input_schema = {
                "type": "object",
                "properties": {
                    "address": {
                        "type": "string",
                        "description": "Wallet address"
                    },
                    "view": {
                        "type": "string",
                        "enum": ["holdings", "summary", "history", "balances", "scan", "swaps", "tx"],
                        "description": "View type to retrieve"
                    },
                    "chain_id": {
                        "type": "string",
                        "description": "Chain ID (required for most views)"
                    },
                    "cursor": {
                        "type": "string",
                        "description": "Pagination cursor for holdings"
                    },
                    "sort_by": {
                        "type": "string",
                        "description": "Sort field for holdings"
                    },
                    "by": {
                        "type": "string",
                        "enum": ["wallet", "token"],
                        "description": "For swaps view: filter by wallet or token"
                    },
                    "long_term": {
                        "type": "boolean",
                        "description": "For swaps view: use long-term storage (all history, higher latency)"
                    },
                    "tx_hash": {
                        "type": "string",
                        "description": "For tx view: transaction hash"
                    }
                },
                "required": ["address", "view"]
            }
        ),
        tool_handler!(client, handle_portfolio),
    )?;
    Ok(())
}

async fn handle_portfolio(client: IrisClient, input: Value) -> Result<Value, ToolError> {
    let params: PortfolioInput = serde_json::from_value(input).map_err(|e| ToolError::InvalidInput(e.to_string()))?;

    match params.view.as_str() {
        "holdings" => {
            let mut query_input = serde_json::json!({
                "walletAddress": params.address,
            });

            if let Value::Object(ref extra_obj) = params.extra {
                if let Some(obj) = query_input.as_object_mut() {
                    if let Some(chain_id) = extra_obj.get("chain_id") {
                        obj.insert("chainId".to_string(), chain_id.clone());
                    }
                    if let Some(cursor) = extra_obj.get("cursor") {
                        obj.insert("cursor".to_string(), cursor.clone());
                    }
                    if let Some(sort_by) = extra_obj.get("sort_by") {
                        obj.insert("sortBy".to_string(), sort_by.clone());
                    }
                }
            }

            client
                .query("portfolio.getHoldings", query_input)
                .await
                .map_err(|e| ToolError::ExecutionError(e.to_string()))
        }
        "summary" => {
            let query_input = serde_json::json!({
                "walletAddress": params.address,
            });
            client
                .query("portfolio.getSummary", query_input)
                .await
                .map_err(|e| ToolError::ExecutionError(e.to_string()))
        }
        "history" => {
            let mut query_input = serde_json::json!({
                "walletAddress": params.address,
            });

            if let Value::Object(ref extra_obj) = params.extra {
                if let Some(obj) = query_input.as_object_mut() {
                    if let Some(chain_id) = extra_obj.get("chain_id") {
                        obj.insert("chainId".to_string(), chain_id.clone());
                    }
                }
            }

            client
                .query("portfolio.getHoldingHistory", query_input)
                .await
                .map_err(|e| ToolError::ExecutionError(e.to_string()))
        }
        "balances" => {
            let query_input = serde_json::json!({
                "walletAddress": params.address,
            });
            client
                .query("portfolio.getNativeBalances", query_input)
                .await
                .map_err(|e| ToolError::ExecutionError(e.to_string()))
        }
        "scan" => {
            let mut query_input = serde_json::json!({
                "walletAddress": params.address,
            });

            if let Value::Object(ref extra_obj) = params.extra {
                if let Some(obj) = query_input.as_object_mut() {
                    if let Some(chain_id) = extra_obj.get("chain_id") {
                        obj.insert("chainId".to_string(), chain_id.clone());
                    }
                }
            }

            client
                .query("portfolio.scanWallet", query_input)
                .await
                .map_err(|e| ToolError::ExecutionError(e.to_string()))
        }
        "swaps" => {
            let mut query_input = serde_json::json!({});

            if let Value::Object(ref extra_obj) = params.extra {
                if let Some(obj) = query_input.as_object_mut() {
                    let by = extra_obj
                        .get("by")
                        .and_then(|v| v.as_str())
                        .unwrap_or("wallet");

                    if by == "wallet" {
                        obj.insert("walletAddress".to_string(), serde_json::json!(params.address));
                    } else {
                        obj.insert("tokenAddress".to_string(), serde_json::json!(params.address));
                    }

                    if let Some(chain_id) = extra_obj.get("chain_id") {
                        obj.insert("chainId".to_string(), chain_id.clone());
                    }
                    if let Some(long_term) = extra_obj.get("long_term") {
                        obj.insert("longTerm".to_string(), long_term.clone());
                    }
                }
            } else {
                query_input
                    .as_object_mut()
                    .unwrap()
                    .insert("walletAddress".to_string(), serde_json::json!(params.address));
            }

            let procedure = if query_input.get("tokenAddress").is_some() {
                "market.getSwaps"
            } else {
                "portfolio.getWalletSwaps"
            };

            client
                .query(procedure, query_input)
                .await
                .map_err(|e| ToolError::ExecutionError(e.to_string()))
        }
        "tx" => {
            let tx_hash = if let Value::Object(ref extra_obj) = params.extra {
                extra_obj
                    .get("tx_hash")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| ToolError::InvalidInput("tx_hash required for tx view".to_string()))?
            } else {
                return Err(ToolError::InvalidInput("tx_hash required for tx view".to_string()));
            };

            let query_input = serde_json::json!({
                "txHash": tx_hash,
            });

            client
                .query("intelligence.getSwapByTxHash", query_input)
                .await
                .map_err(|e| ToolError::ExecutionError(e.to_string()))
        }
        _ => Err(ToolError::InvalidInput(format!(
            "Unknown view: {}. See: https://docs.edge.trade/agents/tools/portfolio",
            params.view
        ))),
    }
}
