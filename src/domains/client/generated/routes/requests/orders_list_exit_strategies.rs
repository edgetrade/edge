#![allow(clippy::derivable_impls)]

use crate::client::{Route, RouteType};
use std::marker::PhantomData;
/// Route metadata for this procedure
pub const ROUTE: Route<(), Vec<ListExitStrategiesResponseItem>> = Route {
    procedure: "orders.listExitStrategies",
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
///Response containing a list of exit strategies.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Response containing a list of exit strategies.",
///  "type": "object",
///  "required": [
///    "executionCount",
///    "exitStrategyId",
///    "lastExecutionDate",
///    "name",
///    "steps"
///  ],
///  "properties": {
///    "executionCount": {
///      "description": "The number of times this exit strategy has been executed.",
///      "type": "number",
///      "name": "Execution Count"
///    },
///    "exitStrategyId": {
///      "description": "The unique identifier of the exit strategy.",
///      "type": "number",
///      "name": "Exit Strategy ID"
///    },
///    "lastExecutionDate": {
///      "description": "The timestamp of the last execution of this exit strategy, if any.",
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
///      "description": "The name of the exit strategy.",
///      "type": "string",
///      "name": "Name"
///    },
///    "steps": {
///      "description": "The steps in the exit strategy.",
///      "type": "array",
///      "items": {
///        "type": "object",
///        "required": [
///          "exitStrategyStepId",
///          "slPercentToSell",
///          "slPercentToTrigger",
///          "tpPercentOfBagToSell",
///          "tpPercentOfCostToSell",
///          "tpPercentToTrigger"
///        ],
///        "properties": {
///          "exitStrategyStepId": {
///            "description": "The unique identifier of the exit strategy step.",
///            "type": "number",
///            "name": "Exit Strategy Step ID"
///          },
///          "slPercentToSell": {
///            "description": "The percentage of position to sell when stop loss is triggered.",
///            "anyOf": [
///              {
///                "type": "number",
///                "maximum": 100.0,
///                "exclusiveMinimum": 0.0
///              },
///              {
///                "type": "null"
///              }
///            ],
///            "name": "Stop Loss Percent to Sell"
///          },
///          "slPercentToTrigger": {
///            "description": "The percentage below entry price to trigger the stop loss.",
///            "anyOf": [
///              {
///                "type": "number",
///                "maximum": 100.0,
///                "exclusiveMinimum": 0.0
///              },
///              {
///                "type": "null"
///              }
///            ],
///            "name": "Stop Loss Percent to Trigger"
///          },
///          "tpPercentOfBagToSell": {
///            "description": "The percentage of bag to sell when take profit is triggered.",
///            "anyOf": [
///              {
///                "type": "number",
///                "maximum": 100.0,
///                "exclusiveMinimum": 0.0
///              },
///              {
///                "type": "null"
///              }
///            ],
///            "name": "Take Profit Percent of Bag to Sell"
///          },
///          "tpPercentOfCostToSell": {
///            "description": "The percentage of cost basis to sell when take profit is triggered.",
///            "anyOf": [
///              {
///                "type": "number",
///                "maximum": 100.0,
///                "exclusiveMinimum": 0.0
///              },
///              {
///                "type": "null"
///              }
///            ],
///            "name": "Take Profit Percent of Cost to Sell"
///          },
///          "tpPercentToTrigger": {
///            "description": "The percentage above entry price to trigger the take profit.",
///            "anyOf": [
///              {
///                "type": "number",
///                "exclusiveMinimum": 0.0
///              },
///              {
///                "type": "null"
///              }
///            ],
///            "name": "Take Profit Percent to Trigger"
///          }
///        },
///        "additionalProperties": false
///      },
///      "name": "Steps"
///    }
///  },
///  "additionalProperties": false,
///  "name": "List Exit Strategies Response"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ListExitStrategiesResponseItem {
    #[serde(rename = "executionCount")]
    pub execution_count: f64,
    #[serde(rename = "exitStrategyId")]
    pub exit_strategy_id: f64,
    ///The timestamp of the last execution of this exit strategy, if any.
    #[serde(rename = "lastExecutionDate")]
    pub last_execution_date: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    ///The name of the exit strategy.
    pub name: ::std::string::String,
    ///The steps in the exit strategy.
    pub steps: ::std::vec::Vec<ListExitStrategiesResponseItemStepsItem>,
}
///`ListExitStrategiesResponseItemStepsItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "exitStrategyStepId",
///    "slPercentToSell",
///    "slPercentToTrigger",
///    "tpPercentOfBagToSell",
///    "tpPercentOfCostToSell",
///    "tpPercentToTrigger"
///  ],
///  "properties": {
///    "exitStrategyStepId": {
///      "description": "The unique identifier of the exit strategy step.",
///      "type": "number",
///      "name": "Exit Strategy Step ID"
///    },
///    "slPercentToSell": {
///      "description": "The percentage of position to sell when stop loss is triggered.",
///      "anyOf": [
///        {
///          "type": "number",
///          "maximum": 100.0,
///          "exclusiveMinimum": 0.0
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Stop Loss Percent to Sell"
///    },
///    "slPercentToTrigger": {
///      "description": "The percentage below entry price to trigger the stop loss.",
///      "anyOf": [
///        {
///          "type": "number",
///          "maximum": 100.0,
///          "exclusiveMinimum": 0.0
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Stop Loss Percent to Trigger"
///    },
///    "tpPercentOfBagToSell": {
///      "description": "The percentage of bag to sell when take profit is triggered.",
///      "anyOf": [
///        {
///          "type": "number",
///          "maximum": 100.0,
///          "exclusiveMinimum": 0.0
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Take Profit Percent of Bag to Sell"
///    },
///    "tpPercentOfCostToSell": {
///      "description": "The percentage of cost basis to sell when take profit is triggered.",
///      "anyOf": [
///        {
///          "type": "number",
///          "maximum": 100.0,
///          "exclusiveMinimum": 0.0
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Take Profit Percent of Cost to Sell"
///    },
///    "tpPercentToTrigger": {
///      "description": "The percentage above entry price to trigger the take profit.",
///      "anyOf": [
///        {
///          "type": "number",
///          "exclusiveMinimum": 0.0
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Take Profit Percent to Trigger"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ListExitStrategiesResponseItemStepsItem {
    #[serde(rename = "exitStrategyStepId")]
    pub exit_strategy_step_id: f64,
    ///The percentage of position to sell when stop loss is triggered.
    #[serde(rename = "slPercentToSell")]
    pub sl_percent_to_sell: ::std::option::Option<f64>,
    ///The percentage below entry price to trigger the stop loss.
    #[serde(rename = "slPercentToTrigger")]
    pub sl_percent_to_trigger: ::std::option::Option<f64>,
    ///The percentage of bag to sell when take profit is triggered.
    #[serde(rename = "tpPercentOfBagToSell")]
    pub tp_percent_of_bag_to_sell: ::std::option::Option<f64>,
    ///The percentage of cost basis to sell when take profit is triggered.
    #[serde(rename = "tpPercentOfCostToSell")]
    pub tp_percent_of_cost_to_sell: ::std::option::Option<f64>,
    ///The percentage above entry price to trigger the take profit.
    #[serde(rename = "tpPercentToTrigger")]
    pub tp_percent_to_trigger: ::std::option::Option<f64>,
}
