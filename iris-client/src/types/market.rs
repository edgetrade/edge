use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Token {
    pub address: String,
    pub name: String,
    pub symbol: String,
    pub chain_id: String,
    pub decimals: u8,
    pub market_cap: Option<f64>,
    pub price_usd: Option<f64>,
    pub liquidity: Option<f64>,
    pub volume_24h: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum SearchResponse {
    Tokens(Vec<Token>),
    Error { error: String },
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Pair {
    pub address: String,
    pub chain_id: String,
    pub token_address: String,
    pub counter_token_address: String,
    pub liquidity: f64,
    pub price_usd: Option<f64>,
    pub price_native: Option<f64>,
    pub volume_24h: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PairMetrics {
    pub price_usd: Option<f64>,
    pub price_native: Option<f64>,
    pub market_cap_usd: Option<f64>,
    pub liquidity: f64,
    pub volume_24h: f64,
    pub price_change_24h: Option<f64>,
    pub buys_24h: u32,
    pub sells_24h: u32,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Candle {
    pub timestamp: i64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Swap {
    pub tx_hash: String,
    pub block_timestamp: i64,
    pub wallet_address: String,
    pub direction: String,
    pub amount_in: String,
    pub amount_out: String,
    pub price_usd: Option<f64>,
    pub price_native: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Graduation {
    pub is_graduated: bool,
    pub graduation_progress: Option<f64>,
    pub destination_pair: Option<String>,
    pub bonding_curve_address: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum InspectResponse {
    TokenOverview(Value),
    TokenHolders(Vec<super::portfolio::Holder>),
    TokenAnalytics(Vec<super::portfolio::Trader>),
    TokenGraduation(Graduation),
    PairOverview(Pair),
    PairMetrics(PairMetrics),
    PairCandles(Vec<Candle>),
    PairSwaps(Vec<Swap>),
    Error { error: String },
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum ScreenResponse {
    Tokens(Vec<Token>),
    Error { error: String },
}
