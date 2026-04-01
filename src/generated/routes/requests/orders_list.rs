#![allow(clippy::derivable_impls)]

use crate::client::{Route, RouteType};
use std::marker::PhantomData;
/// Route metadata for this procedure
pub const ROUTE: Route<ListOrdersRequest, ListOrdersResponse> = Route {
    procedure: "orders.list",
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
///`ListOrdersRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "chainId": {
///      "description": "Filter by blockchain chain ID (e.g., \"1\" for Ethereum, \"8453\" for Base, \"solana\" for Solana)",
///      "type": "string",
///      "name": "Chain ID"
///    },
///    "cursor": {
///      "description": "Opaque cursor for pagination. Pass the value from a previous response to get the next page.",
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Pagination Cursor"
///    },
///    "includeHidden": {
///      "description": "Whether to include hidden/canceled orders in results",
///      "type": "boolean",
///      "name": "Include Hidden Orders"
///    },
///    "isAgentOrder": {
///      "description": "Filter to show only orders created by agents",
///      "type": "boolean",
///      "name": "Agent Orders Only"
///    },
///    "isSignalOrder": {
///      "description": "Filter to show only orders from signal subscriptions",
///      "type": "boolean",
///      "name": "Signal Orders Only"
///    },
///    "limit": {
///      "description": "Maximum number of orders to return (1-200, default: 50)",
///      "type": "number",
///      "maximum": 200.0,
///      "minimum": 1.0,
///      "name": "Result Limit"
///    },
///    "status": {
///      "description": "Filter by order status: Working, Canceled, Inactive, Expired, Placing, Filled, or Rejected",
///      "type": "string",
///      "enum": [
///        "Working",
///        "Canceled",
///        "Inactive",
///        "Expired",
///        "Placing",
///        "Filled",
///        "Rejected"
///      ],
///      "name": "Order Status"
///    },
///    "taskIds": {
///      "description": "Filter by specific task IDs",
///      "type": "array",
///      "items": {
///        "type": "number"
///      },
///      "name": "Task IDs"
///    },
///    "tokenAddresses": {
///      "description": "Filter by specific token contract addresses",
///      "type": "array",
///      "items": {
///        "type": "string"
///      },
///      "name": "Token Addresses"
///    },
///    "triggeredAtEnd": {
///      "description": "Filter orders triggered on or before this timestamp (ISO 8601)",
///      "type": "string",
///      "format": "date-time",
///      "pattern": "^(?:(?:\\d\\d[2468][048]|\\d\\d[13579][26]|\\d\\d0[48]|[02468][048]00|[13579][26]00)-02-29|\\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\\d|30)|(?:02)-(?:0[1-9]|1\\d|2[0-8])))T(?:(?:[01]\\d|2[0-3]):[0-5]\\d(?::[0-5]\\d(?:\\.\\d+)?)?(?:Z))$",
///      "name": "Triggered At End"
///    },
///    "triggeredAtStart": {
///      "description": "Filter orders triggered on or after this timestamp (ISO 8601)",
///      "type": "string",
///      "format": "date-time",
///      "pattern": "^(?:(?:\\d\\d[2468][048]|\\d\\d[13579][26]|\\d\\d0[48]|[02468][048]00|[13579][26]00)-02-29|\\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\\d|30)|(?:02)-(?:0[1-9]|1\\d|2[0-8])))T(?:(?:[01]\\d|2[0-3]):[0-5]\\d(?::[0-5]\\d(?:\\.\\d+)?)?(?:Z))$",
///      "name": "Triggered At Start"
///    },
///    "type": {
///      "description": "Filter by order type: \"limit\" or \"spot\"",
///      "type": "string",
///      "enum": [
///        "limit",
///        "spot"
///      ],
///      "name": "Order Type"
///    },
///    "wallets": {
///      "description": "Filter by specific wallet addresses associated with orders",
///      "type": "array",
///      "items": {
///        "type": "string"
///      },
///      "name": "Wallet Addresses"
///    },
///    "withToken": {
///      "description": "Whether to include full token details in the response",
///      "type": "boolean",
///      "name": "Include Token Details"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ListOrdersRequest {
    ///Filter by blockchain chain ID (e.g., "1" for Ethereum, "8453" for Base, "solana" for Solana)
    #[serde(
        rename = "chainId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub chain_id: ::std::option::Option<::std::string::String>,
    ///Opaque cursor for pagination. Pass the value from a previous response to get the next page.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub cursor: ::std::option::Option<::std::string::String>,
    ///Whether to include hidden/canceled orders in results
    #[serde(
        rename = "includeHidden",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub include_hidden: ::std::option::Option<bool>,
    ///Filter to show only orders created by agents
    #[serde(
        rename = "isAgentOrder",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_agent_order: ::std::option::Option<bool>,
    ///Filter to show only orders from signal subscriptions
    #[serde(
        rename = "isSignalOrder",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_signal_order: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub limit: ::std::option::Option<f64>,
    ///Filter by order status: Working, Canceled, Inactive, Expired, Placing, Filled, or Rejected
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub status: ::std::option::Option<ListOrdersRequestStatus>,
    ///Filter by specific task IDs
    #[serde(
        rename = "taskIds",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub task_ids: ::std::vec::Vec<f64>,
    ///Filter by specific token contract addresses
    #[serde(
        rename = "tokenAddresses",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub token_addresses: ::std::vec::Vec<::std::string::String>,
    ///Filter orders triggered on or before this timestamp (ISO 8601)
    #[serde(
        rename = "triggeredAtEnd",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub triggered_at_end: ::std::option::Option<
        ::chrono::DateTime<::chrono::offset::Utc>,
    >,
    ///Filter orders triggered on or after this timestamp (ISO 8601)
    #[serde(
        rename = "triggeredAtStart",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub triggered_at_start: ::std::option::Option<
        ::chrono::DateTime<::chrono::offset::Utc>,
    >,
    ///Filter by order type: "limit" or "spot"
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<ListOrdersRequestType>,
    ///Filter by specific wallet addresses associated with orders
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub wallets: ::std::vec::Vec<::std::string::String>,
    ///Whether to include full token details in the response
    #[serde(
        rename = "withToken",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub with_token: ::std::option::Option<bool>,
}
impl ::std::default::Default for ListOrdersRequest {
    fn default() -> Self {
        Self {
            chain_id: Default::default(),
            cursor: Default::default(),
            include_hidden: Default::default(),
            is_agent_order: Default::default(),
            is_signal_order: Default::default(),
            limit: Default::default(),
            status: Default::default(),
            task_ids: Default::default(),
            token_addresses: Default::default(),
            triggered_at_end: Default::default(),
            triggered_at_start: Default::default(),
            type_: Default::default(),
            wallets: Default::default(),
            with_token: Default::default(),
        }
    }
}
///Filter by order status: Working, Canceled, Inactive, Expired, Placing, Filled, or Rejected
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Filter by order status: Working, Canceled, Inactive, Expired, Placing, Filled, or Rejected",
///  "type": "string",
///  "enum": [
///    "Working",
///    "Canceled",
///    "Inactive",
///    "Expired",
///    "Placing",
///    "Filled",
///    "Rejected"
///  ],
///  "name": "Order Status"
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
pub enum ListOrdersRequestStatus {
    Working,
    Canceled,
    Inactive,
    Expired,
    Placing,
    Filled,
    Rejected,
}
impl ::std::fmt::Display for ListOrdersRequestStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Working => f.write_str("Working"),
            Self::Canceled => f.write_str("Canceled"),
            Self::Inactive => f.write_str("Inactive"),
            Self::Expired => f.write_str("Expired"),
            Self::Placing => f.write_str("Placing"),
            Self::Filled => f.write_str("Filled"),
            Self::Rejected => f.write_str("Rejected"),
        }
    }
}
impl ::std::str::FromStr for ListOrdersRequestStatus {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "Working" => Ok(Self::Working),
            "Canceled" => Ok(Self::Canceled),
            "Inactive" => Ok(Self::Inactive),
            "Expired" => Ok(Self::Expired),
            "Placing" => Ok(Self::Placing),
            "Filled" => Ok(Self::Filled),
            "Rejected" => Ok(Self::Rejected),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ListOrdersRequestStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ListOrdersRequestStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ListOrdersRequestStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Filter by order type: "limit" or "spot"
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Filter by order type: \"limit\" or \"spot\"",
///  "type": "string",
///  "enum": [
///    "limit",
///    "spot"
///  ],
///  "name": "Order Type"
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
pub enum ListOrdersRequestType {
    #[serde(rename = "limit")]
    Limit,
    #[serde(rename = "spot")]
    Spot,
}
impl ::std::fmt::Display for ListOrdersRequestType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Limit => f.write_str("limit"),
            Self::Spot => f.write_str("spot"),
        }
    }
}
impl ::std::str::FromStr for ListOrdersRequestType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "limit" => Ok(Self::Limit),
            "spot" => Ok(Self::Spot),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ListOrdersRequestType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ListOrdersRequestType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ListOrdersRequestType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///The response from the orders.list endpoint
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "The response from the orders.list endpoint",
///  "type": "object",
///  "required": [
///    "items"
///  ],
///  "properties": {
///    "items": {
///      "type": "array",
///      "items": {
///        "type": "object",
///        "required": [
///          "agentId",
///          "amount",
///          "chainId",
///          "counterTokenAddress",
///          "dateAdded",
///          "dateExpiry",
///          "dateTriggered",
///          "entryStrategyId",
///          "exitStrategyId",
///          "id",
///          "limitTokenMarketcapUsd",
///          "limitTokenPriceUsd",
///          "pairAddress",
///          "pairType",
///          "requestedAtTokenMarketcapUsd",
///          "requestedAtTokenPriceUsd",
///          "side",
///          "signalId",
///          "status",
///          "taskId",
///          "tokenAddress",
///          "transactionHash",
///          "triggeredAtTokenMarketcapNativeToken",
///          "triggeredAtTokenMarketcapUsd",
///          "triggeredAtTokenPriceNativeToken",
///          "triggeredAtTokenPriceUsd",
///          "txId",
///          "type",
///          "wallet"
///        ],
///        "properties": {
///          "agentId": {
///            "description": "The ID of the agent associated with the order",
///            "anyOf": [
///              {
///                "type": "string"
///              },
///              {
///                "type": "null"
///              }
///            ],
///            "name": "Agent ID"
///          },
///          "amount": {
///            "description": "The amount of the order; a discriminated union of native, token, and percentage amounts",
///            "oneOf": [
///              {
///                "description": "The amount of the order in native tokens; must be in base unit amount (eg, wei, lamports, etc.)",
///                "type": "object",
///                "required": [
///                  "type",
///                  "value"
///                ],
///                "properties": {
///                  "type": {
///                    "type": "string",
///                    "const": "native"
///                  },
///                  "value": {
///                    "description": "The amount of the order in native tokens; stringified; must be in base unit amount (eg, wei, lamports, etc.)",
///                    "type": "string",
///                    "name": "Value"
///                  }
///                },
///                "additionalProperties": false,
///                "name": "Native Amount"
///              },
///              {
///                "description": "The amount of the order in tokens",
///                "type": "object",
///                "required": [
///                  "type",
///                  "value"
///                ],
///                "properties": {
///                  "type": {
///                    "type": "string",
///                    "const": "token"
///                  },
///                  "value": {
///                    "description": "The amount of the order in tokens; stringified; must be in base unit amount (eg, like wei, lamports, etc. would be).\n\nExample: if you want to buy/sell 1000 tokens and the token has 6 decimals, you would pass \"1000000000\".",
///                    "type": "string",
///                    "name": "Value"
///                  }
///                },
///                "additionalProperties": false,
///                "name": "Token Amount"
///              },
///              {
///                "description": "The amount of the order as a percentage; must be between 0 and 100",
///                "type": "object",
///                "required": [
///                  "type",
///                  "value"
///                ],
///                "properties": {
///                  "type": {
///                    "type": "string",
///                    "const": "percentage"
///                  },
///                  "value": {
///                    "description": "The amount of the order as a percentage; stringified; must be between 0 and 100. Only applies to sell orders.",
///                    "type": "string",
///                    "name": "Value"
///                  }
///                },
///                "additionalProperties": false,
///                "name": "Percentage Amount"
///              }
///            ],
///            "name": "Amount"
///          },
///          "chainId": {
///            "description": "The blockchain chain ID of the order",
///            "type": "string",
///            "name": "Chain ID"
///          },
///          "counterTokenAddress": {
///            "description": "The contract address of the counter token",
///            "anyOf": [
///              {
///                "type": "string"
///              },
///              {
///                "type": "null"
///              }
///            ],
///            "name": "Counter Token Address"
///          },
///          "dateAdded": {
///            "description": "The timestamp when the order was added",
///            "anyOf": [
///              {
///                "type": "string"
///              },
///              {
///                "type": "null"
///              }
///            ],
///            "name": "Date Added"
///          },
///          "dateExpiry": {
///            "description": "The timestamp when the order expired",
///            "anyOf": [
///              {
///                "type": "string"
///              },
///              {
///                "type": "null"
///              }
///            ],
///            "name": "Date Expiry"
///          },
///          "dateTriggered": {
///            "description": "The timestamp when the order was triggered",
///            "anyOf": [
///              {
///                "type": "string"
///              },
///              {
///                "type": "null"
///              }
///            ],
///            "name": "Date Triggered"
///          },
///          "entryStrategyId": {
///            "description": "The ID of the entry strategy",
///            "anyOf": [
///              {
///                "type": "number"
///              },
///              {
///                "type": "null"
///              }
///            ],
///            "name": "Entry Strategy ID"
///          },
///          "exitStrategyId": {
///            "description": "The ID of the exit strategy",
///            "anyOf": [
///              {
///                "type": "number"
///              },
///              {
///                "type": "null"
///              }
///            ],
///            "name": "Exit Strategy ID"
///          },
///          "id": {
///            "description": "The ID of the order in format {taskType}_{taskId} (e.g., \"limit_123\" or \"spot_456\")",
///            "type": "string",
///            "name": "Order ID"
///          },
///          "limitTokenMarketcapUsd": {
///            "description": "The marketcap of the token in USD when the order was limited",
///            "anyOf": [
///              {
///                "type": "number"
///              },
///              {
///                "type": "null"
///              }
///            ],
///            "name": "Limit Token Marketcap USD"
///          },
///          "limitTokenPriceUsd": {
///            "description": "The price of the token in USD when the order was limited",
///            "anyOf": [
///              {
///                "type": "number"
///              },
///              {
///                "type": "null"
///              }
///            ],
///            "name": "Limit Token Price USD"
///          },
///          "pairAddress": {
///            "description": "The contract address of the pair",
///            "anyOf": [
///              {
///                "type": "string"
///              },
///              {
///                "type": "null"
///              }
///            ],
///            "name": "Pair Address"
///          },
///          "pairType": {
///            "description": "The type of the pair",
///            "anyOf": [
///              {
///                "type": "string"
///              },
///              {
///                "type": "null"
///              }
///            ],
///            "name": "Pair Type"
///          },
///          "requestedAtTokenMarketcapUsd": {
///            "description": "The marketcap of the token in USD when the order was requested",
///            "anyOf": [
///              {
///                "type": "number"
///              },
///              {
///                "type": "null"
///              }
///            ],
///            "name": "Requested At Token Marketcap USD"
///          },
///          "requestedAtTokenPriceUsd": {
///            "description": "The price of the token in USD when the order was requested",
///            "anyOf": [
///              {
///                "type": "number"
///              },
///              {
///                "type": "null"
///              }
///            ],
///            "name": "Requested At Token Price USD"
///          },
///          "side": {
///            "description": "The side of the order: \"buy\" or \"sell\"",
///            "type": "string",
///            "enum": [
///              "buy",
///              "sell"
///            ],
///            "name": "Side"
///          },
///          "signalId": {
///            "description": "The ID of the signal associated with the order",
///            "anyOf": [
///              {
///                "type": "string"
///              },
///              {
///                "type": "null"
///              }
///            ],
///            "name": "Signal ID"
///          },
///          "status": {
///            "description": "The status of the order: \"Working\", \"Canceled\", \"Inactive\", \"Expired\", \"Placing\", \"Filled\", or \"Rejected\"",
///            "type": "string",
///            "enum": [
///              "Working",
///              "Canceled",
///              "Inactive",
///              "Expired",
///              "Placing",
///              "Filled",
///              "Rejected"
///            ],
///            "name": "Order Status"
///          },
///          "taskId": {
///            "description": "The ID of the task associated with the order",
///            "type": "number",
///            "name": "Task ID"
///          },
///          "token": {
///            "type": "object",
///            "required": [
///              "bestPairAddress",
///              "bestPairCounterToken",
///              "bestPairCreatedAt",
///              "bestPairSymbol",
///              "bestPairType",
///              "createdAt",
///              "decimals",
///              "deployerAddress",
///              "effectiveSupply",
///              "liquidityUsd",
///              "marketcapUsd",
///              "name",
///              "socialLinks",
///              "symbol",
///              "tokenChainId",
///              "tokenContractAddress",
///              "tokenPriceNativeToken",
///              "tokenPriceUsd",
///              "tradingOpenedAt"
///            ],
///            "properties": {
///              "bestPairAddress": {
///                "description": "The address of the best pair for the token",
///                "anyOf": [
///                  {
///                    "type": "string"
///                  },
///                  {
///                    "type": "null"
///                  }
///                ],
///                "name": "Best Pair Address"
///              },
///              "bestPairCounterToken": {
///                "anyOf": [
///                  {
///                    "type": "object",
///                    "required": [
///                      "decimals",
///                      "name",
///                      "symbol",
///                      "tokenChainId",
///                      "tokenContractAddress"
///                    ],
///                    "properties": {
///                      "decimals": {
///                        "description": "The number of decimals of the token",
///                        "type": "number",
///                        "name": "Token Decimals"
///                      },
///                      "name": {
///                        "description": "The name of the token",
///                        "anyOf": [
///                          {
///                            "type": "string"
///                          },
///                          {
///                            "type": "null"
///                          }
///                        ],
///                        "name": "Token Name"
///                      },
///                      "symbol": {
///                        "description": "The symbol of the token",
///                        "anyOf": [
///                          {
///                            "type": "string"
///                          },
///                          {
///                            "type": "null"
///                          }
///                        ],
///                        "name": "Token Symbol"
///                      },
///                      "tokenChainId": {
///                        "description": "The blockchain chain ID of the token",
///                        "type": "string",
///                        "name": "Token Chain ID"
///                      },
///                      "tokenContractAddress": {
///                        "description": "The contract address of the token",
///                        "type": "string",
///                        "name": "Token Contract Address"
///                      }
///                    },
///                    "additionalProperties": false
///                  },
///                  {
///                    "type": "null"
///                  }
///                ]
///              },
///              "bestPairCreatedAt": {
///                "description": "The timestamp when the best pair was created",
///                "anyOf": [
///                  {
///                    "type": "string"
///                  },
///                  {
///                    "type": "null"
///                  }
///                ],
///                "name": "Best Pair Created At"
///              },
///              "bestPairSymbol": {
///                "description": "The symbol of the best pair for the token",
///                "anyOf": [
///                  {
///                    "type": "string"
///                  },
///                  {
///                    "type": "null"
///                  }
///                ],
///                "name": "Best Pair Symbol"
///              },
///              "bestPairType": {
///                "description": "The type of the best pair for the token",
///                "type": "string",
///                "name": "Best Pair Type"
///              },
///              "createdAt": {
///                "description": "The timestamp when the token was created",
///                "type": "string",
///                "name": "Created At"
///              },
///              "decimals": {
///                "description": "The number of decimals of the token",
///                "type": "number",
///                "name": "Token Decimals"
///              },
///              "deployerAddress": {
///                "description": "The address of the deployer of the token",
///                "type": "string",
///                "name": "Deployer Address"
///              },
///              "effectiveSupply": {
///                "description": "The effective supply of the token",
///                "anyOf": [
///                  {
///                    "type": "string"
///                  },
///                  {
///                    "type": "null"
///                  }
///                ],
///                "name": "Effective Supply"
///              },
///              "liquidityUsd": {
///                "description": "The liquidity of the token in USD",
///                "anyOf": [
///                  {
///                    "type": "number"
///                  },
///                  {
///                    "type": "null"
///                  }
///                ],
///                "name": "Liquidity USD"
///              },
///              "marketcapUsd": {
///                "description": "The marketcap of the token in USD",
///                "anyOf": [
///                  {
///                    "type": "number"
///                  },
///                  {
///                    "type": "null"
///                  }
///                ],
///                "name": "Marketcap USD"
///              },
///              "name": {
///                "description": "The name of the token",
///                "anyOf": [
///                  {
///                    "type": "string"
///                  },
///                  {
///                    "type": "null"
///                  }
///                ],
///                "name": "Token Name"
///              },
///              "socialLinks": {
///                "description": "The social links of the token",
///                "type": "array",
///                "items": {
///                  "type": "object",
///                  "required": [
///                    "platform",
///                    "url"
///                  ],
///                  "properties": {
///                    "platform": {
///                      "type": "string",
///                      "enum": [
///                        "None",
///                        "Discord",
///                        "Facebook",
///                        "Github",
///                        "Instagram",
///                        "Medium",
///                        "OpenSea",
///                        "Reddit",
///                        "Telegram",
///                        "TikTok",
///                        "Twitch",
///                        "Twitter",
///                        "Website",
///                        "WeChat",
///                        "Weibo",
///                        "YouTube"
///                      ]
///                    },
///                    "url": {
///                      "type": "string"
///                    }
///                  },
///                  "additionalProperties": false
///                },
///                "name": "Social Links"
///              },
///              "symbol": {
///                "description": "The symbol of the token",
///                "anyOf": [
///                  {
///                    "type": "string"
///                  },
///                  {
///                    "type": "null"
///                  }
///                ],
///                "name": "Token Symbol"
///              },
///              "tokenChainId": {
///                "description": "The blockchain chain ID of the token",
///                "type": "string",
///                "name": "Token Chain ID"
///              },
///              "tokenContractAddress": {
///                "description": "The contract address of the token",
///                "type": "string",
///                "name": "Token Contract Address"
///              },
///              "tokenPriceNativeToken": {
///                "description": "The price of the token in native tokens",
///                "anyOf": [
///                  {
///                    "type": "number"
///                  },
///                  {
///                    "type": "null"
///                  }
///                ],
///                "name": "Token Price Native Token"
///              },
///              "tokenPriceUsd": {
///                "description": "The price of the token in USD",
///                "anyOf": [
///                  {
///                    "type": "number"
///                  },
///                  {
///                    "type": "null"
///                  }
///                ],
///                "name": "Token Price USD"
///              },
///              "tradingOpenedAt": {
///                "description": "The timestamp when the trading was opened for the token",
///                "anyOf": [
///                  {
///                    "type": "string"
///                  },
///                  {
///                    "type": "null"
///                  }
///                ],
///                "name": "Trading Opened At"
///              }
///            },
///            "additionalProperties": false
///          },
///          "tokenAddress": {
///            "description": "The contract address of the token",
///            "type": "string",
///            "name": "Token Address"
///          },
///          "transactionHash": {
///            "description": "The hash of the transaction associated with the order",
///            "anyOf": [
///              {
///                "type": "string"
///              },
///              {
///                "type": "null"
///              }
///            ],
///            "name": "Transaction Hash"
///          },
///          "triggeredAtTokenMarketcapNativeToken": {
///            "description": "The marketcap of the token in native tokens when the order was triggered",
///            "anyOf": [
///              {
///                "type": "number"
///              },
///              {
///                "type": "null"
///              }
///            ],
///            "name": "Triggered At Token Marketcap Native Token"
///          },
///          "triggeredAtTokenMarketcapUsd": {
///            "description": "The marketcap of the token in USD when the order was triggered",
///            "anyOf": [
///              {
///                "type": "number"
///              },
///              {
///                "type": "null"
///              }
///            ],
///            "name": "Triggered At Token Marketcap USD"
///          },
///          "triggeredAtTokenPriceNativeToken": {
///            "description": "The price of the token in native tokens when the order was triggered",
///            "anyOf": [
///              {
///                "type": "number"
///              },
///              {
///                "type": "null"
///              }
///            ],
///            "name": "Triggered At Token Price Native Token"
///          },
///          "triggeredAtTokenPriceUsd": {
///            "description": "The price of the token in USD when the order was triggered",
///            "anyOf": [
///              {
///                "type": "number"
///              },
///              {
///                "type": "null"
///              }
///            ],
///            "name": "Triggered At Token Price USD"
///          },
///          "txId": {
///            "description": "The ID of the transaction associated with the order",
///            "anyOf": [
///              {
///                "type": "string"
///              },
///              {
///                "type": "null"
///              }
///            ],
///            "name": "Transaction ID"
///          },
///          "type": {
///            "description": "The type of the order: \"limit\" or \"spot\"",
///            "type": "string",
///            "enum": [
///              "limit",
///              "spot"
///            ],
///            "name": "Order Type"
///          },
///          "wallet": {
///            "description": "The wallet address associated with the order",
///            "anyOf": [
///              {
///                "type": "string"
///              },
///              {
///                "type": "null"
///              }
///            ],
///            "name": "Wallet Address"
///          }
///        },
///        "additionalProperties": false
///      }
///    },
///    "next": {
///      "description": "The next cursor for pagination",
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Next"
///    }
///  },
///  "additionalProperties": false,
///  "name": "Get User Orders Response"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ListOrdersResponse {
    pub items: ::std::vec::Vec<ListOrdersResponseItemsItem>,
    ///The next cursor for pagination
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub next: ::std::option::Option<::std::string::String>,
}
///`ListOrdersResponseItemsItem`
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
///          "additionalProperties": false,
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
///          "additionalProperties": false,
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
///          "additionalProperties": false,
///          "name": "Percentage Amount"
///        }
///      ],
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
///      "enum": [
///        "buy",
///        "sell"
///      ],
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
///      "enum": [
///        "Working",
///        "Canceled",
///        "Inactive",
///        "Expired",
///        "Placing",
///        "Filled",
///        "Rejected"
///      ],
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
///                  "description": "The number of decimals of the token",
///                  "type": "number",
///                  "name": "Token Decimals"
///                },
///                "name": {
///                  "description": "The name of the token",
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
///                  "description": "The symbol of the token",
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
///                  "description": "The blockchain chain ID of the token",
///                  "type": "string",
///                  "name": "Token Chain ID"
///                },
///                "tokenContractAddress": {
///                  "description": "The contract address of the token",
///                  "type": "string",
///                  "name": "Token Contract Address"
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
///          "type": "string",
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
///                "type": "string",
///                "enum": [
///                  "None",
///                  "Discord",
///                  "Facebook",
///                  "Github",
///                  "Instagram",
///                  "Medium",
///                  "OpenSea",
///                  "Reddit",
///                  "Telegram",
///                  "TikTok",
///                  "Twitch",
///                  "Twitter",
///                  "Website",
///                  "WeChat",
///                  "Weibo",
///                  "YouTube"
///                ]
///              },
///              "url": {
///                "type": "string"
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
///      "enum": [
///        "limit",
///        "spot"
///      ],
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
pub struct ListOrdersResponseItemsItem {
    ///The ID of the agent associated with the order
    #[serde(rename = "agentId")]
    pub agent_id: ::std::option::Option<::std::string::String>,
    ///The amount of the order; a discriminated union of native, token, and percentage amounts
    pub amount: ListOrdersResponseItemsItemAmount,
    ///The blockchain chain ID of the order
    #[serde(rename = "chainId")]
    pub chain_id: ::std::string::String,
    ///The contract address of the counter token
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
    pub side: ListOrdersResponseItemsItemSide,
    ///The ID of the signal associated with the order
    #[serde(rename = "signalId")]
    pub signal_id: ::std::option::Option<::std::string::String>,
    ///The status of the order: "Working", "Canceled", "Inactive", "Expired", "Placing", "Filled", or "Rejected"
    pub status: ListOrdersResponseItemsItemStatus,
    #[serde(rename = "taskId")]
    pub task_id: f64,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub token: ::std::option::Option<ListOrdersResponseItemsItemToken>,
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
    pub type_: ListOrdersResponseItemsItemType,
    ///The wallet address associated with the order
    pub wallet: ::std::option::Option<::std::string::String>,
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
///      "additionalProperties": false,
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
///      "additionalProperties": false,
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
///      "additionalProperties": false,
///      "name": "Percentage Amount"
///    }
///  ],
///  "name": "Amount"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(tag = "type", content = "value")]
pub enum ListOrdersResponseItemsItemAmount {
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
///The side of the order: "buy" or "sell"
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "The side of the order: \"buy\" or \"sell\"",
///  "type": "string",
///  "enum": [
///    "buy",
///    "sell"
///  ],
///  "name": "Side"
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
pub enum ListOrdersResponseItemsItemSide {
    #[serde(rename = "buy")]
    Buy,
    #[serde(rename = "sell")]
    Sell,
}
impl ::std::fmt::Display for ListOrdersResponseItemsItemSide {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Buy => f.write_str("buy"),
            Self::Sell => f.write_str("sell"),
        }
    }
}
impl ::std::str::FromStr for ListOrdersResponseItemsItemSide {
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
impl ::std::convert::TryFrom<&str> for ListOrdersResponseItemsItemSide {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for ListOrdersResponseItemsItemSide {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ListOrdersResponseItemsItemSide {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///The status of the order: "Working", "Canceled", "Inactive", "Expired", "Placing", "Filled", or "Rejected"
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "The status of the order: \"Working\", \"Canceled\", \"Inactive\", \"Expired\", \"Placing\", \"Filled\", or \"Rejected\"",
///  "type": "string",
///  "enum": [
///    "Working",
///    "Canceled",
///    "Inactive",
///    "Expired",
///    "Placing",
///    "Filled",
///    "Rejected"
///  ],
///  "name": "Order Status"
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
pub enum ListOrdersResponseItemsItemStatus {
    Working,
    Canceled,
    Inactive,
    Expired,
    Placing,
    Filled,
    Rejected,
}
impl ::std::fmt::Display for ListOrdersResponseItemsItemStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Working => f.write_str("Working"),
            Self::Canceled => f.write_str("Canceled"),
            Self::Inactive => f.write_str("Inactive"),
            Self::Expired => f.write_str("Expired"),
            Self::Placing => f.write_str("Placing"),
            Self::Filled => f.write_str("Filled"),
            Self::Rejected => f.write_str("Rejected"),
        }
    }
}
impl ::std::str::FromStr for ListOrdersResponseItemsItemStatus {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "Working" => Ok(Self::Working),
            "Canceled" => Ok(Self::Canceled),
            "Inactive" => Ok(Self::Inactive),
            "Expired" => Ok(Self::Expired),
            "Placing" => Ok(Self::Placing),
            "Filled" => Ok(Self::Filled),
            "Rejected" => Ok(Self::Rejected),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ListOrdersResponseItemsItemStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for ListOrdersResponseItemsItemStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for ListOrdersResponseItemsItemStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`ListOrdersResponseItemsItemToken`
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
///              "description": "The number of decimals of the token",
///              "type": "number",
///              "name": "Token Decimals"
///            },
///            "name": {
///              "description": "The name of the token",
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
///              "description": "The symbol of the token",
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
///              "description": "The blockchain chain ID of the token",
///              "type": "string",
///              "name": "Token Chain ID"
///            },
///            "tokenContractAddress": {
///              "description": "The contract address of the token",
///              "type": "string",
///              "name": "Token Contract Address"
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
///      "type": "string",
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
///            "type": "string",
///            "enum": [
///              "None",
///              "Discord",
///              "Facebook",
///              "Github",
///              "Instagram",
///              "Medium",
///              "OpenSea",
///              "Reddit",
///              "Telegram",
///              "TikTok",
///              "Twitch",
///              "Twitter",
///              "Website",
///              "WeChat",
///              "Weibo",
///              "YouTube"
///            ]
///          },
///          "url": {
///            "type": "string"
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
pub struct ListOrdersResponseItemsItemToken {
    ///The address of the best pair for the token
    #[serde(rename = "bestPairAddress")]
    pub best_pair_address: ::std::option::Option<::std::string::String>,
    #[serde(rename = "bestPairCounterToken")]
    pub best_pair_counter_token: ::std::option::Option<
        ListOrdersResponseItemsItemTokenBestPairCounterToken,
    >,
    ///The timestamp when the best pair was created
    #[serde(rename = "bestPairCreatedAt")]
    pub best_pair_created_at: ::std::option::Option<::std::string::String>,
    ///The symbol of the best pair for the token
    #[serde(rename = "bestPairSymbol")]
    pub best_pair_symbol: ::std::option::Option<::std::string::String>,
    ///The type of the best pair for the token
    #[serde(rename = "bestPairType")]
    pub best_pair_type: ::std::string::String,
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
    ///The marketcap of the token in USD
    #[serde(rename = "marketcapUsd")]
    pub marketcap_usd: ::std::option::Option<f64>,
    ///The name of the token
    pub name: ::std::option::Option<::std::string::String>,
    ///The social links of the token
    #[serde(rename = "socialLinks")]
    pub social_links: ::std::vec::Vec<ListOrdersResponseItemsItemTokenSocialLinksItem>,
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
    ///The timestamp when the trading was opened for the token
    #[serde(rename = "tradingOpenedAt")]
    pub trading_opened_at: ::std::option::Option<::std::string::String>,
}
///`ListOrdersResponseItemsItemTokenBestPairCounterToken`
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
///      "description": "The number of decimals of the token",
///      "type": "number",
///      "name": "Token Decimals"
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
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ListOrdersResponseItemsItemTokenBestPairCounterToken {
    pub decimals: f64,
    ///The name of the token
    pub name: ::std::option::Option<::std::string::String>,
    ///The symbol of the token
    pub symbol: ::std::option::Option<::std::string::String>,
    ///The blockchain chain ID of the token
    #[serde(rename = "tokenChainId")]
    pub token_chain_id: ::std::string::String,
    ///The contract address of the token
    #[serde(rename = "tokenContractAddress")]
    pub token_contract_address: ::std::string::String,
}
///`ListOrdersResponseItemsItemTokenSocialLinksItem`
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
///      "type": "string",
///      "enum": [
///        "None",
///        "Discord",
///        "Facebook",
///        "Github",
///        "Instagram",
///        "Medium",
///        "OpenSea",
///        "Reddit",
///        "Telegram",
///        "TikTok",
///        "Twitch",
///        "Twitter",
///        "Website",
///        "WeChat",
///        "Weibo",
///        "YouTube"
///      ]
///    },
///    "url": {
///      "type": "string"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ListOrdersResponseItemsItemTokenSocialLinksItem {
    pub platform: ListOrdersResponseItemsItemTokenSocialLinksItemPlatform,
    pub url: ::std::string::String,
}
///`ListOrdersResponseItemsItemTokenSocialLinksItemPlatform`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "None",
///    "Discord",
///    "Facebook",
///    "Github",
///    "Instagram",
///    "Medium",
///    "OpenSea",
///    "Reddit",
///    "Telegram",
///    "TikTok",
///    "Twitch",
///    "Twitter",
///    "Website",
///    "WeChat",
///    "Weibo",
///    "YouTube"
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
pub enum ListOrdersResponseItemsItemTokenSocialLinksItemPlatform {
    None,
    Discord,
    Facebook,
    Github,
    Instagram,
    Medium,
    OpenSea,
    Reddit,
    Telegram,
    TikTok,
    Twitch,
    Twitter,
    Website,
    WeChat,
    Weibo,
    YouTube,
}
impl ::std::fmt::Display for ListOrdersResponseItemsItemTokenSocialLinksItemPlatform {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::None => f.write_str("None"),
            Self::Discord => f.write_str("Discord"),
            Self::Facebook => f.write_str("Facebook"),
            Self::Github => f.write_str("Github"),
            Self::Instagram => f.write_str("Instagram"),
            Self::Medium => f.write_str("Medium"),
            Self::OpenSea => f.write_str("OpenSea"),
            Self::Reddit => f.write_str("Reddit"),
            Self::Telegram => f.write_str("Telegram"),
            Self::TikTok => f.write_str("TikTok"),
            Self::Twitch => f.write_str("Twitch"),
            Self::Twitter => f.write_str("Twitter"),
            Self::Website => f.write_str("Website"),
            Self::WeChat => f.write_str("WeChat"),
            Self::Weibo => f.write_str("Weibo"),
            Self::YouTube => f.write_str("YouTube"),
        }
    }
}
impl ::std::str::FromStr for ListOrdersResponseItemsItemTokenSocialLinksItemPlatform {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "None" => Ok(Self::None),
            "Discord" => Ok(Self::Discord),
            "Facebook" => Ok(Self::Facebook),
            "Github" => Ok(Self::Github),
            "Instagram" => Ok(Self::Instagram),
            "Medium" => Ok(Self::Medium),
            "OpenSea" => Ok(Self::OpenSea),
            "Reddit" => Ok(Self::Reddit),
            "Telegram" => Ok(Self::Telegram),
            "TikTok" => Ok(Self::TikTok),
            "Twitch" => Ok(Self::Twitch),
            "Twitter" => Ok(Self::Twitter),
            "Website" => Ok(Self::Website),
            "WeChat" => Ok(Self::WeChat),
            "Weibo" => Ok(Self::Weibo),
            "YouTube" => Ok(Self::YouTube),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str>
for ListOrdersResponseItemsItemTokenSocialLinksItemPlatform {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for ListOrdersResponseItemsItemTokenSocialLinksItemPlatform {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for ListOrdersResponseItemsItemTokenSocialLinksItemPlatform {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///The type of the order: "limit" or "spot"
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "The type of the order: \"limit\" or \"spot\"",
///  "type": "string",
///  "enum": [
///    "limit",
///    "spot"
///  ],
///  "name": "Order Type"
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
pub enum ListOrdersResponseItemsItemType {
    #[serde(rename = "limit")]
    Limit,
    #[serde(rename = "spot")]
    Spot,
}
impl ::std::fmt::Display for ListOrdersResponseItemsItemType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Limit => f.write_str("limit"),
            Self::Spot => f.write_str("spot"),
        }
    }
}
impl ::std::str::FromStr for ListOrdersResponseItemsItemType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "limit" => Ok(Self::Limit),
            "spot" => Ok(Self::Spot),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ListOrdersResponseItemsItemType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for ListOrdersResponseItemsItemType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ListOrdersResponseItemsItemType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
