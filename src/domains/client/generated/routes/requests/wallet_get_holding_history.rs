#![allow(clippy::derivable_impls)]

use crate::client::{Route, RouteType};
use std::marker::PhantomData;
/// Route metadata for this procedure
pub const ROUTE: Route<WalletHistoryRequest, Vec<WalletHistoryResponseItem>> = Route {
    procedure: "wallet.getHoldingHistory",
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
///Request options for retrieving holding history data. Requires at least one wallet or signal detail.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Request options for retrieving holding history data. Requires at least one wallet or signal detail.",
///  "type": "object",
///  "required": [
///    "resolution"
///  ],
///  "properties": {
///    "chainId": {
///      "description": "The chain ID to filter holdings by (e.g., \"1\" for Ethereum, \"8453\" for Base, \"solana\" for Solana). Optional.",
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
///      "description": "Array of token contract addresses to exclude from results. Defaults to empty array.",
///      "default": [],
///      "type": "array",
///      "items": {
///        "type": "string"
///      },
///      "name": "Exclude Token Contract Addresses"
///    },
///    "resolution": {
///      "description": "The time resolution for holding history data: \"day\" or \"hour\"",
///      "type": "string",
///      "enum": [
///        "day",
///        "hour"
///      ],
///      "name": "Resolution"
///    },
///    "signalDetails": {
///      "description": "Array of signal filtering criteria. Optional.",
///      "type": "array",
///      "items": {
///        "description": "Signal filtering criteria with date range",
///        "type": "object",
///        "required": [
///          "signalId",
///          "startDate"
///        ],
///        "properties": {
///          "endDate": {
///            "description": "End date for signal filtering (ISO 8601 datetime). Optional.",
///            "anyOf": [
///              {
///                "type": "string",
///                "format": "date-time",
///                "pattern": "^(?:(?:\\d\\d[2468][048]|\\d\\d[13579][26]|\\d\\d0[48]|[02468][048]00|[13579][26]00)-02-29|\\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\\d|30)|(?:02)-(?:0[1-9]|1\\d|2[0-8])))T(?:(?:[01]\\d|2[0-3]):[0-5]\\d(?::[0-5]\\d(?:\\.\\d+)?)?(?:Z))$"
///              },
///              {
///                "type": "null"
///              }
///            ],
///            "name": "End Date"
///          },
///          "signalId": {
///            "description": "Unique identifier for the signal (UUID format)",
///            "type": "string",
///            "name": "Signal ID"
///          },
///          "startDate": {
///            "description": "Start date for signal filtering (ISO 8601 datetime)",
///            "type": "string",
///            "format": "date-time",
///            "pattern": "^(?:(?:\\d\\d[2468][048]|\\d\\d[13579][26]|\\d\\d0[48]|[02468][048]00|[13579][26]00)-02-29|\\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\\d|30)|(?:02)-(?:0[1-9]|1\\d|2[0-8])))T(?:(?:[01]\\d|2[0-3]):[0-5]\\d(?::[0-5]\\d(?:\\.\\d+)?)?(?:Z))$",
///            "name": "Start Date"
///          }
///        },
///        "name": "Signal Detail"
///      },
///      "name": "Signal Details"
///    },
///    "walletDetails": {
///      "description": "Array of wallet filtering criteria. Optional.",
///      "type": "array",
///      "items": {
///        "description": "Wallet filtering criteria with date range",
///        "type": "object",
///        "required": [
///          "startDate",
///          "walletAddress"
///        ],
///        "properties": {
///          "endDate": {
///            "description": "End date for wallet filtering (ISO 8601 datetime). Optional.",
///            "anyOf": [
///              {
///                "type": "string",
///                "format": "date-time",
///                "pattern": "^(?:(?:\\d\\d[2468][048]|\\d\\d[13579][26]|\\d\\d0[48]|[02468][048]00|[13579][26]00)-02-29|\\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\\d|30)|(?:02)-(?:0[1-9]|1\\d|2[0-8])))T(?:(?:[01]\\d|2[0-3]):[0-5]\\d(?::[0-5]\\d(?:\\.\\d+)?)?(?:Z))$"
///              },
///              {
///                "type": "null"
///              }
///            ],
///            "name": "End Date"
///          },
///          "startDate": {
///            "description": "Start date for wallet filtering (ISO 8601 datetime)",
///            "type": "string",
///            "format": "date-time",
///            "pattern": "^(?:(?:\\d\\d[2468][048]|\\d\\d[13579][26]|\\d\\d0[48]|[02468][048]00|[13579][26]00)-02-29|\\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\\d|30)|(?:02)-(?:0[1-9]|1\\d|2[0-8])))T(?:(?:[01]\\d|2[0-3]):[0-5]\\d(?::[0-5]\\d(?:\\.\\d+)?)?(?:Z))$",
///            "name": "Start Date"
///          },
///          "walletAddress": {
///            "description": "Wallet address to filter holdings by",
///            "type": "string",
///            "name": "Wallet Address"
///          }
///        },
///        "name": "Wallet Detail"
///      },
///      "name": "Wallet Details"
///    }
///  },
///  "name": "Get Holding History Request"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct WalletHistoryRequest {
    ///The chain ID to filter holdings by (e.g., "1" for Ethereum, "8453" for Base, "solana" for Solana). Optional.
    #[serde(rename = "chainId", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub chain_id: ::std::option::Option<::std::string::String>,
    ///Array of token contract addresses to exclude from results. Defaults to empty array.
    #[serde(
        rename = "excludeTokenContractAddresses",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub exclude_token_contract_addresses: ::std::vec::Vec<::std::string::String>,
    ///The time resolution for holding history data: "day" or "hour"
    pub resolution: WalletHistoryRequestResolution,
    ///Array of signal filtering criteria. Optional.
    #[serde(rename = "signalDetails", default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub signal_details: ::std::vec::Vec<WalletHistoryRequestSignalDetailsItem>,
    ///Array of wallet filtering criteria. Optional.
    #[serde(rename = "walletDetails", default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub wallet_details: ::std::vec::Vec<WalletHistoryRequestWalletDetailsItem>,
}
///The time resolution for holding history data: "day" or "hour"
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "The time resolution for holding history data: \"day\" or \"hour\"",
///  "type": "string",
///  "enum": [
///    "day",
///    "hour"
///  ],
///  "name": "Resolution"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WalletHistoryRequestResolution {
    #[serde(rename = "day")]
    Day,
    #[serde(rename = "hour")]
    Hour,
}
impl ::std::fmt::Display for WalletHistoryRequestResolution {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Day => f.write_str("day"),
            Self::Hour => f.write_str("hour"),
        }
    }
}
impl ::std::str::FromStr for WalletHistoryRequestResolution {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "day" => Ok(Self::Day),
            "hour" => Ok(Self::Hour),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for WalletHistoryRequestResolution {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for WalletHistoryRequestResolution {
    type Error = self::error::ConversionError;
    fn try_from(value: &::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for WalletHistoryRequestResolution {
    type Error = self::error::ConversionError;
    fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Signal filtering criteria with date range
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Signal filtering criteria with date range",
///  "type": "object",
///  "required": [
///    "signalId",
///    "startDate"
///  ],
///  "properties": {
///    "endDate": {
///      "description": "End date for signal filtering (ISO 8601 datetime). Optional.",
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
///      "name": "End Date"
///    },
///    "signalId": {
///      "description": "Unique identifier for the signal (UUID format)",
///      "type": "string",
///      "name": "Signal ID"
///    },
///    "startDate": {
///      "description": "Start date for signal filtering (ISO 8601 datetime)",
///      "type": "string",
///      "format": "date-time",
///      "pattern": "^(?:(?:\\d\\d[2468][048]|\\d\\d[13579][26]|\\d\\d0[48]|[02468][048]00|[13579][26]00)-02-29|\\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\\d|30)|(?:02)-(?:0[1-9]|1\\d|2[0-8])))T(?:(?:[01]\\d|2[0-3]):[0-5]\\d(?::[0-5]\\d(?:\\.\\d+)?)?(?:Z))$",
///      "name": "Start Date"
///    }
///  },
///  "name": "Signal Detail"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct WalletHistoryRequestSignalDetailsItem {
    ///End date for signal filtering (ISO 8601 datetime). Optional.
    #[serde(rename = "endDate", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub end_date: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    ///Unique identifier for the signal (UUID format)
    #[serde(rename = "signalId")]
    pub signal_id: ::std::string::String,
    ///Start date for signal filtering (ISO 8601 datetime)
    #[serde(rename = "startDate")]
    pub start_date: ::chrono::DateTime<::chrono::offset::Utc>,
}
///Wallet filtering criteria with date range
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Wallet filtering criteria with date range",
///  "type": "object",
///  "required": [
///    "startDate",
///    "walletAddress"
///  ],
///  "properties": {
///    "endDate": {
///      "description": "End date for wallet filtering (ISO 8601 datetime). Optional.",
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
///      "name": "End Date"
///    },
///    "startDate": {
///      "description": "Start date for wallet filtering (ISO 8601 datetime)",
///      "type": "string",
///      "format": "date-time",
///      "pattern": "^(?:(?:\\d\\d[2468][048]|\\d\\d[13579][26]|\\d\\d0[48]|[02468][048]00|[13579][26]00)-02-29|\\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\\d|30)|(?:02)-(?:0[1-9]|1\\d|2[0-8])))T(?:(?:[01]\\d|2[0-3]):[0-5]\\d(?::[0-5]\\d(?:\\.\\d+)?)?(?:Z))$",
///      "name": "Start Date"
///    },
///    "walletAddress": {
///      "description": "Wallet address to filter holdings by",
///      "type": "string",
///      "name": "Wallet Address"
///    }
///  },
///  "name": "Wallet Detail"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct WalletHistoryRequestWalletDetailsItem {
    ///End date for wallet filtering (ISO 8601 datetime). Optional.
    #[serde(rename = "endDate", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub end_date: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    ///Start date for wallet filtering (ISO 8601 datetime)
    #[serde(rename = "startDate")]
    pub start_date: ::chrono::DateTime<::chrono::offset::Utc>,
    ///Wallet address to filter holdings by
    #[serde(rename = "walletAddress")]
    pub wallet_address: ::std::string::String,
}
///`WalletHistoryResponseItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "balanceAtPeriodEnd",
///    "buysInPeriod",
///    "chainId",
///    "costBasisAtPeriodEnd",
///    "periodStart",
///    "realizedPnlInPeriod",
///    "sellsInPeriod",
///    "tokenAddress",
///    "tradeCycleId",
///    "walletAddress"
///  ],
///  "properties": {
///    "balanceAtPeriodEnd": {
///      "description": "Token balance at the end of the period.",
///      "type": "number",
///      "name": "Balance at Period End"
///    },
///    "buysInPeriod": {
///      "description": "Number of buy transactions in the period.",
///      "type": "integer",
///      "maximum": 9007199254740991.0,
///      "minimum": 0.0,
///      "name": "Buys in Period"
///    },
///    "chainId": {
///      "description": "The blockchain chain ID.",
///      "type": "string",
///      "name": "Chain ID"
///    },
///    "costBasisAtPeriodEnd": {
///      "description": "Cost basis at the end of the period.",
///      "type": "number",
///      "name": "Cost Basis at Period End"
///    },
///    "periodStart": {
///      "description": "The start timestamp of the period.",
///      "type": "string",
///      "name": "Period Start"
///    },
///    "realizedPnlInPeriod": {
///      "description": "Realized profit/loss during the period.",
///      "type": "number",
///      "name": "Realized PnL in Period"
///    },
///    "sellsInPeriod": {
///      "description": "Number of sell transactions in the period.",
///      "type": "integer",
///      "maximum": 9007199254740991.0,
///      "minimum": 0.0,
///      "name": "Sells in Period"
///    },
///    "tokenAddress": {
///      "description": "The token contract address.",
///      "type": "string",
///      "name": "Token Address"
///    },
///    "tradeCycleId": {
///      "description": "The trade cycle identifier.",
///      "type": "number",
///      "name": "Trade Cycle ID"
///    },
///    "walletAddress": {
///      "description": "The wallet address.",
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
pub struct WalletHistoryResponseItem {
    #[serde(rename = "balanceAtPeriodEnd")]
    pub balance_at_period_end: f64,
    ///Number of buy transactions in the period.
    #[serde(rename = "buysInPeriod")]
    pub buys_in_period: i64,
    ///The blockchain chain ID.
    #[serde(rename = "chainId")]
    pub chain_id: ::std::string::String,
    #[serde(rename = "costBasisAtPeriodEnd")]
    pub cost_basis_at_period_end: f64,
    ///The start timestamp of the period.
    #[serde(rename = "periodStart")]
    pub period_start: ::std::string::String,
    #[serde(rename = "realizedPnlInPeriod")]
    pub realized_pnl_in_period: f64,
    ///Number of sell transactions in the period.
    #[serde(rename = "sellsInPeriod")]
    pub sells_in_period: i64,
    ///The token contract address.
    #[serde(rename = "tokenAddress")]
    pub token_address: ::std::string::String,
    #[serde(rename = "tradeCycleId")]
    pub trade_cycle_id: f64,
    ///The wallet address.
    #[serde(rename = "walletAddress")]
    pub wallet_address: ::std::string::String,
}
