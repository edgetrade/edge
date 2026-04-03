#![allow(clippy::derivable_impls)]

use crate::client::{Route, RouteType};
use std::marker::PhantomData;
/// Route metadata for this procedure
pub const ROUTE: Route<PairOhlcvRequest, PairOhlcvResponse> = Route {
    procedure: "pairs.getPairCandles",
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
///Request to retrieve OHLC (Open, High, Low, Close) price bars for a specific liquidity pair based on swap events
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Request to retrieve OHLC (Open, High, Low, Close) price bars for a specific liquidity pair based on swap events",
///  "type": "object",
///  "required": [
///    "interval",
///    "pairChainId",
///    "pairContractAddress"
///  ],
///  "properties": {
///    "countBack": {
///      "description": "The maximum number of bars to return, counting back from the \"to\" timestamp",
///      "anyOf": [
///        {
///          "type": "number",
///          "maximum": 5000.0
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Count Back"
///    },
///    "from": {
///      "description": "Start time for the OHLC data range (unix timestamp ms) - not inclusive",
///      "anyOf": [
///        {
///          "type": "number"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "From Timestamp"
///    },
///    "interval": {
///      "description": "The time interval for each OHLC bar",
///      "type": "string",
///      "enum": [
///        "1sec",
///        "10sec",
///        "1min",
///        "5min",
///        "15min",
///        "1hr",
///        "4hr",
///        "6hr",
///        "1day"
///      ],
///      "name": "Interval"
///    },
///    "pairChainId": {
///      "description": "The chain ID where the pair exists (e.g., \"1\" for Ethereum, \"8453\" for Base, \"solana\" for Solana)",
///      "type": "string",
///      "name": "Pair Chain ID"
///    },
///    "pairContractAddress": {
///      "description": "The contract address of the liquidity pair to query OHLC data for",
///      "type": "string",
///      "name": "Pair Contract Address"
///    },
///    "previousPairAddress": {
///      "description": "Bonding curve that graduated to the current pair (optional)",
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Previous Pair Address"
///    },
///    "priceType": {
///      "description": "The price type of the bars to load (e.g., TOKEN_PRICE_USD, TOKEN_PRICE_NATIVE_TOKEN)",
///      "default": "TOKEN_PRICE_USD",
///      "type": "string",
///      "name": "Price Type"
///    },
///    "to": {
///      "description": "End time for the OHLC data range (unix timestamp ms) - not inclusive",
///      "anyOf": [
///        {
///          "type": "number"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "To Timestamp"
///    }
///  },
///  "name": "Get Pair Candles Request"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct PairOhlcvRequest {
    ///The maximum number of bars to return, counting back from the "to" timestamp
    #[serde(
        rename = "countBack",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub count_back: ::std::option::Option<f64>,
    ///Start time for the OHLC data range (unix timestamp ms) - not inclusive
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub from: ::std::option::Option<f64>,
    ///The time interval for each OHLC bar
    pub interval: PairOhlcvRequestInterval,
    ///The chain ID where the pair exists (e.g., "1" for Ethereum, "8453" for Base, "solana" for Solana)
    #[serde(rename = "pairChainId")]
    pub pair_chain_id: ::std::string::String,
    ///The contract address of the liquidity pair to query OHLC data for
    #[serde(rename = "pairContractAddress")]
    pub pair_contract_address: ::std::string::String,
    ///Bonding curve that graduated to the current pair (optional)
    #[serde(
        rename = "previousPairAddress",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub previous_pair_address: ::std::option::Option<::std::string::String>,
    ///The price type of the bars to load (e.g., TOKEN_PRICE_USD, TOKEN_PRICE_NATIVE_TOKEN)
    #[serde(rename = "priceType", default = "defaults::pair_ohlcv_request_price_type")]
    pub price_type: ::std::string::String,
    ///End time for the OHLC data range (unix timestamp ms) - not inclusive
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub to: ::std::option::Option<f64>,
}
///The time interval for each OHLC bar
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "The time interval for each OHLC bar",
///  "type": "string",
///  "enum": [
///    "1sec",
///    "10sec",
///    "1min",
///    "5min",
///    "15min",
///    "1hr",
///    "4hr",
///    "6hr",
///    "1day"
///  ],
///  "name": "Interval"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PairOhlcvRequestInterval {
    #[serde(rename = "1sec")]
    X1sec,
    #[serde(rename = "10sec")]
    X10sec,
    #[serde(rename = "1min")]
    X1min,
    #[serde(rename = "5min")]
    X5min,
    #[serde(rename = "15min")]
    X15min,
    #[serde(rename = "1hr")]
    X1hr,
    #[serde(rename = "4hr")]
    X4hr,
    #[serde(rename = "6hr")]
    X6hr,
    #[serde(rename = "1day")]
    X1day,
}
impl ::std::fmt::Display for PairOhlcvRequestInterval {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::X1sec => f.write_str("1sec"),
            Self::X10sec => f.write_str("10sec"),
            Self::X1min => f.write_str("1min"),
            Self::X5min => f.write_str("5min"),
            Self::X15min => f.write_str("15min"),
            Self::X1hr => f.write_str("1hr"),
            Self::X4hr => f.write_str("4hr"),
            Self::X6hr => f.write_str("6hr"),
            Self::X1day => f.write_str("1day"),
        }
    }
}
impl ::std::str::FromStr for PairOhlcvRequestInterval {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "1sec" => Ok(Self::X1sec),
            "10sec" => Ok(Self::X10sec),
            "1min" => Ok(Self::X1min),
            "5min" => Ok(Self::X5min),
            "15min" => Ok(Self::X15min),
            "1hr" => Ok(Self::X1hr),
            "4hr" => Ok(Self::X4hr),
            "6hr" => Ok(Self::X6hr),
            "1day" => Ok(Self::X1day),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PairOhlcvRequestInterval {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for PairOhlcvRequestInterval {
    type Error = self::error::ConversionError;
    fn try_from(value: &::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PairOhlcvRequestInterval {
    type Error = self::error::ConversionError;
    fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`PairOhlcvResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "bars"
///  ],
///  "properties": {
///    "bars": {
///      "type": "array",
///      "items": {
///        "description": "A single OHLC (Open, High, Low, Close) data point with volume and timestamp",
///        "type": "object",
///        "required": [
///          "c",
///          "h",
///          "l",
///          "o",
///          "t",
///          "v"
///        ],
///        "properties": {
///          "c": {
///            "description": "The closing price for the time interval",
///            "type": "number",
///            "name": "Close Price"
///          },
///          "h": {
///            "description": "The highest price during the time interval",
///            "type": "number",
///            "name": "High Price"
///          },
///          "l": {
///            "description": "The lowest price during the time interval",
///            "type": "number",
///            "name": "Low Price"
///          },
///          "o": {
///            "description": "The opening price for the time interval",
///            "type": "number",
///            "name": "Open Price"
///          },
///          "t": {
///            "description": "Timestamp of the OHLC bar in unix milliseconds",
///            "type": "number",
///            "name": "Timestamp"
///          },
///          "v": {
///            "description": "The trading volume during the time interval",
///            "type": "number",
///            "name": "Volume"
///          }
///        },
///        "additionalProperties": false,
///        "name": "OHLC Bar"
///      }
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct PairOhlcvResponse {
    pub bars: ::std::vec::Vec<PairOhlcvResponseBarsItem>,
}
///A single OHLC (Open, High, Low, Close) data point with volume and timestamp
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "A single OHLC (Open, High, Low, Close) data point with volume and timestamp",
///  "type": "object",
///  "required": [
///    "c",
///    "h",
///    "l",
///    "o",
///    "t",
///    "v"
///  ],
///  "properties": {
///    "c": {
///      "description": "The closing price for the time interval",
///      "type": "number",
///      "name": "Close Price"
///    },
///    "h": {
///      "description": "The highest price during the time interval",
///      "type": "number",
///      "name": "High Price"
///    },
///    "l": {
///      "description": "The lowest price during the time interval",
///      "type": "number",
///      "name": "Low Price"
///    },
///    "o": {
///      "description": "The opening price for the time interval",
///      "type": "number",
///      "name": "Open Price"
///    },
///    "t": {
///      "description": "Timestamp of the OHLC bar in unix milliseconds",
///      "type": "number",
///      "name": "Timestamp"
///    },
///    "v": {
///      "description": "The trading volume during the time interval",
///      "type": "number",
///      "name": "Volume"
///    }
///  },
///  "additionalProperties": false,
///  "name": "OHLC Bar"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct PairOhlcvResponseBarsItem {
    pub c: f64,
    pub h: f64,
    pub l: f64,
    pub o: f64,
    pub t: f64,
    pub v: f64,
}
/// Generation of default values for serde.
pub mod defaults {
    pub(super) fn pair_ohlcv_request_price_type() -> ::std::string::String {
        "TOKEN_PRICE_USD".to_string()
    }
}
