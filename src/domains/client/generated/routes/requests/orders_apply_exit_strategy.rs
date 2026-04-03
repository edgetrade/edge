#![allow(clippy::derivable_impls)]

use crate::client::{Route, RouteType};
use std::marker::PhantomData;
/// Route metadata for this procedure
pub const ROUTE: Route<ApplyExitStrategyRequest, ApplyExitStrategyResponse> = Route {
    procedure: "orders.applyExitStrategy",
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
///Wire format for apply exit strategy request
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Wire format for apply exit strategy request",
///  "type": "object",
///  "required": [
///    "exitStrategyId",
///    "tokenChainId",
///    "tokenContractAddress"
///  ],
///  "properties": {
///    "exitStrategyId": {
///      "description": "The ID of the exit strategy to apply",
///      "type": "number",
///      "name": "Exit Strategy ID"
///    },
///    "tokenChainId": {
///      "description": "The chain ID where the token exists (e.g., \"1\" for Ethereum, \"solana\" for Solana)",
///      "type": "string",
///      "name": "Token Chain ID"
///    },
///    "tokenContractAddress": {
///      "description": "The contract address of the token to apply the exit strategy to",
///      "type": "string",
///      "name": "Token Contract Address"
///    }
///  },
///  "name": "ApplyExitStrategyRequestWire"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ApplyExitStrategyRequest {
    #[serde(rename = "exitStrategyId")]
    pub exit_strategy_id: f64,
    ///The chain ID where the token exists (e.g., "1" for Ethereum, "solana" for Solana)
    #[serde(rename = "tokenChainId")]
    pub token_chain_id: ::std::string::String,
    ///The contract address of the token to apply the exit strategy to
    #[serde(rename = "tokenContractAddress")]
    pub token_contract_address: ::std::string::String,
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
pub struct ApplyExitStrategyResponse {
    ///The error message if the request was not successful
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub error: ::std::option::Option<::std::string::String>,
    ///Whether the request was successful
    pub success: bool,
}
