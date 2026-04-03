#![allow(clippy::derivable_impls)]

use crate::client::{Route, RouteType};
use std::marker::PhantomData;
/// Route metadata for this procedure
pub const ROUTE: Route<ChangeEncryptedWalletsRequest, ChangeEncryptedWalletsResponse> = Route {
    procedure: "agent.changeEncryptedWallets",
    route_type: RouteType::Mutation,
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
///Information about a wallet
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Information about a wallet",
///  "type": "object",
///  "properties": {
///    "address": {
///      "description": "The blockchain address of the wallet; optional if name is provided",
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Wallet Address"
///    },
///    "name": {
///      "description": "The human-readable name of the wallet; optional if address is provided",
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Wallet Name"
///    }
///  },
///  "name": "Wallet Info"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ChangeEncryptedWalletsRequest {
    ///The blockchain address of the wallet; optional if name is provided
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub address: ::std::option::Option<::std::string::String>,
    ///The human-readable name of the wallet; optional if address is provided
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for ChangeEncryptedWalletsRequest {
    fn default() -> Self {
        Self {
            address: Default::default(),
            name: Default::default(),
        }
    }
}
///Common output for all requests that are either successful or not
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Common output for all requests that are either successful or not",
///  "type": "object",
///  "required": [
///    "success"
///  ],
///  "properties": {
///    "error": {
///      "description": "The error message if the request was not successful",
///      "type": "string"
///    },
///    "success": {
///      "description": "Whether the request was successful",
///      "type": "boolean"
///    }
///  },
///  "additionalProperties": false,
///  "name": "Simple success or failure response"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ChangeEncryptedWalletsResponse {
    ///The error message if the request was not successful
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub error: ::std::option::Option<::std::string::String>,
    ///Whether the request was successful
    pub success: bool,
}
