#![allow(clippy::derivable_impls)]

use crate::client::{Route, RouteType};
use std::marker::PhantomData;
/// Route metadata for this procedure
pub const ROUTE: Route<CreateEntryStrategyRequest, CreateEntryStrategyResponse> = Route {
    procedure: "orders.createEntryStrategy",
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
///Request to create a new entry strategy
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Request to create a new entry strategy",
///  "type": "object",
///  "required": [
///    "entry_strategy"
///  ],
///  "properties": {
///    "entry_strategy": {
///      "description": "The entry strategy configuration to create",
///      "type": "object",
///      "required": [
///        "chain_id",
///        "name",
///        "steps"
///      ],
///      "properties": {
///        "chain_id": {
///          "description": "The blockchain chain ID where the strategy will be executed",
///          "type": "number",
///          "name": "Chain ID"
///        },
///        "name": {
///          "description": "The name of the entry strategy",
///          "type": "string",
///          "name": "Strategy Name"
///        },
///        "steps": {
///          "description": "The ordered steps that make up the entry strategy",
///          "type": "array",
///          "items": {
///            "description": "A single step in the entry strategy",
///            "type": "object",
///            "required": [
///              "buy_amount_native_token",
///              "percent_to_trigger"
///            ],
///            "properties": {
///              "buy_amount_native_token": {
///                "description": "The amount to buy in native token base units (as string)",
///                "type": "string",
///                "name": "Buy Amount Native Token"
///              },
///              "percent_to_trigger": {
///                "description": "The percentage threshold to trigger the step (0-99)",
///                "type": "number",
///                "name": "Percent to Trigger"
///              }
///            },
///            "name": "Entry Strategy Step"
///          },
///          "name": "Strategy Steps"
///        }
///      },
///      "name": "Entry Strategy"
///    }
///  },
///  "name": "Create Entry Strategy Request"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct CreateEntryStrategyRequest {
    pub entry_strategy: CreateEntryStrategyRequestEntryStrategy,
}
///The entry strategy configuration to create
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "The entry strategy configuration to create",
///  "type": "object",
///  "required": [
///    "chain_id",
///    "name",
///    "steps"
///  ],
///  "properties": {
///    "chain_id": {
///      "description": "The blockchain chain ID where the strategy will be executed",
///      "type": "number",
///      "name": "Chain ID"
///    },
///    "name": {
///      "description": "The name of the entry strategy",
///      "type": "string",
///      "name": "Strategy Name"
///    },
///    "steps": {
///      "description": "The ordered steps that make up the entry strategy",
///      "type": "array",
///      "items": {
///        "description": "A single step in the entry strategy",
///        "type": "object",
///        "required": [
///          "buy_amount_native_token",
///          "percent_to_trigger"
///        ],
///        "properties": {
///          "buy_amount_native_token": {
///            "description": "The amount to buy in native token base units (as string)",
///            "type": "string",
///            "name": "Buy Amount Native Token"
///          },
///          "percent_to_trigger": {
///            "description": "The percentage threshold to trigger the step (0-99)",
///            "type": "number",
///            "name": "Percent to Trigger"
///          }
///        },
///        "name": "Entry Strategy Step"
///      },
///      "name": "Strategy Steps"
///    }
///  },
///  "name": "Entry Strategy"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct CreateEntryStrategyRequestEntryStrategy {
    pub chain_id: f64,
    ///The name of the entry strategy
    pub name: ::std::string::String,
    ///The ordered steps that make up the entry strategy
    pub steps: ::std::vec::Vec<CreateEntryStrategyRequestEntryStrategyStepsItem>,
}
///A single step in the entry strategy
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "A single step in the entry strategy",
///  "type": "object",
///  "required": [
///    "buy_amount_native_token",
///    "percent_to_trigger"
///  ],
///  "properties": {
///    "buy_amount_native_token": {
///      "description": "The amount to buy in native token base units (as string)",
///      "type": "string",
///      "name": "Buy Amount Native Token"
///    },
///    "percent_to_trigger": {
///      "description": "The percentage threshold to trigger the step (0-99)",
///      "type": "number",
///      "name": "Percent to Trigger"
///    }
///  },
///  "name": "Entry Strategy Step"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct CreateEntryStrategyRequestEntryStrategyStepsItem {
    ///The amount to buy in native token base units (as string)
    pub buy_amount_native_token: ::std::string::String,
    pub percent_to_trigger: f64,
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
pub struct CreateEntryStrategyResponse {
    ///The error message if the request was not successful
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub error: ::std::option::Option<::std::string::String>,
    ///Whether the request was successful
    pub success: bool,
}
