#![allow(clippy::derivable_impls)]

use crate::client::{Route, RouteType};
use std::marker::PhantomData;
/// Route metadata for this procedure
pub const ROUTE: Route<OnPortfolioUpdatesRequest, OnPortfolioUpdatesResponse> = Route {
    procedure: "alerts.onPortfolioUpdates",
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
///`OnPortfolioUpdatesRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "walletAddress"
///  ],
///  "properties": {
///    "chainId": {
///      "anyOf": [
///        {},
///        {
///          "type": "string",
///          "const": "solana"
///        }
///      ]
///    },
///    "walletAddress": {
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "anyOf": [
///            {},
///            {},
///            {
///              "type": "string"
///            }
///          ]
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct OnPortfolioUpdatesRequest {
    #[serde(rename = "chainId", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub chain_id: ::std::option::Option<OnPortfolioUpdatesRequestChainId>,
    #[serde(rename = "walletAddress")]
    pub wallet_address: OnPortfolioUpdatesRequestWalletAddress,
}
///`OnPortfolioUpdatesRequestChainId`
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
pub struct OnPortfolioUpdatesRequestChainId {
    #[serde(flatten, default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_0: ::std::option::Option<::serde_json::Value>,
    #[serde(flatten, default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_1: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for OnPortfolioUpdatesRequestChainId {
    fn default() -> Self {
        Self {
            subtype_0: Default::default(),
            subtype_1: Default::default(),
        }
    }
}
///`OnPortfolioUpdatesRequestWalletAddress`
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
pub struct OnPortfolioUpdatesRequestWalletAddress {
    #[serde(flatten, default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_0: ::std::option::Option<::std::string::String>,
    #[serde(flatten, default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_1: ::std::option::Option<OnPortfolioUpdatesRequestWalletAddressSubtype1>,
}
impl ::std::default::Default for OnPortfolioUpdatesRequestWalletAddress {
    fn default() -> Self {
        Self {
            subtype_0: Default::default(),
            subtype_1: Default::default(),
        }
    }
}
///`OnPortfolioUpdatesRequestWalletAddressSubtype1`
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
pub struct OnPortfolioUpdatesRequestWalletAddressSubtype1 {
    #[serde(flatten, default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_0: ::std::option::Option<::serde_json::Value>,
    #[serde(flatten, default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_1: ::std::option::Option<::serde_json::Value>,
    #[serde(flatten, default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_2: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for OnPortfolioUpdatesRequestWalletAddressSubtype1 {
    fn default() -> Self {
        Self {
            subtype_0: Default::default(),
            subtype_1: Default::default(),
            subtype_2: Default::default(),
        }
    }
}
///`OnPortfolioUpdatesResponse`
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
pub struct OnPortfolioUpdatesResponse {
    pub data: ::serde_json::Value,
    pub event: ::std::string::String,
}
