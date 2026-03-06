use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Clone)]
pub struct McpManifest {
    pub tools: Vec<ToolDef>,
    pub resources: Vec<ResourceDef>,
    pub prompts: Vec<PromptDef>,
    pub skills: Vec<SkillDef>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ToolDef {
    pub name: String,
    pub description: String,
    #[serde(rename = "inputSchema")]
    pub input_schema: Value,
    pub procedure: String,
    /// "query", "mutation", or "subscription"
    pub kind: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ResourceDef {
    pub uri: String,
    pub name: String,
    pub description: String,
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    pub content: Value,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PromptDef {
    pub name: String,
    pub description: String,
    pub arguments: Vec<PromptArgument>,
    pub messages: Vec<Value>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PromptArgument {
    pub name: String,
    pub description: String,
    pub required: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SkillDef {
    pub name: String,
    pub description: String,
    pub content: String,
}
