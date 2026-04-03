#![allow(clippy::derivable_impls)]

use crate::client::{Route, RouteType};
use std::marker::PhantomData;
/// Route metadata for this procedure
pub const ROUTE: Route<OrderRequest, OrderResponse> = Route {
    procedure: "orders.get",
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
///`OrderRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "id"
///  ],
///  "properties": {
///    "id": {
///      "description": "The order ID in format {taskType}_{taskId} (e.g., \"limit_123\" or \"spot_456\")",
///      "type": "string",
///      "name": "Order ID"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct OrderRequest {
    ///The order ID in format {taskType}_{taskId} (e.g., "limit_123" or "spot_456")
    pub id: ::std::string::String,
}
///Returns a single order by ID
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Returns a single order by ID",
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
///      "description": "The amount of the order",
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
///      "additionalProperties": false,
///      "name": "Amount"
///    },
///    "chainId": {
///      "description": "The blockchain chain ID of the order",
///      "type": "string",
///      "name": "Chain ID"
///    },
///    "counterTokenAddress": {
///      "description": "The contract address of the counter token",
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
///      "description": "The timestamp when the order expires",
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
///      "description": "The marketcap of the token in USD when the order was limited",
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
///      "description": "The price of the token in USD when the order was limited",
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
///      "description": "The contract address of the pair",
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
///      "description": "The marketcap of the token in USD when the order was requested",
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
///      "description": "The price of the token in USD when the order was requested",
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
///      "description": "The token associated with the order",
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
///          "description": "The counter token of the best pair",
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
///                  "name": "Token Decimals"
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
///                  "name": "Token Name"
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
///                  "name": "Token Symbol"
///                },
///                "tokenChainId": {
///                  "description": "The blockchain chain ID of the counter token",
///                  "type": "string",
///                  "name": "Token Chain ID"
///                },
///                "tokenContractAddress": {
///                  "description": "The contract address of the counter token",
///                  "type": "string",
///                  "name": "Token Contract Address"
///                }
///              },
///              "additionalProperties": false
///            },
///            {
///              "type": "null"
///            }
///          ],
///          "name": "Best Pair Counter Token"
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
///          "name": "Token Decimals"
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
///          "description": "The URL of the token logo",
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
///          "description": "The social links of the token",
///          "type": "array",
///          "items": {
///            "type": "object",
///            "required": [
///              "platform",
///              "url"
///            ],
///            "properties": {
///              "platform": {
///                "description": "The social platform name",
///                "type": "string",
///                "name": "Platform"
///              },
///              "url": {
///                "description": "The social platform URL",
///                "type": "string",
///                "name": "URL"
///              }
///            },
///            "additionalProperties": false
///          },
///          "name": "Social Links"
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
///          "description": "The blockchain chain ID of the token",
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
///          "description": "The timestamp when trading was opened for the token",
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
///      "additionalProperties": false,
///      "name": "Token"
///    },
///    "tokenAddress": {
///      "description": "The contract address of the token",
///      "type": "string",
///      "name": "Token Address"
///    },
///    "transactionHash": {
///      "description": "The hash of the transaction associated with the order",
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
///      "description": "The marketcap of the token in native tokens when the order was triggered",
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
///      "description": "The marketcap of the token in USD when the order was triggered",
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
///      "description": "The price of the token in native tokens when the order was triggered",
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
///      "description": "The price of the token in USD when the order was triggered",
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
///  "additionalProperties": false,
///  "name": "Get Order Response"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct OrderResponse {
    ///The ID of the agent associated with the order
    #[serde(rename = "agentId")]
    pub agent_id: ::std::option::Option<::std::string::String>,
    pub amount: OrderResponseAmount,
    ///The blockchain chain ID of the order
    #[serde(rename = "chainId")]
    pub chain_id: ::std::string::String,
    ///The contract address of the counter token
    #[serde(rename = "counterTokenAddress")]
    pub counter_token_address: ::std::option::Option<::std::string::String>,
    ///The timestamp when the order was added
    #[serde(rename = "dateAdded")]
    pub date_added: ::std::option::Option<::std::string::String>,
    ///The timestamp when the order expires
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
    ///The marketcap of the token in USD when the order was limited
    #[serde(rename = "limitTokenMarketcapUsd")]
    pub limit_token_marketcap_usd: ::std::option::Option<f64>,
    ///The price of the token in USD when the order was limited
    #[serde(rename = "limitTokenPriceUsd")]
    pub limit_token_price_usd: ::std::option::Option<f64>,
    ///The contract address of the pair
    #[serde(rename = "pairAddress")]
    pub pair_address: ::std::option::Option<::std::string::String>,
    ///The type of the pair
    #[serde(rename = "pairType")]
    pub pair_type: ::std::option::Option<::std::string::String>,
    ///The marketcap of the token in USD when the order was requested
    #[serde(rename = "requestedAtTokenMarketcapUsd")]
    pub requested_at_token_marketcap_usd: ::std::option::Option<f64>,
    ///The price of the token in USD when the order was requested
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
    pub token: ::std::option::Option<OrderResponseToken>,
    ///The contract address of the token
    #[serde(rename = "tokenAddress")]
    pub token_address: ::std::string::String,
    ///The hash of the transaction associated with the order
    #[serde(rename = "transactionHash")]
    pub transaction_hash: ::std::option::Option<::std::string::String>,
    ///The marketcap of the token in native tokens when the order was triggered
    #[serde(rename = "triggeredAtTokenMarketcapNativeToken")]
    pub triggered_at_token_marketcap_native_token: ::std::option::Option<f64>,
    ///The marketcap of the token in USD when the order was triggered
    #[serde(rename = "triggeredAtTokenMarketcapUsd")]
    pub triggered_at_token_marketcap_usd: ::std::option::Option<f64>,
    ///The price of the token in native tokens when the order was triggered
    #[serde(rename = "triggeredAtTokenPriceNativeToken")]
    pub triggered_at_token_price_native_token: ::std::option::Option<f64>,
    ///The price of the token in USD when the order was triggered
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
///The amount of the order
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "The amount of the order",
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
///  "additionalProperties": false,
///  "name": "Amount"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct OrderResponseAmount {
    ///Type of amount: native currency, token amount, or percentage
    #[serde(rename = "type")]
    pub type_: OrderResponseAmountType,
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
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum OrderResponseAmountType {
    #[serde(rename = "native")]
    Native,
    #[serde(rename = "token")]
    Token,
    #[serde(rename = "percentage")]
    Percentage,
}
impl ::std::fmt::Display for OrderResponseAmountType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Native => f.write_str("native"),
            Self::Token => f.write_str("token"),
            Self::Percentage => f.write_str("percentage"),
        }
    }
}
impl ::std::str::FromStr for OrderResponseAmountType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "native" => Ok(Self::Native),
            "token" => Ok(Self::Token),
            "percentage" => Ok(Self::Percentage),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for OrderResponseAmountType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for OrderResponseAmountType {
    type Error = self::error::ConversionError;
    fn try_from(value: &::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for OrderResponseAmountType {
    type Error = self::error::ConversionError;
    fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///The token associated with the order
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "The token associated with the order",
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
///      "description": "The counter token of the best pair",
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
///              "name": "Token Decimals"
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
///              "name": "Token Name"
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
///              "name": "Token Symbol"
///            },
///            "tokenChainId": {
///              "description": "The blockchain chain ID of the counter token",
///              "type": "string",
///              "name": "Token Chain ID"
///            },
///            "tokenContractAddress": {
///              "description": "The contract address of the counter token",
///              "type": "string",
///              "name": "Token Contract Address"
///            }
///          },
///          "additionalProperties": false
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Best Pair Counter Token"
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
///      "name": "Token Decimals"
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
///      "description": "The URL of the token logo",
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
///      "description": "The social links of the token",
///      "type": "array",
///      "items": {
///        "type": "object",
///        "required": [
///          "platform",
///          "url"
///        ],
///        "properties": {
///          "platform": {
///            "description": "The social platform name",
///            "type": "string",
///            "name": "Platform"
///          },
///          "url": {
///            "description": "The social platform URL",
///            "type": "string",
///            "name": "URL"
///          }
///        },
///        "additionalProperties": false
///      },
///      "name": "Social Links"
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
///      "description": "The blockchain chain ID of the token",
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
///      "description": "The timestamp when trading was opened for the token",
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
///  "additionalProperties": false,
///  "name": "Token"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct OrderResponseToken {
    ///The address of the best pair for the token
    #[serde(rename = "bestPairAddress")]
    pub best_pair_address: ::std::option::Option<::std::string::String>,
    ///The counter token of the best pair
    #[serde(rename = "bestPairCounterToken")]
    pub best_pair_counter_token: ::std::option::Option<OrderResponseTokenBestPairCounterToken>,
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
    ///The URL of the token logo
    #[serde(rename = "logoUrl")]
    pub logo_url: ::std::option::Option<::std::string::String>,
    ///The marketcap of the token in USD
    #[serde(rename = "marketcapUsd")]
    pub marketcap_usd: ::std::option::Option<f64>,
    ///The name of the token
    pub name: ::std::option::Option<::std::string::String>,
    ///The social links of the token
    #[serde(rename = "socialLinks")]
    pub social_links: ::std::vec::Vec<OrderResponseTokenSocialLinksItem>,
    ///The symbol of the token
    pub symbol: ::std::option::Option<::std::string::String>,
    ///The blockchain chain ID of the token
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
    ///The timestamp when trading was opened for the token
    #[serde(rename = "tradingOpenedAt")]
    pub trading_opened_at: ::std::option::Option<::std::string::String>,
}
///`OrderResponseTokenBestPairCounterToken`
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
///      "name": "Token Decimals"
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
///      "name": "Token Name"
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
///      "name": "Token Symbol"
///    },
///    "tokenChainId": {
///      "description": "The blockchain chain ID of the counter token",
///      "type": "string",
///      "name": "Token Chain ID"
///    },
///    "tokenContractAddress": {
///      "description": "The contract address of the counter token",
///      "type": "string",
///      "name": "Token Contract Address"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct OrderResponseTokenBestPairCounterToken {
    pub decimals: f64,
    ///The name of the counter token
    pub name: ::std::option::Option<::std::string::String>,
    ///The symbol of the counter token
    pub symbol: ::std::option::Option<::std::string::String>,
    ///The blockchain chain ID of the counter token
    #[serde(rename = "tokenChainId")]
    pub token_chain_id: ::std::string::String,
    ///The contract address of the counter token
    #[serde(rename = "tokenContractAddress")]
    pub token_contract_address: ::std::string::String,
}
///`OrderResponseTokenSocialLinksItem`
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
///      "description": "The social platform name",
///      "type": "string",
///      "name": "Platform"
///    },
///    "url": {
///      "description": "The social platform URL",
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
pub struct OrderResponseTokenSocialLinksItem {
    ///The social platform name
    pub platform: ::std::string::String,
    ///The social platform URL
    pub url: ::std::string::String,
}
