#![allow(clippy::derivable_impls)]

use crate::client::{Route, RouteType};
use std::marker::PhantomData;
/// Route metadata for this procedure
pub const ROUTE: Route<CreateExitStrategyRequest, CreateExitStrategyResponse> = Route {
    procedure: "orders.createExitStrategy",
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
///Request to create a new exit strategy with steps
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Request to create a new exit strategy with steps",
///  "type": "object",
///  "required": [
///    "exit_strategy"
///  ],
///  "properties": {
///    "exit_strategy": {
///      "description": "The exit strategy configuration to create",
///      "type": "object",
///      "required": [
///        "name",
///        "steps"
///      ],
///      "properties": {
///        "name": {
///          "description": "The name of the exit strategy",
///          "type": "string",
///          "name": "Exit Strategy Name"
///        },
///        "steps": {
///          "description": "Ordered steps defining the exit strategy behavior",
///          "type": "array",
///          "items": {
///            "description": "Configuration for a single step in the exit strategy",
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
///                "description": "Percentage to sell when stop loss triggers",
///                "anyOf": [
///                  {
///                    "type": "number"
///                  },
///                  {
///                    "type": "null"
///                  }
///                ],
///                "name": "Stop Loss Percent To Sell"
///              },
///              "sl_percent_to_trigger": {
///                "description": "Percentage loss to trigger stop loss",
///                "anyOf": [
///                  {
///                    "type": "number"
///                  },
///                  {
///                    "type": "null"
///                  }
///                ],
///                "name": "Stop Loss Percent To Trigger"
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
///                "name": "Take Profit Percent Of Bag To Sell"
///              },
///              "tp_percent_of_cost_to_sell": {
///                "description": "Percentage of cost to sell at take profit",
///                "anyOf": [
///                  {
///                    "type": "number"
///                  },
///                  {
///                    "type": "null"
///                  }
///                ],
///                "name": "Take Profit Percent Of Cost To Sell"
///              },
///              "tp_percent_to_trigger": {
///                "description": "Percentage gain to trigger take profit",
///                "anyOf": [
///                  {
///                    "type": "number"
///                  },
///                  {
///                    "type": "null"
///                  }
///                ],
///                "name": "Take Profit Percent To Trigger"
///              }
///            },
///            "name": "Exit Strategy Step"
///          },
///          "name": "Exit Strategy Steps"
///        }
///      },
///      "name": "Exit Strategy"
///    }
///  },
///  "name": "Create Exit Strategy Request"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct CreateExitStrategyRequest {
    pub exit_strategy: CreateExitStrategyRequestExitStrategy,
}
///The exit strategy configuration to create
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "The exit strategy configuration to create",
///  "type": "object",
///  "required": [
///    "name",
///    "steps"
///  ],
///  "properties": {
///    "name": {
///      "description": "The name of the exit strategy",
///      "type": "string",
///      "name": "Exit Strategy Name"
///    },
///    "steps": {
///      "description": "Ordered steps defining the exit strategy behavior",
///      "type": "array",
///      "items": {
///        "description": "Configuration for a single step in the exit strategy",
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
///            "description": "Percentage to sell when stop loss triggers",
///            "anyOf": [
///              {
///                "type": "number"
///              },
///              {
///                "type": "null"
///              }
///            ],
///            "name": "Stop Loss Percent To Sell"
///          },
///          "sl_percent_to_trigger": {
///            "description": "Percentage loss to trigger stop loss",
///            "anyOf": [
///              {
///                "type": "number"
///              },
///              {
///                "type": "null"
///              }
///            ],
///            "name": "Stop Loss Percent To Trigger"
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
///            "name": "Take Profit Percent Of Bag To Sell"
///          },
///          "tp_percent_of_cost_to_sell": {
///            "description": "Percentage of cost to sell at take profit",
///            "anyOf": [
///              {
///                "type": "number"
///              },
///              {
///                "type": "null"
///              }
///            ],
///            "name": "Take Profit Percent Of Cost To Sell"
///          },
///          "tp_percent_to_trigger": {
///            "description": "Percentage gain to trigger take profit",
///            "anyOf": [
///              {
///                "type": "number"
///              },
///              {
///                "type": "null"
///              }
///            ],
///            "name": "Take Profit Percent To Trigger"
///          }
///        },
///        "name": "Exit Strategy Step"
///      },
///      "name": "Exit Strategy Steps"
///    }
///  },
///  "name": "Exit Strategy"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct CreateExitStrategyRequestExitStrategy {
    ///The name of the exit strategy
    pub name: ::std::string::String,
    ///Ordered steps defining the exit strategy behavior
    pub steps: ::std::vec::Vec<CreateExitStrategyRequestExitStrategyStepsItem>,
}
///Configuration for a single step in the exit strategy
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Configuration for a single step in the exit strategy",
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
///      "description": "Percentage to sell when stop loss triggers",
///      "anyOf": [
///        {
///          "type": "number"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Stop Loss Percent To Sell"
///    },
///    "sl_percent_to_trigger": {
///      "description": "Percentage loss to trigger stop loss",
///      "anyOf": [
///        {
///          "type": "number"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Stop Loss Percent To Trigger"
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
///      "name": "Take Profit Percent Of Bag To Sell"
///    },
///    "tp_percent_of_cost_to_sell": {
///      "description": "Percentage of cost to sell at take profit",
///      "anyOf": [
///        {
///          "type": "number"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Take Profit Percent Of Cost To Sell"
///    },
///    "tp_percent_to_trigger": {
///      "description": "Percentage gain to trigger take profit",
///      "anyOf": [
///        {
///          "type": "number"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Take Profit Percent To Trigger"
///    }
///  },
///  "name": "Exit Strategy Step"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct CreateExitStrategyRequestExitStrategyStepsItem {
    ///Percentage to sell when stop loss triggers
    pub sl_percent_to_sell: ::std::option::Option<f64>,
    ///Percentage loss to trigger stop loss
    pub sl_percent_to_trigger: ::std::option::Option<f64>,
    ///Percentage of bag to sell at take profit
    pub tp_percent_of_bag_to_sell: ::std::option::Option<f64>,
    ///Percentage of cost to sell at take profit
    pub tp_percent_of_cost_to_sell: ::std::option::Option<f64>,
    ///Percentage gain to trigger take profit
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
pub struct CreateExitStrategyResponse {
    ///The error message if the request was not successful
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub error: ::std::option::Option<::std::string::String>,
    ///Whether the request was successful
    pub success: bool,
}
