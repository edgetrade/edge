use crate::client::IrisClient;
use crate::docs_url;
use rmcp::{Server, ToolError, tool, tool_handler};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
struct SearchInput {
    query: String,
    chain_id: Option<String>,
}

pub fn register(server: &Server, client: IrisClient) -> Result<(), Box<dyn std::error::Error>> {
    server.add_tool(
        tool!(
            name = "search",
            description = concat!("Search tokens by name or address. See: ", docs_url!(), "/tools/search"),
            input_schema = {
                "type": "object",
                "properties": {
                    "query": {
                        "type": "string",
                        "description": "Token name or address to search"
                    },
                    "chain_id": {
                        "type": "string",
                        "description": "Optional chain ID to filter results"
                    }
                },
                "required": ["query"]
            }
        ),
        tool_handler!(client, handle_search),
    )?;
    Ok(())
}

async fn handle_search(client: IrisClient, input: Value) -> Result<Value, ToolError> {
    let params: SearchInput = serde_json::from_value(input).map_err(|e| ToolError::InvalidInput(e.to_string()))?;

    let query_input = serde_json::json!({
        "query": params.query,
        "chainId": params.chain_id,
    });

    client
        .query("market.searchTokens", query_input)
        .await
        .map_err(|e| ToolError::ExecutionError(e.to_string()))
}
