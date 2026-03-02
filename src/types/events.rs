use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct AlertEvent {
    pub subscription_id: String,
    pub alert_type: String,
    pub timestamp: i64,
    pub data: Value,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct WebhookRegistration {
    pub alert_type: String,
    pub webhook_url: String,
    pub webhook_secret: Option<String>,
    pub chain_id: Option<String>,
    pub address: Option<String>,
    pub wallet_address: Option<String>,
    pub wallet_addresses: Option<Vec<String>>,
    pub interval: Option<String>,
    pub threshold: Option<f64>,
    pub direction: Option<String>,
    pub filters: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum AlertsResponse {
    Events(Vec<AlertEvent>),
    Subscription { message: String, subscription_id: u32 },
    Success { message: String },
    Error { error: String },
}
