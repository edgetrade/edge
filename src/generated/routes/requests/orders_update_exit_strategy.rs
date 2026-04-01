#![allow(clippy::derivable_impls)]

use crate::client::{Route, RouteType};
use std::marker::PhantomData;
/// Route metadata for this procedure
pub const ROUTE: Route<UpdateExitStrategyRequest, UpdateExitStrategyResponse> = Route {
    procedure: "orders.updateExitStrategy",
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
///Request payload for updating an exit strategy
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Request payload for updating an exit strategy",
///  "type": "object",
///  "required": [
///    "exit_strategy"
///  ],
///  "properties": {
///    "exit_strategy": {
///      "description": "The exit strategy data to update",
///      "type": "object",
///      "required": [
///        "exit_strategy_id",
///        "name",
///        "steps"
///      ],
///      "properties": {
///        "exit_strategy_id": {
///          "description": "Unique identifier of the exit strategy",
///          "type": "number",
///          "name": "Exit Strategy ID"
///        },
///        "name": {
///          "description": "Human-readable name of the exit strategy",
///          "type": "string",
///          "name": "Strategy Name"
///        },
///        "steps": {
///          "description": "Ordered list of exit strategy steps",
///          "type": "array",
///          "items": {
///            "type": "object",
///            "required": [
///              "sl_percent_to_sell",
///              "sl_percent_to_trigger",
///              "tp_percent_of_bag_to_sell",
///              "tp_percent_of_cost_to_sell",
///              "tp_percent_to_trigger"
///            ],
///            "properties": {
///              "sl_percent_to_sell": {
///                "description": "Percentage of position to sell at stop loss (0-100)",
///                "anyOf": [
///                  {
///                    "type": "number"
///                  },
///                  {
///                    "type": "null"
///                  }
///                ],
///                "name": "Stop Loss Percent to Sell"
///              },
///              "sl_percent_to_trigger": {
///                "description": "Percentage below entry to trigger stop loss",
///                "anyOf": [
///                  {
///                    "type": "number"
///                  },
///                  {
///                    "type": "null"
///                  }
///                ],
///                "name": "Stop Loss Trigger Percent"
///              },
///              "tp_percent_of_bag_to_sell": {
///                "description": "Percentage of bag to sell at take profit",
///                "anyOf": [
///                  {
///                    "type": "number"
///                  },
///                  {
///                    "type": "null"
///                  }
///                ],
///                "name": "Take Profit Bag Percent to Sell"
///              },
///              "tp_percent_of_cost_to_sell": {
///                "description": "Percentage of cost basis to sell at take profit",
///                "anyOf": [
///                  {
///                    "type": "number"
///                  },
///                  {
///                    "type": "null"
///                  }
///                ],
///                "name": "Take Profit Cost Percent to Sell"
///              },
///              "tp_percent_to_trigger": {
///                "description": "Percentage above entry to trigger take profit",
///                "anyOf": [
///                  {
///                    "type": "number"
///                  },
///                  {
///                    "type": "null"
///                  }
///                ],
///                "name": "Take Profit Trigger Percent"
///              }
///            }
///          },
///          "name": "Strategy Steps"
///        }
///      },
///      "name": "Exit Strategy"
///    }
///  },
///  "name": "Update Exit Strategy Request"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct UpdateExitStrategyRequest {
    pub exit_strategy: UpdateExitStrategyRequestExitStrategy,
}
///The exit strategy data to update
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "The exit strategy data to update",
///  "type": "object",
///  "required": [
///    "exit_strategy_id",
///    "name",
///    "steps"
///  ],
///  "properties": {
///    "exit_strategy_id": {
///      "description": "Unique identifier of the exit strategy",
///      "type": "number",
///      "name": "Exit Strategy ID"
///    },
///    "name": {
///      "description": "Human-readable name of the exit strategy",
///      "type": "string",
///      "name": "Strategy Name"
///    },
///    "steps": {
///      "description": "Ordered list of exit strategy steps",
///      "type": "array",
///      "items": {
///        "type": "object",
///        "required": [
///          "sl_percent_to_sell",
///          "sl_percent_to_trigger",
///          "tp_percent_of_bag_to_sell",
///          "tp_percent_of_cost_to_sell",
///          "tp_percent_to_trigger"
///        ],
///        "properties": {
///          "sl_percent_to_sell": {
///            "description": "Percentage of position to sell at stop loss (0-100)",
///            "anyOf": [
///              {
///                "type": "number"
///              },
///              {
///                "type": "null"
///              }
///            ],
///            "name": "Stop Loss Percent to Sell"
///          },
///          "sl_percent_to_trigger": {
///            "description": "Percentage below entry to trigger stop loss",
///            "anyOf": [
///              {
///                "type": "number"
///              },
///              {
///                "type": "null"
///              }
///            ],
///            "name": "Stop Loss Trigger Percent"
///          },
///          "tp_percent_of_bag_to_sell": {
///            "description": "Percentage of bag to sell at take profit",
///            "anyOf": [
///              {
///                "type": "number"
///              },
///              {
///                "type": "null"
///              }
///            ],
///            "name": "Take Profit Bag Percent to Sell"
///          },
///          "tp_percent_of_cost_to_sell": {
///            "description": "Percentage of cost basis to sell at take profit",
///            "anyOf": [
///              {
///                "type": "number"
///              },
///              {
///                "type": "null"
///              }
///            ],
///            "name": "Take Profit Cost Percent to Sell"
///          },
///          "tp_percent_to_trigger": {
///            "description": "Percentage above entry to trigger take profit",
///            "anyOf": [
///              {
///                "type": "number"
///              },
///              {
///                "type": "null"
///              }
///            ],
///            "name": "Take Profit Trigger Percent"
///          }
///        }
///      },
///      "name": "Strategy Steps"
///    }
///  },
///  "name": "Exit Strategy"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct UpdateExitStrategyRequestExitStrategy {
    pub exit_strategy_id: f64,
    ///Human-readable name of the exit strategy
    pub name: ::std::string::String,
    ///Ordered list of exit strategy steps
    pub steps: ::std::vec::Vec<UpdateExitStrategyRequestExitStrategyStepsItem>,
}
///`UpdateExitStrategyRequestExitStrategyStepsItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "sl_percent_to_sell",
///    "sl_percent_to_trigger",
///    "tp_percent_of_bag_to_sell",
///    "tp_percent_of_cost_to_sell",
///    "tp_percent_to_trigger"
///  ],
///  "properties": {
///    "sl_percent_to_sell": {
///      "description": "Percentage of position to sell at stop loss (0-100)",
///      "anyOf": [
///        {
///          "type": "number"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Stop Loss Percent to Sell"
///    },
///    "sl_percent_to_trigger": {
///      "description": "Percentage below entry to trigger stop loss",
///      "anyOf": [
///        {
///          "type": "number"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Stop Loss Trigger Percent"
///    },
///    "tp_percent_of_bag_to_sell": {
///      "description": "Percentage of bag to sell at take profit",
///      "anyOf": [
///        {
///          "type": "number"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Take Profit Bag Percent to Sell"
///    },
///    "tp_percent_of_cost_to_sell": {
///      "description": "Percentage of cost basis to sell at take profit",
///      "anyOf": [
///        {
///          "type": "number"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Take Profit Cost Percent to Sell"
///    },
///    "tp_percent_to_trigger": {
///      "description": "Percentage above entry to trigger take profit",
///      "anyOf": [
///        {
///          "type": "number"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Take Profit Trigger Percent"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct UpdateExitStrategyRequestExitStrategyStepsItem {
    ///Percentage of position to sell at stop loss (0-100)
    pub sl_percent_to_sell: ::std::option::Option<f64>,
    ///Percentage below entry to trigger stop loss
    pub sl_percent_to_trigger: ::std::option::Option<f64>,
    ///Percentage of bag to sell at take profit
    pub tp_percent_of_bag_to_sell: ::std::option::Option<f64>,
    ///Percentage of cost basis to sell at take profit
    pub tp_percent_of_cost_to_sell: ::std::option::Option<f64>,
    ///Percentage above entry to trigger take profit
    pub tp_percent_to_trigger: ::std::option::Option<f64>,
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
pub struct UpdateExitStrategyResponse {
    ///The error message if the request was not successful
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub error: ::std::option::Option<::std::string::String>,
    ///Whether the request was successful
    pub success: bool,
}
