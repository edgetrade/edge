#![allow(clippy::derivable_impls)]

use crate::client::{Route, RouteType};
use std::marker::PhantomData;
/// Route metadata for this procedure
pub const ROUTE: Route<NativeBalancesRequest, Vec<NativeBalancesResponseItem>> = Route {
    procedure: "wallet.getNativeBalances",
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
///Request to get native token balances for a list of wallet addresses
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Request to get native token balances for a list of wallet addresses",
///  "type": "object",
///  "required": [
///    "wallets"
///  ],
///  "properties": {
///    "mustRefresh": {
///      "description": "Force refresh balances from the blockchain instead of using cached values",
///      "anyOf": [
///        {
///          "type": "boolean"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Must Refresh"
///    },
///    "wallets": {
///      "description": "Array of wallet addresses to fetch native balances for",
///      "type": "array",
///      "items": {
///        "type": "string"
///      },
///      "name": "Wallet Addresses"
///    }
///  },
///  "name": "Get Native Balances Request"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct NativeBalancesRequest {
    ///Force refresh balances from the blockchain instead of using cached values
    #[serde(
        rename = "mustRefresh",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub must_refresh: ::std::option::Option<bool>,
    ///Array of wallet addresses to fetch native balances for
    pub wallets: ::std::vec::Vec<::std::string::String>,
}
///Response for getting native balances.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Response for getting native balances.",
///  "type": "object",
///  "required": [
///    "balances",
///    "wallet"
///  ],
///  "properties": {
///    "balances": {
///      "description": "The balances of the tokens.",
///      "type": "object",
///      "additionalProperties": {
///        "type": "object",
///        "additionalProperties": {
///          "type": "string"
///        },
///        "propertyNames": {
///          "type": "string"
///        }
///      },
///      "propertyNames": {
///        "type": "string"
///      },
///      "name": "Balances"
///    },
///    "wallet": {
///      "description": "The address of the wallet.",
///      "type": "string",
///      "name": "Wallet"
///    }
///  },
///  "additionalProperties": false,
///  "name": "Get Native Balances Response Item"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct NativeBalancesResponseItem {
    ///The balances of the tokens.
    pub balances: ::std::collections::HashMap<
        ::std::string::String,
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
    ///The address of the wallet.
    pub wallet: ::std::string::String,
}
