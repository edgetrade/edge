use crate::client::IrisClient;
use rmcp::{Server, ToolError, tool, tool_handler};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
struct ScreenInput {
    chain_id: String,
    min_mcap: Option<f64>,
    max_mcap: Option<f64>,
    min_liquidity: Option<f64>,
    min_volume_24h: Option<f64>,
    max_sniper_pct: Option<f64>,
    max_insider_pct: Option<f64>,
    max_top10_pct: Option<f64>,
    has_twitter: Option<bool>,
    has_telegram: Option<bool>,
    dexscreener_paid: Option<bool>,
    graduation_min_pct: Option<f64>,
    sort_by: Option<String>,
    limit: Option<u32>,
}

pub fn register(server: &Server, client: IrisClient) -> Result<(), Box<dyn std::error::Error>> {
    server.add_tool(
        tool!(
            name = "screen",
            description = "Screen tokens by market cap, liquidity, volume, holder metrics, and social presence. See: https://docs.edge.trade/agents/tools/screen",
            input_schema = {
                "type": "object",
                "properties": {
                    "chain_id": {
                        "type": "string",
                        "description": "Chain ID"
                    },
                    "min_mcap": {
                        "type": "number",
                        "description": "Minimum market cap in USD"
                    },
                    "max_mcap": {
                        "type": "number",
                        "description": "Maximum market cap in USD"
                    },
                    "min_liquidity": {
                        "type": "number",
                        "description": "Minimum liquidity in USD"
                    },
                    "min_volume_24h": {
                        "type": "number",
                        "description": "Minimum 24h volume in USD"
                    },
                    "max_sniper_pct": {
                        "type": "number",
                        "description": "Maximum sniper percentage (0-100)"
                    },
                    "max_insider_pct": {
                        "type": "number",
                        "description": "Maximum insider percentage (0-100)"
                    },
                    "max_top10_pct": {
                        "type": "number",
                        "description": "Maximum top 10 holder percentage (0-100)"
                    },
                    "has_twitter": {
                        "type": "boolean",
                        "description": "Filter for tokens with Twitter"
                    },
                    "has_telegram": {
                        "type": "boolean",
                        "description": "Filter for tokens with Telegram"
                    },
                    "dexscreener_paid": {
                        "type": "boolean",
                        "description": "Filter for tokens with paid DexScreener ads"
                    },
                    "graduation_min_pct": {
                        "type": "number",
                        "description": "Minimum graduation percentage"
                    },
                    "sort_by": {
                        "type": "string",
                        "description": "Sort field (mcap, volume_24h, liquidity, etc.)"
                    },
                    "limit": {
                        "type": "integer",
                        "description": "Maximum number of results"
                    }
                },
                "required": ["chain_id"]
            }
        ),
        tool_handler!(client, handle_screen),
    )?;
    Ok(())
}

async fn handle_screen(client: IrisClient, input: Value) -> Result<Value, ToolError> {
    let params: ScreenInput = serde_json::from_value(input).map_err(|e| ToolError::InvalidInput(e.to_string()))?;

    let mut query_input = serde_json::json!({
        "chainId": params.chain_id,
    });

    let obj = query_input.as_object_mut().unwrap();

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

    client
        .query("market.screenTokens", query_input)
        .await
        .map_err(|e| ToolError::ExecutionError(e.to_string()))
}
