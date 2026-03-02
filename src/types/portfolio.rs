use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Holding {
    pub token_address: String,
    pub chain_id: String,
    pub balance: String,
    pub value_usd: Option<f64>,
    pub cost_basis: Option<f64>,
    pub unrealized_pnl: Option<f64>,
    pub realized_pnl: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PortfolioSummary {
    pub total_value_usd: f64,
    pub total_cost_basis: f64,
    pub total_unrealized_pnl: f64,
    pub total_realized_pnl: f64,
    pub holdings_count: u32,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct HoldingHistory {
    pub timestamp: i64,
    pub token_address: String,
    pub chain_id: String,
    pub balance: String,
    pub value_usd: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct NativeBalance {
    pub chain_id: String,
    pub balance: String,
    pub value_usd: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Holder {
    pub wallet_address: String,
    pub balance: String,
    pub percentage: f64,
    pub is_sniper: bool,
    pub is_insider: bool,
    pub is_deployer: bool,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Trader {
    pub wallet_address: String,
    pub realized_pnl: f64,
    pub unrealized_pnl: f64,
    pub total_trades: u32,
    pub win_rate: f64,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum PortfolioResponse {
    Holdings(Vec<Holding>),
    Summary(PortfolioSummary),
    History(Vec<HoldingHistory>),
    Balances(Vec<NativeBalance>),
    ScanResult(Value),
    Swaps(Vec<Value>),
    Transaction(Value),
    Error { error: String },
}
