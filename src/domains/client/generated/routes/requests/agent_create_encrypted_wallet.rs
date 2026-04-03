#![allow(clippy::derivable_impls)]

use crate::client::{Route, RouteType};
use std::marker::PhantomData;
/// Route metadata for this procedure
pub const ROUTE: Route<CreateEncryptedWalletRequest, CreateEncryptedWalletResponse> = Route {
    procedure: "agent.createEncryptedWallet",
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
///Input for creating a new encrypted wallet for an agent
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Input for creating a new encrypted wallet for an agent",
///  "type": "object",
///  "required": [
///    "address",
///    "blob",
///    "envelope",
///    "name"
///  ],
///  "properties": {
///    "address": {
///      "description": "The address of the wallet",
///      "type": "string"
///    },
///    "blob": {
///      "description": "The encrypted blob comprising the wallet material",
///      "type": "string",
///      "format": "base64",
///      "pattern": "^$|^(?:[0-9a-zA-Z+/]{4})*(?:(?:[0-9a-zA-Z+/]{2}==)|(?:[0-9a-zA-Z+/]{3}=))?$",
///      "contentEncoding": "base64"
///    },
///    "envelope": {
///      "description": "The encrypted envelope for the edge vault containing the user's encryption key",
///      "type": "string",
///      "format": "base64",
///      "pattern": "^$|^(?:[0-9a-zA-Z+/]{4})*(?:(?:[0-9a-zA-Z+/]{2}==)|(?:[0-9a-zA-Z+/]{3}=))?$",
///      "contentEncoding": "base64"
///    },
///    "name": {
///      "description": "The name of the wallet",
///      "type": "string"
///    }
///  },
///  "name": "Create Encrypted Wallet"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct CreateEncryptedWalletRequest {
    ///The address of the wallet
    pub address: ::std::string::String,
    ///The encrypted blob comprising the wallet material
    pub blob: ::std::string::String,
    ///The encrypted envelope for the edge vault containing the user's encryption key
    pub envelope: ::std::string::String,
    ///The name of the wallet
    pub name: ::std::string::String,
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
pub struct CreateEncryptedWalletResponse {
    ///The error message if the request was not successful
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub error: ::std::option::Option<::std::string::String>,
    ///Whether the request was successful
    pub success: bool,
}
