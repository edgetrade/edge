#![allow(clippy::derivable_impls)]

use crate::client::{Route, RouteType};
use std::marker::PhantomData;
/// Route metadata for this procedure
pub const ROUTE: Route<OnWalletSwapsRequest, OnWalletSwapsResponse> = Route {
    procedure: "alerts.onWalletSwaps",
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
///`OnWalletSwapsRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "walletAddresses"
///  ],
///  "properties": {
///    "walletAddresses": {
///      "type": "array",
///      "items": {
///        "anyOf": [
///          {
///            "type": "string"
///          },
///          {
///            "anyOf": [
///              {},
///              {},
///              {
///                "type": "string"
///              }
///            ]
///          }
///        ]
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct OnWalletSwapsRequest {
    #[serde(rename = "walletAddresses")]
    pub wallet_addresses: ::std::vec::Vec<OnWalletSwapsRequestWalletAddressesItem>,
}
///`OnWalletSwapsRequestWalletAddressesItem`
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
pub struct OnWalletSwapsRequestWalletAddressesItem {
    #[serde(flatten, default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_0: ::std::option::Option<::std::string::String>,
    #[serde(flatten, default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_1: ::std::option::Option<OnWalletSwapsRequestWalletAddressesItemSubtype1>,
}
impl ::std::default::Default for OnWalletSwapsRequestWalletAddressesItem {
    fn default() -> Self {
        Self {
            subtype_0: Default::default(),
            subtype_1: Default::default(),
        }
    }
}
///`OnWalletSwapsRequestWalletAddressesItemSubtype1`
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
pub struct OnWalletSwapsRequestWalletAddressesItemSubtype1 {
    #[serde(flatten, default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_0: ::std::option::Option<::serde_json::Value>,
    #[serde(flatten, default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_1: ::std::option::Option<::serde_json::Value>,
    #[serde(flatten, default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_2: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for OnWalletSwapsRequestWalletAddressesItemSubtype1 {
    fn default() -> Self {
        Self {
            subtype_0: Default::default(),
            subtype_1: Default::default(),
            subtype_2: Default::default(),
        }
    }
}
///`OnWalletSwapsResponse`
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
pub struct OnWalletSwapsResponse {
    pub data: ::serde_json::Value,
    pub event: ::std::string::String,
}
