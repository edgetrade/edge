#![allow(clippy::derivable_impls)]

use crate::client::{Route, RouteType};
use std::marker::PhantomData;
/// Route metadata for this procedure
pub const ROUTE: Route<PlaceLimitOrderRequest, Vec<PlaceLimitOrderResponseItem>> = Route {
    procedure: "orders.placeLimitOrder",
    route_type: RouteType::Mutation,
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
///Request to place a limit order
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Request to place a limit order",
///  "type": "object",
///  "required": [
///    "order"
///  ],
///  "properties": {
///    "order": {
///      "description": "The limit order to place",
///      "type": "object",
///      "required": [
///        "amount",
///        "entryStrategyId",
///        "exitStrategyId",
///        "expiration",
///        "pairAddress",
///        "side",
///        "tokenId",
///        "txPreset",
///        "wallets"
///      ],
///      "properties": {
///        "amount": {
///          "description": "The amount value and type to trade",
///          "type": "object",
///          "required": [
///            "type",
///            "value"
///          ],
///          "properties": {
///            "type": {
///              "description": "Type of amount: native currency, token amount, or percentage",
///              "type": "string",
///              "enum": [
///                "native",
///                "token",
///                "percentage"
///              ],
///              "name": "Amount Type"
///            },
///            "value": {
///              "description": "The amount value as a string",
///              "type": "string",
///              "name": "Amount Value"
///            }
///          },
///          "name": "Order Amount"
///        },
///        "counterTokenAddress": {
///          "description": "The address of the counter token (optional)",
///          "anyOf": [
///            {
///              "type": "string"
///            },
///            {
///              "type": "null"
///            }
///          ],
///          "name": "Counter Token Address"
///        },
///        "entryStrategyId": {
///          "description": "The ID of the entry strategy to apply (optional)",
///          "anyOf": [
///            {
///              "type": "number"
///            },
///            {
///              "type": "null"
///            }
///          ],
///          "name": "Entry Strategy ID"
///        },
///        "exitStrategyId": {
///          "description": "The ID of the exit strategy to apply (optional)",
///          "anyOf": [
///            {
///              "type": "number"
///            },
///            {
///              "type": "null"
///            }
///          ],
///          "name": "Exit Strategy ID"
///        },
///        "expiration": {
///          "description": "Order expiration time in seconds from creation",
///          "type": "number",
///          "name": "Expiration"
///        },
///        "pairAddress": {
///          "description": "The address of the trading pair (optional)",
///          "anyOf": [
///            {
///              "type": "string"
///            },
///            {
///              "type": "null"
///            }
///          ],
///          "name": "Pair Address"
///        },
///        "side": {
///          "description": "The side of the order: buy or sell the token",
///          "type": "string",
///          "enum": [
///            "buy",
///            "sell"
///          ],
///          "name": "Order Side"
///        },
///        "tokenId": {
///          "description": "The unique identifier of the token to trade",
///          "type": "object",
///          "required": [
///            "chainType",
///            "tokenChainId",
///            "tokenContractAddress"
///          ],
///          "properties": {
///            "chainType": {
///              "description": "The chain type of the token",
///              "type": "string",
///              "enum": [
///                "EVM",
///                "SVM"
///              ],
///              "name": "Chain Type"
///            },
///            "tokenChainId": {
///              "description": "The chain ID of the token",
///              "type": "string",
///              "name": "Token Chain ID"
///            },
///            "tokenContractAddress": {
///              "description": "The contract address of the token",
///              "type": "string",
///              "name": "Token Contract Address"
///            }
///          },
///          "name": "Token ID"
///        },
///        "triggerMarketcapUsd": {
///          "description": "The marketcap threshold to trigger the order (optional)",
///          "anyOf": [
///            {
///              "type": "number"
///            },
///            {
///              "type": "null"
///            }
///          ],
///          "name": "Trigger Marketcap USD"
///        },
///        "triggerTokenPriceUsd": {
///          "description": "The token price threshold to trigger the order (optional)",
///          "anyOf": [
///            {
///              "type": "number"
///            },
///            {
///              "type": "null"
///            }
///          ],
///          "name": "Trigger Token Price USD"
///        },
///        "txPreset": {
///          "description": "Transaction preset configuration",
///          "type": "object",
///          "required": [
///            "bribe",
///            "key",
///            "maxBaseGas",
///            "method",
///            "priorityGas",
///            "slippage"
///          ],
///          "properties": {
///            "bribe": {
///              "description": "Bribe amount in lamports for SVM chains (as string)",
///              "type": "string",
///              "name": "Bribe"
///            },
///            "key": {
///              "description": "The trade preset key identifier",
///              "type": "string",
///              "enum": [
///                "a",
///                "b",
///                "c",
///                "d"
///              ],
///              "name": "Preset Key"
///            },
///            "maxBaseGas": {
///              "description": "Maximum base gas in wei (as string)",
///              "type": "string",
///              "name": "Max Base Gas"
///            },
///            "method": {
///              "description": "Trading method: normal or flashbot",
///              "type": "string",
///              "enum": [
///                "normal",
///                "flashbot"
///              ],
///              "name": "Trade Method"
///            },
///            "priorityGas": {
///              "description": "Priority gas in wei (as string)",
///              "type": "string",
///              "name": "Priority Gas"
///            },
///            "slippage": {
///              "description": "Slippage tolerance percentage",
///              "type": "number",
///              "name": "Slippage"
///            }
///          },
///          "name": "Transaction Preset"
///        },
///        "wallets": {
///          "description": "Wallets that will be used to trade the token",
///          "type": "array",
///          "items": {
///            "type": "object",
///            "required": [
///              "address"
///            ],
///            "properties": {
///              "address": {
///                "description": "The wallet address",
///                "type": "string",
///                "name": "Wallet Address"
///              }
///            }
///          },
///          "minItems": 1,
///          "name": "Wallets"
///        }
///      },
///      "name": "Order"
///    }
///  },
///  "name": "Place Order Request"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct PlaceLimitOrderRequest {
    pub order: PlaceLimitOrderRequestOrder,
}
///The limit order to place
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "The limit order to place",
///  "type": "object",
///  "required": [
///    "amount",
///    "entryStrategyId",
///    "exitStrategyId",
///    "expiration",
///    "pairAddress",
///    "side",
///    "tokenId",
///    "txPreset",
///    "wallets"
///  ],
///  "properties": {
///    "amount": {
///      "description": "The amount value and type to trade",
///      "type": "object",
///      "required": [
///        "type",
///        "value"
///      ],
///      "properties": {
///        "type": {
///          "description": "Type of amount: native currency, token amount, or percentage",
///          "type": "string",
///          "enum": [
///            "native",
///            "token",
///            "percentage"
///          ],
///          "name": "Amount Type"
///        },
///        "value": {
///          "description": "The amount value as a string",
///          "type": "string",
///          "name": "Amount Value"
///        }
///      },
///      "name": "Order Amount"
///    },
///    "counterTokenAddress": {
///      "description": "The address of the counter token (optional)",
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Counter Token Address"
///    },
///    "entryStrategyId": {
///      "description": "The ID of the entry strategy to apply (optional)",
///      "anyOf": [
///        {
///          "type": "number"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Entry Strategy ID"
///    },
///    "exitStrategyId": {
///      "description": "The ID of the exit strategy to apply (optional)",
///      "anyOf": [
///        {
///          "type": "number"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Exit Strategy ID"
///    },
///    "expiration": {
///      "description": "Order expiration time in seconds from creation",
///      "type": "number",
///      "name": "Expiration"
///    },
///    "pairAddress": {
///      "description": "The address of the trading pair (optional)",
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Pair Address"
///    },
///    "side": {
///      "description": "The side of the order: buy or sell the token",
///      "type": "string",
///      "enum": [
///        "buy",
///        "sell"
///      ],
///      "name": "Order Side"
///    },
///    "tokenId": {
///      "description": "The unique identifier of the token to trade",
///      "type": "object",
///      "required": [
///        "chainType",
///        "tokenChainId",
///        "tokenContractAddress"
///      ],
///      "properties": {
///        "chainType": {
///          "description": "The chain type of the token",
///          "type": "string",
///          "enum": [
///            "EVM",
///            "SVM"
///          ],
///          "name": "Chain Type"
///        },
///        "tokenChainId": {
///          "description": "The chain ID of the token",
///          "type": "string",
///          "name": "Token Chain ID"
///        },
///        "tokenContractAddress": {
///          "description": "The contract address of the token",
///          "type": "string",
///          "name": "Token Contract Address"
///        }
///      },
///      "name": "Token ID"
///    },
///    "triggerMarketcapUsd": {
///      "description": "The marketcap threshold to trigger the order (optional)",
///      "anyOf": [
///        {
///          "type": "number"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Trigger Marketcap USD"
///    },
///    "triggerTokenPriceUsd": {
///      "description": "The token price threshold to trigger the order (optional)",
///      "anyOf": [
///        {
///          "type": "number"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Trigger Token Price USD"
///    },
///    "txPreset": {
///      "description": "Transaction preset configuration",
///      "type": "object",
///      "required": [
///        "bribe",
///        "key",
///        "maxBaseGas",
///        "method",
///        "priorityGas",
///        "slippage"
///      ],
///      "properties": {
///        "bribe": {
///          "description": "Bribe amount in lamports for SVM chains (as string)",
///          "type": "string",
///          "name": "Bribe"
///        },
///        "key": {
///          "description": "The trade preset key identifier",
///          "type": "string",
///          "enum": [
///            "a",
///            "b",
///            "c",
///            "d"
///          ],
///          "name": "Preset Key"
///        },
///        "maxBaseGas": {
///          "description": "Maximum base gas in wei (as string)",
///          "type": "string",
///          "name": "Max Base Gas"
///        },
///        "method": {
///          "description": "Trading method: normal or flashbot",
///          "type": "string",
///          "enum": [
///            "normal",
///            "flashbot"
///          ],
///          "name": "Trade Method"
///        },
///        "priorityGas": {
///          "description": "Priority gas in wei (as string)",
///          "type": "string",
///          "name": "Priority Gas"
///        },
///        "slippage": {
///          "description": "Slippage tolerance percentage",
///          "type": "number",
///          "name": "Slippage"
///        }
///      },
///      "name": "Transaction Preset"
///    },
///    "wallets": {
///      "description": "Wallets that will be used to trade the token",
///      "type": "array",
///      "items": {
///        "type": "object",
///        "required": [
///          "address"
///        ],
///        "properties": {
///          "address": {
///            "description": "The wallet address",
///            "type": "string",
///            "name": "Wallet Address"
///          }
///        }
///      },
///      "minItems": 1,
///      "name": "Wallets"
///    }
///  },
///  "name": "Order"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct PlaceLimitOrderRequestOrder {
    pub amount: PlaceLimitOrderRequestOrderAmount,
    ///The address of the counter token (optional)
    #[serde(
        rename = "counterTokenAddress",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub counter_token_address: ::std::option::Option<::std::string::String>,
    ///The ID of the entry strategy to apply (optional)
    #[serde(rename = "entryStrategyId")]
    pub entry_strategy_id: ::std::option::Option<f64>,
    ///The ID of the exit strategy to apply (optional)
    #[serde(rename = "exitStrategyId")]
    pub exit_strategy_id: ::std::option::Option<f64>,
    pub expiration: f64,
    ///The address of the trading pair (optional)
    #[serde(rename = "pairAddress")]
    pub pair_address: ::std::option::Option<::std::string::String>,
    ///The side of the order: buy or sell the token
    pub side: PlaceLimitOrderRequestOrderSide,
    #[serde(rename = "tokenId")]
    pub token_id: PlaceLimitOrderRequestOrderTokenId,
    ///The marketcap threshold to trigger the order (optional)
    #[serde(
        rename = "triggerMarketcapUsd",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub trigger_marketcap_usd: ::std::option::Option<f64>,
    ///The token price threshold to trigger the order (optional)
    #[serde(
        rename = "triggerTokenPriceUsd",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub trigger_token_price_usd: ::std::option::Option<f64>,
    #[serde(rename = "txPreset")]
    pub tx_preset: PlaceLimitOrderRequestOrderTxPreset,
    ///Wallets that will be used to trade the token
    pub wallets: ::std::vec::Vec<PlaceLimitOrderRequestOrderWalletsItem>,
}
///The amount value and type to trade
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "The amount value and type to trade",
///  "type": "object",
///  "required": [
///    "type",
///    "value"
///  ],
///  "properties": {
///    "type": {
///      "description": "Type of amount: native currency, token amount, or percentage",
///      "type": "string",
///      "enum": [
///        "native",
///        "token",
///        "percentage"
///      ],
///      "name": "Amount Type"
///    },
///    "value": {
///      "description": "The amount value as a string",
///      "type": "string",
///      "name": "Amount Value"
///    }
///  },
///  "name": "Order Amount"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct PlaceLimitOrderRequestOrderAmount {
    ///Type of amount: native currency, token amount, or percentage
    #[serde(rename = "type")]
    pub type_: PlaceLimitOrderRequestOrderAmountType,
    ///The amount value as a string
    pub value: ::std::string::String,
}
///Type of amount: native currency, token amount, or percentage
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Type of amount: native currency, token amount, or percentage",
///  "type": "string",
///  "enum": [
///    "native",
///    "token",
///    "percentage"
///  ],
///  "name": "Amount Type"
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
pub enum PlaceLimitOrderRequestOrderAmountType {
    #[serde(rename = "native")]
    Native,
    #[serde(rename = "token")]
    Token,
    #[serde(rename = "percentage")]
    Percentage,
}
impl ::std::fmt::Display for PlaceLimitOrderRequestOrderAmountType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Native => f.write_str("native"),
            Self::Token => f.write_str("token"),
            Self::Percentage => f.write_str("percentage"),
        }
    }
}
impl ::std::str::FromStr for PlaceLimitOrderRequestOrderAmountType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "native" => Ok(Self::Native),
            "token" => Ok(Self::Token),
            "percentage" => Ok(Self::Percentage),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PlaceLimitOrderRequestOrderAmountType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for PlaceLimitOrderRequestOrderAmountType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for PlaceLimitOrderRequestOrderAmountType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///The side of the order: buy or sell the token
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "The side of the order: buy or sell the token",
///  "type": "string",
///  "enum": [
///    "buy",
///    "sell"
///  ],
///  "name": "Order Side"
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
pub enum PlaceLimitOrderRequestOrderSide {
    #[serde(rename = "buy")]
    Buy,
    #[serde(rename = "sell")]
    Sell,
}
impl ::std::fmt::Display for PlaceLimitOrderRequestOrderSide {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Buy => f.write_str("buy"),
            Self::Sell => f.write_str("sell"),
        }
    }
}
impl ::std::str::FromStr for PlaceLimitOrderRequestOrderSide {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "buy" => Ok(Self::Buy),
            "sell" => Ok(Self::Sell),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PlaceLimitOrderRequestOrderSide {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for PlaceLimitOrderRequestOrderSide {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PlaceLimitOrderRequestOrderSide {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///The unique identifier of the token to trade
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "The unique identifier of the token to trade",
///  "type": "object",
///  "required": [
///    "chainType",
///    "tokenChainId",
///    "tokenContractAddress"
///  ],
///  "properties": {
///    "chainType": {
///      "description": "The chain type of the token",
///      "type": "string",
///      "enum": [
///        "EVM",
///        "SVM"
///      ],
///      "name": "Chain Type"
///    },
///    "tokenChainId": {
///      "description": "The chain ID of the token",
///      "type": "string",
///      "name": "Token Chain ID"
///    },
///    "tokenContractAddress": {
///      "description": "The contract address of the token",
///      "type": "string",
///      "name": "Token Contract Address"
///    }
///  },
///  "name": "Token ID"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct PlaceLimitOrderRequestOrderTokenId {
    ///The chain type of the token
    #[serde(rename = "chainType")]
    pub chain_type: PlaceLimitOrderRequestOrderTokenIdChainType,
    ///The chain ID of the token
    #[serde(rename = "tokenChainId")]
    pub token_chain_id: ::std::string::String,
    ///The contract address of the token
    #[serde(rename = "tokenContractAddress")]
    pub token_contract_address: ::std::string::String,
}
///The chain type of the token
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "The chain type of the token",
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
pub enum PlaceLimitOrderRequestOrderTokenIdChainType {
    #[serde(rename = "EVM")]
    Evm,
    #[serde(rename = "SVM")]
    Svm,
}
impl ::std::fmt::Display for PlaceLimitOrderRequestOrderTokenIdChainType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Evm => f.write_str("EVM"),
            Self::Svm => f.write_str("SVM"),
        }
    }
}
impl ::std::str::FromStr for PlaceLimitOrderRequestOrderTokenIdChainType {
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
impl ::std::convert::TryFrom<&str> for PlaceLimitOrderRequestOrderTokenIdChainType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for PlaceLimitOrderRequestOrderTokenIdChainType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for PlaceLimitOrderRequestOrderTokenIdChainType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Transaction preset configuration
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Transaction preset configuration",
///  "type": "object",
///  "required": [
///    "bribe",
///    "key",
///    "maxBaseGas",
///    "method",
///    "priorityGas",
///    "slippage"
///  ],
///  "properties": {
///    "bribe": {
///      "description": "Bribe amount in lamports for SVM chains (as string)",
///      "type": "string",
///      "name": "Bribe"
///    },
///    "key": {
///      "description": "The trade preset key identifier",
///      "type": "string",
///      "enum": [
///        "a",
///        "b",
///        "c",
///        "d"
///      ],
///      "name": "Preset Key"
///    },
///    "maxBaseGas": {
///      "description": "Maximum base gas in wei (as string)",
///      "type": "string",
///      "name": "Max Base Gas"
///    },
///    "method": {
///      "description": "Trading method: normal or flashbot",
///      "type": "string",
///      "enum": [
///        "normal",
///        "flashbot"
///      ],
///      "name": "Trade Method"
///    },
///    "priorityGas": {
///      "description": "Priority gas in wei (as string)",
///      "type": "string",
///      "name": "Priority Gas"
///    },
///    "slippage": {
///      "description": "Slippage tolerance percentage",
///      "type": "number",
///      "name": "Slippage"
///    }
///  },
///  "name": "Transaction Preset"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct PlaceLimitOrderRequestOrderTxPreset {
    ///Bribe amount in lamports for SVM chains (as string)
    pub bribe: ::std::string::String,
    ///The trade preset key identifier
    pub key: PlaceLimitOrderRequestOrderTxPresetKey,
    ///Maximum base gas in wei (as string)
    #[serde(rename = "maxBaseGas")]
    pub max_base_gas: ::std::string::String,
    ///Trading method: normal or flashbot
    pub method: PlaceLimitOrderRequestOrderTxPresetMethod,
    ///Priority gas in wei (as string)
    #[serde(rename = "priorityGas")]
    pub priority_gas: ::std::string::String,
    pub slippage: f64,
}
///The trade preset key identifier
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "The trade preset key identifier",
///  "type": "string",
///  "enum": [
///    "a",
///    "b",
///    "c",
///    "d"
///  ],
///  "name": "Preset Key"
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
pub enum PlaceLimitOrderRequestOrderTxPresetKey {
    #[serde(rename = "a")]
    A,
    #[serde(rename = "b")]
    B,
    #[serde(rename = "c")]
    C,
    #[serde(rename = "d")]
    D,
}
impl ::std::fmt::Display for PlaceLimitOrderRequestOrderTxPresetKey {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::A => f.write_str("a"),
            Self::B => f.write_str("b"),
            Self::C => f.write_str("c"),
            Self::D => f.write_str("d"),
        }
    }
}
impl ::std::str::FromStr for PlaceLimitOrderRequestOrderTxPresetKey {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "a" => Ok(Self::A),
            "b" => Ok(Self::B),
            "c" => Ok(Self::C),
            "d" => Ok(Self::D),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PlaceLimitOrderRequestOrderTxPresetKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for PlaceLimitOrderRequestOrderTxPresetKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for PlaceLimitOrderRequestOrderTxPresetKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Trading method: normal or flashbot
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Trading method: normal or flashbot",
///  "type": "string",
///  "enum": [
///    "normal",
///    "flashbot"
///  ],
///  "name": "Trade Method"
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
pub enum PlaceLimitOrderRequestOrderTxPresetMethod {
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "flashbot")]
    Flashbot,
}
impl ::std::fmt::Display for PlaceLimitOrderRequestOrderTxPresetMethod {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Normal => f.write_str("normal"),
            Self::Flashbot => f.write_str("flashbot"),
        }
    }
}
impl ::std::str::FromStr for PlaceLimitOrderRequestOrderTxPresetMethod {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "normal" => Ok(Self::Normal),
            "flashbot" => Ok(Self::Flashbot),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PlaceLimitOrderRequestOrderTxPresetMethod {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for PlaceLimitOrderRequestOrderTxPresetMethod {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for PlaceLimitOrderRequestOrderTxPresetMethod {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`PlaceLimitOrderRequestOrderWalletsItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "address"
///  ],
///  "properties": {
///    "address": {
///      "description": "The wallet address",
///      "type": "string",
///      "name": "Wallet Address"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct PlaceLimitOrderRequestOrderWalletsItem {
    ///The wallet address
    pub address: ::std::string::String,
}
///`PlaceLimitOrderResponseItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "agentId",
///    "amount",
///    "chainId",
///    "counterTokenAddress",
///    "dateAdded",
///    "dateExpiry",
///    "dateTriggered",
///    "entryStrategyId",
///    "exitStrategyId",
///    "id",
///    "limitTokenMarketcapUsd",
///    "limitTokenPriceUsd",
///    "pairAddress",
///    "pairType",
///    "requestedAtTokenMarketcapUsd",
///    "requestedAtTokenPriceUsd",
///    "side",
///    "signalId",
///    "status",
///    "taskId",
///    "tokenAddress",
///    "transactionHash",
///    "triggeredAtTokenMarketcapNativeToken",
///    "triggeredAtTokenMarketcapUsd",
///    "triggeredAtTokenPriceNativeToken",
///    "triggeredAtTokenPriceUsd",
///    "txId",
///    "type",
///    "wallet"
///  ],
///  "properties": {
///    "agentId": {
///      "description": "The ID of the agent associated with the order",
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Agent ID"
///    },
///    "amount": {
///      "type": "object",
///      "required": [
///        "type",
///        "value"
///      ],
///      "properties": {
///        "type": {
///          "type": "string",
///          "enum": [
///            "native",
///            "token",
///            "percentage"
///          ]
///        },
///        "value": {
///          "type": "string"
///        }
///      },
///      "additionalProperties": false
///    },
///    "chainId": {
///      "description": "The blockchain chain ID of the order",
///      "type": "string",
///      "name": "Chain ID"
///    },
///    "counterTokenAddress": {
///      "description": "The address of the counter token",
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Counter Token Address"
///    },
///    "dateAdded": {
///      "description": "The timestamp when the order was added",
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Date Added"
///    },
///    "dateExpiry": {
///      "description": "The timestamp when the order expired",
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Date Expiry"
///    },
///    "dateTriggered": {
///      "description": "The timestamp when the order was triggered",
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Date Triggered"
///    },
///    "entryStrategyId": {
///      "description": "The ID of the entry strategy",
///      "anyOf": [
///        {
///          "type": "number"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Entry Strategy ID"
///    },
///    "exitStrategyId": {
///      "description": "The ID of the exit strategy",
///      "anyOf": [
///        {
///          "type": "number"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Exit Strategy ID"
///    },
///    "id": {
///      "description": "The ID of the order in format {taskType}_{taskId} (e.g., \"limit_123\" or \"spot_456\")",
///      "type": "string",
///      "name": "Order ID"
///    },
///    "limitTokenMarketcapUsd": {
///      "description": "The marketcap of the token at the time the order was placed",
///      "anyOf": [
///        {
///          "type": "number"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Limit Token Marketcap USD"
///    },
///    "limitTokenPriceUsd": {
///      "description": "The price of the token at the time the order was placed",
///      "anyOf": [
///        {
///          "type": "number"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Limit Token Price USD"
///    },
///    "pairAddress": {
///      "description": "The address of the pair",
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Pair Address"
///    },
///    "pairType": {
///      "description": "The type of the pair",
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Pair Type"
///    },
///    "requestedAtTokenMarketcapUsd": {
///      "description": "The marketcap of the token at the time the order was requested",
///      "anyOf": [
///        {
///          "type": "number"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Requested At Token Marketcap USD"
///    },
///    "requestedAtTokenPriceUsd": {
///      "description": "The price of the token at the time the order was requested",
///      "anyOf": [
///        {
///          "type": "number"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Requested At Token Price USD"
///    },
///    "side": {
///      "description": "The side of the order: \"buy\" or \"sell\"",
///      "type": "string",
///      "name": "Side"
///    },
///    "signalId": {
///      "description": "The ID of the signal associated with the order",
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Signal ID"
///    },
///    "status": {
///      "description": "The status of the order: \"Working\", \"Canceled\", \"Inactive\", \"Expired\", \"Placing\", \"Filled\", or \"Rejected\"",
///      "type": "string",
///      "name": "Order Status"
///    },
///    "taskId": {
///      "description": "The ID of the task associated with the order",
///      "type": "number",
///      "name": "Task ID"
///    },
///    "token": {
///      "type": "object",
///      "required": [
///        "bestPairAddress",
///        "bestPairCounterToken",
///        "bestPairCreatedAt",
///        "bestPairSymbol",
///        "bestPairType",
///        "createdAt",
///        "decimals",
///        "deployerAddress",
///        "effectiveSupply",
///        "liquidityUsd",
///        "logoUrl",
///        "marketcapUsd",
///        "name",
///        "socialLinks",
///        "symbol",
///        "tokenChainId",
///        "tokenContractAddress",
///        "tokenPriceNativeToken",
///        "tokenPriceUsd",
///        "tradingOpenedAt"
///      ],
///      "properties": {
///        "bestPairAddress": {
///          "description": "The address of the best pair for the token",
///          "anyOf": [
///            {
///              "type": "string"
///            },
///            {
///              "type": "null"
///            }
///          ],
///          "name": "Best Pair Address"
///        },
///        "bestPairCounterToken": {
///          "anyOf": [
///            {
///              "type": "object",
///              "required": [
///                "decimals",
///                "name",
///                "symbol",
///                "tokenChainId",
///                "tokenContractAddress"
///              ],
///              "properties": {
///                "decimals": {
///                  "description": "The number of decimals of the counter token",
///                  "type": "number",
///                  "name": "Decimals"
///                },
///                "name": {
///                  "description": "The name of the counter token",
///                  "anyOf": [
///                    {
///                      "type": "string"
///                    },
///                    {
///                      "type": "null"
///                    }
///                  ],
///                  "name": "Counter Token Name"
///                },
///                "symbol": {
///                  "description": "The symbol of the counter token",
///                  "anyOf": [
///                    {
///                      "type": "string"
///                    },
///                    {
///                      "type": "null"
///                    }
///                  ],
///                  "name": "Counter Token Symbol"
///                },
///                "tokenChainId": {
///                  "description": "The chain ID of the counter token",
///                  "type": "string",
///                  "name": "Counter Token Chain ID"
///                },
///                "tokenContractAddress": {
///                  "description": "The contract address of the counter token",
///                  "type": "string",
///                  "name": "Counter Token Contract Address"
///                }
///              },
///              "additionalProperties": false
///            },
///            {
///              "type": "null"
///            }
///          ]
///        },
///        "bestPairCreatedAt": {
///          "description": "The timestamp when the best pair was created",
///          "anyOf": [
///            {
///              "type": "string"
///            },
///            {
///              "type": "null"
///            }
///          ],
///          "name": "Best Pair Created At"
///        },
///        "bestPairSymbol": {
///          "description": "The symbol of the best pair for the token",
///          "anyOf": [
///            {
///              "type": "string"
///            },
///            {
///              "type": "null"
///            }
///          ],
///          "name": "Best Pair Symbol"
///        },
///        "bestPairType": {
///          "description": "The type of the best pair for the token",
///          "anyOf": [
///            {
///              "type": "string"
///            },
///            {
///              "type": "null"
///            }
///          ],
///          "name": "Best Pair Type"
///        },
///        "createdAt": {
///          "description": "The timestamp when the token was created",
///          "type": "string",
///          "name": "Created At"
///        },
///        "decimals": {
///          "description": "The number of decimals of the token",
///          "type": "number",
///          "name": "Decimals"
///        },
///        "deployerAddress": {
///          "description": "The address of the deployer of the token",
///          "type": "string",
///          "name": "Deployer Address"
///        },
///        "effectiveSupply": {
///          "description": "The effective supply of the token",
///          "anyOf": [
///            {
///              "type": "string"
///            },
///            {
///              "type": "null"
///            }
///          ],
///          "name": "Effective Supply"
///        },
///        "liquidityUsd": {
///          "description": "The liquidity of the token in USD",
///          "anyOf": [
///            {
///              "type": "number"
///            },
///            {
///              "type": "null"
///            }
///          ],
///          "name": "Liquidity USD"
///        },
///        "logoUrl": {
///          "description": "The URL of the logo of the token",
///          "anyOf": [
///            {
///              "type": "string"
///            },
///            {
///              "type": "null"
///            }
///          ],
///          "name": "Logo URL"
///        },
///        "marketcapUsd": {
///          "description": "The marketcap of the token in USD",
///          "anyOf": [
///            {
///              "type": "number"
///            },
///            {
///              "type": "null"
///            }
///          ],
///          "name": "Marketcap USD"
///        },
///        "name": {
///          "description": "The name of the token",
///          "anyOf": [
///            {
///              "type": "string"
///            },
///            {
///              "type": "null"
///            }
///          ],
///          "name": "Token Name"
///        },
///        "socialLinks": {
///          "type": "array",
///          "items": {
///            "type": "object",
///            "required": [
///              "platform",
///              "url"
///            ],
///            "properties": {
///              "platform": {
///                "description": "The platform of the social link",
///                "type": "string",
///                "name": "Platform"
///              },
///              "url": {
///                "description": "The URL of the social link",
///                "type": "string",
///                "name": "URL"
///              }
///            },
///            "additionalProperties": false
///          }
///        },
///        "symbol": {
///          "description": "The symbol of the token",
///          "anyOf": [
///            {
///              "type": "string"
///            },
///            {
///              "type": "null"
///            }
///          ],
///          "name": "Token Symbol"
///        },
///        "tokenChainId": {
///          "description": "The chain ID of the token",
///          "type": "string",
///          "name": "Token Chain ID"
///        },
///        "tokenContractAddress": {
///          "description": "The contract address of the token",
///          "type": "string",
///          "name": "Token Contract Address"
///        },
///        "tokenPriceNativeToken": {
///          "description": "The price of the token in native tokens",
///          "anyOf": [
///            {
///              "type": "number"
///            },
///            {
///              "type": "null"
///            }
///          ],
///          "name": "Token Price Native Token"
///        },
///        "tokenPriceUsd": {
///          "description": "The price of the token in USD",
///          "anyOf": [
///            {
///              "type": "number"
///            },
///            {
///              "type": "null"
///            }
///          ],
///          "name": "Token Price USD"
///        },
///        "tradingOpenedAt": {
///          "description": "The timestamp when the trading was opened for the token",
///          "anyOf": [
///            {
///              "type": "string"
///            },
///            {
///              "type": "null"
///            }
///          ],
///          "name": "Trading Opened At"
///        }
///      },
///      "additionalProperties": false
///    },
///    "tokenAddress": {
///      "description": "The address of the token",
///      "type": "string",
///      "name": "Token Address"
///    },
///    "transactionHash": {
///      "description": "The hash of the transaction",
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Transaction Hash"
///    },
///    "triggeredAtTokenMarketcapNativeToken": {
///      "description": "The marketcap of the token in native tokens at the time the order was triggered",
///      "anyOf": [
///        {
///          "type": "number"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Triggered At Token Marketcap Native Token"
///    },
///    "triggeredAtTokenMarketcapUsd": {
///      "description": "The marketcap of the token at the time the order was triggered",
///      "anyOf": [
///        {
///          "type": "number"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Triggered At Token Marketcap USD"
///    },
///    "triggeredAtTokenPriceNativeToken": {
///      "description": "The price of the token in native tokens at the time the order was triggered",
///      "anyOf": [
///        {
///          "type": "number"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Triggered At Token Price Native Token"
///    },
///    "triggeredAtTokenPriceUsd": {
///      "description": "The price of the token at the time the order was triggered",
///      "anyOf": [
///        {
///          "type": "number"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Triggered At Token Price USD"
///    },
///    "txId": {
///      "description": "The ID of the transaction associated with the order",
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Transaction ID"
///    },
///    "type": {
///      "description": "The type of the order: \"limit\" or \"spot\"",
///      "type": "string",
///      "name": "Order Type"
///    },
///    "wallet": {
///      "description": "The wallet address associated with the order",
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Wallet Address"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct PlaceLimitOrderResponseItem {
    ///The ID of the agent associated with the order
    #[serde(rename = "agentId")]
    pub agent_id: ::std::option::Option<::std::string::String>,
    pub amount: PlaceLimitOrderResponseItemAmount,
    ///The blockchain chain ID of the order
    #[serde(rename = "chainId")]
    pub chain_id: ::std::string::String,
    ///The address of the counter token
    #[serde(rename = "counterTokenAddress")]
    pub counter_token_address: ::std::option::Option<::std::string::String>,
    ///The timestamp when the order was added
    #[serde(rename = "dateAdded")]
    pub date_added: ::std::option::Option<::std::string::String>,
    ///The timestamp when the order expired
    #[serde(rename = "dateExpiry")]
    pub date_expiry: ::std::option::Option<::std::string::String>,
    ///The timestamp when the order was triggered
    #[serde(rename = "dateTriggered")]
    pub date_triggered: ::std::option::Option<::std::string::String>,
    ///The ID of the entry strategy
    #[serde(rename = "entryStrategyId")]
    pub entry_strategy_id: ::std::option::Option<f64>,
    ///The ID of the exit strategy
    #[serde(rename = "exitStrategyId")]
    pub exit_strategy_id: ::std::option::Option<f64>,
    ///The ID of the order in format {taskType}_{taskId} (e.g., "limit_123" or "spot_456")
    pub id: ::std::string::String,
    ///The marketcap of the token at the time the order was placed
    #[serde(rename = "limitTokenMarketcapUsd")]
    pub limit_token_marketcap_usd: ::std::option::Option<f64>,
    ///The price of the token at the time the order was placed
    #[serde(rename = "limitTokenPriceUsd")]
    pub limit_token_price_usd: ::std::option::Option<f64>,
    ///The address of the pair
    #[serde(rename = "pairAddress")]
    pub pair_address: ::std::option::Option<::std::string::String>,
    ///The type of the pair
    #[serde(rename = "pairType")]
    pub pair_type: ::std::option::Option<::std::string::String>,
    ///The marketcap of the token at the time the order was requested
    #[serde(rename = "requestedAtTokenMarketcapUsd")]
    pub requested_at_token_marketcap_usd: ::std::option::Option<f64>,
    ///The price of the token at the time the order was requested
    #[serde(rename = "requestedAtTokenPriceUsd")]
    pub requested_at_token_price_usd: ::std::option::Option<f64>,
    ///The side of the order: "buy" or "sell"
    pub side: ::std::string::String,
    ///The ID of the signal associated with the order
    #[serde(rename = "signalId")]
    pub signal_id: ::std::option::Option<::std::string::String>,
    ///The status of the order: "Working", "Canceled", "Inactive", "Expired", "Placing", "Filled", or "Rejected"
    pub status: ::std::string::String,
    #[serde(rename = "taskId")]
    pub task_id: f64,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub token: ::std::option::Option<PlaceLimitOrderResponseItemToken>,
    ///The address of the token
    #[serde(rename = "tokenAddress")]
    pub token_address: ::std::string::String,
    ///The hash of the transaction
    #[serde(rename = "transactionHash")]
    pub transaction_hash: ::std::option::Option<::std::string::String>,
    ///The marketcap of the token in native tokens at the time the order was triggered
    #[serde(rename = "triggeredAtTokenMarketcapNativeToken")]
    pub triggered_at_token_marketcap_native_token: ::std::option::Option<f64>,
    ///The marketcap of the token at the time the order was triggered
    #[serde(rename = "triggeredAtTokenMarketcapUsd")]
    pub triggered_at_token_marketcap_usd: ::std::option::Option<f64>,
    ///The price of the token in native tokens at the time the order was triggered
    #[serde(rename = "triggeredAtTokenPriceNativeToken")]
    pub triggered_at_token_price_native_token: ::std::option::Option<f64>,
    ///The price of the token at the time the order was triggered
    #[serde(rename = "triggeredAtTokenPriceUsd")]
    pub triggered_at_token_price_usd: ::std::option::Option<f64>,
    ///The ID of the transaction associated with the order
    #[serde(rename = "txId")]
    pub tx_id: ::std::option::Option<::std::string::String>,
    ///The type of the order: "limit" or "spot"
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
    ///The wallet address associated with the order
    pub wallet: ::std::option::Option<::std::string::String>,
}
///`PlaceLimitOrderResponseItemAmount`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "type",
///    "value"
///  ],
///  "properties": {
///    "type": {
///      "type": "string",
///      "enum": [
///        "native",
///        "token",
///        "percentage"
///      ]
///    },
///    "value": {
///      "type": "string"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct PlaceLimitOrderResponseItemAmount {
    #[serde(rename = "type")]
    pub type_: PlaceLimitOrderResponseItemAmountType,
    pub value: ::std::string::String,
}
///`PlaceLimitOrderResponseItemAmountType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "native",
///    "token",
///    "percentage"
///  ]
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
pub enum PlaceLimitOrderResponseItemAmountType {
    #[serde(rename = "native")]
    Native,
    #[serde(rename = "token")]
    Token,
    #[serde(rename = "percentage")]
    Percentage,
}
impl ::std::fmt::Display for PlaceLimitOrderResponseItemAmountType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Native => f.write_str("native"),
            Self::Token => f.write_str("token"),
            Self::Percentage => f.write_str("percentage"),
        }
    }
}
impl ::std::str::FromStr for PlaceLimitOrderResponseItemAmountType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "native" => Ok(Self::Native),
            "token" => Ok(Self::Token),
            "percentage" => Ok(Self::Percentage),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PlaceLimitOrderResponseItemAmountType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for PlaceLimitOrderResponseItemAmountType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for PlaceLimitOrderResponseItemAmountType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`PlaceLimitOrderResponseItemToken`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "bestPairAddress",
///    "bestPairCounterToken",
///    "bestPairCreatedAt",
///    "bestPairSymbol",
///    "bestPairType",
///    "createdAt",
///    "decimals",
///    "deployerAddress",
///    "effectiveSupply",
///    "liquidityUsd",
///    "logoUrl",
///    "marketcapUsd",
///    "name",
///    "socialLinks",
///    "symbol",
///    "tokenChainId",
///    "tokenContractAddress",
///    "tokenPriceNativeToken",
///    "tokenPriceUsd",
///    "tradingOpenedAt"
///  ],
///  "properties": {
///    "bestPairAddress": {
///      "description": "The address of the best pair for the token",
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Best Pair Address"
///    },
///    "bestPairCounterToken": {
///      "anyOf": [
///        {
///          "type": "object",
///          "required": [
///            "decimals",
///            "name",
///            "symbol",
///            "tokenChainId",
///            "tokenContractAddress"
///          ],
///          "properties": {
///            "decimals": {
///              "description": "The number of decimals of the counter token",
///              "type": "number",
///              "name": "Decimals"
///            },
///            "name": {
///              "description": "The name of the counter token",
///              "anyOf": [
///                {
///                  "type": "string"
///                },
///                {
///                  "type": "null"
///                }
///              ],
///              "name": "Counter Token Name"
///            },
///            "symbol": {
///              "description": "The symbol of the counter token",
///              "anyOf": [
///                {
///                  "type": "string"
///                },
///                {
///                  "type": "null"
///                }
///              ],
///              "name": "Counter Token Symbol"
///            },
///            "tokenChainId": {
///              "description": "The chain ID of the counter token",
///              "type": "string",
///              "name": "Counter Token Chain ID"
///            },
///            "tokenContractAddress": {
///              "description": "The contract address of the counter token",
///              "type": "string",
///              "name": "Counter Token Contract Address"
///            }
///          },
///          "additionalProperties": false
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "bestPairCreatedAt": {
///      "description": "The timestamp when the best pair was created",
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Best Pair Created At"
///    },
///    "bestPairSymbol": {
///      "description": "The symbol of the best pair for the token",
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Best Pair Symbol"
///    },
///    "bestPairType": {
///      "description": "The type of the best pair for the token",
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Best Pair Type"
///    },
///    "createdAt": {
///      "description": "The timestamp when the token was created",
///      "type": "string",
///      "name": "Created At"
///    },
///    "decimals": {
///      "description": "The number of decimals of the token",
///      "type": "number",
///      "name": "Decimals"
///    },
///    "deployerAddress": {
///      "description": "The address of the deployer of the token",
///      "type": "string",
///      "name": "Deployer Address"
///    },
///    "effectiveSupply": {
///      "description": "The effective supply of the token",
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Effective Supply"
///    },
///    "liquidityUsd": {
///      "description": "The liquidity of the token in USD",
///      "anyOf": [
///        {
///          "type": "number"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Liquidity USD"
///    },
///    "logoUrl": {
///      "description": "The URL of the logo of the token",
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Logo URL"
///    },
///    "marketcapUsd": {
///      "description": "The marketcap of the token in USD",
///      "anyOf": [
///        {
///          "type": "number"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Marketcap USD"
///    },
///    "name": {
///      "description": "The name of the token",
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Token Name"
///    },
///    "socialLinks": {
///      "type": "array",
///      "items": {
///        "type": "object",
///        "required": [
///          "platform",
///          "url"
///        ],
///        "properties": {
///          "platform": {
///            "description": "The platform of the social link",
///            "type": "string",
///            "name": "Platform"
///          },
///          "url": {
///            "description": "The URL of the social link",
///            "type": "string",
///            "name": "URL"
///          }
///        },
///        "additionalProperties": false
///      }
///    },
///    "symbol": {
///      "description": "The symbol of the token",
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Token Symbol"
///    },
///    "tokenChainId": {
///      "description": "The chain ID of the token",
///      "type": "string",
///      "name": "Token Chain ID"
///    },
///    "tokenContractAddress": {
///      "description": "The contract address of the token",
///      "type": "string",
///      "name": "Token Contract Address"
///    },
///    "tokenPriceNativeToken": {
///      "description": "The price of the token in native tokens",
///      "anyOf": [
///        {
///          "type": "number"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Token Price Native Token"
///    },
///    "tokenPriceUsd": {
///      "description": "The price of the token in USD",
///      "anyOf": [
///        {
///          "type": "number"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Token Price USD"
///    },
///    "tradingOpenedAt": {
///      "description": "The timestamp when the trading was opened for the token",
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Trading Opened At"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct PlaceLimitOrderResponseItemToken {
    ///The address of the best pair for the token
    #[serde(rename = "bestPairAddress")]
    pub best_pair_address: ::std::option::Option<::std::string::String>,
    #[serde(rename = "bestPairCounterToken")]
    pub best_pair_counter_token: ::std::option::Option<
        PlaceLimitOrderResponseItemTokenBestPairCounterToken,
    >,
    ///The timestamp when the best pair was created
    #[serde(rename = "bestPairCreatedAt")]
    pub best_pair_created_at: ::std::option::Option<::std::string::String>,
    ///The symbol of the best pair for the token
    #[serde(rename = "bestPairSymbol")]
    pub best_pair_symbol: ::std::option::Option<::std::string::String>,
    ///The type of the best pair for the token
    #[serde(rename = "bestPairType")]
    pub best_pair_type: ::std::option::Option<::std::string::String>,
    ///The timestamp when the token was created
    #[serde(rename = "createdAt")]
    pub created_at: ::std::string::String,
    pub decimals: f64,
    ///The address of the deployer of the token
    #[serde(rename = "deployerAddress")]
    pub deployer_address: ::std::string::String,
    ///The effective supply of the token
    #[serde(rename = "effectiveSupply")]
    pub effective_supply: ::std::option::Option<::std::string::String>,
    ///The liquidity of the token in USD
    #[serde(rename = "liquidityUsd")]
    pub liquidity_usd: ::std::option::Option<f64>,
    ///The URL of the logo of the token
    #[serde(rename = "logoUrl")]
    pub logo_url: ::std::option::Option<::std::string::String>,
    ///The marketcap of the token in USD
    #[serde(rename = "marketcapUsd")]
    pub marketcap_usd: ::std::option::Option<f64>,
    ///The name of the token
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "socialLinks")]
    pub social_links: ::std::vec::Vec<PlaceLimitOrderResponseItemTokenSocialLinksItem>,
    ///The symbol of the token
    pub symbol: ::std::option::Option<::std::string::String>,
    ///The chain ID of the token
    #[serde(rename = "tokenChainId")]
    pub token_chain_id: ::std::string::String,
    ///The contract address of the token
    #[serde(rename = "tokenContractAddress")]
    pub token_contract_address: ::std::string::String,
    ///The price of the token in native tokens
    #[serde(rename = "tokenPriceNativeToken")]
    pub token_price_native_token: ::std::option::Option<f64>,
    ///The price of the token in USD
    #[serde(rename = "tokenPriceUsd")]
    pub token_price_usd: ::std::option::Option<f64>,
    ///The timestamp when the trading was opened for the token
    #[serde(rename = "tradingOpenedAt")]
    pub trading_opened_at: ::std::option::Option<::std::string::String>,
}
///`PlaceLimitOrderResponseItemTokenBestPairCounterToken`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "decimals",
///    "name",
///    "symbol",
///    "tokenChainId",
///    "tokenContractAddress"
///  ],
///  "properties": {
///    "decimals": {
///      "description": "The number of decimals of the counter token",
///      "type": "number",
///      "name": "Decimals"
///    },
///    "name": {
///      "description": "The name of the counter token",
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Counter Token Name"
///    },
///    "symbol": {
///      "description": "The symbol of the counter token",
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Counter Token Symbol"
///    },
///    "tokenChainId": {
///      "description": "The chain ID of the counter token",
///      "type": "string",
///      "name": "Counter Token Chain ID"
///    },
///    "tokenContractAddress": {
///      "description": "The contract address of the counter token",
///      "type": "string",
///      "name": "Counter Token Contract Address"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct PlaceLimitOrderResponseItemTokenBestPairCounterToken {
    pub decimals: f64,
    ///The name of the counter token
    pub name: ::std::option::Option<::std::string::String>,
    ///The symbol of the counter token
    pub symbol: ::std::option::Option<::std::string::String>,
    ///The chain ID of the counter token
    #[serde(rename = "tokenChainId")]
    pub token_chain_id: ::std::string::String,
    ///The contract address of the counter token
    #[serde(rename = "tokenContractAddress")]
    pub token_contract_address: ::std::string::String,
}
///`PlaceLimitOrderResponseItemTokenSocialLinksItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "platform",
///    "url"
///  ],
///  "properties": {
///    "platform": {
///      "description": "The platform of the social link",
///      "type": "string",
///      "name": "Platform"
///    },
///    "url": {
///      "description": "The URL of the social link",
///      "type": "string",
///      "name": "URL"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct PlaceLimitOrderResponseItemTokenSocialLinksItem {
    ///The platform of the social link
    pub platform: ::std::string::String,
    ///The URL of the social link
    pub url: ::std::string::String,
}
