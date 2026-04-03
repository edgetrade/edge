use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct McpManifest {
    pub tools: Vec<ToolDef>,
    pub resources: Vec<ResourceDef>,
    pub prompts: Vec<PromptDef>,
    pub skills: Vec<SkillDef>,
}

/// A namespace-level MCP tool. Each tool groups related procedures under a
/// single `action` parameter. The `actions` array describes every available
/// operation with its full input schema and routing information.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ToolDef {
    pub name: String,
    pub description: String,
    #[serde(rename = "inputSchema")]
    pub input_schema: Value,
    /// Always "namespace" for tools emitted by this manifest.
    pub kind: String,
    pub actions: Vec<ActionDef>,
}

/// A single operation within a namespace tool.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ActionDef {
    pub name: String,
    pub description: String,
    #[serde(rename = "inputSchema")]
    pub input_schema: Value,
    /// Full tRPC procedure path, e.g. "market.searchTokens".
    pub procedure: String,
    /// "query", "mutation", or "subscription".
    pub kind: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ResourceDef {
    pub uri: String,
    pub name: String,
    pub description: String,
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    pub content: Value,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PromptDef {
    pub name: String,
    pub description: String,
    pub arguments: Vec<PromptArgument>,
    pub messages: Vec<Value>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PromptArgument {
    pub name: String,
    pub description: String,
    pub required: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SkillDef {
    pub name: String,
    pub description: String,
    pub content: String,
}
