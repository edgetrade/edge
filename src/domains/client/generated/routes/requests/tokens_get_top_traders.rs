#![allow(clippy::derivable_impls)]

use crate::client::{Route, RouteType};
use std::marker::PhantomData;
/// Route metadata for this procedure
pub const ROUTE: Route<TokenTopTradersRequest, Vec<TokenTopTradersResponseItem>> = Route {
    procedure: "tokens.getTopTraders",
    route_type: RouteType::Query,
    input_schema: PhantomData,
    output_schema: PhantomData,
};
/// Error types.
pub mod error {
    /// Error from a `TryFrom` or `FromStr` implementation.
    pub struct ConversionError(::std::borrow::Cow<'static, str>);
    impl ::std::error::Error for ConversionError {}
    impl ::std::fmt::Display for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl ::std::fmt::Debug for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
///Request to get top traders for a specific token
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Request to get top traders for a specific token",
///  "type": "object",
///  "required": [
///    "chainId",
///    "tokenContractAddress"
///  ],
///  "properties": {
///    "chainId": {
///      "description": "The chain ID where the token exists (e.g., \"1\" for Ethereum, \"8453\" for Base, \"solana\" for Solana)",
///      "type": "string",
///      "name": "Chain ID"
///    },
///    "tokenContractAddress": {
///      "description": "The contract address of the token to query top traders for",
///      "type": "string",
///      "name": "Token Contract Address"
///    }
///  },
///  "name": "Get Top Traders Request"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct TokenTopTradersRequest {
    ///The chain ID where the token exists (e.g., "1" for Ethereum, "8453" for Base, "solana" for Solana)
    #[serde(rename = "chainId")]
    pub chain_id: ::std::string::String,
    ///The contract address of the token to query top traders for
    #[serde(rename = "tokenContractAddress")]
    pub token_contract_address: ::std::string::String,
}
///Response containing top traders data with statistics
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Response containing top traders data with statistics",
///  "type": "object",
///  "required": [
///    "avgCostNativeToken",
///    "avgCostUsd",
///    "avgSoldNativeToken",
///    "avgSoldUsd",
///    "avgTokensBought",
///    "avgTokensSold",
///    "chainId",
///    "currentCostBasisUsd",
///    "firstBuy",
///    "holderRank",
///    "lastSell",
///    "mostRecentTx",
///    "realisedPnlUsd",
///    "tokenAddress",
///    "tokenBalance",
///    "tokensBought",
///    "tokensSold",
///    "totalBuys",
///    "totalCostNativeToken",
///    "totalCostUsd",
///    "totalSells",
///    "totalSoldNativeToken",
///    "totalSoldUsd",
///    "walletAddress"
///  ],
///  "properties": {
///    "avgCostNativeToken": {
///      "description": "Average cost per transaction in native token",
///      "type": "number",
///      "name": "Average Cost Native Token"
///    },
///    "avgCostUsd": {
///      "description": "Average cost per transaction in USD",
///      "type": "number",
///      "name": "Average Cost USD"
///    },
///    "avgSoldNativeToken": {
///      "description": "Average sold per transaction in native token",
///      "type": "number",
///      "name": "Average Sold Native Token"
///    },
///    "avgSoldUsd": {
///      "description": "Average sold per transaction in USD",
///      "type": "number",
///      "name": "Average Sold USD"
///    },
///    "avgTokensBought": {
///      "description": "Average tokens bought per transaction",
///      "type": "number",
///      "name": "Average Tokens Bought"
///    },
///    "avgTokensSold": {
///      "description": "Average tokens sold per transaction",
///      "type": "number",
///      "name": "Average Tokens Sold"
///    },
///    "chainId": {
///      "description": "The blockchain chain ID",
///      "type": "string",
///      "name": "Chain ID"
///    },
///    "currentCostBasisUsd": {
///      "description": "Current cost basis in USD",
///      "type": "number",
///      "name": "Current Cost Basis USD"
///    },
///    "firstBuy": {
///      "description": "Timestamp of first buy transaction",
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "First Buy"
///    },
///    "holderRank": {
///      "description": "Rank of the holder",
///      "type": "number",
///      "name": "Holder Rank"
///    },
///    "lastSell": {
///      "description": "Timestamp of last sell transaction",
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Last Sell"
///    },
///    "mostRecentTx": {
///      "description": "Timestamp of most recent transaction",
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Most Recent Transaction"
///    },
///    "realisedPnlUsd": {
///      "description": "Realised profit/loss in USD",
///      "type": "number",
///      "name": "Realised PnL USD"
///    },
///    "tokenAddress": {
///      "description": "The contract address of the token",
///      "type": "string",
///      "name": "Token Address"
///    },
///    "tokenBalance": {
///      "description": "The current token balance",
///      "type": "string",
///      "name": "Token Balance"
///    },
///    "tokensBought": {
///      "description": "Total tokens bought",
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Tokens Bought"
///    },
///    "tokensSold": {
///      "description": "Total tokens sold",
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Tokens Sold"
///    },
///    "totalBuys": {
///      "description": "Total number of buy transactions",
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Total Buys"
///    },
///    "totalCostNativeToken": {
///      "description": "Total cost in native token",
///      "type": "number",
///      "name": "Total Cost Native Token"
///    },
///    "totalCostUsd": {
///      "description": "Total cost in USD",
///      "type": "number",
///      "name": "Total Cost USD"
///    },
///    "totalSells": {
///      "description": "Total number of sell transactions",
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Total Sells"
///    },
///    "totalSoldNativeToken": {
///      "description": "Total sold in native token",
///      "type": "number",
///      "name": "Total Sold Native Token"
///    },
///    "totalSoldUsd": {
///      "description": "Total sold in USD",
///      "type": "number",
///      "name": "Total Sold USD"
///    },
///    "walletAddress": {
///      "description": "The wallet address of the trader",
///      "type": "string",
///      "name": "Wallet Address"
///    }
///  },
///  "additionalProperties": false,
///  "name": "Get Top Traders Response"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TokenTopTradersResponseItem {
    #[serde(rename = "avgCostNativeToken")]
    pub avg_cost_native_token: f64,
    #[serde(rename = "avgCostUsd")]
    pub avg_cost_usd: f64,
    #[serde(rename = "avgSoldNativeToken")]
    pub avg_sold_native_token: f64,
    #[serde(rename = "avgSoldUsd")]
    pub avg_sold_usd: f64,
    #[serde(rename = "avgTokensBought")]
    pub avg_tokens_bought: f64,
    #[serde(rename = "avgTokensSold")]
    pub avg_tokens_sold: f64,
    ///The blockchain chain ID
    #[serde(rename = "chainId")]
    pub chain_id: ::std::string::String,
    #[serde(rename = "currentCostBasisUsd")]
    pub current_cost_basis_usd: f64,
    ///Timestamp of first buy transaction
    #[serde(rename = "firstBuy")]
    pub first_buy: ::std::option::Option<::std::string::String>,
    #[serde(rename = "holderRank")]
    pub holder_rank: f64,
    ///Timestamp of last sell transaction
    #[serde(rename = "lastSell")]
    pub last_sell: ::std::option::Option<::std::string::String>,
    ///Timestamp of most recent transaction
    #[serde(rename = "mostRecentTx")]
    pub most_recent_tx: ::std::option::Option<::std::string::String>,
    #[serde(rename = "realisedPnlUsd")]
    pub realised_pnl_usd: f64,
    ///The contract address of the token
    #[serde(rename = "tokenAddress")]
    pub token_address: ::std::string::String,
    ///The current token balance
    #[serde(rename = "tokenBalance")]
    pub token_balance: ::std::string::String,
    ///Total tokens bought
    #[serde(rename = "tokensBought")]
    pub tokens_bought: ::std::option::Option<::std::string::String>,
    ///Total tokens sold
    #[serde(rename = "tokensSold")]
    pub tokens_sold: ::std::option::Option<::std::string::String>,
    ///Total number of buy transactions
    #[serde(rename = "totalBuys")]
    pub total_buys: ::std::option::Option<::std::string::String>,
    #[serde(rename = "totalCostNativeToken")]
    pub total_cost_native_token: f64,
    #[serde(rename = "totalCostUsd")]
    pub total_cost_usd: f64,
    ///Total number of sell transactions
    #[serde(rename = "totalSells")]
    pub total_sells: ::std::option::Option<::std::string::String>,
    #[serde(rename = "totalSoldNativeToken")]
    pub total_sold_native_token: f64,
    #[serde(rename = "totalSoldUsd")]
    pub total_sold_usd: f64,
    ///The wallet address of the trader
    #[serde(rename = "walletAddress")]
    pub wallet_address: ::std::string::String,
}
