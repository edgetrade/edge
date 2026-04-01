#![allow(clippy::derivable_impls)]

use crate::client::{Route, RouteType};
use std::marker::PhantomData;
/// Route metadata for this procedure
pub const ROUTE: Route<SearchSwapsRequest, Vec<SearchSwapsResponseItem>> = Route {
    procedure: "intelligence.searchSwaps",
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
        fn fmt(
            &self,
            f: &mut ::std::fmt::Formatter<'_>,
        ) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl ::std::fmt::Debug for ConversionError {
        fn fmt(
            &self,
            f: &mut ::std::fmt::Formatter<'_>,
        ) -> Result<(), ::std::fmt::Error> {
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
///Input for getting a swap by its transaction hash and block timestamp.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Input for getting a swap by its transaction hash and block timestamp.",
///  "type": "object",
///  "required": [
///    "blockTimestamp",
///    "chainId",
///    "txHash"
///  ],
///  "properties": {
///    "blockTimestamp": {
///      "description": "The timestamp of the block of the swap in unix epoch seconds.",
///      "type": "number",
///      "name": "Block Timestamp"
///    },
///    "chainId": {
///      "description": "The chain ID of the swap.",
///      "type": "string",
///      "name": "Chain ID"
///    },
///    "txHash": {
///      "description": "The transaction hash of the swap.",
///      "type": "string",
///      "name": "Transaction Hash"
///    }
///  },
///  "name": "Get Swap Request"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SearchSwapsRequest {
    #[serde(rename = "blockTimestamp")]
    pub block_timestamp: f64,
    ///The chain ID of the swap.
    #[serde(rename = "chainId")]
    pub chain_id: ::std::string::String,
    ///The transaction hash of the swap.
    #[serde(rename = "txHash")]
    pub tx_hash: ::std::string::String,
}
///Output schema for a single swap item retrieved by transaction hash.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Output schema for a single swap item retrieved by transaction hash.",
///  "type": "object",
///  "required": [
///    "block_timestamp",
///    "chain_id",
///    "chain_type",
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
///      "description": "The timestamp of the block when the swap occurred.",
///      "type": "number",
///      "name": "Block Timestamp"
///    },
///    "chain_id": {
///      "description": "The chain ID of the swap.",
///      "type": "string",
///      "name": "Chain ID"
///    },
///    "chain_type": {
///      "description": "The chain type of the swap.",
///      "type": "string",
///      "enum": [
///        "EVM",
///        "SVM"
///      ],
///      "name": "Chain Type"
///    },
///    "counter_token_decimals": {
///      "description": "The number of decimals of the counter token.",
///      "type": "number",
///      "name": "Counter Token Decimals"
///    },
///    "counter_token_is_native_token": {
///      "description": "Whether the counter token is the native token of the chain.",
///      "type": "boolean",
///      "name": "Counter Token Is Native Token"
///    },
///    "from_address": {
///      "description": "The address of the from of the swap.",
///      "type": "string",
///      "name": "From Address"
///    },
///    "log_index": {
///      "description": "The index of the log of the swap.",
///      "type": "number",
///      "name": "Log Index"
///    },
///    "marketcap_native_token": {
///      "description": "The marketcap of the token in the native token of the chain.",
///      "type": "string",
///      "name": "Marketcap Native Token"
///    },
///    "marketcap_usd": {
///      "description": "The marketcap of the token in USD.",
///      "type": "string",
///      "name": "Marketcap USD"
///    },
///    "pair_contract_address": {
///      "description": "The address of the pair of the swap.",
///      "type": "string",
///      "name": "Pair Contract Address"
///    },
///    "swap_value_native_token": {
///      "description": "The value of the swap in the native token of the chain.",
///      "type": "string",
///      "name": "Swap Value Native Token"
///    },
///    "swap_value_usd": {
///      "description": "The value of the swap in USD.",
///      "type": "string",
///      "name": "Swap Value USD"
///    },
///    "to_address": {
///      "description": "The address of the to of the swap.",
///      "type": "string",
///      "name": "To Address"
///    },
///    "token_contract_address": {
///      "description": "The address of the token of the swap.",
///      "type": "string",
///      "name": "Token Contract Address"
///    },
///    "token_decimals": {
///      "description": "The number of decimals of the token.",
///      "type": "number",
///      "name": "Token Decimals"
///    },
///    "token_price_native_token": {
///      "description": "The price of the token in the native token of the chain.",
///      "type": "string",
///      "name": "Token Price Native Token"
///    },
///    "token_price_usd": {
///      "description": "The price of the token in USD.",
///      "type": "string",
///      "name": "Token Price USD"
///    },
///    "tokens_bought": {
///      "description": "The amount of tokens bought of the swap.",
///      "type": "string",
///      "name": "Tokens Bought"
///    },
///    "tokens_sold": {
///      "description": "The amount of tokens sold of the swap.",
///      "type": "string",
///      "name": "Tokens Sold"
///    },
///    "transaction_hash": {
///      "description": "The transaction hash of the swap.",
///      "type": "string",
///      "name": "Transaction Hash"
///    },
///    "transaction_index": {
///      "description": "The index of the transaction of the swap.",
///      "type": "number",
///      "name": "Transaction Index"
///    },
///    "via_address": {
///      "description": "The address of the via of the swap.",
///      "type": "string",
///      "name": "Via Address"
///    }
///  },
///  "additionalProperties": false,
///  "name": "Get Swap Response Item"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct SearchSwapsResponseItem {
    pub block_timestamp: f64,
    ///The chain ID of the swap.
    pub chain_id: ::std::string::String,
    ///The chain type of the swap.
    pub chain_type: SearchSwapsResponseItemChainType,
    pub counter_token_decimals: f64,
    ///Whether the counter token is the native token of the chain.
    pub counter_token_is_native_token: bool,
    ///The address of the from of the swap.
    pub from_address: ::std::string::String,
    pub log_index: f64,
    ///The marketcap of the token in the native token of the chain.
    pub marketcap_native_token: ::std::string::String,
    ///The marketcap of the token in USD.
    pub marketcap_usd: ::std::string::String,
    ///The address of the pair of the swap.
    pub pair_contract_address: ::std::string::String,
    ///The value of the swap in the native token of the chain.
    pub swap_value_native_token: ::std::string::String,
    ///The value of the swap in USD.
    pub swap_value_usd: ::std::string::String,
    ///The address of the to of the swap.
    pub to_address: ::std::string::String,
    ///The address of the token of the swap.
    pub token_contract_address: ::std::string::String,
    pub token_decimals: f64,
    ///The price of the token in the native token of the chain.
    pub token_price_native_token: ::std::string::String,
    ///The price of the token in USD.
    pub token_price_usd: ::std::string::String,
    ///The amount of tokens bought of the swap.
    pub tokens_bought: ::std::string::String,
    ///The amount of tokens sold of the swap.
    pub tokens_sold: ::std::string::String,
    ///The transaction hash of the swap.
    pub transaction_hash: ::std::string::String,
    pub transaction_index: f64,
    ///The address of the via of the swap.
    pub via_address: ::std::string::String,
}
///The chain type of the swap.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "The chain type of the swap.",
///  "type": "string",
///  "enum": [
///    "EVM",
///    "SVM"
///  ],
///  "name": "Chain Type"
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum SearchSwapsResponseItemChainType {
    #[serde(rename = "EVM")]
    Evm,
    #[serde(rename = "SVM")]
    Svm,
}
impl ::std::fmt::Display for SearchSwapsResponseItemChainType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Evm => f.write_str("EVM"),
            Self::Svm => f.write_str("SVM"),
        }
    }
}
impl ::std::str::FromStr for SearchSwapsResponseItemChainType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "EVM" => Ok(Self::Evm),
            "SVM" => Ok(Self::Svm),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SearchSwapsResponseItemChainType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for SearchSwapsResponseItemChainType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for SearchSwapsResponseItemChainType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
