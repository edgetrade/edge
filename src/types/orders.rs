use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct LimitOrder {
    pub id: String,
    pub wallet_address: String,
    pub chain_id: String,
    pub pair_address: String,
    pub direction: String,
    pub trigger_price: f64,
    pub amount_native: f64,
    pub slippage: f64,
    pub status: String,
    pub created_at: i64,
    pub expires_at: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EntryStrategy {
    pub id: String,
    pub name: String,
    pub steps: Vec<EntryStep>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EntryStep {
    pub buy_amount_native_token: f64,
    pub percent_to_trigger: f64,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ExitStrategy {
    pub id: String,
    pub name: String,
    pub steps: Vec<ExitStep>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ExitStep {
    pub tp_percent_to_trigger: Option<f64>,
    pub tp_percent_of_bag_to_sell: Option<f64>,
    pub sl_percent_to_trigger: Option<f64>,
    pub sl_percent_to_sell: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PriceImpact {
    pub estimated_price: f64,
    pub price_impact_percent: f64,
    pub minimum_received: f64,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum TradeResponse {
    Orders(Vec<LimitOrder>),
    Order(LimitOrder),
    EntryStrategies(Vec<EntryStrategy>),
    EntryStrategy(EntryStrategy),
    ExitStrategies(Vec<ExitStrategy>),
    ExitStrategy(ExitStrategy),
    PriceImpact(PriceImpact),
    BuildResult(Value),
    SubmitResult(Value),
    Success { message: String },
    Error { error: String },
}
