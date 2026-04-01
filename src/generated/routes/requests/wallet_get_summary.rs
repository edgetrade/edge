#![allow(clippy::derivable_impls)]

use crate::client::{Route, RouteType};
use std::marker::PhantomData;
/// Route metadata for this procedure
pub const ROUTE: Route<WalletSummaryRequest, Vec<WalletSummaryResponseItem>> = Route {
    procedure: "wallet.getSummary",
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
///Options for retrieving a summary of token holdings across wallets
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Options for retrieving a summary of token holdings across wallets",
///  "type": "object",
///  "required": [
///    "filters"
///  ],
///  "properties": {
///    "filters": {
///      "description": "Filters to apply to the holdings summary query",
///      "type": "object",
///      "required": [
///        "chainId",
///        "isInTrade"
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
///            "description": "Token contract address to exclude",
///            "type": "string",
///            "name": "Excluded Token Contract Address"
///          },
///          "name": "Excluded Token Contract Addresses"
///        },
///        "isInTrade": {
///          "description": "Whether to include only tokens currently in an active trade",
///          "type": "boolean",
///          "name": "Is In Trade"
///        },
///        "minMostRecentTx": {
///          "description": "Filter holdings with most recent transaction after this timestamp (ISO 8601 datetime)",
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
///        "tokenContractAddresses": {
///          "description": "Array of specific token contract addresses to include",
///          "type": "array",
///          "items": {
///            "description": "Token contract address to include",
///            "type": "string",
///            "name": "Token Contract Address"
///          },
///          "name": "Token Contract Addresses"
///        }
///      },
///      "name": "Filters"
///    },
///    "signals": {
///      "description": "Array of signal UUIDs to filter holdings by",
///      "type": "array",
///      "items": {
///        "description": "Signal UUID to include in the holdings summary",
///        "type": "string",
///        "name": "Signal ID"
///      },
///      "name": "Signals"
///    },
///    "wallets": {
///      "description": "Array of wallet addresses to filter holdings by",
///      "type": "array",
///      "items": {
///        "description": "Wallet address to include in the holdings summary",
///        "type": "string",
///        "name": "Wallet Address"
///      },
///      "name": "Wallets"
///    }
///  },
///  "name": "Get Summary Request"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct WalletSummaryRequest {
    pub filters: WalletSummaryRequestFilters,
    ///Array of signal UUIDs to filter holdings by
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub signals: ::std::vec::Vec<::std::string::String>,
    ///Array of wallet addresses to filter holdings by
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub wallets: ::std::vec::Vec<::std::string::String>,
}
///Filters to apply to the holdings summary query
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Filters to apply to the holdings summary query",
///  "type": "object",
///  "required": [
///    "chainId",
///    "isInTrade"
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
///        "description": "Token contract address to exclude",
///        "type": "string",
///        "name": "Excluded Token Contract Address"
///      },
///      "name": "Excluded Token Contract Addresses"
///    },
///    "isInTrade": {
///      "description": "Whether to include only tokens currently in an active trade",
///      "type": "boolean",
///      "name": "Is In Trade"
///    },
///    "minMostRecentTx": {
///      "description": "Filter holdings with most recent transaction after this timestamp (ISO 8601 datetime)",
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
///    "tokenContractAddresses": {
///      "description": "Array of specific token contract addresses to include",
///      "type": "array",
///      "items": {
///        "description": "Token contract address to include",
///        "type": "string",
///        "name": "Token Contract Address"
///      },
///      "name": "Token Contract Addresses"
///    }
///  },
///  "name": "Filters"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct WalletSummaryRequestFilters {
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
    ///Whether to include only tokens currently in an active trade
    #[serde(rename = "isInTrade")]
    pub is_in_trade: bool,
    ///Filter holdings with most recent transaction after this timestamp (ISO 8601 datetime)
    #[serde(
        rename = "minMostRecentTx",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub min_most_recent_tx: ::std::option::Option<
        ::chrono::DateTime<::chrono::offset::Utc>,
    >,
    ///Array of specific token contract addresses to include
    #[serde(
        rename = "tokenContractAddresses",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub token_contract_addresses: ::std::vec::Vec<::std::string::String>,
}
///`WalletSummaryResponseItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "chainId",
///    "hugeWins",
///    "largeLosses",
///    "largeWins",
///    "losses",
///    "mediumLosses",
///    "mediumWins",
///    "mostRecentTx",
///    "remainingUsd",
///    "totalCostUsd",
///    "totalHoldings",
///    "totalSoldUsd",
///    "walletAddress",
///    "wins"
///  ],
///  "properties": {
///    "chainId": {
///      "description": "The chain ID where the holdings are located",
///      "type": "string",
///      "name": "Chain ID"
///    },
///    "hugeWins": {
///      "description": "Number of huge winning trades",
///      "type": "number",
///      "name": "Huge Wins"
///    },
///    "largeLosses": {
///      "description": "Number of large losing trades",
///      "type": "number",
///      "name": "Large Losses"
///    },
///    "largeWins": {
///      "description": "Number of large winning trades",
///      "type": "number",
///      "name": "Large Wins"
///    },
///    "losses": {
///      "description": "Number of losing trades",
///      "type": "number",
///      "name": "Losses"
///    },
///    "mediumLosses": {
///      "description": "Number of medium losing trades",
///      "type": "number",
///      "name": "Medium Losses"
///    },
///    "mediumWins": {
///      "description": "Number of medium winning trades",
///      "type": "number",
///      "name": "Medium Wins"
///    },
///    "mostRecentTx": {
///      "description": "ISO 8601 timestamp of the most recent transaction",
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
///    "remainingUsd": {
///      "description": "Remaining value of holdings in USD",
///      "type": "number",
///      "name": "Remaining USD"
///    },
///    "totalCostUsd": {
///      "description": "Total cost basis of holdings in USD",
///      "type": "number",
///      "name": "Total Cost USD"
///    },
///    "totalHoldings": {
///      "description": "Total number of token holdings",
///      "type": "number",
///      "name": "Total Holdings"
///    },
///    "totalSoldUsd": {
///      "description": "Total value of tokens sold in USD",
///      "type": "number",
///      "name": "Total Sold USD"
///    },
///    "walletAddress": {
///      "description": "The wallet address for these holdings",
///      "type": "string",
///      "name": "Wallet Address"
///    },
///    "wins": {
///      "description": "Number of winning trades",
///      "type": "number",
///      "name": "Wins"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct WalletSummaryResponseItem {
    ///The chain ID where the holdings are located
    #[serde(rename = "chainId")]
    pub chain_id: ::std::string::String,
    #[serde(rename = "hugeWins")]
    pub huge_wins: f64,
    #[serde(rename = "largeLosses")]
    pub large_losses: f64,
    #[serde(rename = "largeWins")]
    pub large_wins: f64,
    pub losses: f64,
    #[serde(rename = "mediumLosses")]
    pub medium_losses: f64,
    #[serde(rename = "mediumWins")]
    pub medium_wins: f64,
    ///ISO 8601 timestamp of the most recent transaction
    #[serde(rename = "mostRecentTx")]
    pub most_recent_tx: ::std::option::Option<::std::string::String>,
    #[serde(rename = "remainingUsd")]
    pub remaining_usd: f64,
    #[serde(rename = "totalCostUsd")]
    pub total_cost_usd: f64,
    #[serde(rename = "totalHoldings")]
    pub total_holdings: f64,
    #[serde(rename = "totalSoldUsd")]
    pub total_sold_usd: f64,
    ///The wallet address for these holdings
    #[serde(rename = "walletAddress")]
    pub wallet_address: ::std::string::String,
    pub wins: f64,
}
