#![allow(clippy::derivable_impls)]

use crate::client::{Route, RouteType};
use std::marker::PhantomData;
/// Route metadata for this procedure
pub const ROUTE: Route<(), Vec<ListEntryStrategiesResponseItem>> = Route {
    procedure: "orders.listEntryStrategies",
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
///Response containing a list of entry strategies.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Response containing a list of entry strategies.",
///  "type": "object",
///  "required": [
///    "chainId",
///    "entryStrategyId",
///    "executionCount",
///    "lastExecutionDate",
///    "name",
///    "steps"
///  ],
///  "properties": {
///    "chainId": {
///      "description": "The blockchain chain ID.",
///      "type": "string",
///      "name": "Chain ID"
///    },
///    "entryStrategyId": {
///      "description": "The unique identifier of the entry strategy.",
///      "type": "number",
///      "name": "Entry Strategy ID"
///    },
///    "executionCount": {
///      "description": "The number of times this entry strategy has been executed.",
///      "type": "number",
///      "name": "Execution Count"
///    },
///    "lastExecutionDate": {
///      "description": "The timestamp of the last execution of this entry strategy, if any.",
///      "anyOf": [
///        {
///          "type": "string",
///          "format": "date-time",
///          "pattern": "^(?:(?:\\d\\d[2468][048]|\\d\\d[13579][26]|\\d\\d0[48]|[02468][048]00|[13579][26]00)-02-29|\\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\\d|30)|(?:02)-(?:0[1-9]|1\\d|2[0-8])))T(?:(?:[01]\\d|2[0-3]):[0-5]\\d(?::[0-5]\\d(?:\\.\\d+)?)?(?:Z))$"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Last Execution Date"
///    },
///    "name": {
///      "description": "The name of the entry strategy.",
///      "type": "string",
///      "name": "Name"
///    },
///    "steps": {
///      "description": "The steps in the entry strategy.",
///      "type": "array",
///      "items": {
///        "type": "object",
///        "required": [
///          "buyAmountNativeToken",
///          "entryStrategyStepId",
///          "percentToTrigger"
///        ],
///        "properties": {
///          "buyAmountNativeToken": {
///            "description": "The amount to buy in native token base units.",
///            "type": "string",
///            "name": "Buy Amount Native Token"
///          },
///          "entryStrategyStepId": {
///            "description": "The unique identifier of the entry strategy step.",
///            "type": "number",
///            "name": "Entry Strategy Step ID"
///          },
///          "percentToTrigger": {
///            "description": "The percentage price drop to trigger this step.",
///            "type": "number",
///            "name": "Percent to Trigger"
///          }
///        },
///        "additionalProperties": false
///      },
///      "name": "Steps"
///    }
///  },
///  "additionalProperties": false,
///  "name": "List Entry Strategies Response"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ListEntryStrategiesResponseItem {
    ///The blockchain chain ID.
    #[serde(rename = "chainId")]
    pub chain_id: ::std::string::String,
    #[serde(rename = "entryStrategyId")]
    pub entry_strategy_id: f64,
    #[serde(rename = "executionCount")]
    pub execution_count: f64,
    ///The timestamp of the last execution of this entry strategy, if any.
    #[serde(rename = "lastExecutionDate")]
    pub last_execution_date: ::std::option::Option<
        ::chrono::DateTime<::chrono::offset::Utc>,
    >,
    ///The name of the entry strategy.
    pub name: ::std::string::String,
    ///The steps in the entry strategy.
    pub steps: ::std::vec::Vec<ListEntryStrategiesResponseItemStepsItem>,
}
///`ListEntryStrategiesResponseItemStepsItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "buyAmountNativeToken",
///    "entryStrategyStepId",
///    "percentToTrigger"
///  ],
///  "properties": {
///    "buyAmountNativeToken": {
///      "description": "The amount to buy in native token base units.",
///      "type": "string",
///      "name": "Buy Amount Native Token"
///    },
///    "entryStrategyStepId": {
///      "description": "The unique identifier of the entry strategy step.",
///      "type": "number",
///      "name": "Entry Strategy Step ID"
///    },
///    "percentToTrigger": {
///      "description": "The percentage price drop to trigger this step.",
///      "type": "number",
///      "name": "Percent to Trigger"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ListEntryStrategiesResponseItemStepsItem {
    ///The amount to buy in native token base units.
    #[serde(rename = "buyAmountNativeToken")]
    pub buy_amount_native_token: ::std::string::String,
    #[serde(rename = "entryStrategyStepId")]
    pub entry_strategy_step_id: f64,
    #[serde(rename = "percentToTrigger")]
    pub percent_to_trigger: f64,
}
