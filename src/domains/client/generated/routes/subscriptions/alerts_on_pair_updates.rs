#![allow(clippy::derivable_impls)]

use crate::client::{Route, RouteType};
use std::marker::PhantomData;
/// Route metadata for this procedure
pub const ROUTE: Route<(), OnPairUpdatesResponse> = Route {
    procedure: "alerts.onPairUpdates",
    route_type: RouteType::Subscription,
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
///`OnPairUpdatesRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "interval",
///        "pairChainId",
///        "pairContractAddress",
///        "type"
///      ],
///      "properties": {
///        "interval": {
///          "type": "string",
///          "enum": [
///            "5m",
///            "15m",
///            "1h",
///            "4h",
///            "24h"
///          ]
///        },
///        "pairChainId": {
///          "anyOf": [
///            {},
///            {
///              "type": "string",
///              "const": "solana"
///            }
///          ]
///        },
///        "pairContractAddress": {
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
///        "type": {
///          "type": "string",
///          "const": "metrics"
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "pairChainId",
///        "pairContractAddress",
///        "type"
///      ],
///      "properties": {
///        "pairChainId": {
///          "anyOf": [
///            {},
///            {
///              "type": "string",
///              "const": "solana"
///            }
///          ]
///        },
///        "pairContractAddress": {
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
///        "type": {
///          "type": "string",
///          "const": "state"
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum OnPairUpdatesRequest {
    #[serde(rename = "metrics")]
    Metrics {
        interval: OnPairUpdatesRequestInterval,
        #[serde(rename = "pairChainId")]
        pair_chain_id: OnPairUpdatesRequestPairChainId,
        #[serde(rename = "pairContractAddress")]
        pair_contract_address: OnPairUpdatesRequestPairContractAddress,
    },
    #[serde(rename = "state")]
    State {
        #[serde(rename = "pairChainId")]
        pair_chain_id: OnPairUpdatesRequestPairChainId,
        #[serde(rename = "pairContractAddress")]
        pair_contract_address: OnPairUpdatesRequestPairContractAddress,
    },
}
///`OnPairUpdatesRequestInterval`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "5m",
///    "15m",
///    "1h",
///    "4h",
///    "24h"
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum OnPairUpdatesRequestInterval {
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
impl ::std::fmt::Display for OnPairUpdatesRequestInterval {
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
impl ::std::str::FromStr for OnPairUpdatesRequestInterval {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
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
impl ::std::convert::TryFrom<&str> for OnPairUpdatesRequestInterval {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for OnPairUpdatesRequestInterval {
    type Error = self::error::ConversionError;
    fn try_from(value: &::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for OnPairUpdatesRequestInterval {
    type Error = self::error::ConversionError;
    fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`OnPairUpdatesRequestPairChainId`
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
pub struct OnPairUpdatesRequestPairChainId {
    #[serde(flatten, default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_0: ::std::option::Option<::serde_json::Value>,
    #[serde(flatten, default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_1: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for OnPairUpdatesRequestPairChainId {
    fn default() -> Self {
        Self {
            subtype_0: Default::default(),
            subtype_1: Default::default(),
        }
    }
}
///`OnPairUpdatesRequestPairContractAddress`
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
pub struct OnPairUpdatesRequestPairContractAddress {
    #[serde(flatten, default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_0: ::std::option::Option<::std::string::String>,
    #[serde(flatten, default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_1: ::std::option::Option<OnPairUpdatesRequestPairContractAddressSubtype1>,
}
impl ::std::default::Default for OnPairUpdatesRequestPairContractAddress {
    fn default() -> Self {
        Self {
            subtype_0: Default::default(),
            subtype_1: Default::default(),
        }
    }
}
///`OnPairUpdatesRequestPairContractAddressSubtype1`
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
pub struct OnPairUpdatesRequestPairContractAddressSubtype1 {
    #[serde(flatten, default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_0: ::std::option::Option<::serde_json::Value>,
    #[serde(flatten, default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_1: ::std::option::Option<::serde_json::Value>,
    #[serde(flatten, default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_2: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for OnPairUpdatesRequestPairContractAddressSubtype1 {
    fn default() -> Self {
        Self {
            subtype_0: Default::default(),
            subtype_1: Default::default(),
            subtype_2: Default::default(),
        }
    }
}
///`OnPairUpdatesResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "data",
///    "event"
///  ],
///  "properties": {
///    "data": {},
///    "event": {
///      "type": "string"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct OnPairUpdatesResponse {
    pub data: ::serde_json::Value,
    pub event: ::std::string::String,
}
