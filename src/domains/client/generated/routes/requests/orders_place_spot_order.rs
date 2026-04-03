#![allow(clippy::derivable_impls)]

use crate::client::{Route, RouteType};
use std::marker::PhantomData;
/// Route metadata for this procedure
pub const ROUTE: Route<PlaceSpotOrderRequest, Vec<PlaceSpotOrderResponseItem>> = Route {
    procedure: "orders.placeSpotOrder",
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
///`PlaceSpotOrderRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "envelope",
///    "order"
///  ],
///  "properties": {
///    "envelope": {
///      "description": "The encrypted envelope for the edge vault",
///      "type": "string",
///      "format": "base64",
///      "pattern": "^$|^(?:[0-9a-zA-Z+/]{4})*(?:(?:[0-9a-zA-Z+/]{2}==)|(?:[0-9a-zA-Z+/]{3}=))?$",
///      "contentEncoding": "base64",
///      "name": "Envelope"
///    },
///    "order": {
///      "description": "The order to place",
///      "type": "object",
///      "required": [
///        "amount",
///        "chainId",
///        "exitStrategyId",
///        "side",
///        "txPreset"
///      ],
///      "properties": {
///        "amount": {
///          "description": "The amount of the order; a discriminated union of native, token, and percentage amounts",
///          "oneOf": [
///            {
///              "description": "The amount of the order in native tokens; must be in base unit amount (eg, wei, lamports, etc.)",
///              "type": "object",
///              "required": [
///                "type",
///                "value"
///              ],
///              "properties": {
///                "type": {
///                  "type": "string",
///                  "const": "native"
///                },
///                "value": {
///                  "description": "The amount of the order in native tokens; stringified; must be in base unit amount (eg, wei, lamports, etc.)",
///                  "type": "string",
///                  "name": "Value"
///                }
///              },
///              "name": "Native Amount"
///            },
///            {
///              "description": "The amount of the order in tokens",
///              "type": "object",
///              "required": [
///                "type",
///                "value"
///              ],
///              "properties": {
///                "type": {
///                  "type": "string",
///                  "const": "token"
///                },
///                "value": {
///                  "description": "The amount of the order in tokens; stringified; must be in base unit amount (eg, like wei, lamports, etc. would be).\n\nExample: if you want to buy/sell 1000 tokens and the token has 6 decimals, you would pass \"1000000000\".",
///                  "type": "string",
///                  "name": "Value"
///                }
///              },
///              "name": "Token Amount"
///            },
///            {
///              "description": "The amount of the order as a percentage; must be between 0 and 100",
///              "type": "object",
///              "required": [
///                "type",
///                "value"
///              ],
///              "properties": {
///                "type": {
///                  "type": "string",
///                  "const": "percentage"
///                },
///                "value": {
///                  "description": "The amount of the order as a percentage; stringified; must be between 0 and 100. Only applies to sell orders.",
///                  "type": "string",
///                  "name": "Value"
///                }
///              },
///              "name": "Percentage Amount"
///            }
///          ],
///          "name": "Amount"
///        },
///        "chainId": {
///          "description": "The chain ID of the token",
///          "type": "string",
///          "name": "Chain ID"
///        },
///        "exitStrategyId": {
///          "description": "The ID of the exit strategy to use for the order (optional)",
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
///        "pairContractAddress": {
///          "description": "The contract address of the pair; optional however one of pairContractAddress or tokenContractAddress must be provided",
///          "type": "string",
///          "name": "Pair Contract Address"
///        },
///        "side": {
///          "description": "The side of the order; must be \"buy\" or \"sell\"",
///          "type": "string",
///          "enum": [
///            "buy",
///            "sell"
///          ],
///          "name": "Side"
///        },
///        "tokenContractAddress": {
///          "description": "The contract address of the token; optional however one of pairContractAddress or tokenContractAddress must be provided",
///          "type": "string",
///          "name": "Token Contract Address"
///        },
///        "txPreset": {
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
///              "description": "The bribe for the order",
///              "type": "string",
///              "name": "Bribe"
///            },
///            "key": {
///              "description": "The key for the transaction preset",
///              "type": "string",
///              "name": "Preset Key"
///            },
///            "maxBaseGas": {
///              "description": "The maximum base gas for the order",
///              "type": "string",
///              "name": "Max Base Gas"
///            },
///            "method": {
///              "description": "The method for the order; must be \"flashbot\" or \"normal\". Only applies to EVM chains.",
///              "type": "string",
///              "enum": [
///                "flashbot",
///                "normal"
///              ],
///              "name": "Method"
///            },
///            "priorityGas": {
///              "description": "The priority gas for the order",
///              "type": "string",
///              "name": "Priority Gas"
///            },
///            "slippage": {
///              "description": "The slippage for the order",
///              "type": "string",
///              "name": "Slippage"
///            }
///          }
///        }
///      },
///      "name": "Order"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct PlaceSpotOrderRequest {
    ///The encrypted envelope for the edge vault
    pub envelope: ::std::string::String,
    pub order: PlaceSpotOrderRequestOrder,
}
///The order to place
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "The order to place",
///  "type": "object",
///  "required": [
///    "amount",
///    "chainId",
///    "exitStrategyId",
///    "side",
///    "txPreset"
///  ],
///  "properties": {
///    "amount": {
///      "description": "The amount of the order; a discriminated union of native, token, and percentage amounts",
///      "oneOf": [
///        {
///          "description": "The amount of the order in native tokens; must be in base unit amount (eg, wei, lamports, etc.)",
///          "type": "object",
///          "required": [
///            "type",
///            "value"
///          ],
///          "properties": {
///            "type": {
///              "type": "string",
///              "const": "native"
///            },
///            "value": {
///              "description": "The amount of the order in native tokens; stringified; must be in base unit amount (eg, wei, lamports, etc.)",
///              "type": "string",
///              "name": "Value"
///            }
///          },
///          "name": "Native Amount"
///        },
///        {
///          "description": "The amount of the order in tokens",
///          "type": "object",
///          "required": [
///            "type",
///            "value"
///          ],
///          "properties": {
///            "type": {
///              "type": "string",
///              "const": "token"
///            },
///            "value": {
///              "description": "The amount of the order in tokens; stringified; must be in base unit amount (eg, like wei, lamports, etc. would be).\n\nExample: if you want to buy/sell 1000 tokens and the token has 6 decimals, you would pass \"1000000000\".",
///              "type": "string",
///              "name": "Value"
///            }
///          },
///          "name": "Token Amount"
///        },
///        {
///          "description": "The amount of the order as a percentage; must be between 0 and 100",
///          "type": "object",
///          "required": [
///            "type",
///            "value"
///          ],
///          "properties": {
///            "type": {
///              "type": "string",
///              "const": "percentage"
///            },
///            "value": {
///              "description": "The amount of the order as a percentage; stringified; must be between 0 and 100. Only applies to sell orders.",
///              "type": "string",
///              "name": "Value"
///            }
///          },
///          "name": "Percentage Amount"
///        }
///      ],
///      "name": "Amount"
///    },
///    "chainId": {
///      "description": "The chain ID of the token",
///      "type": "string",
///      "name": "Chain ID"
///    },
///    "exitStrategyId": {
///      "description": "The ID of the exit strategy to use for the order (optional)",
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
///    "pairContractAddress": {
///      "description": "The contract address of the pair; optional however one of pairContractAddress or tokenContractAddress must be provided",
///      "type": "string",
///      "name": "Pair Contract Address"
///    },
///    "side": {
///      "description": "The side of the order; must be \"buy\" or \"sell\"",
///      "type": "string",
///      "enum": [
///        "buy",
///        "sell"
///      ],
///      "name": "Side"
///    },
///    "tokenContractAddress": {
///      "description": "The contract address of the token; optional however one of pairContractAddress or tokenContractAddress must be provided",
///      "type": "string",
///      "name": "Token Contract Address"
///    },
///    "txPreset": {
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
///          "description": "The bribe for the order",
///          "type": "string",
///          "name": "Bribe"
///        },
///        "key": {
///          "description": "The key for the transaction preset",
///          "type": "string",
///          "name": "Preset Key"
///        },
///        "maxBaseGas": {
///          "description": "The maximum base gas for the order",
///          "type": "string",
///          "name": "Max Base Gas"
///        },
///        "method": {
///          "description": "The method for the order; must be \"flashbot\" or \"normal\". Only applies to EVM chains.",
///          "type": "string",
///          "enum": [
///            "flashbot",
///            "normal"
///          ],
///          "name": "Method"
///        },
///        "priorityGas": {
///          "description": "The priority gas for the order",
///          "type": "string",
///          "name": "Priority Gas"
///        },
///        "slippage": {
///          "description": "The slippage for the order",
///          "type": "string",
///          "name": "Slippage"
///        }
///      }
///    }
///  },
///  "name": "Order"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct PlaceSpotOrderRequestOrder {
    ///The amount of the order; a discriminated union of native, token, and percentage amounts
    pub amount: PlaceSpotOrderRequestOrderAmount,
    ///The chain ID of the token
    #[serde(rename = "chainId")]
    pub chain_id: ::std::string::String,
    ///The ID of the exit strategy to use for the order (optional)
    #[serde(rename = "exitStrategyId")]
    pub exit_strategy_id: ::std::option::Option<f64>,
    ///The contract address of the pair; optional however one of pairContractAddress or tokenContractAddress must be provided
    #[serde(
        rename = "pairContractAddress",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub pair_contract_address: ::std::option::Option<::std::string::String>,
    ///The side of the order; must be "buy" or "sell"
    pub side: PlaceSpotOrderRequestOrderSide,
    ///The contract address of the token; optional however one of pairContractAddress or tokenContractAddress must be provided
    #[serde(
        rename = "tokenContractAddress",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub token_contract_address: ::std::option::Option<::std::string::String>,
    #[serde(rename = "txPreset")]
    pub tx_preset: PlaceSpotOrderRequestOrderTxPreset,
}
///The amount of the order; a discriminated union of native, token, and percentage amounts
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "The amount of the order; a discriminated union of native, token, and percentage amounts",
///  "oneOf": [
///    {
///      "description": "The amount of the order in native tokens; must be in base unit amount (eg, wei, lamports, etc.)",
///      "type": "object",
///      "required": [
///        "type",
///        "value"
///      ],
///      "properties": {
///        "type": {
///          "type": "string",
///          "const": "native"
///        },
///        "value": {
///          "description": "The amount of the order in native tokens; stringified; must be in base unit amount (eg, wei, lamports, etc.)",
///          "type": "string",
///          "name": "Value"
///        }
///      },
///      "name": "Native Amount"
///    },
///    {
///      "description": "The amount of the order in tokens",
///      "type": "object",
///      "required": [
///        "type",
///        "value"
///      ],
///      "properties": {
///        "type": {
///          "type": "string",
///          "const": "token"
///        },
///        "value": {
///          "description": "The amount of the order in tokens; stringified; must be in base unit amount (eg, like wei, lamports, etc. would be).\n\nExample: if you want to buy/sell 1000 tokens and the token has 6 decimals, you would pass \"1000000000\".",
///          "type": "string",
///          "name": "Value"
///        }
///      },
///      "name": "Token Amount"
///    },
///    {
///      "description": "The amount of the order as a percentage; must be between 0 and 100",
///      "type": "object",
///      "required": [
///        "type",
///        "value"
///      ],
///      "properties": {
///        "type": {
///          "type": "string",
///          "const": "percentage"
///        },
///        "value": {
///          "description": "The amount of the order as a percentage; stringified; must be between 0 and 100. Only applies to sell orders.",
///          "type": "string",
///          "name": "Value"
///        }
///      },
///      "name": "Percentage Amount"
///    }
///  ],
///  "name": "Amount"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(tag = "type", content = "value")]
pub enum PlaceSpotOrderRequestOrderAmount {
    ///The amount of the order in native tokens; must be in base unit amount (eg, wei, lamports, etc.)
    #[serde(rename = "native")]
    Native(::std::string::String),
    ///The amount of the order in tokens
    #[serde(rename = "token")]
    Token(::std::string::String),
    ///The amount of the order as a percentage; must be between 0 and 100
    #[serde(rename = "percentage")]
    Percentage(::std::string::String),
}
///The side of the order; must be "buy" or "sell"
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "The side of the order; must be \"buy\" or \"sell\"",
///  "type": "string",
///  "enum": [
///    "buy",
///    "sell"
///  ],
///  "name": "Side"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PlaceSpotOrderRequestOrderSide {
    #[serde(rename = "buy")]
    Buy,
    #[serde(rename = "sell")]
    Sell,
}
impl ::std::fmt::Display for PlaceSpotOrderRequestOrderSide {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Buy => f.write_str("buy"),
            Self::Sell => f.write_str("sell"),
        }
    }
}
impl ::std::str::FromStr for PlaceSpotOrderRequestOrderSide {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "buy" => Ok(Self::Buy),
            "sell" => Ok(Self::Sell),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PlaceSpotOrderRequestOrderSide {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for PlaceSpotOrderRequestOrderSide {
    type Error = self::error::ConversionError;
    fn try_from(value: &::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PlaceSpotOrderRequestOrderSide {
    type Error = self::error::ConversionError;
    fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`PlaceSpotOrderRequestOrderTxPreset`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
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
///      "description": "The bribe for the order",
///      "type": "string",
///      "name": "Bribe"
///    },
///    "key": {
///      "description": "The key for the transaction preset",
///      "type": "string",
///      "name": "Preset Key"
///    },
///    "maxBaseGas": {
///      "description": "The maximum base gas for the order",
///      "type": "string",
///      "name": "Max Base Gas"
///    },
///    "method": {
///      "description": "The method for the order; must be \"flashbot\" or \"normal\". Only applies to EVM chains.",
///      "type": "string",
///      "enum": [
///        "flashbot",
///        "normal"
///      ],
///      "name": "Method"
///    },
///    "priorityGas": {
///      "description": "The priority gas for the order",
///      "type": "string",
///      "name": "Priority Gas"
///    },
///    "slippage": {
///      "description": "The slippage for the order",
///      "type": "string",
///      "name": "Slippage"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct PlaceSpotOrderRequestOrderTxPreset {
    ///The bribe for the order
    pub bribe: ::std::string::String,
    ///The key for the transaction preset
    pub key: ::std::string::String,
    ///The maximum base gas for the order
    #[serde(rename = "maxBaseGas")]
    pub max_base_gas: ::std::string::String,
    ///The method for the order; must be "flashbot" or "normal". Only applies to EVM chains.
    pub method: PlaceSpotOrderRequestOrderTxPresetMethod,
    ///The priority gas for the order
    #[serde(rename = "priorityGas")]
    pub priority_gas: ::std::string::String,
    ///The slippage for the order
    pub slippage: ::std::string::String,
}
///The method for the order; must be "flashbot" or "normal". Only applies to EVM chains.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "The method for the order; must be \"flashbot\" or \"normal\". Only applies to EVM chains.",
///  "type": "string",
///  "enum": [
///    "flashbot",
///    "normal"
///  ],
///  "name": "Method"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PlaceSpotOrderRequestOrderTxPresetMethod {
    #[serde(rename = "flashbot")]
    Flashbot,
    #[serde(rename = "normal")]
    Normal,
}
impl ::std::fmt::Display for PlaceSpotOrderRequestOrderTxPresetMethod {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Flashbot => f.write_str("flashbot"),
            Self::Normal => f.write_str("normal"),
        }
    }
}
impl ::std::str::FromStr for PlaceSpotOrderRequestOrderTxPresetMethod {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "flashbot" => Ok(Self::Flashbot),
            "normal" => Ok(Self::Normal),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PlaceSpotOrderRequestOrderTxPresetMethod {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for PlaceSpotOrderRequestOrderTxPresetMethod {
    type Error = self::error::ConversionError;
    fn try_from(value: &::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PlaceSpotOrderRequestOrderTxPresetMethod {
    type Error = self::error::ConversionError;
    fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`PlaceSpotOrderResponseItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "transactions",
///    "wallet"
///  ],
///  "properties": {
///    "transactions": {
///      "description": "Gives a list of transactions for the wallet, because a single spot order request may be executed via multiple transactions",
///      "type": "array",
///      "items": {
///        "anyOf": [
///          {
///            "description": "Gives the hash of the successful transaction",
///            "type": "object",
///            "required": [
///              "error",
///              "hash"
///            ],
///            "properties": {
///              "error": {
///                "type": "null"
///              },
///              "hash": {
///                "type": "string"
///              }
///            },
///            "additionalProperties": false,
///            "name": "Successful Transaction"
///          },
///          {
///            "description": "Gives the error message of the failed transaction",
///            "type": "object",
///            "required": [
///              "error",
///              "hash"
///            ],
///            "properties": {
///              "error": {
///                "type": "string"
///              },
///              "hash": {
///                "type": "null"
///              }
///            },
///            "additionalProperties": false,
///            "name": "Failed Transaction"
///          }
///        ]
///      },
///      "name": "Wallet Transactions"
///    },
///    "wallet": {
///      "type": "string"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct PlaceSpotOrderResponseItem {
    ///Gives a list of transactions for the wallet, because a single spot order request may be executed via multiple transactions
    pub transactions: ::std::vec::Vec<PlaceSpotOrderResponseItemTransactionsItem>,
    pub wallet: ::std::string::String,
}
///`PlaceSpotOrderResponseItemTransactionsItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "anyOf": [
///    {
///      "description": "Gives the hash of the successful transaction",
///      "type": "object",
///      "required": [
///        "error",
///        "hash"
///      ],
///      "properties": {
///        "error": {
///          "type": "null"
///        },
///        "hash": {
///          "type": "string"
///        }
///      },
///      "additionalProperties": false,
///      "name": "Successful Transaction"
///    },
///    {
///      "description": "Gives the error message of the failed transaction",
///      "type": "object",
///      "required": [
///        "error",
///        "hash"
///      ],
///      "properties": {
///        "error": {
///          "type": "string"
///        },
///        "hash": {
///          "type": "null"
///        }
///      },
///      "additionalProperties": false,
///      "name": "Failed Transaction"
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct PlaceSpotOrderResponseItemTransactionsItem {
    #[serde(flatten, default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_0: ::std::option::Option<PlaceSpotOrderResponseItemTransactionsItemSubtype0>,
    #[serde(flatten, default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_1: ::std::option::Option<PlaceSpotOrderResponseItemTransactionsItemSubtype1>,
}
impl ::std::default::Default for PlaceSpotOrderResponseItemTransactionsItem {
    fn default() -> Self {
        Self {
            subtype_0: Default::default(),
            subtype_1: Default::default(),
        }
    }
}
///Gives the hash of the successful transaction
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Gives the hash of the successful transaction",
///  "type": "object",
///  "required": [
///    "error",
///    "hash"
///  ],
///  "properties": {
///    "error": {
///      "type": "null"
///    },
///    "hash": {
///      "type": "string"
///    }
///  },
///  "additionalProperties": false,
///  "name": "Successful Transaction"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct PlaceSpotOrderResponseItemTransactionsItemSubtype0 {
    pub error: (),
    pub hash: ::std::string::String,
}
///Gives the error message of the failed transaction
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Gives the error message of the failed transaction",
///  "type": "object",
///  "required": [
///    "error",
///    "hash"
///  ],
///  "properties": {
///    "error": {
///      "type": "string"
///    },
///    "hash": {
///      "type": "null"
///    }
///  },
///  "additionalProperties": false,
///  "name": "Failed Transaction"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct PlaceSpotOrderResponseItemTransactionsItemSubtype1 {
    pub error: ::std::string::String,
    pub hash: (),
}
