#![allow(clippy::derivable_impls)]

use crate::client::{Route, RouteType};
use std::marker::PhantomData;
/// Route metadata for this procedure
pub const ROUTE: Route<PairSwapsRequest, Vec<PairSwapsResponseItem>> = Route {
    procedure: "pairs.getSwaps",
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
///Request to retrieve swap events with filtering by chain, token, pair, maker, and block range
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Request to retrieve swap events with filtering by chain, token, pair, maker, and block range",
///  "type": "object",
///  "required": [
///    "chainId",
///    "fromBlock",
///    "limit",
///    "makerAddress",
///    "pairAddress",
///    "toBlock",
///    "tokenAddress"
///  ],
///  "properties": {
///    "chainId": {
///      "description": "Blockchain identifier (e.g., \"1\" for Ethereum, \"8453\" for Base, \"solana\" for Solana)",
///      "type": "string",
///      "name": "Chain ID"
///    },
///    "fromBlock": {
///      "description": "Starting block number for the swap query range",
///      "type": "number",
///      "name": "From Block"
///    },
///    "limit": {
///      "description": "Maximum number of swap records to return",
///      "type": "number",
///      "name": "Limit"
///    },
///    "makerAddress": {
///      "description": "The address of the swap maker/trader to filter by",
///      "type": "string",
///      "name": "Maker Address"
///    },
///    "pairAddress": {
///      "description": "The liquidity pair address to filter swaps by",
///      "type": "string",
///      "name": "Pair Address"
///    },
///    "toBlock": {
///      "description": "Ending block number for the swap query range",
///      "type": "number",
///      "name": "To Block"
///    },
///    "tokenAddress": {
///      "description": "The token contract address to filter swaps by",
///      "type": "string",
///      "name": "Token Address"
///    }
///  },
///  "name": "Get Swaps Request"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct PairSwapsRequest {
    ///Blockchain identifier (e.g., "1" for Ethereum, "8453" for Base, "solana" for Solana)
    #[serde(rename = "chainId")]
    pub chain_id: ::std::string::String,
    #[serde(rename = "fromBlock")]
    pub from_block: f64,
    pub limit: f64,
    ///The address of the swap maker/trader to filter by
    #[serde(rename = "makerAddress")]
    pub maker_address: ::std::string::String,
    ///The liquidity pair address to filter swaps by
    #[serde(rename = "pairAddress")]
    pub pair_address: ::std::string::String,
    #[serde(rename = "toBlock")]
    pub to_block: f64,
    ///The token contract address to filter swaps by
    #[serde(rename = "tokenAddress")]
    pub token_address: ::std::string::String,
}
///Response to retrieve swap events with filtering by chain, token, pair, maker, and block range
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Response to retrieve swap events with filtering by chain, token, pair, maker, and block range",
///  "type": "object",
///  "required": [
///    "block_timestamp",
///    "chain_id",
///    "counter_token_decimals",
///    "counter_token_is_native_token",
///    "from_address",
///    "log_index",
///    "marketcap_native_token",
///    "marketcap_usd",
///    "pair_contract_address",
///    "swap_value_native_token",
///    "swap_value_usd",
///    "to_address",
///    "token_contract_address",
///    "token_decimals",
///    "token_price_native_token",
///    "token_price_usd",
///    "tokens_bought",
///    "tokens_sold",
///    "transaction_hash",
///    "transaction_index",
///    "via_address"
///  ],
///  "properties": {
///    "block_timestamp": {
///      "description": "The ISO 8601 timestamp when the swap occurred",
///      "type": "string",
///      "name": "Block Timestamp"
///    },
///    "chain_id": {
///      "description": "The blockchain chain ID where the swap occurred",
///      "type": "string",
///      "name": "Chain ID"
///    },
///    "counter_token_decimals": {
///      "description": "The number of decimals for the counter token in the pair",
///      "type": "number",
///      "name": "Counter Token Decimals"
///    },
///    "counter_token_is_native_token": {
///      "description": "Whether the counter token is the native token of the chain",
///      "type": "boolean",
///      "name": "Counter Token Is Native Token"
///    },
///    "from_address": {
///      "description": "The address the swap transaction originated from",
///      "type": "string",
///      "name": "From Address"
///    },
///    "log_index": {
///      "description": "The index of the log event within the transaction",
///      "type": "number",
///      "name": "Log Index"
///    },
///    "marketcap_native_token": {
///      "description": "The market capitalization of the token in native tokens",
///      "type": "string",
///      "name": "Marketcap Native Token"
///    },
///    "marketcap_usd": {
///      "description": "The market capitalization of the token in USD",
///      "type": "string",
///      "name": "Marketcap USD"
///    },
///    "pair_contract_address": {
///      "description": "The contract address of the liquidity pair",
///      "type": "string",
///      "name": "Pair Contract Address"
///    },
///    "swap_value_native_token": {
///      "description": "The value of the swap in the native token of the chain",
///      "type": "string",
///      "name": "Swap Value Native Token"
///    },
///    "swap_value_usd": {
///      "description": "The value of the swap in USD",
///      "type": "string",
///      "name": "Swap Value USD"
///    },
///    "to_address": {
///      "description": "The address receiving the swapped tokens",
///      "type": "string",
///      "name": "To Address"
///    },
///    "token_contract_address": {
///      "description": "The contract address of the token being swapped",
///      "type": "string",
///      "name": "Token Contract Address"
///    },
///    "token_ct_price": {
///      "description": "The counter-token amount that buys 1 token (human units), mostly useful for mixed pairs",
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Token Counter Token Price"
///    },
///    "token_decimals": {
///      "description": "The number of decimals for the token",
///      "type": "number",
///      "name": "Token Decimals"
///    },
///    "token_price_native_token": {
///      "description": "The price of the token in the native token of the chain",
///      "type": "string",
///      "name": "Token Price Native Token"
///    },
///    "token_price_usd": {
///      "description": "The price of the token in USD",
///      "type": "string",
///      "name": "Token Price USD"
///    },
///    "tokens_bought": {
///      "description": "The amount of tokens bought in the swap",
///      "type": "string",
///      "name": "Tokens Bought"
///    },
///    "tokens_sold": {
///      "description": "The amount of tokens sold in the swap",
///      "type": "string",
///      "name": "Tokens Sold"
///    },
///    "transaction_hash": {
///      "description": "The hash of the transaction containing the swap",
///      "type": "string",
///      "name": "Transaction Hash"
///    },
///    "transaction_index": {
///      "description": "The index of the transaction within the block",
///      "type": "number",
///      "name": "Transaction Index"
///    },
///    "via_address": {
///      "description": "The address that initiated the swap event",
///      "type": "string",
///      "name": "Via Address"
///    }
///  },
///  "additionalProperties": false,
///  "name": "Get Swaps Response"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct PairSwapsResponseItem {
    ///The ISO 8601 timestamp when the swap occurred
    pub block_timestamp: ::std::string::String,
    ///The blockchain chain ID where the swap occurred
    pub chain_id: ::std::string::String,
    pub counter_token_decimals: f64,
    ///Whether the counter token is the native token of the chain
    pub counter_token_is_native_token: bool,
    ///The address the swap transaction originated from
    pub from_address: ::std::string::String,
    pub log_index: f64,
    ///The market capitalization of the token in native tokens
    pub marketcap_native_token: ::std::string::String,
    ///The market capitalization of the token in USD
    pub marketcap_usd: ::std::string::String,
    ///The contract address of the liquidity pair
    pub pair_contract_address: ::std::string::String,
    ///The value of the swap in the native token of the chain
    pub swap_value_native_token: ::std::string::String,
    ///The value of the swap in USD
    pub swap_value_usd: ::std::string::String,
    ///The address receiving the swapped tokens
    pub to_address: ::std::string::String,
    ///The contract address of the token being swapped
    pub token_contract_address: ::std::string::String,
    ///The counter-token amount that buys 1 token (human units), mostly useful for mixed pairs
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub token_ct_price: ::std::option::Option<::std::string::String>,
    pub token_decimals: f64,
    ///The price of the token in the native token of the chain
    pub token_price_native_token: ::std::string::String,
    ///The price of the token in USD
    pub token_price_usd: ::std::string::String,
    ///The amount of tokens bought in the swap
    pub tokens_bought: ::std::string::String,
    ///The amount of tokens sold in the swap
    pub tokens_sold: ::std::string::String,
    ///The hash of the transaction containing the swap
    pub transaction_hash: ::std::string::String,
    pub transaction_index: f64,
    ///The address that initiated the swap event
    pub via_address: ::std::string::String,
}
