#![allow(clippy::derivable_impls)]

use crate::client::{Route, RouteType};
use std::marker::PhantomData;
/// Route metadata for this procedure
pub const ROUTE: Route<WalletSwapsRequest, Vec<WalletSwapsResponseItem>> = Route {
    procedure: "wallet.getWalletSwaps",
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
///`WalletSwapsRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "limit"
///  ],
///  "properties": {
///    "chainId": {
///      "anyOf": [
///        {
///          "anyOf": [
///            {},
///            {
///              "type": "string",
///              "const": "solana"
///            }
///          ]
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "limit": {
///      "description": "Number of swaps to return (1–500). Prefer 20 unless doing analysis.",
///      "type": "number",
///      "maximum": 500.0,
///      "minimum": 1.0
///    },
///    "makerAddresses": {
///      "description": "Wallet addresses to filter swaps by. Must be provided to retrieve swaps for a specific wallet — omitting it returns no results.",
///      "anyOf": [
///        {
///          "type": "array",
///          "items": {
///            "anyOf": [
///              {
///                "type": "string"
///              },
///              {
///                "anyOf": [
///                  {},
///                  {},
///                  {
///                    "type": "string"
///                  }
///                ]
///              }
///            ]
///          }
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "pairContractAddress": {
///      "anyOf": [
///        {
///          "anyOf": [
///            {
///              "type": "string"
///            },
///            {
///              "anyOf": [
///                {},
///                {},
///                {
///                  "type": "string"
///                }
///              ]
///            }
///          ]
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "previousPairContractAddress": {
///      "anyOf": [
///        {
///          "anyOf": [
///            {
///              "type": "string"
///            },
///            {
///              "anyOf": [
///                {},
///                {},
///                {
///                  "type": "string"
///                }
///              ]
///            }
///          ]
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "tokenContractAddress": {
///      "anyOf": [
///        {
///          "anyOf": [
///            {
///              "type": "string"
///            },
///            {
///              "anyOf": [
///                {},
///                {},
///                {
///                  "type": "string"
///                }
///              ]
///            }
///          ]
///        },
///        {
///          "type": "null"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct WalletSwapsRequest {
    #[serde(
        rename = "chainId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub chain_id: ::std::option::Option<WalletSwapsRequestChainId>,
    pub limit: f64,
    ///Wallet addresses to filter swaps by. Must be provided to retrieve swaps for a specific wallet — omitting it returns no results.
    #[serde(
        rename = "makerAddresses",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub maker_addresses: ::std::option::Option<
        ::std::vec::Vec<WalletSwapsRequestMakerAddressesItem>,
    >,
    #[serde(
        rename = "pairContractAddress",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub pair_contract_address: ::std::option::Option<
        WalletSwapsRequestPairContractAddress,
    >,
    #[serde(
        rename = "previousPairContractAddress",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub previous_pair_contract_address: ::std::option::Option<
        WalletSwapsRequestPreviousPairContractAddress,
    >,
    #[serde(
        rename = "tokenContractAddress",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub token_contract_address: ::std::option::Option<
        WalletSwapsRequestTokenContractAddress,
    >,
}
///`WalletSwapsRequestChainId`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "anyOf": [
///    {},
///    {
///      "type": "string",
///      "const": "solana"
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct WalletSwapsRequestChainId {
    #[serde(flatten, default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_0: ::std::option::Option<::serde_json::Value>,
    #[serde(flatten, default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_1: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for WalletSwapsRequestChainId {
    fn default() -> Self {
        Self {
            subtype_0: Default::default(),
            subtype_1: Default::default(),
        }
    }
}
///`WalletSwapsRequestMakerAddressesItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "anyOf": [
///    {
///      "type": "string"
///    },
///    {
///      "anyOf": [
///        {},
///        {},
///        {
///          "type": "string"
///        }
///      ]
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct WalletSwapsRequestMakerAddressesItem {
    #[serde(flatten, default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_0: ::std::option::Option<::std::string::String>,
    #[serde(flatten, default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_1: ::std::option::Option<WalletSwapsRequestMakerAddressesItemSubtype1>,
}
impl ::std::default::Default for WalletSwapsRequestMakerAddressesItem {
    fn default() -> Self {
        Self {
            subtype_0: Default::default(),
            subtype_1: Default::default(),
        }
    }
}
///`WalletSwapsRequestMakerAddressesItemSubtype1`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "anyOf": [
///    {},
///    {},
///    {
///      "type": "string"
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct WalletSwapsRequestMakerAddressesItemSubtype1 {
    #[serde(flatten, default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_0: ::std::option::Option<::serde_json::Value>,
    #[serde(flatten, default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_1: ::std::option::Option<::serde_json::Value>,
    #[serde(flatten, default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_2: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for WalletSwapsRequestMakerAddressesItemSubtype1 {
    fn default() -> Self {
        Self {
            subtype_0: Default::default(),
            subtype_1: Default::default(),
            subtype_2: Default::default(),
        }
    }
}
///`WalletSwapsRequestPairContractAddress`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "anyOf": [
///    {
///      "type": "string"
///    },
///    {
///      "anyOf": [
///        {},
///        {},
///        {
///          "type": "string"
///        }
///      ]
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct WalletSwapsRequestPairContractAddress {
    #[serde(flatten, default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_0: ::std::option::Option<::std::string::String>,
    #[serde(flatten, default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_1: ::std::option::Option<WalletSwapsRequestPairContractAddressSubtype1>,
}
impl ::std::default::Default for WalletSwapsRequestPairContractAddress {
    fn default() -> Self {
        Self {
            subtype_0: Default::default(),
            subtype_1: Default::default(),
        }
    }
}
///`WalletSwapsRequestPairContractAddressSubtype1`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "anyOf": [
///    {},
///    {},
///    {
///      "type": "string"
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct WalletSwapsRequestPairContractAddressSubtype1 {
    #[serde(flatten, default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_0: ::std::option::Option<::serde_json::Value>,
    #[serde(flatten, default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_1: ::std::option::Option<::serde_json::Value>,
    #[serde(flatten, default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_2: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for WalletSwapsRequestPairContractAddressSubtype1 {
    fn default() -> Self {
        Self {
            subtype_0: Default::default(),
            subtype_1: Default::default(),
            subtype_2: Default::default(),
        }
    }
}
///`WalletSwapsRequestPreviousPairContractAddress`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "anyOf": [
///    {
///      "type": "string"
///    },
///    {
///      "anyOf": [
///        {},
///        {},
///        {
///          "type": "string"
///        }
///      ]
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct WalletSwapsRequestPreviousPairContractAddress {
    #[serde(flatten, default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_0: ::std::option::Option<::std::string::String>,
    #[serde(flatten, default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_1: ::std::option::Option<
        WalletSwapsRequestPreviousPairContractAddressSubtype1,
    >,
}
impl ::std::default::Default for WalletSwapsRequestPreviousPairContractAddress {
    fn default() -> Self {
        Self {
            subtype_0: Default::default(),
            subtype_1: Default::default(),
        }
    }
}
///`WalletSwapsRequestPreviousPairContractAddressSubtype1`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "anyOf": [
///    {},
///    {},
///    {
///      "type": "string"
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct WalletSwapsRequestPreviousPairContractAddressSubtype1 {
    #[serde(flatten, default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_0: ::std::option::Option<::serde_json::Value>,
    #[serde(flatten, default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_1: ::std::option::Option<::serde_json::Value>,
    #[serde(flatten, default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_2: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for WalletSwapsRequestPreviousPairContractAddressSubtype1 {
    fn default() -> Self {
        Self {
            subtype_0: Default::default(),
            subtype_1: Default::default(),
            subtype_2: Default::default(),
        }
    }
}
///`WalletSwapsRequestTokenContractAddress`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "anyOf": [
///    {
///      "type": "string"
///    },
///    {
///      "anyOf": [
///        {},
///        {},
///        {
///          "type": "string"
///        }
///      ]
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct WalletSwapsRequestTokenContractAddress {
    #[serde(flatten, default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_0: ::std::option::Option<::std::string::String>,
    #[serde(flatten, default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_1: ::std::option::Option<WalletSwapsRequestTokenContractAddressSubtype1>,
}
impl ::std::default::Default for WalletSwapsRequestTokenContractAddress {
    fn default() -> Self {
        Self {
            subtype_0: Default::default(),
            subtype_1: Default::default(),
        }
    }
}
///`WalletSwapsRequestTokenContractAddressSubtype1`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "anyOf": [
///    {},
///    {},
///    {
///      "type": "string"
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct WalletSwapsRequestTokenContractAddressSubtype1 {
    #[serde(flatten, default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_0: ::std::option::Option<::serde_json::Value>,
    #[serde(flatten, default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_1: ::std::option::Option<::serde_json::Value>,
    #[serde(flatten, default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_2: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for WalletSwapsRequestTokenContractAddressSubtype1 {
    fn default() -> Self {
        Self {
            subtype_0: Default::default(),
            subtype_1: Default::default(),
            subtype_2: Default::default(),
        }
    }
}
///`WalletSwapsResponseItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "blockTimestamp",
///    "chainId",
///    "counterTokenDecimals",
///    "fromAddress",
///    "isNativeTokenReferencePair",
///    "logIndex",
///    "marketcapNativeToken",
///    "marketcapUsd",
///    "pairContractAddress",
///    "swapValueNativeToken",
///    "swapValueUsd",
///    "toAddress",
///    "tokenCTPrice",
///    "tokenContractAddress",
///    "tokenDecimals",
///    "tokenPriceNativeToken",
///    "tokenPriceUsd",
///    "tokensBought",
///    "tokensSold",
///    "transactionHash",
///    "transactionIndex",
///    "viaAddress"
///  ],
///  "properties": {
///    "blockTimestamp": {
///      "description": "The timestamp of the block when the swap occurred",
///      "type": "string",
///      "name": "Block Timestamp"
///    },
///    "chainId": {
///      "description": "The chain ID where the swap occurred",
///      "type": "string",
///      "name": "Chain ID"
///    },
///    "counterTokenDecimals": {
///      "description": "The decimals of the counter token",
///      "type": "number",
///      "name": "Counter Token Decimals"
///    },
///    "fromAddress": {
///      "description": "The address of the wallet making the swap",
///      "type": "string",
///      "name": "From Address"
///    },
///    "isNativeTokenReferencePair": {
///      "description": "Whether the pair is a native token reference pair",
///      "type": "boolean",
///      "name": "Is Native Token Reference Pair"
///    },
///    "logIndex": {
///      "description": "The index of the log event within the transaction",
///      "type": "number",
///      "name": "Log Index"
///    },
///    "marketcapNativeToken": {
///      "description": "The marketcap of the token in the native token of the chain",
///      "type": "string",
///      "name": "Marketcap Native Token"
///    },
///    "marketcapUsd": {
///      "description": "The marketcap of the token in USD",
///      "type": "string",
///      "name": "Marketcap USD"
///    },
///    "pairContractAddress": {
///      "description": "The contract address of the pair being swapped",
///      "type": "string",
///      "name": "Pair Contract Address"
///    },
///    "swapValueNativeToken": {
///      "description": "The value of the swap in the native token of the chain",
///      "type": "string",
///      "name": "Swap Value Native Token"
///    },
///    "swapValueUsd": {
///      "description": "The value of the swap in USD",
///      "type": "string",
///      "name": "Swap Value USD"
///    },
///    "toAddress": {
///      "description": "The address of the wallet receiving the swapped tokens",
///      "type": "string",
///      "name": "To Address"
///    },
///    "tokenCTPrice": {
///      "description": "The counter-token amount that buys you 1 token (human units), mostly useful for mixed pairs",
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Token CT Price"
///    },
///    "tokenContractAddress": {
///      "description": "The contract address of the token being swapped",
///      "type": "string",
///      "name": "Token Contract Address"
///    },
///    "tokenDecimals": {
///      "description": "The decimals of the token",
///      "type": "number",
///      "name": "Token Decimals"
///    },
///    "tokenPriceNativeToken": {
///      "description": "The price of the token in the native token of the chain",
///      "type": "string",
///      "name": "Token Price Native Token"
///    },
///    "tokenPriceUsd": {
///      "description": "The price of the token in USD",
///      "type": "string",
///      "name": "Token Price USD"
///    },
///    "tokensBought": {
///      "description": "The amount of tokens bought in the swap",
///      "type": "string",
///      "name": "Tokens Bought"
///    },
///    "tokensSold": {
///      "description": "The amount of tokens sold in the swap",
///      "type": "string",
///      "name": "Tokens Sold"
///    },
///    "transactionHash": {
///      "description": "The hash of the transaction containing the swap",
///      "type": "string",
///      "name": "Transaction Hash"
///    },
///    "transactionIndex": {
///      "description": "The index of the transaction within the block",
///      "type": "number",
///      "name": "Transaction Index"
///    },
///    "viaAddress": {
///      "description": "The address of the wallet that initiated the swap",
///      "type": "string",
///      "name": "Via Address"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct WalletSwapsResponseItem {
    ///The timestamp of the block when the swap occurred
    #[serde(rename = "blockTimestamp")]
    pub block_timestamp: ::std::string::String,
    ///The chain ID where the swap occurred
    #[serde(rename = "chainId")]
    pub chain_id: ::std::string::String,
    #[serde(rename = "counterTokenDecimals")]
    pub counter_token_decimals: f64,
    ///The address of the wallet making the swap
    #[serde(rename = "fromAddress")]
    pub from_address: ::std::string::String,
    ///Whether the pair is a native token reference pair
    #[serde(rename = "isNativeTokenReferencePair")]
    pub is_native_token_reference_pair: bool,
    #[serde(rename = "logIndex")]
    pub log_index: f64,
    ///The marketcap of the token in the native token of the chain
    #[serde(rename = "marketcapNativeToken")]
    pub marketcap_native_token: ::std::string::String,
    ///The marketcap of the token in USD
    #[serde(rename = "marketcapUsd")]
    pub marketcap_usd: ::std::string::String,
    ///The contract address of the pair being swapped
    #[serde(rename = "pairContractAddress")]
    pub pair_contract_address: ::std::string::String,
    ///The value of the swap in the native token of the chain
    #[serde(rename = "swapValueNativeToken")]
    pub swap_value_native_token: ::std::string::String,
    ///The value of the swap in USD
    #[serde(rename = "swapValueUsd")]
    pub swap_value_usd: ::std::string::String,
    ///The address of the wallet receiving the swapped tokens
    #[serde(rename = "toAddress")]
    pub to_address: ::std::string::String,
    ///The contract address of the token being swapped
    #[serde(rename = "tokenContractAddress")]
    pub token_contract_address: ::std::string::String,
    ///The counter-token amount that buys you 1 token (human units), mostly useful for mixed pairs
    #[serde(rename = "tokenCTPrice")]
    pub token_ct_price: ::std::option::Option<::std::string::String>,
    #[serde(rename = "tokenDecimals")]
    pub token_decimals: f64,
    ///The price of the token in the native token of the chain
    #[serde(rename = "tokenPriceNativeToken")]
    pub token_price_native_token: ::std::string::String,
    ///The price of the token in USD
    #[serde(rename = "tokenPriceUsd")]
    pub token_price_usd: ::std::string::String,
    ///The amount of tokens bought in the swap
    #[serde(rename = "tokensBought")]
    pub tokens_bought: ::std::string::String,
    ///The amount of tokens sold in the swap
    #[serde(rename = "tokensSold")]
    pub tokens_sold: ::std::string::String,
    ///The hash of the transaction containing the swap
    #[serde(rename = "transactionHash")]
    pub transaction_hash: ::std::string::String,
    #[serde(rename = "transactionIndex")]
    pub transaction_index: f64,
    ///The address of the wallet that initiated the swap
    #[serde(rename = "viaAddress")]
    pub via_address: ::std::string::String,
}
