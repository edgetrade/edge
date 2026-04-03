#![allow(clippy::derivable_impls)]

use crate::client::{Route, RouteType};
use std::marker::PhantomData;
/// Route metadata for this procedure
pub const ROUTE: Route<WalletHoldingsRequest, WalletHoldingsResponse> = Route {
    procedure: "wallet.getHoldings",
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
///Query options for fetching portfolio holdings. At least one wallet or signal is required.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Query options for fetching portfolio holdings. At least one wallet or signal is required.",
///  "type": "object",
///  "required": [
///    "filters",
///    "limit"
///  ],
///  "properties": {
///    "cursor": {
///      "description": "Pagination cursor for fetching the next page of results",
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Cursor"
///    },
///    "filters": {
///      "description": "Filter options for querying portfolio holdings",
///      "type": "object",
///      "required": [
///        "chainId"
///      ],
///      "properties": {
///        "chainId": {
///          "description": "Chain to filter by. Pass null (or omit) to return holdings across all chains.",
///          "anyOf": [
///            {
///              "type": "string"
///            },
///            {
///              "type": "null"
///            }
///          ],
///          "name": "Chain ID"
///        },
///        "excludeTokenContractAddresses": {
///          "description": "Array of token contract addresses to exclude from results",
///          "type": "array",
///          "items": {
///            "type": "string"
///          },
///          "name": "Exclude Token Contract Addresses"
///        },
///        "isInTrade": {
///          "description": "Filter to only return holdings that are currently in an active trade",
///          "anyOf": [
///            {
///              "type": "boolean"
///            },
///            {
///              "type": "null"
///            }
///          ],
///          "name": "Is In Trade"
///        },
///        "minMostRecentTx": {
///          "description": "Filter holdings to only include those with transactions after this timestamp (ISO 8601 datetime)",
///          "anyOf": [
///            {
///              "type": "string",
///              "format": "date-time",
///              "pattern": "^(?:(?:\\d\\d[2468][048]|\\d\\d[13579][26]|\\d\\d0[48]|[02468][048]00|[13579][26]00)-02-29|\\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\\d|30)|(?:02)-(?:0[1-9]|1\\d|2[0-8])))T(?:(?:[01]\\d|2[0-3]):[0-5]\\d(?::[0-5]\\d(?:\\.\\d+)?)?(?:Z))$"
///            },
///            {
///              "type": "null"
///            }
///          ],
///          "name": "Minimum Most Recent Transaction"
///        },
///        "mustIncludeToken": {
///          "description": "A specific token that must be included in the results",
///          "anyOf": [
///            {
///              "type": "object",
///              "required": [
///                "tokenChainId",
///                "tokenContractAddress"
///              ],
///              "properties": {
///                "tokenChainId": {
///                  "description": "Chain ID of the token that must be included",
///                  "type": "string",
///                  "name": "Token Chain ID"
///                },
///                "tokenContractAddress": {
///                  "description": "Contract address of the token that must be included",
///                  "type": "string",
///                  "name": "Token Contract Address"
///                }
///              }
///            },
///            {
///              "type": "null"
///            }
///          ],
///          "name": "Must Include Token"
///        },
///        "searchText": {
///          "description": "Search text to filter tokens by name or symbol",
///          "anyOf": [
///            {
///              "type": "string"
///            },
///            {
///              "type": "null"
///            }
///          ],
///          "name": "Search Text"
///        },
///        "tokenContractAddresses": {
///          "description": "Array of token contract addresses to filter by",
///          "type": "array",
///          "items": {
///            "type": "string"
///          },
///          "name": "Token Contract Addresses"
///        }
///      },
///      "name": "Get Portfolio Holdings Filter"
///    },
///    "limit": {
///      "description": "Maximum number of holdings to return (1-100)",
///      "type": "number",
///      "maximum": 100.0,
///      "minimum": 1.0,
///      "name": "Limit"
///    },
///    "signals": {
///      "description": "Array of signal UUIDs to query holdings for",
///      "type": "array",
///      "items": {
///        "type": "string"
///      },
///      "name": "Signal UUIDs"
///    },
///    "sortByColumn": {
///      "description": "Column to sort the results by",
///      "default": "remainingUsd",
///      "type": "string",
///      "enum": [
///        "firstBuy",
///        "mostRecentTx",
///        "lastSell",
///        "totalCostUsd",
///        "totalCostNativeToken",
///        "totalSoldUsd",
///        "totalSoldNativeToken",
///        "remainingUsd",
///        "remainingNativeToken",
///        "pnlUsd",
///        "pnlNativeToken",
///        "pnlUsdPercent",
///        "pnlNativeTokenPercent"
///      ],
///      "name": "Sort By Column"
///    },
///    "sortDirection": {
///      "description": "Sort direction: \"asc\" for ascending, \"desc\" for descending",
///      "default": "desc",
///      "type": "string",
///      "enum": [
///        "asc",
///        "desc"
///      ],
///      "name": "Sort Direction"
///    },
///    "wallets": {
///      "description": "Array of wallet addresses to query holdings for",
///      "type": "array",
///      "items": {
///        "type": "string"
///      },
///      "name": "Wallet Addresses"
///    }
///  },
///  "name": "Get Holdings Request"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct WalletHoldingsRequest {
    ///Pagination cursor for fetching the next page of results
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub cursor: ::std::option::Option<::std::string::String>,
    pub filters: WalletHoldingsRequestFilters,
    pub limit: f64,
    ///Array of signal UUIDs to query holdings for
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub signals: ::std::vec::Vec<::std::string::String>,
    ///Column to sort the results by
    #[serde(
        rename = "sortByColumn",
        default = "defaults::wallet_holdings_request_sort_by_column"
    )]
    pub sort_by_column: WalletHoldingsRequestSortByColumn,
    ///Sort direction: "asc" for ascending, "desc" for descending
    #[serde(
        rename = "sortDirection",
        default = "defaults::wallet_holdings_request_sort_direction"
    )]
    pub sort_direction: WalletHoldingsRequestSortDirection,
    ///Array of wallet addresses to query holdings for
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub wallets: ::std::vec::Vec<::std::string::String>,
}
///Filter options for querying portfolio holdings
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Filter options for querying portfolio holdings",
///  "type": "object",
///  "required": [
///    "chainId"
///  ],
///  "properties": {
///    "chainId": {
///      "description": "Chain to filter by. Pass null (or omit) to return holdings across all chains.",
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Chain ID"
///    },
///    "excludeTokenContractAddresses": {
///      "description": "Array of token contract addresses to exclude from results",
///      "type": "array",
///      "items": {
///        "type": "string"
///      },
///      "name": "Exclude Token Contract Addresses"
///    },
///    "isInTrade": {
///      "description": "Filter to only return holdings that are currently in an active trade",
///      "anyOf": [
///        {
///          "type": "boolean"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Is In Trade"
///    },
///    "minMostRecentTx": {
///      "description": "Filter holdings to only include those with transactions after this timestamp (ISO 8601 datetime)",
///      "anyOf": [
///        {
///          "type": "string",
///          "format": "date-time",
///          "pattern": "^(?:(?:\\d\\d[2468][048]|\\d\\d[13579][26]|\\d\\d0[48]|[02468][048]00|[13579][26]00)-02-29|\\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\\d|30)|(?:02)-(?:0[1-9]|1\\d|2[0-8])))T(?:(?:[01]\\d|2[0-3]):[0-5]\\d(?::[0-5]\\d(?:\\.\\d+)?)?(?:Z))$"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Minimum Most Recent Transaction"
///    },
///    "mustIncludeToken": {
///      "description": "A specific token that must be included in the results",
///      "anyOf": [
///        {
///          "type": "object",
///          "required": [
///            "tokenChainId",
///            "tokenContractAddress"
///          ],
///          "properties": {
///            "tokenChainId": {
///              "description": "Chain ID of the token that must be included",
///              "type": "string",
///              "name": "Token Chain ID"
///            },
///            "tokenContractAddress": {
///              "description": "Contract address of the token that must be included",
///              "type": "string",
///              "name": "Token Contract Address"
///            }
///          }
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Must Include Token"
///    },
///    "searchText": {
///      "description": "Search text to filter tokens by name or symbol",
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Search Text"
///    },
///    "tokenContractAddresses": {
///      "description": "Array of token contract addresses to filter by",
///      "type": "array",
///      "items": {
///        "type": "string"
///      },
///      "name": "Token Contract Addresses"
///    }
///  },
///  "name": "Get Portfolio Holdings Filter"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct WalletHoldingsRequestFilters {
    ///Chain to filter by. Pass null (or omit) to return holdings across all chains.
    #[serde(rename = "chainId")]
    pub chain_id: ::std::option::Option<::std::string::String>,
    ///Array of token contract addresses to exclude from results
    #[serde(
        rename = "excludeTokenContractAddresses",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub exclude_token_contract_addresses: ::std::vec::Vec<::std::string::String>,
    ///Filter to only return holdings that are currently in an active trade
    #[serde(
        rename = "isInTrade",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub is_in_trade: ::std::option::Option<bool>,
    ///Filter holdings to only include those with transactions after this timestamp (ISO 8601 datetime)
    #[serde(
        rename = "minMostRecentTx",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub min_most_recent_tx: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    ///A specific token that must be included in the results
    #[serde(
        rename = "mustIncludeToken",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub must_include_token: ::std::option::Option<WalletHoldingsRequestFiltersMustIncludeToken>,
    ///Search text to filter tokens by name or symbol
    #[serde(
        rename = "searchText",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub search_text: ::std::option::Option<::std::string::String>,
    ///Array of token contract addresses to filter by
    #[serde(
        rename = "tokenContractAddresses",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub token_contract_addresses: ::std::vec::Vec<::std::string::String>,
}
///`WalletHoldingsRequestFiltersMustIncludeToken`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "tokenChainId",
///    "tokenContractAddress"
///  ],
///  "properties": {
///    "tokenChainId": {
///      "description": "Chain ID of the token that must be included",
///      "type": "string",
///      "name": "Token Chain ID"
///    },
///    "tokenContractAddress": {
///      "description": "Contract address of the token that must be included",
///      "type": "string",
///      "name": "Token Contract Address"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct WalletHoldingsRequestFiltersMustIncludeToken {
    ///Chain ID of the token that must be included
    #[serde(rename = "tokenChainId")]
    pub token_chain_id: ::std::string::String,
    ///Contract address of the token that must be included
    #[serde(rename = "tokenContractAddress")]
    pub token_contract_address: ::std::string::String,
}
///Column to sort the results by
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Column to sort the results by",
///  "default": "remainingUsd",
///  "type": "string",
///  "enum": [
///    "firstBuy",
///    "mostRecentTx",
///    "lastSell",
///    "totalCostUsd",
///    "totalCostNativeToken",
///    "totalSoldUsd",
///    "totalSoldNativeToken",
///    "remainingUsd",
///    "remainingNativeToken",
///    "pnlUsd",
///    "pnlNativeToken",
///    "pnlUsdPercent",
///    "pnlNativeTokenPercent"
///  ],
///  "name": "Sort By Column"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WalletHoldingsRequestSortByColumn {
    #[serde(rename = "firstBuy")]
    FirstBuy,
    #[serde(rename = "mostRecentTx")]
    MostRecentTx,
    #[serde(rename = "lastSell")]
    LastSell,
    #[serde(rename = "totalCostUsd")]
    TotalCostUsd,
    #[serde(rename = "totalCostNativeToken")]
    TotalCostNativeToken,
    #[serde(rename = "totalSoldUsd")]
    TotalSoldUsd,
    #[serde(rename = "totalSoldNativeToken")]
    TotalSoldNativeToken,
    #[serde(rename = "remainingUsd")]
    RemainingUsd,
    #[serde(rename = "remainingNativeToken")]
    RemainingNativeToken,
    #[serde(rename = "pnlUsd")]
    PnlUsd,
    #[serde(rename = "pnlNativeToken")]
    PnlNativeToken,
    #[serde(rename = "pnlUsdPercent")]
    PnlUsdPercent,
    #[serde(rename = "pnlNativeTokenPercent")]
    PnlNativeTokenPercent,
}
impl ::std::fmt::Display for WalletHoldingsRequestSortByColumn {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::FirstBuy => f.write_str("firstBuy"),
            Self::MostRecentTx => f.write_str("mostRecentTx"),
            Self::LastSell => f.write_str("lastSell"),
            Self::TotalCostUsd => f.write_str("totalCostUsd"),
            Self::TotalCostNativeToken => f.write_str("totalCostNativeToken"),
            Self::TotalSoldUsd => f.write_str("totalSoldUsd"),
            Self::TotalSoldNativeToken => f.write_str("totalSoldNativeToken"),
            Self::RemainingUsd => f.write_str("remainingUsd"),
            Self::RemainingNativeToken => f.write_str("remainingNativeToken"),
            Self::PnlUsd => f.write_str("pnlUsd"),
            Self::PnlNativeToken => f.write_str("pnlNativeToken"),
            Self::PnlUsdPercent => f.write_str("pnlUsdPercent"),
            Self::PnlNativeTokenPercent => f.write_str("pnlNativeTokenPercent"),
        }
    }
}
impl ::std::str::FromStr for WalletHoldingsRequestSortByColumn {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "firstBuy" => Ok(Self::FirstBuy),
            "mostRecentTx" => Ok(Self::MostRecentTx),
            "lastSell" => Ok(Self::LastSell),
            "totalCostUsd" => Ok(Self::TotalCostUsd),
            "totalCostNativeToken" => Ok(Self::TotalCostNativeToken),
            "totalSoldUsd" => Ok(Self::TotalSoldUsd),
            "totalSoldNativeToken" => Ok(Self::TotalSoldNativeToken),
            "remainingUsd" => Ok(Self::RemainingUsd),
            "remainingNativeToken" => Ok(Self::RemainingNativeToken),
            "pnlUsd" => Ok(Self::PnlUsd),
            "pnlNativeToken" => Ok(Self::PnlNativeToken),
            "pnlUsdPercent" => Ok(Self::PnlUsdPercent),
            "pnlNativeTokenPercent" => Ok(Self::PnlNativeTokenPercent),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for WalletHoldingsRequestSortByColumn {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for WalletHoldingsRequestSortByColumn {
    type Error = self::error::ConversionError;
    fn try_from(value: &::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for WalletHoldingsRequestSortByColumn {
    type Error = self::error::ConversionError;
    fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for WalletHoldingsRequestSortByColumn {
    fn default() -> Self {
        WalletHoldingsRequestSortByColumn::RemainingUsd
    }
}
///Sort direction: "asc" for ascending, "desc" for descending
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Sort direction: \"asc\" for ascending, \"desc\" for descending",
///  "default": "desc",
///  "type": "string",
///  "enum": [
///    "asc",
///    "desc"
///  ],
///  "name": "Sort Direction"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WalletHoldingsRequestSortDirection {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}
impl ::std::fmt::Display for WalletHoldingsRequestSortDirection {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Asc => f.write_str("asc"),
            Self::Desc => f.write_str("desc"),
        }
    }
}
impl ::std::str::FromStr for WalletHoldingsRequestSortDirection {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "asc" => Ok(Self::Asc),
            "desc" => Ok(Self::Desc),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for WalletHoldingsRequestSortDirection {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for WalletHoldingsRequestSortDirection {
    type Error = self::error::ConversionError;
    fn try_from(value: &::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for WalletHoldingsRequestSortDirection {
    type Error = self::error::ConversionError;
    fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for WalletHoldingsRequestSortDirection {
    fn default() -> Self {
        WalletHoldingsRequestSortDirection::Desc
    }
}
///Response containing paginated portfolio holdings with detailed statistics
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Response containing paginated portfolio holdings with detailed statistics",
///  "type": "object",
///  "required": [
///    "items",
///    "next"
///  ],
///  "properties": {
///    "items": {
///      "description": "Array of portfolio holdings",
///      "type": "array",
///      "items": {
///        "type": "object",
///        "required": [
///          "atas",
///          "avgCostNativeToken",
///          "avgCostUsd",
///          "avgSoldNativeToken",
///          "avgSoldUsd",
///          "avgTokensBought",
///          "avgTokensSold",
///          "chainId",
///          "currentBalance",
///          "currentCostBasisUsd",
///          "firstBuy",
///          "inTrade",
///          "isInsider",
///          "isSniper",
///          "lastSell",
///          "mostRecentTx",
///          "pnlNativeToken",
///          "pnlNativeTokenPercent",
///          "pnlUsd",
///          "pnlUsdPercent",
///          "remainingNativeToken",
///          "remainingUsd",
///          "tokenAddress",
///          "tokenBestPairAddress",
///          "tokenBestPairType",
///          "tokenDecimals",
///          "tokenName",
///          "tokenPriceNativeToken",
///          "tokenPriceUsd",
///          "tokenSymbol",
///          "tokensBought",
///          "tokensSold",
///          "totalBuys",
///          "totalCostNativeToken",
///          "totalCostUsd",
///          "totalSells",
///          "totalSoldNativeToken",
///          "totalSoldUsd",
///          "totalTransfers",
///          "transferedIn",
///          "transferedOut",
///          "walletAddress"
///        ],
///        "properties": {
///          "atas": {
///            "type": "array",
///            "items": {
///              "type": "object",
///              "required": [
///                "ataAddress",
///                "balance"
///              ],
///              "properties": {
///                "ataAddress": {
///                  "description": "The ATA address of the holding",
///                  "type": "string",
///                  "name": "ATA Address"
///                },
///                "balance": {
///                  "description": "The balance of the holding",
///                  "anyOf": [
///                    {
///                      "type": "string"
///                    },
///                    {
///                      "type": "null"
///                    }
///                  ],
///                  "name": "Balance"
///                }
///              },
///              "additionalProperties": false
///            }
///          },
///          "avgCostNativeToken": {
///            "description": "The average cost of the holding in native token",
///            "anyOf": [
///              {
///                "type": "number"
///              },
///              {
///                "type": "null"
///              }
///            ],
///            "name": "Average Cost Native Token"
///          },
///          "avgCostUsd": {
///            "description": "The average cost of the holding in USD",
///            "anyOf": [
///              {
///                "type": "number"
///              },
///              {
///                "type": "null"
///              }
///            ],
///            "name": "Average Cost USD"
///          },
///          "avgSoldNativeToken": {
///            "description": "The average sold of the holding in native token",
///            "anyOf": [
///              {
///                "type": "number"
///              },
///              {
///                "type": "null"
///              }
///            ],
///            "name": "Average Sold Native Token"
///          },
///          "avgSoldUsd": {
///            "description": "The average sold of the holding in USD",
///            "anyOf": [
///              {
///                "type": "number"
///              },
///              {
///                "type": "null"
///              }
///            ],
///            "name": "Average Sold USD"
///          },
///          "avgTokensBought": {
///            "description": "The average number of tokens bought of the holding",
///            "anyOf": [
///              {
///                "type": "number"
///              },
///              {
///                "type": "null"
///              }
///            ],
///            "name": "Average Tokens Bought"
///          },
///          "avgTokensSold": {
///            "description": "The average number of tokens sold of the holding",
///            "anyOf": [
///              {
///                "type": "number"
///              },
///              {
///                "type": "null"
///              }
///            ],
///            "name": "Average Tokens Sold"
///          },
///          "chainId": {
///            "description": "The chain ID where the holding is located",
///            "type": "string",
///            "name": "Chain ID"
///          },
///          "currentBalance": {
///            "description": "The current balance of the holding",
///            "anyOf": [
///              {
///                "type": "string"
///              },
///              {
///                "type": "null"
///              }
///            ],
///            "name": "Current Balance"
///          },
///          "currentCostBasisUsd": {
///            "description": "The current cost basis of the holding in USD",
///            "anyOf": [
///              {
///                "type": "number"
///              },
///              {
///                "type": "null"
///              }
///            ],
///            "name": "Current Cost Basis USD"
///          },
///          "firstBuy": {
///            "description": "The first buy date of the holding",
///            "anyOf": [
///              {
///                "type": "string"
///              },
///              {
///                "type": "null"
///              }
///            ],
///            "name": "First Buy"
///          },
///          "inTrade": {
///            "description": "Whether the holding is in a trade",
///            "anyOf": [
///              {
///                "type": "boolean"
///              },
///              {
///                "type": "null"
///              }
///            ],
///            "name": "In Trade"
///          },
///          "isInsider": {
///            "description": "Whether the holding is an insider",
///            "anyOf": [
///              {
///                "type": "boolean"
///              },
///              {
///                "type": "null"
///              }
///            ],
///            "name": "Is Insider"
///          },
///          "isSniper": {
///            "description": "Whether the holding is a sniper",
///            "anyOf": [
///              {
///                "type": "boolean"
///              },
///              {
///                "type": "null"
///              }
///            ],
///            "name": "Is Sniper"
///          },
///          "lastSell": {
///            "description": "The last sell date of the holding",
///            "anyOf": [
///              {
///                "type": "string"
///              },
///              {
///                "type": "null"
///              }
///            ],
///            "name": "Last Sell"
///          },
///          "mostRecentTx": {
///            "description": "The most recent transaction date of the holding",
///            "anyOf": [
///              {
///                "type": "string"
///              },
///              {
///                "type": "null"
///              }
///            ],
///            "name": "Most Recent Transaction"
///          },
///          "pnlNativeToken": {
///            "description": "The PnL of the holding in native tokens",
///            "type": "number",
///            "name": "PnL Native Token"
///          },
///          "pnlNativeTokenPercent": {
///            "description": "The PnL percentage of the holding in native tokens",
///            "type": "number",
///            "name": "PnL Native Token Percent"
///          },
///          "pnlUsd": {
///            "description": "The PnL of the holding in USD",
///            "type": "number",
///            "name": "PnL USD"
///          },
///          "pnlUsdPercent": {
///            "description": "The PnL percentage of the holding in USD",
///            "type": "number",
///            "name": "PnL USD Percent"
///          },
///          "remainingNativeToken": {
///            "description": "The remaining value of the holding in native tokens",
///            "type": "number",
///            "name": "Remaining Native Token"
///          },
///          "remainingUsd": {
///            "description": "The remaining value of the holding in USD",
///            "type": "number",
///            "name": "Remaining USD"
///          },
///          "tokenAddress": {
///            "description": "The token address of the holding",
///            "type": "string",
///            "name": "Token Address"
///          },
///          "tokenBestPairAddress": {
///            "description": "The address of the best pair of the token",
///            "type": "string",
///            "name": "Token Best Pair Address"
///          },
///          "tokenBestPairType": {
///            "description": "The type of the best pair of the token",
///            "anyOf": [
///              {
///                "type": "string"
///              },
///              {
///                "type": "null"
///              }
///            ],
///            "name": "Token Best Pair Type"
///          },
///          "tokenDecimals": {
///            "description": "The decimals of the token",
///            "type": "number",
///            "name": "Token Decimals"
///          },
///          "tokenLogoUrl": {
///            "description": "The logo URL of the token",
///            "type": "string",
///            "name": "Token Logo URL"
///          },
///          "tokenName": {
///            "description": "The name of the token",
///            "type": "string",
///            "name": "Token Name"
///          },
///          "tokenPriceNativeToken": {
///            "description": "The price of the token in native tokens",
///            "type": "number",
///            "name": "Token Price Native Token"
///          },
///          "tokenPriceUsd": {
///            "description": "The price of the token in USD",
///            "type": "number",
///            "name": "Token Price USD"
///          },
///          "tokenSymbol": {
///            "description": "The symbol of the token",
///            "type": "string",
///            "name": "Token Symbol"
///          },
///          "tokensBought": {
///            "description": "The number of tokens bought of the holding",
///            "anyOf": [
///              {
///                "type": "string"
///              },
///              {
///                "type": "null"
///              }
///            ],
///            "name": "Tokens Bought"
///          },
///          "tokensSold": {
///            "description": "The number of tokens sold of the holding",
///            "anyOf": [
///              {
///                "type": "string"
///              },
///              {
///                "type": "null"
///              }
///            ],
///            "name": "Tokens Sold"
///          },
///          "totalBuys": {
///            "description": "The total number of buys of the holding",
///            "anyOf": [
///              {
///                "type": "number"
///              },
///              {
///                "type": "null"
///              }
///            ],
///            "name": "Total Buys"
///          },
///          "totalCostNativeToken": {
///            "description": "The total cost of the holding in native token",
///            "anyOf": [
///              {
///                "type": "number"
///              },
///              {
///                "type": "null"
///              }
///            ],
///            "name": "Total Cost Native Token"
///          },
///          "totalCostUsd": {
///            "description": "The total cost of the holding in USD",
///            "anyOf": [
///              {
///                "type": "number"
///              },
///              {
///                "type": "null"
///              }
///            ],
///            "name": "Total Cost USD"
///          },
///          "totalSells": {
///            "description": "The total number of sells of the holding",
///            "anyOf": [
///              {
///                "type": "number"
///              },
///              {
///                "type": "null"
///              }
///            ],
///            "name": "Total Sells"
///          },
///          "totalSoldNativeToken": {
///            "description": "The total sold of the holding in native token",
///            "anyOf": [
///              {
///                "type": "number"
///              },
///              {
///                "type": "null"
///              }
///            ],
///            "name": "Total Sold Native Token"
///          },
///          "totalSoldUsd": {
///            "description": "The total sold of the holding in USD",
///            "anyOf": [
///              {
///                "type": "number"
///              },
///              {
///                "type": "null"
///              }
///            ],
///            "name": "Total Sold USD"
///          },
///          "totalTransfers": {
///            "description": "The total number of transfers of the holding",
///            "anyOf": [
///              {
///                "type": "number"
///              },
///              {
///                "type": "null"
///              }
///            ],
///            "name": "Total Transfers"
///          },
///          "transferedIn": {
///            "description": "The number of tokens transferred in of the holding",
///            "anyOf": [
///              {
///                "type": "string"
///              },
///              {
///                "type": "null"
///              }
///            ],
///            "name": "Transferred In"
///          },
///          "transferedOut": {
///            "description": "The number of tokens transferred out of the holding",
///            "anyOf": [
///              {
///                "type": "string"
///              },
///              {
///                "type": "null"
///              }
///            ],
///            "name": "Transferred Out"
///          },
///          "walletAddress": {
///            "description": "The wallet address of the holding",
///            "type": "string",
///            "name": "Wallet Address"
///          }
///        },
///        "additionalProperties": false
///      },
///      "name": "Items"
///    },
///    "next": {
///      "description": "Pagination cursor for fetching the next page of results",
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Next Cursor"
///    }
///  },
///  "additionalProperties": false,
///  "name": "Get Holdings Response"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct WalletHoldingsResponse {
    ///Array of portfolio holdings
    pub items: ::std::vec::Vec<WalletHoldingsResponseItemsItem>,
    ///Pagination cursor for fetching the next page of results
    pub next: ::std::option::Option<::std::string::String>,
}
///`WalletHoldingsResponseItemsItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
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
///    "currentBalance",
///    "currentCostBasisUsd",
///    "firstBuy",
///    "inTrade",
///    "isInsider",
///    "isSniper",
///    "lastSell",
///    "mostRecentTx",
///    "pnlNativeToken",
///    "pnlNativeTokenPercent",
///    "pnlUsd",
///    "pnlUsdPercent",
///    "remainingNativeToken",
///    "remainingUsd",
///    "tokenAddress",
///    "tokenBestPairAddress",
///    "tokenBestPairType",
///    "tokenDecimals",
///    "tokenName",
///    "tokenPriceNativeToken",
///    "tokenPriceUsd",
///    "tokenSymbol",
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
///    "walletAddress"
///  ],
///  "properties": {
///    "atas": {
///      "type": "array",
///      "items": {
///        "type": "object",
///        "required": [
///          "ataAddress",
///          "balance"
///        ],
///        "properties": {
///          "ataAddress": {
///            "description": "The ATA address of the holding",
///            "type": "string",
///            "name": "ATA Address"
///          },
///          "balance": {
///            "description": "The balance of the holding",
///            "anyOf": [
///              {
///                "type": "string"
///              },
///              {
///                "type": "null"
///              }
///            ],
///            "name": "Balance"
///          }
///        },
///        "additionalProperties": false
///      }
///    },
///    "avgCostNativeToken": {
///      "description": "The average cost of the holding in native token",
///      "anyOf": [
///        {
///          "type": "number"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Average Cost Native Token"
///    },
///    "avgCostUsd": {
///      "description": "The average cost of the holding in USD",
///      "anyOf": [
///        {
///          "type": "number"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Average Cost USD"
///    },
///    "avgSoldNativeToken": {
///      "description": "The average sold of the holding in native token",
///      "anyOf": [
///        {
///          "type": "number"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Average Sold Native Token"
///    },
///    "avgSoldUsd": {
///      "description": "The average sold of the holding in USD",
///      "anyOf": [
///        {
///          "type": "number"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Average Sold USD"
///    },
///    "avgTokensBought": {
///      "description": "The average number of tokens bought of the holding",
///      "anyOf": [
///        {
///          "type": "number"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Average Tokens Bought"
///    },
///    "avgTokensSold": {
///      "description": "The average number of tokens sold of the holding",
///      "anyOf": [
///        {
///          "type": "number"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Average Tokens Sold"
///    },
///    "chainId": {
///      "description": "The chain ID where the holding is located",
///      "type": "string",
///      "name": "Chain ID"
///    },
///    "currentBalance": {
///      "description": "The current balance of the holding",
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Current Balance"
///    },
///    "currentCostBasisUsd": {
///      "description": "The current cost basis of the holding in USD",
///      "anyOf": [
///        {
///          "type": "number"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Current Cost Basis USD"
///    },
///    "firstBuy": {
///      "description": "The first buy date of the holding",
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
///      "description": "Whether the holding is in a trade",
///      "anyOf": [
///        {
///          "type": "boolean"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "In Trade"
///    },
///    "isInsider": {
///      "description": "Whether the holding is an insider",
///      "anyOf": [
///        {
///          "type": "boolean"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Is Insider"
///    },
///    "isSniper": {
///      "description": "Whether the holding is a sniper",
///      "anyOf": [
///        {
///          "type": "boolean"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Is Sniper"
///    },
///    "lastSell": {
///      "description": "The last sell date of the holding",
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
///      "description": "The most recent transaction date of the holding",
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
///    "pnlNativeToken": {
///      "description": "The PnL of the holding in native tokens",
///      "type": "number",
///      "name": "PnL Native Token"
///    },
///    "pnlNativeTokenPercent": {
///      "description": "The PnL percentage of the holding in native tokens",
///      "type": "number",
///      "name": "PnL Native Token Percent"
///    },
///    "pnlUsd": {
///      "description": "The PnL of the holding in USD",
///      "type": "number",
///      "name": "PnL USD"
///    },
///    "pnlUsdPercent": {
///      "description": "The PnL percentage of the holding in USD",
///      "type": "number",
///      "name": "PnL USD Percent"
///    },
///    "remainingNativeToken": {
///      "description": "The remaining value of the holding in native tokens",
///      "type": "number",
///      "name": "Remaining Native Token"
///    },
///    "remainingUsd": {
///      "description": "The remaining value of the holding in USD",
///      "type": "number",
///      "name": "Remaining USD"
///    },
///    "tokenAddress": {
///      "description": "The token address of the holding",
///      "type": "string",
///      "name": "Token Address"
///    },
///    "tokenBestPairAddress": {
///      "description": "The address of the best pair of the token",
///      "type": "string",
///      "name": "Token Best Pair Address"
///    },
///    "tokenBestPairType": {
///      "description": "The type of the best pair of the token",
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Token Best Pair Type"
///    },
///    "tokenDecimals": {
///      "description": "The decimals of the token",
///      "type": "number",
///      "name": "Token Decimals"
///    },
///    "tokenLogoUrl": {
///      "description": "The logo URL of the token",
///      "type": "string",
///      "name": "Token Logo URL"
///    },
///    "tokenName": {
///      "description": "The name of the token",
///      "type": "string",
///      "name": "Token Name"
///    },
///    "tokenPriceNativeToken": {
///      "description": "The price of the token in native tokens",
///      "type": "number",
///      "name": "Token Price Native Token"
///    },
///    "tokenPriceUsd": {
///      "description": "The price of the token in USD",
///      "type": "number",
///      "name": "Token Price USD"
///    },
///    "tokenSymbol": {
///      "description": "The symbol of the token",
///      "type": "string",
///      "name": "Token Symbol"
///    },
///    "tokensBought": {
///      "description": "The number of tokens bought of the holding",
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
///      "description": "The number of tokens sold of the holding",
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
///      "description": "The total number of buys of the holding",
///      "anyOf": [
///        {
///          "type": "number"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Total Buys"
///    },
///    "totalCostNativeToken": {
///      "description": "The total cost of the holding in native token",
///      "anyOf": [
///        {
///          "type": "number"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Total Cost Native Token"
///    },
///    "totalCostUsd": {
///      "description": "The total cost of the holding in USD",
///      "anyOf": [
///        {
///          "type": "number"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Total Cost USD"
///    },
///    "totalSells": {
///      "description": "The total number of sells of the holding",
///      "anyOf": [
///        {
///          "type": "number"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Total Sells"
///    },
///    "totalSoldNativeToken": {
///      "description": "The total sold of the holding in native token",
///      "anyOf": [
///        {
///          "type": "number"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Total Sold Native Token"
///    },
///    "totalSoldUsd": {
///      "description": "The total sold of the holding in USD",
///      "anyOf": [
///        {
///          "type": "number"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Total Sold USD"
///    },
///    "totalTransfers": {
///      "description": "The total number of transfers of the holding",
///      "anyOf": [
///        {
///          "type": "number"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Total Transfers"
///    },
///    "transferedIn": {
///      "description": "The number of tokens transferred in of the holding",
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Transferred In"
///    },
///    "transferedOut": {
///      "description": "The number of tokens transferred out of the holding",
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Transferred Out"
///    },
///    "walletAddress": {
///      "description": "The wallet address of the holding",
///      "type": "string",
///      "name": "Wallet Address"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct WalletHoldingsResponseItemsItem {
    pub atas: ::std::vec::Vec<WalletHoldingsResponseItemsItemAtasItem>,
    ///The average cost of the holding in native token
    #[serde(rename = "avgCostNativeToken")]
    pub avg_cost_native_token: ::std::option::Option<f64>,
    ///The average cost of the holding in USD
    #[serde(rename = "avgCostUsd")]
    pub avg_cost_usd: ::std::option::Option<f64>,
    ///The average sold of the holding in native token
    #[serde(rename = "avgSoldNativeToken")]
    pub avg_sold_native_token: ::std::option::Option<f64>,
    ///The average sold of the holding in USD
    #[serde(rename = "avgSoldUsd")]
    pub avg_sold_usd: ::std::option::Option<f64>,
    ///The average number of tokens bought of the holding
    #[serde(rename = "avgTokensBought")]
    pub avg_tokens_bought: ::std::option::Option<f64>,
    ///The average number of tokens sold of the holding
    #[serde(rename = "avgTokensSold")]
    pub avg_tokens_sold: ::std::option::Option<f64>,
    ///The chain ID where the holding is located
    #[serde(rename = "chainId")]
    pub chain_id: ::std::string::String,
    ///The current balance of the holding
    #[serde(rename = "currentBalance")]
    pub current_balance: ::std::option::Option<::std::string::String>,
    ///The current cost basis of the holding in USD
    #[serde(rename = "currentCostBasisUsd")]
    pub current_cost_basis_usd: ::std::option::Option<f64>,
    ///The first buy date of the holding
    #[serde(rename = "firstBuy")]
    pub first_buy: ::std::option::Option<::std::string::String>,
    ///Whether the holding is in a trade
    #[serde(rename = "inTrade")]
    pub in_trade: ::std::option::Option<bool>,
    ///Whether the holding is an insider
    #[serde(rename = "isInsider")]
    pub is_insider: ::std::option::Option<bool>,
    ///Whether the holding is a sniper
    #[serde(rename = "isSniper")]
    pub is_sniper: ::std::option::Option<bool>,
    ///The last sell date of the holding
    #[serde(rename = "lastSell")]
    pub last_sell: ::std::option::Option<::std::string::String>,
    ///The most recent transaction date of the holding
    #[serde(rename = "mostRecentTx")]
    pub most_recent_tx: ::std::option::Option<::std::string::String>,
    #[serde(rename = "pnlNativeToken")]
    pub pnl_native_token: f64,
    #[serde(rename = "pnlNativeTokenPercent")]
    pub pnl_native_token_percent: f64,
    #[serde(rename = "pnlUsd")]
    pub pnl_usd: f64,
    #[serde(rename = "pnlUsdPercent")]
    pub pnl_usd_percent: f64,
    #[serde(rename = "remainingNativeToken")]
    pub remaining_native_token: f64,
    #[serde(rename = "remainingUsd")]
    pub remaining_usd: f64,
    ///The token address of the holding
    #[serde(rename = "tokenAddress")]
    pub token_address: ::std::string::String,
    ///The address of the best pair of the token
    #[serde(rename = "tokenBestPairAddress")]
    pub token_best_pair_address: ::std::string::String,
    ///The type of the best pair of the token
    #[serde(rename = "tokenBestPairType")]
    pub token_best_pair_type: ::std::option::Option<::std::string::String>,
    #[serde(rename = "tokenDecimals")]
    pub token_decimals: f64,
    ///The logo URL of the token
    #[serde(
        rename = "tokenLogoUrl",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub token_logo_url: ::std::option::Option<::std::string::String>,
    ///The name of the token
    #[serde(rename = "tokenName")]
    pub token_name: ::std::string::String,
    #[serde(rename = "tokenPriceNativeToken")]
    pub token_price_native_token: f64,
    #[serde(rename = "tokenPriceUsd")]
    pub token_price_usd: f64,
    ///The symbol of the token
    #[serde(rename = "tokenSymbol")]
    pub token_symbol: ::std::string::String,
    ///The number of tokens bought of the holding
    #[serde(rename = "tokensBought")]
    pub tokens_bought: ::std::option::Option<::std::string::String>,
    ///The number of tokens sold of the holding
    #[serde(rename = "tokensSold")]
    pub tokens_sold: ::std::option::Option<::std::string::String>,
    ///The total number of buys of the holding
    #[serde(rename = "totalBuys")]
    pub total_buys: ::std::option::Option<f64>,
    ///The total cost of the holding in native token
    #[serde(rename = "totalCostNativeToken")]
    pub total_cost_native_token: ::std::option::Option<f64>,
    ///The total cost of the holding in USD
    #[serde(rename = "totalCostUsd")]
    pub total_cost_usd: ::std::option::Option<f64>,
    ///The total number of sells of the holding
    #[serde(rename = "totalSells")]
    pub total_sells: ::std::option::Option<f64>,
    ///The total sold of the holding in native token
    #[serde(rename = "totalSoldNativeToken")]
    pub total_sold_native_token: ::std::option::Option<f64>,
    ///The total sold of the holding in USD
    #[serde(rename = "totalSoldUsd")]
    pub total_sold_usd: ::std::option::Option<f64>,
    ///The total number of transfers of the holding
    #[serde(rename = "totalTransfers")]
    pub total_transfers: ::std::option::Option<f64>,
    ///The number of tokens transferred in of the holding
    #[serde(rename = "transferedIn")]
    pub transfered_in: ::std::option::Option<::std::string::String>,
    ///The number of tokens transferred out of the holding
    #[serde(rename = "transferedOut")]
    pub transfered_out: ::std::option::Option<::std::string::String>,
    ///The wallet address of the holding
    #[serde(rename = "walletAddress")]
    pub wallet_address: ::std::string::String,
}
///`WalletHoldingsResponseItemsItemAtasItem`
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
///      "description": "The ATA address of the holding",
///      "type": "string",
///      "name": "ATA Address"
///    },
///    "balance": {
///      "description": "The balance of the holding",
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Balance"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct WalletHoldingsResponseItemsItemAtasItem {
    ///The ATA address of the holding
    #[serde(rename = "ataAddress")]
    pub ata_address: ::std::string::String,
    ///The balance of the holding
    pub balance: ::std::option::Option<::std::string::String>,
}
/// Generation of default values for serde.
pub mod defaults {
    pub(super) fn wallet_holdings_request_sort_by_column() -> super::WalletHoldingsRequestSortByColumn {
        super::WalletHoldingsRequestSortByColumn::RemainingUsd
    }
    pub(super) fn wallet_holdings_request_sort_direction() -> super::WalletHoldingsRequestSortDirection {
        super::WalletHoldingsRequestSortDirection::Desc
    }
}
