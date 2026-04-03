#![allow(clippy::derivable_impls)]

use crate::client::{Route, RouteType};
use std::marker::PhantomData;
/// Route metadata for this procedure
pub const ROUTE: Route<(), OnTokenUpdatesResponse> = Route {
    procedure: "alerts.onTokenUpdates",
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
///`OnTokenUpdatesRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "chainId",
///        "tokenAddress",
///        "type"
///      ],
///      "properties": {
///        "chainId": {
///          "anyOf": [
///            {},
///            {
///              "type": "string",
///              "const": "solana"
///            }
///          ]
///        },
///        "tokenAddress": {
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
///          "const": "holders"
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "chainId",
///        "tokenAddress",
///        "type"
///      ],
///      "properties": {
///        "chainId": {
///          "anyOf": [
///            {},
///            {
///              "type": "string",
///              "const": "solana"
///            }
///          ]
///        },
///        "tokenAddress": {
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
///          "const": "updates"
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum OnTokenUpdatesRequest {
    #[serde(rename = "holders")]
    Holders {
        #[serde(rename = "chainId")]
        chain_id: OnTokenUpdatesRequestChainId,
        #[serde(rename = "tokenAddress")]
        token_address: OnTokenUpdatesRequestTokenAddress,
    },
    #[serde(rename = "updates")]
    Updates {
        #[serde(rename = "chainId")]
        chain_id: OnTokenUpdatesRequestChainId,
        #[serde(rename = "tokenAddress")]
        token_address: OnTokenUpdatesRequestTokenAddress,
    },
}
///`OnTokenUpdatesRequestChainId`
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
pub struct OnTokenUpdatesRequestChainId {
    #[serde(flatten, default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_0: ::std::option::Option<::serde_json::Value>,
    #[serde(flatten, default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_1: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for OnTokenUpdatesRequestChainId {
    fn default() -> Self {
        Self {
            subtype_0: Default::default(),
            subtype_1: Default::default(),
        }
    }
}
///`OnTokenUpdatesRequestTokenAddress`
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
pub struct OnTokenUpdatesRequestTokenAddress {
    #[serde(flatten, default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_0: ::std::option::Option<::std::string::String>,
    #[serde(flatten, default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_1: ::std::option::Option<OnTokenUpdatesRequestTokenAddressSubtype1>,
}
impl ::std::default::Default for OnTokenUpdatesRequestTokenAddress {
    fn default() -> Self {
        Self {
            subtype_0: Default::default(),
            subtype_1: Default::default(),
        }
    }
}
///`OnTokenUpdatesRequestTokenAddressSubtype1`
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
pub struct OnTokenUpdatesRequestTokenAddressSubtype1 {
    #[serde(flatten, default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_0: ::std::option::Option<::serde_json::Value>,
    #[serde(flatten, default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_1: ::std::option::Option<::serde_json::Value>,
    #[serde(flatten, default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_2: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for OnTokenUpdatesRequestTokenAddressSubtype1 {
    fn default() -> Self {
        Self {
            subtype_0: Default::default(),
            subtype_1: Default::default(),
            subtype_2: Default::default(),
        }
    }
}
///`OnTokenUpdatesResponse`
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
pub struct OnTokenUpdatesResponse {
    pub data: ::serde_json::Value,
    pub event: ::std::string::String,
}
