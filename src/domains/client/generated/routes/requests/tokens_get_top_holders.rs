#![allow(clippy::derivable_impls)]

use crate::client::{Route, RouteType};
use std::marker::PhantomData;
/// Route metadata for this procedure
pub const ROUTE: Route<TokenTopHoldersRequest, Vec<TokenTopHoldersResponseItem>> = Route {
    procedure: "tokens.getTopHolders",
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
///Request to get the top holders for a specific token
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Request to get the top holders for a specific token",
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
///      "description": "The contract address of the token to query top holders for",
///      "type": "string",
///      "name": "Token Contract Address"
///    },
///    "walletAddresses": {
///      "description": "Optional array of wallet addresses to filter the top holders by",
///      "anyOf": [
///        {
///          "type": "array",
///          "items": {
///            "type": "string"
///          }
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Wallet Addresses"
///    }
///  },
///  "name": "Get Top Holders Request"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct TokenTopHoldersRequest {
    ///The chain ID where the token exists (e.g., "1" for Ethereum, "8453" for Base, "solana" for Solana)
    #[serde(rename = "chainId")]
    pub chain_id: ::std::string::String,
    ///The contract address of the token to query top holders for
    #[serde(rename = "tokenContractAddress")]
    pub token_contract_address: ::std::string::String,
    ///Optional array of wallet addresses to filter the top holders by
    #[serde(
        rename = "walletAddresses",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub wallet_addresses: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
///Holding information for a single wallet/token combination
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Holding information for a single wallet/token combination",
///  "type": "object",
///  "required": [
///    "atas",
///    "avgCostNativeToken",
///    "avgCostUsd",
///    "avgSoldNativeToken",
///    "avgSoldUsd",
///    "avgTokensBought",
///    "avgTokensSold",
///    "chainId",
///    "currentCostBasisUsd",
///    "dateUpdated",
///    "firstBuy",
///    "inTrade",
///    "isInsider",
///    "isSniper",
///    "lastSell",
///    "mostRecentTx",
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
///    "totalTransfers",
///    "transferedIn",
///    "transferedOut",
///    "walletAddress",
///    "worthNativeToken",
///    "worthUsd"
///  ],
///  "properties": {
///    "atas": {
///      "description": "Associated token accounts (for Solana)",
///      "type": "array",
///      "items": {
///        "type": "object",
///        "required": [
///          "ataAddress",
///          "balance"
///        ],
///        "properties": {
///          "ataAddress": {
///            "type": "string"
///          },
///          "balance": {
///            "type": "string"
///          }
///        },
///        "additionalProperties": false
///      },
///      "name": "ATAs"
///    },
///    "avgCostNativeToken": {
///      "description": "Average cost in native token",
///      "type": "number",
///      "name": "Average Cost Native Token"
///    },
///    "avgCostUsd": {
///      "description": "Average cost in USD",
///      "type": "number",
///      "name": "Average Cost USD"
///    },
///    "avgSoldNativeToken": {
///      "description": "Average sold in native token",
///      "type": "number",
///      "name": "Average Sold Native Token"
///    },
///    "avgSoldUsd": {
///      "description": "Average sold in USD",
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
///    "dateUpdated": {
///      "description": "Last update timestamp (ISO 8601)",
///      "type": "string",
///      "name": "Date Updated"
///    },
///    "firstBuy": {
///      "description": "Timestamp of first buy (ISO 8601)",
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
///    "inTrade": {
///      "description": "Whether the position is currently in a trade",
///      "type": "boolean",
///      "name": "In Trade"
///    },
///    "isInsider": {
///      "description": "Whether the wallet is flagged as an insider",
///      "type": "boolean",
///      "name": "Is Insider"
///    },
///    "isSniper": {
///      "description": "Whether the wallet is flagged as a sniper",
///      "type": "boolean",
///      "name": "Is Sniper"
///    },
///    "lastSell": {
///      "description": "Timestamp of last sell (ISO 8601)",
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
///      "description": "Timestamp of most recent transaction (ISO 8601)",
///      "type": "string",
///      "name": "Most Recent Transaction"
///    },
///    "tokenAddress": {
///      "description": "The contract address of the token",
///      "type": "string",
///      "name": "Token Address"
///    },
///    "tokenBalance": {
///      "description": "Current token balance in base units",
///      "type": "string",
///      "name": "Token Balance"
///    },
///    "tokensBought": {
///      "description": "Total tokens bought in base units",
///      "type": "string",
///      "name": "Tokens Bought"
///    },
///    "tokensSold": {
///      "description": "Total tokens sold in base units",
///      "type": "string",
///      "name": "Tokens Sold"
///    },
///    "totalBuys": {
///      "description": "Number of buy transactions",
///      "type": "number",
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
///      "description": "Number of sell transactions",
///      "type": "number",
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
///    "totalTransfers": {
///      "description": "Total number of transfers",
///      "type": "number",
///      "name": "Total Transfers"
///    },
///    "transferedIn": {
///      "description": "Total tokens transferred in (base units)",
///      "type": "string",
///      "name": "Transferred In"
///    },
///    "transferedOut": {
///      "description": "Total tokens transferred out (base units)",
///      "type": "string",
///      "name": "Transferred Out"
///    },
///    "walletAddress": {
///      "description": "The wallet address holding the token",
///      "type": "string",
///      "name": "Wallet Address"
///    },
///    "worthNativeToken": {
///      "description": "Current worth in native token",
///      "type": "number",
///      "name": "Worth Native Token"
///    },
///    "worthUsd": {
///      "description": "Current worth in USD",
///      "type": "number",
///      "name": "Worth USD"
///    }
///  },
///  "additionalProperties": false,
///  "name": "Token Holding"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TokenTopHoldersResponseItem {
    ///Associated token accounts (for Solana)
    pub atas: ::std::vec::Vec<TokenTopHoldersResponseItemAtasItem>,
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
    ///Last update timestamp (ISO 8601)
    #[serde(rename = "dateUpdated")]
    pub date_updated: ::std::string::String,
    ///Timestamp of first buy (ISO 8601)
    #[serde(rename = "firstBuy")]
    pub first_buy: ::std::option::Option<::std::string::String>,
    ///Whether the position is currently in a trade
    #[serde(rename = "inTrade")]
    pub in_trade: bool,
    ///Whether the wallet is flagged as an insider
    #[serde(rename = "isInsider")]
    pub is_insider: bool,
    ///Whether the wallet is flagged as a sniper
    #[serde(rename = "isSniper")]
    pub is_sniper: bool,
    ///Timestamp of last sell (ISO 8601)
    #[serde(rename = "lastSell")]
    pub last_sell: ::std::option::Option<::std::string::String>,
    ///Timestamp of most recent transaction (ISO 8601)
    #[serde(rename = "mostRecentTx")]
    pub most_recent_tx: ::std::string::String,
    ///The contract address of the token
    #[serde(rename = "tokenAddress")]
    pub token_address: ::std::string::String,
    ///Current token balance in base units
    #[serde(rename = "tokenBalance")]
    pub token_balance: ::std::string::String,
    ///Total tokens bought in base units
    #[serde(rename = "tokensBought")]
    pub tokens_bought: ::std::string::String,
    ///Total tokens sold in base units
    #[serde(rename = "tokensSold")]
    pub tokens_sold: ::std::string::String,
    #[serde(rename = "totalBuys")]
    pub total_buys: f64,
    #[serde(rename = "totalCostNativeToken")]
    pub total_cost_native_token: f64,
    #[serde(rename = "totalCostUsd")]
    pub total_cost_usd: f64,
    #[serde(rename = "totalSells")]
    pub total_sells: f64,
    #[serde(rename = "totalSoldNativeToken")]
    pub total_sold_native_token: f64,
    #[serde(rename = "totalSoldUsd")]
    pub total_sold_usd: f64,
    #[serde(rename = "totalTransfers")]
    pub total_transfers: f64,
    ///Total tokens transferred in (base units)
    #[serde(rename = "transferedIn")]
    pub transfered_in: ::std::string::String,
    ///Total tokens transferred out (base units)
    #[serde(rename = "transferedOut")]
    pub transfered_out: ::std::string::String,
    ///The wallet address holding the token
    #[serde(rename = "walletAddress")]
    pub wallet_address: ::std::string::String,
    #[serde(rename = "worthNativeToken")]
    pub worth_native_token: f64,
    #[serde(rename = "worthUsd")]
    pub worth_usd: f64,
}
///`TokenTopHoldersResponseItemAtasItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "ataAddress",
///    "balance"
///  ],
///  "properties": {
///    "ataAddress": {
///      "type": "string"
///    },
///    "balance": {
///      "type": "string"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TokenTopHoldersResponseItemAtasItem {
    #[serde(rename = "ataAddress")]
    pub ata_address: ::std::string::String,
    pub balance: ::std::string::String,
}
