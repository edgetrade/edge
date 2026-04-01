#![allow(clippy::derivable_impls)]

use crate::client::{Route, RouteType};
use std::marker::PhantomData;
/// Route metadata for this procedure
pub const ROUTE: Route<UpdateEntryStrategyRequest, UpdateEntryStrategyResponse> = Route {
    procedure: "orders.updateEntryStrategy",
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
///Request to update an entry strategy
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Request to update an entry strategy",
///  "type": "object",
///  "required": [
///    "entry_strategy"
///  ],
///  "properties": {
///    "entry_strategy": {
///      "description": "The entry strategy data to update",
///      "type": "object",
///      "required": [
///        "entry_strategy_id",
///        "name",
///        "steps"
///      ],
///      "properties": {
///        "entry_strategy_id": {
///          "description": "The unique identifier of the entry strategy to update",
///          "type": "number",
///          "name": "Entry Strategy ID"
///        },
///        "name": {
///          "description": "The name of the entry strategy",
///          "type": "string",
///          "name": "Name"
///        },
///        "steps": {
///          "description": "The steps of the entry strategy",
///          "type": "array",
///          "items": {
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
///            }
///          },
///          "name": "Steps"
///        }
///      },
///      "name": "Entry Strategy"
///    }
///  },
///  "name": "Update Entry Strategy Request"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct UpdateEntryStrategyRequest {
    pub entry_strategy: UpdateEntryStrategyRequestEntryStrategy,
}
///The entry strategy data to update
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "The entry strategy data to update",
///  "type": "object",
///  "required": [
///    "entry_strategy_id",
///    "name",
///    "steps"
///  ],
///  "properties": {
///    "entry_strategy_id": {
///      "description": "The unique identifier of the entry strategy to update",
///      "type": "number",
///      "name": "Entry Strategy ID"
///    },
///    "name": {
///      "description": "The name of the entry strategy",
///      "type": "string",
///      "name": "Name"
///    },
///    "steps": {
///      "description": "The steps of the entry strategy",
///      "type": "array",
///      "items": {
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
///        }
///      },
///      "name": "Steps"
///    }
///  },
///  "name": "Entry Strategy"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct UpdateEntryStrategyRequestEntryStrategy {
    pub entry_strategy_id: f64,
    ///The name of the entry strategy
    pub name: ::std::string::String,
    ///The steps of the entry strategy
    pub steps: ::std::vec::Vec<UpdateEntryStrategyRequestEntryStrategyStepsItem>,
}
///`UpdateEntryStrategyRequestEntryStrategyStepsItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
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
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct UpdateEntryStrategyRequestEntryStrategyStepsItem {
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
pub struct UpdateEntryStrategyResponse {
    ///The error message if the request was not successful
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub error: ::std::option::Option<::std::string::String>,
    ///Whether the request was successful
    pub success: bool,
}
