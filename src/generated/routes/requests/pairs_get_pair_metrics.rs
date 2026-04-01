#![allow(clippy::derivable_impls)]

use crate::client::{Route, RouteType};
use std::marker::PhantomData;
/// Route metadata for this procedure
pub const ROUTE: Route<PairMetricsRequest, PairMetricsResponse> = Route {
    procedure: "pairs.getPairMetrics",
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
///Request to get metrics data for a specific liquidity pair
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Request to get metrics data for a specific liquidity pair",
///  "type": "object",
///  "required": [
///    "interval",
///    "pairChainId",
///    "pairContractAddress"
///  ],
///  "properties": {
///    "interval": {
///      "description": "Time interval for metrics aggregation: 5m, 15m, 1h, 4h, or 24h",
///      "type": "string",
///      "enum": [
///        "5m",
///        "15m",
///        "1h",
///        "4h",
///        "24h"
///      ],
///      "name": "Interval"
///    },
///    "pairChainId": {
///      "description": "The chain ID where the pair exists (e.g., \"1\" for Ethereum, \"8453\" for Base, \"solana\" for Solana)",
///      "type": "string",
///      "name": "Pair Chain ID"
///    },
///    "pairContractAddress": {
///      "description": "The contract address of the liquidity pair to query metrics for",
///      "type": "string",
///      "name": "Pair Contract Address"
///    }
///  },
///  "name": "Get Pair Metrics Request"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct PairMetricsRequest {
    ///Time interval for metrics aggregation: 5m, 15m, 1h, 4h, or 24h
    pub interval: PairMetricsRequestInterval,
    ///The chain ID where the pair exists (e.g., "1" for Ethereum, "8453" for Base, "solana" for Solana)
    #[serde(rename = "pairChainId")]
    pub pair_chain_id: ::std::string::String,
    ///The contract address of the liquidity pair to query metrics for
    #[serde(rename = "pairContractAddress")]
    pub pair_contract_address: ::std::string::String,
}
///Time interval for metrics aggregation: 5m, 15m, 1h, 4h, or 24h
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Time interval for metrics aggregation: 5m, 15m, 1h, 4h, or 24h",
///  "type": "string",
///  "enum": [
///    "5m",
///    "15m",
///    "1h",
///    "4h",
///    "24h"
///  ],
///  "name": "Interval"
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
pub enum PairMetricsRequestInterval {
    #[serde(rename = "5m")]
    X5m,
    #[serde(rename = "15m")]
    X15m,
    #[serde(rename = "1h")]
    X1h,
    #[serde(rename = "4h")]
    X4h,
    #[serde(rename = "24h")]
    X24h,
}
impl ::std::fmt::Display for PairMetricsRequestInterval {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::X5m => f.write_str("5m"),
            Self::X15m => f.write_str("15m"),
            Self::X1h => f.write_str("1h"),
            Self::X4h => f.write_str("4h"),
            Self::X24h => f.write_str("24h"),
        }
    }
}
impl ::std::str::FromStr for PairMetricsRequestInterval {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "5m" => Ok(Self::X5m),
            "15m" => Ok(Self::X15m),
            "1h" => Ok(Self::X1h),
            "4h" => Ok(Self::X4h),
            "24h" => Ok(Self::X24h),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PairMetricsRequestInterval {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for PairMetricsRequestInterval {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PairMetricsRequestInterval {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`PairMetricsResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "15m",
///    "1h",
///    "24h",
///    "4h",
///    "5m"
///  ],
///  "properties": {
///    "15m": {
///      "type": "object",
///      "required": [
///        "buyTxns",
///        "buyVolumeNativeToken",
///        "buyVolumeUsd",
///        "buyers",
///        "endPriceUsd",
///        "priceChangePercent",
///        "sellTxns",
///        "sellVolumeNativeToken",
///        "sellVolumeUsd",
///        "sellers",
///        "startPriceUsd",
///        "totalMakers",
///        "totalTxns",
///        "volumeNativeToken",
///        "volumeUsd"
///      ],
///      "properties": {
///        "buyTxns": {
///          "type": "number"
///        },
///        "buyVolumeNativeToken": {
///          "type": "number"
///        },
///        "buyVolumeUsd": {
///          "type": "number"
///        },
///        "buyers": {
///          "type": "number"
///        },
///        "endPriceUsd": {
///          "type": "number"
///        },
///        "priceChangePercent": {
///          "type": "number"
///        },
///        "sellTxns": {
///          "type": "number"
///        },
///        "sellVolumeNativeToken": {
///          "type": "number"
///        },
///        "sellVolumeUsd": {
///          "type": "number"
///        },
///        "sellers": {
///          "type": "number"
///        },
///        "startPriceUsd": {
///          "type": "number"
///        },
///        "totalMakers": {
///          "type": "number"
///        },
///        "totalTxns": {
///          "type": "number"
///        },
///        "volumeNativeToken": {
///          "type": "number"
///        },
///        "volumeUsd": {
///          "type": "number"
///        }
///      },
///      "additionalProperties": false
///    },
///    "1h": {
///      "type": "object",
///      "required": [
///        "buyTxns",
///        "buyVolumeNativeToken",
///        "buyVolumeUsd",
///        "buyers",
///        "endPriceUsd",
///        "priceChangePercent",
///        "sellTxns",
///        "sellVolumeNativeToken",
///        "sellVolumeUsd",
///        "sellers",
///        "startPriceUsd",
///        "totalMakers",
///        "totalTxns",
///        "volumeNativeToken",
///        "volumeUsd"
///      ],
///      "properties": {
///        "buyTxns": {
///          "type": "number"
///        },
///        "buyVolumeNativeToken": {
///          "type": "number"
///        },
///        "buyVolumeUsd": {
///          "type": "number"
///        },
///        "buyers": {
///          "type": "number"
///        },
///        "endPriceUsd": {
///          "type": "number"
///        },
///        "priceChangePercent": {
///          "type": "number"
///        },
///        "sellTxns": {
///          "type": "number"
///        },
///        "sellVolumeNativeToken": {
///          "type": "number"
///        },
///        "sellVolumeUsd": {
///          "type": "number"
///        },
///        "sellers": {
///          "type": "number"
///        },
///        "startPriceUsd": {
///          "type": "number"
///        },
///        "totalMakers": {
///          "type": "number"
///        },
///        "totalTxns": {
///          "type": "number"
///        },
///        "volumeNativeToken": {
///          "type": "number"
///        },
///        "volumeUsd": {
///          "type": "number"
///        }
///      },
///      "additionalProperties": false
///    },
///    "24h": {
///      "type": "object",
///      "required": [
///        "buyTxns",
///        "buyVolumeNativeToken",
///        "buyVolumeUsd",
///        "buyers",
///        "endPriceUsd",
///        "priceChangePercent",
///        "sellTxns",
///        "sellVolumeNativeToken",
///        "sellVolumeUsd",
///        "sellers",
///        "startPriceUsd",
///        "totalMakers",
///        "totalTxns",
///        "volumeNativeToken",
///        "volumeUsd"
///      ],
///      "properties": {
///        "buyTxns": {
///          "type": "number"
///        },
///        "buyVolumeNativeToken": {
///          "type": "number"
///        },
///        "buyVolumeUsd": {
///          "type": "number"
///        },
///        "buyers": {
///          "type": "number"
///        },
///        "endPriceUsd": {
///          "type": "number"
///        },
///        "priceChangePercent": {
///          "type": "number"
///        },
///        "sellTxns": {
///          "type": "number"
///        },
///        "sellVolumeNativeToken": {
///          "type": "number"
///        },
///        "sellVolumeUsd": {
///          "type": "number"
///        },
///        "sellers": {
///          "type": "number"
///        },
///        "startPriceUsd": {
///          "type": "number"
///        },
///        "totalMakers": {
///          "type": "number"
///        },
///        "totalTxns": {
///          "type": "number"
///        },
///        "volumeNativeToken": {
///          "type": "number"
///        },
///        "volumeUsd": {
///          "type": "number"
///        }
///      },
///      "additionalProperties": false
///    },
///    "4h": {
///      "type": "object",
///      "required": [
///        "buyTxns",
///        "buyVolumeNativeToken",
///        "buyVolumeUsd",
///        "buyers",
///        "endPriceUsd",
///        "priceChangePercent",
///        "sellTxns",
///        "sellVolumeNativeToken",
///        "sellVolumeUsd",
///        "sellers",
///        "startPriceUsd",
///        "totalMakers",
///        "totalTxns",
///        "volumeNativeToken",
///        "volumeUsd"
///      ],
///      "properties": {
///        "buyTxns": {
///          "type": "number"
///        },
///        "buyVolumeNativeToken": {
///          "type": "number"
///        },
///        "buyVolumeUsd": {
///          "type": "number"
///        },
///        "buyers": {
///          "type": "number"
///        },
///        "endPriceUsd": {
///          "type": "number"
///        },
///        "priceChangePercent": {
///          "type": "number"
///        },
///        "sellTxns": {
///          "type": "number"
///        },
///        "sellVolumeNativeToken": {
///          "type": "number"
///        },
///        "sellVolumeUsd": {
///          "type": "number"
///        },
///        "sellers": {
///          "type": "number"
///        },
///        "startPriceUsd": {
///          "type": "number"
///        },
///        "totalMakers": {
///          "type": "number"
///        },
///        "totalTxns": {
///          "type": "number"
///        },
///        "volumeNativeToken": {
///          "type": "number"
///        },
///        "volumeUsd": {
///          "type": "number"
///        }
///      },
///      "additionalProperties": false
///    },
///    "5m": {
///      "type": "object",
///      "required": [
///        "buyTxns",
///        "buyVolumeNativeToken",
///        "buyVolumeUsd",
///        "buyers",
///        "endPriceUsd",
///        "priceChangePercent",
///        "sellTxns",
///        "sellVolumeNativeToken",
///        "sellVolumeUsd",
///        "sellers",
///        "startPriceUsd",
///        "totalMakers",
///        "totalTxns",
///        "volumeNativeToken",
///        "volumeUsd"
///      ],
///      "properties": {
///        "buyTxns": {
///          "type": "number"
///        },
///        "buyVolumeNativeToken": {
///          "type": "number"
///        },
///        "buyVolumeUsd": {
///          "type": "number"
///        },
///        "buyers": {
///          "type": "number"
///        },
///        "endPriceUsd": {
///          "type": "number"
///        },
///        "priceChangePercent": {
///          "type": "number"
///        },
///        "sellTxns": {
///          "type": "number"
///        },
///        "sellVolumeNativeToken": {
///          "type": "number"
///        },
///        "sellVolumeUsd": {
///          "type": "number"
///        },
///        "sellers": {
///          "type": "number"
///        },
///        "startPriceUsd": {
///          "type": "number"
///        },
///        "totalMakers": {
///          "type": "number"
///        },
///        "totalTxns": {
///          "type": "number"
///        },
///        "volumeNativeToken": {
///          "type": "number"
///        },
///        "volumeUsd": {
///          "type": "number"
///        }
///      },
///      "additionalProperties": false
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct PairMetricsResponse {
    #[serde(rename = "15m")]
    pub x15m: PairMetricsResponse15m,
    #[serde(rename = "1h")]
    pub x1h: PairMetricsResponse1h,
    #[serde(rename = "24h")]
    pub x24h: PairMetricsResponse24h,
    #[serde(rename = "4h")]
    pub x4h: PairMetricsResponse4h,
    #[serde(rename = "5m")]
    pub x5m: PairMetricsResponse5m,
}
///`PairMetricsResponse15m`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "buyTxns",
///    "buyVolumeNativeToken",
///    "buyVolumeUsd",
///    "buyers",
///    "endPriceUsd",
///    "priceChangePercent",
///    "sellTxns",
///    "sellVolumeNativeToken",
///    "sellVolumeUsd",
///    "sellers",
///    "startPriceUsd",
///    "totalMakers",
///    "totalTxns",
///    "volumeNativeToken",
///    "volumeUsd"
///  ],
///  "properties": {
///    "buyTxns": {
///      "type": "number"
///    },
///    "buyVolumeNativeToken": {
///      "type": "number"
///    },
///    "buyVolumeUsd": {
///      "type": "number"
///    },
///    "buyers": {
///      "type": "number"
///    },
///    "endPriceUsd": {
///      "type": "number"
///    },
///    "priceChangePercent": {
///      "type": "number"
///    },
///    "sellTxns": {
///      "type": "number"
///    },
///    "sellVolumeNativeToken": {
///      "type": "number"
///    },
///    "sellVolumeUsd": {
///      "type": "number"
///    },
///    "sellers": {
///      "type": "number"
///    },
///    "startPriceUsd": {
///      "type": "number"
///    },
///    "totalMakers": {
///      "type": "number"
///    },
///    "totalTxns": {
///      "type": "number"
///    },
///    "volumeNativeToken": {
///      "type": "number"
///    },
///    "volumeUsd": {
///      "type": "number"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct PairMetricsResponse15m {
    #[serde(rename = "buyTxns")]
    pub buy_txns: f64,
    #[serde(rename = "buyVolumeNativeToken")]
    pub buy_volume_native_token: f64,
    #[serde(rename = "buyVolumeUsd")]
    pub buy_volume_usd: f64,
    pub buyers: f64,
    #[serde(rename = "endPriceUsd")]
    pub end_price_usd: f64,
    #[serde(rename = "priceChangePercent")]
    pub price_change_percent: f64,
    #[serde(rename = "sellTxns")]
    pub sell_txns: f64,
    #[serde(rename = "sellVolumeNativeToken")]
    pub sell_volume_native_token: f64,
    #[serde(rename = "sellVolumeUsd")]
    pub sell_volume_usd: f64,
    pub sellers: f64,
    #[serde(rename = "startPriceUsd")]
    pub start_price_usd: f64,
    #[serde(rename = "totalMakers")]
    pub total_makers: f64,
    #[serde(rename = "totalTxns")]
    pub total_txns: f64,
    #[serde(rename = "volumeNativeToken")]
    pub volume_native_token: f64,
    #[serde(rename = "volumeUsd")]
    pub volume_usd: f64,
}
///`PairMetricsResponse1h`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "buyTxns",
///    "buyVolumeNativeToken",
///    "buyVolumeUsd",
///    "buyers",
///    "endPriceUsd",
///    "priceChangePercent",
///    "sellTxns",
///    "sellVolumeNativeToken",
///    "sellVolumeUsd",
///    "sellers",
///    "startPriceUsd",
///    "totalMakers",
///    "totalTxns",
///    "volumeNativeToken",
///    "volumeUsd"
///  ],
///  "properties": {
///    "buyTxns": {
///      "type": "number"
///    },
///    "buyVolumeNativeToken": {
///      "type": "number"
///    },
///    "buyVolumeUsd": {
///      "type": "number"
///    },
///    "buyers": {
///      "type": "number"
///    },
///    "endPriceUsd": {
///      "type": "number"
///    },
///    "priceChangePercent": {
///      "type": "number"
///    },
///    "sellTxns": {
///      "type": "number"
///    },
///    "sellVolumeNativeToken": {
///      "type": "number"
///    },
///    "sellVolumeUsd": {
///      "type": "number"
///    },
///    "sellers": {
///      "type": "number"
///    },
///    "startPriceUsd": {
///      "type": "number"
///    },
///    "totalMakers": {
///      "type": "number"
///    },
///    "totalTxns": {
///      "type": "number"
///    },
///    "volumeNativeToken": {
///      "type": "number"
///    },
///    "volumeUsd": {
///      "type": "number"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct PairMetricsResponse1h {
    #[serde(rename = "buyTxns")]
    pub buy_txns: f64,
    #[serde(rename = "buyVolumeNativeToken")]
    pub buy_volume_native_token: f64,
    #[serde(rename = "buyVolumeUsd")]
    pub buy_volume_usd: f64,
    pub buyers: f64,
    #[serde(rename = "endPriceUsd")]
    pub end_price_usd: f64,
    #[serde(rename = "priceChangePercent")]
    pub price_change_percent: f64,
    #[serde(rename = "sellTxns")]
    pub sell_txns: f64,
    #[serde(rename = "sellVolumeNativeToken")]
    pub sell_volume_native_token: f64,
    #[serde(rename = "sellVolumeUsd")]
    pub sell_volume_usd: f64,
    pub sellers: f64,
    #[serde(rename = "startPriceUsd")]
    pub start_price_usd: f64,
    #[serde(rename = "totalMakers")]
    pub total_makers: f64,
    #[serde(rename = "totalTxns")]
    pub total_txns: f64,
    #[serde(rename = "volumeNativeToken")]
    pub volume_native_token: f64,
    #[serde(rename = "volumeUsd")]
    pub volume_usd: f64,
}
///`PairMetricsResponse24h`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "buyTxns",
///    "buyVolumeNativeToken",
///    "buyVolumeUsd",
///    "buyers",
///    "endPriceUsd",
///    "priceChangePercent",
///    "sellTxns",
///    "sellVolumeNativeToken",
///    "sellVolumeUsd",
///    "sellers",
///    "startPriceUsd",
///    "totalMakers",
///    "totalTxns",
///    "volumeNativeToken",
///    "volumeUsd"
///  ],
///  "properties": {
///    "buyTxns": {
///      "type": "number"
///    },
///    "buyVolumeNativeToken": {
///      "type": "number"
///    },
///    "buyVolumeUsd": {
///      "type": "number"
///    },
///    "buyers": {
///      "type": "number"
///    },
///    "endPriceUsd": {
///      "type": "number"
///    },
///    "priceChangePercent": {
///      "type": "number"
///    },
///    "sellTxns": {
///      "type": "number"
///    },
///    "sellVolumeNativeToken": {
///      "type": "number"
///    },
///    "sellVolumeUsd": {
///      "type": "number"
///    },
///    "sellers": {
///      "type": "number"
///    },
///    "startPriceUsd": {
///      "type": "number"
///    },
///    "totalMakers": {
///      "type": "number"
///    },
///    "totalTxns": {
///      "type": "number"
///    },
///    "volumeNativeToken": {
///      "type": "number"
///    },
///    "volumeUsd": {
///      "type": "number"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct PairMetricsResponse24h {
    #[serde(rename = "buyTxns")]
    pub buy_txns: f64,
    #[serde(rename = "buyVolumeNativeToken")]
    pub buy_volume_native_token: f64,
    #[serde(rename = "buyVolumeUsd")]
    pub buy_volume_usd: f64,
    pub buyers: f64,
    #[serde(rename = "endPriceUsd")]
    pub end_price_usd: f64,
    #[serde(rename = "priceChangePercent")]
    pub price_change_percent: f64,
    #[serde(rename = "sellTxns")]
    pub sell_txns: f64,
    #[serde(rename = "sellVolumeNativeToken")]
    pub sell_volume_native_token: f64,
    #[serde(rename = "sellVolumeUsd")]
    pub sell_volume_usd: f64,
    pub sellers: f64,
    #[serde(rename = "startPriceUsd")]
    pub start_price_usd: f64,
    #[serde(rename = "totalMakers")]
    pub total_makers: f64,
    #[serde(rename = "totalTxns")]
    pub total_txns: f64,
    #[serde(rename = "volumeNativeToken")]
    pub volume_native_token: f64,
    #[serde(rename = "volumeUsd")]
    pub volume_usd: f64,
}
///`PairMetricsResponse4h`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "buyTxns",
///    "buyVolumeNativeToken",
///    "buyVolumeUsd",
///    "buyers",
///    "endPriceUsd",
///    "priceChangePercent",
///    "sellTxns",
///    "sellVolumeNativeToken",
///    "sellVolumeUsd",
///    "sellers",
///    "startPriceUsd",
///    "totalMakers",
///    "totalTxns",
///    "volumeNativeToken",
///    "volumeUsd"
///  ],
///  "properties": {
///    "buyTxns": {
///      "type": "number"
///    },
///    "buyVolumeNativeToken": {
///      "type": "number"
///    },
///    "buyVolumeUsd": {
///      "type": "number"
///    },
///    "buyers": {
///      "type": "number"
///    },
///    "endPriceUsd": {
///      "type": "number"
///    },
///    "priceChangePercent": {
///      "type": "number"
///    },
///    "sellTxns": {
///      "type": "number"
///    },
///    "sellVolumeNativeToken": {
///      "type": "number"
///    },
///    "sellVolumeUsd": {
///      "type": "number"
///    },
///    "sellers": {
///      "type": "number"
///    },
///    "startPriceUsd": {
///      "type": "number"
///    },
///    "totalMakers": {
///      "type": "number"
///    },
///    "totalTxns": {
///      "type": "number"
///    },
///    "volumeNativeToken": {
///      "type": "number"
///    },
///    "volumeUsd": {
///      "type": "number"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct PairMetricsResponse4h {
    #[serde(rename = "buyTxns")]
    pub buy_txns: f64,
    #[serde(rename = "buyVolumeNativeToken")]
    pub buy_volume_native_token: f64,
    #[serde(rename = "buyVolumeUsd")]
    pub buy_volume_usd: f64,
    pub buyers: f64,
    #[serde(rename = "endPriceUsd")]
    pub end_price_usd: f64,
    #[serde(rename = "priceChangePercent")]
    pub price_change_percent: f64,
    #[serde(rename = "sellTxns")]
    pub sell_txns: f64,
    #[serde(rename = "sellVolumeNativeToken")]
    pub sell_volume_native_token: f64,
    #[serde(rename = "sellVolumeUsd")]
    pub sell_volume_usd: f64,
    pub sellers: f64,
    #[serde(rename = "startPriceUsd")]
    pub start_price_usd: f64,
    #[serde(rename = "totalMakers")]
    pub total_makers: f64,
    #[serde(rename = "totalTxns")]
    pub total_txns: f64,
    #[serde(rename = "volumeNativeToken")]
    pub volume_native_token: f64,
    #[serde(rename = "volumeUsd")]
    pub volume_usd: f64,
}
///`PairMetricsResponse5m`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "buyTxns",
///    "buyVolumeNativeToken",
///    "buyVolumeUsd",
///    "buyers",
///    "endPriceUsd",
///    "priceChangePercent",
///    "sellTxns",
///    "sellVolumeNativeToken",
///    "sellVolumeUsd",
///    "sellers",
///    "startPriceUsd",
///    "totalMakers",
///    "totalTxns",
///    "volumeNativeToken",
///    "volumeUsd"
///  ],
///  "properties": {
///    "buyTxns": {
///      "type": "number"
///    },
///    "buyVolumeNativeToken": {
///      "type": "number"
///    },
///    "buyVolumeUsd": {
///      "type": "number"
///    },
///    "buyers": {
///      "type": "number"
///    },
///    "endPriceUsd": {
///      "type": "number"
///    },
///    "priceChangePercent": {
///      "type": "number"
///    },
///    "sellTxns": {
///      "type": "number"
///    },
///    "sellVolumeNativeToken": {
///      "type": "number"
///    },
///    "sellVolumeUsd": {
///      "type": "number"
///    },
///    "sellers": {
///      "type": "number"
///    },
///    "startPriceUsd": {
///      "type": "number"
///    },
///    "totalMakers": {
///      "type": "number"
///    },
///    "totalTxns": {
///      "type": "number"
///    },
///    "volumeNativeToken": {
///      "type": "number"
///    },
///    "volumeUsd": {
///      "type": "number"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct PairMetricsResponse5m {
    #[serde(rename = "buyTxns")]
    pub buy_txns: f64,
    #[serde(rename = "buyVolumeNativeToken")]
    pub buy_volume_native_token: f64,
    #[serde(rename = "buyVolumeUsd")]
    pub buy_volume_usd: f64,
    pub buyers: f64,
    #[serde(rename = "endPriceUsd")]
    pub end_price_usd: f64,
    #[serde(rename = "priceChangePercent")]
    pub price_change_percent: f64,
    #[serde(rename = "sellTxns")]
    pub sell_txns: f64,
    #[serde(rename = "sellVolumeNativeToken")]
    pub sell_volume_native_token: f64,
    #[serde(rename = "sellVolumeUsd")]
    pub sell_volume_usd: f64,
    pub sellers: f64,
    #[serde(rename = "startPriceUsd")]
    pub start_price_usd: f64,
    #[serde(rename = "totalMakers")]
    pub total_makers: f64,
    #[serde(rename = "totalTxns")]
    pub total_txns: f64,
    #[serde(rename = "volumeNativeToken")]
    pub volume_native_token: f64,
    #[serde(rename = "volumeUsd")]
    pub volume_usd: f64,
}
