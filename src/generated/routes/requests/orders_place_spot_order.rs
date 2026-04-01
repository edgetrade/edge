#![allow(clippy::derivable_impls)]

use crate::client::{Route, RouteType};
use std::marker::PhantomData;
/// Route metadata for this procedure
pub const ROUTE: Route<PlaceSpotOrderRequest, Vec<PlaceSpotOrderResponseItem>> = Route {
    procedure: "orders.placeSpotOrder",
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
///`PlaceSpotOrderRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "envelope",
///    "order"
///  ],
///  "properties": {
///    "envelope": {
///      "description": "The encrypted envelope for the edge vault",
///      "type": "string",
///      "format": "base64",
///      "pattern": "^$|^(?:[0-9a-zA-Z+/]{4})*(?:(?:[0-9a-zA-Z+/]{2}==)|(?:[0-9a-zA-Z+/]{3}=))?$",
///      "contentEncoding": "base64",
///      "name": "Envelope"
///    },
///    "order": {
///      "description": "The order to place",
///      "type": "object",
///      "required": [
///        "amount",
///        "exitStrategyId",
///        "pairId",
///        "side",
///        "txPreset",
///        "wallets"
///      ],
///      "properties": {
///        "amount": {
///          "description": "The amount of the order; a discriminated union of native, token, and percentage amounts",
///          "oneOf": [
///            {
///              "description": "The amount of the order in native tokens; must be in base unit amount (eg, wei, lamports, etc.)",
///              "type": "object",
///              "required": [
///                "type",
///                "value"
///              ],
///              "properties": {
///                "type": {
///                  "type": "string",
///                  "const": "native"
///                },
///                "value": {
///                  "description": "The amount of the order in native tokens; stringified; must be in base unit amount (eg, wei, lamports, etc.)",
///                  "type": "string",
///                  "name": "Value"
///                }
///              },
///              "name": "Native Amount"
///            },
///            {
///              "description": "The amount of the order in tokens",
///              "type": "object",
///              "required": [
///                "type",
///                "value"
///              ],
///              "properties": {
///                "type": {
///                  "type": "string",
///                  "const": "token"
///                },
///                "value": {
///                  "description": "The amount of the order in tokens; stringified; must be in base unit amount (eg, like wei, lamports, etc. would be).\n\nExample: if you want to buy/sell 1000 tokens and the token has 6 decimals, you would pass \"1000000000\".",
///                  "type": "string",
///                  "name": "Value"
///                }
///              },
///              "name": "Token Amount"
///            },
///            {
///              "description": "The amount of the order as a percentage; must be between 0 and 100",
///              "type": "object",
///              "required": [
///                "type",
///                "value"
///              ],
///              "properties": {
///                "type": {
///                  "type": "string",
///                  "const": "percentage"
///                },
///                "value": {
///                  "description": "The amount of the order as a percentage; stringified; must be between 0 and 100. Only applies to sell orders.",
///                  "type": "string",
///                  "name": "Value"
///                }
///              },
///              "name": "Percentage Amount"
///            }
///          ],
///          "name": "Amount"
///        },
///        "exitStrategyId": {
///          "description": "The ID of the exit strategy to use for the order (optional)",
///          "anyOf": [
///            {
///              "type": "number"
///            },
///            {
///              "type": "null"
///            }
///          ],
///          "name": "Exit Strategy ID"
///        },
///        "pairId": {
///          "description": "The ID of the pair to trade the token on",
///          "type": "object",
///          "required": [
///            "chainType",
///            "pairChainId",
///            "pairContractAddress",
///            "pairType"
///          ],
///          "properties": {
///            "chainType": {
///              "description": "The chain type of the pair; must be \"EVM\" or \"SVM\"",
///              "type": "string",
///              "enum": [
///                "EVM",
///                "SVM"
///              ],
///              "name": "Chain Type"
///            },
///            "pairChainId": {
///              "description": "The chain ID of the pair; stringified",
///              "type": "string",
///              "name": "Pair Chain ID"
///            },
///            "pairContractAddress": {
///              "description": "The contract address of the pair",
///              "type": "string",
///              "name": "Pair Contract Address"
///            },
///            "pairType": {
///              "description": "The type of the pair must be one of the following: (params) => {\n    const ctx = initializeContext({ ...params, processors });\n    process(schema, ctx);\n    extractDefs(ctx, schema);\n    return finalize(ctx, schema);\n}, [object Object], union, (...checks) => {\n        return inst.clone(util.mergeDefs(def, {\n            checks: [\n                ...(def.checks ?? []),\n                ...checks.map((ch) => typeof ch === \"function\" ? { _zod: { check: ch, def: { check: \"custom\" }, onattach: [] } } : ch),\n            ],\n        }), {\n            parent: true,\n        });\n    }, (...checks) => {\n        return inst.clone(util.mergeDefs(def, {\n            checks: [\n                ...(def.checks ?? []),\n                ...checks.map((ch) => typeof ch === \"function\" ? { _zod: { check: ch, def: { check: \"custom\" }, onattach: [] } } : ch),\n            ],\n        }), {\n            parent: true,\n        });\n    }, (def, params) => core.clone(inst, def, params), () => inst, (reg, meta) => {\n        reg.add(inst, meta);\n        return inst;\n    }, (data, params) => parse.parse(inst, data, params, { callee: inst.parse }), (data, params) => parse.safeParse(inst, data, params), async (data, params) => parse.parseAsync(inst, data, params, { callee: inst.parseAsync }), async (data, params) => parse.safeParseAsync(inst, data, params), async (data, params) => parse.safeParseAsync(inst, data, params), (data, params) => parse.encode(inst, data, params), (data, params) => parse.decode(inst, data, params), async (data, params) => parse.encodeAsync(inst, data, params), async (data, params) => parse.decodeAsync(inst, data, params), (data, params) => parse.safeEncode(inst, data, params), (data, params) => parse.safeDecode(inst, data, params), async (data, params) => parse.safeEncodeAsync(inst, data, params), async (data, params) => parse.safeDecodeAsync(inst, data, params), (check, params) => inst.check(refine(check, params)), (refinement) => inst.check(superRefine(refinement)), (fn) => inst.check(checks.overwrite(fn)), () => optional(inst), () => exactOptional(inst), () => nullable(inst), () => optional(nullable(inst)), (params) => nonoptional(inst, params), () => array(inst), (arg) => union([inst, arg]), (arg) => intersection(inst, arg), (tx) => pipe(inst, transform(tx)), (def) => _default(inst, def), (def) => prefault(inst, def), (params) => _catch(inst, params), (target) => pipe(inst, target), () => readonly(inst), (description) => {\n        const cl = inst.clone();\n        core.globalRegistry.add(cl, { description });\n        return cl;\n    }, (...args) => {\n        if (args.length === 0) {\n            return core.globalRegistry.get(inst);\n        }\n        const cl = inst.clone();\n        core.globalRegistry.add(cl, args[0]);\n        return cl;\n    }, () => inst.safeParse(undefined).success, () => inst.safeParse(null).success, (fn) => fn(inst), [object Object],[object Object]",
///              "anyOf": [
///                {
///                  "type": "string",
///                  "enum": [
///                    "aerodrome_v2",
///                    "aerodrome_v3",
///                    "baseswap_v2",
///                    "dyorswap_v2",
///                    "equalizer_v2",
///                    "equalizer_v3",
///                    "lynex_v2",
///                    "lynex_v3",
///                    "monoswap_v2",
///                    "monoswap_v3",
///                    "pancakeswap_v2",
///                    "pancakeswap_v3",
///                    "quickswap_v2",
///                    "quickswap_v3",
///                    "spookyswap_v2",
///                    "spookyswap_v3",
///                    "sushiswap_v2",
///                    "sushiswap_v3",
///                    "thruster_v2_2",
///                    "thruster_v3",
///                    "traderjoe_v2_0",
///                    "traderjoe_v2_1",
///                    "uniswap_v2",
///                    "uniswap_v3",
///                    "uniswap_v4",
///                    "unknown_v2",
///                    "unknown_v3",
///                    "unknown_v4",
///                    "velodrome_v2",
///                    "velodrome_v3",
///                    "fourmeme_bc"
///                  ]
///                },
///                {
///                  "type": "string",
///                  "enum": [
///                    "meteora_damm_v1",
///                    "meteora_damm_v2",
///                    "meteora_dlmm",
///                    "orca_whirlpools",
///                    "pump_amm",
///                    "raydium_clmm",
///                    "raydium_cpmm",
///                    "raydium_v4",
///                    "pump_bc",
///                    "launchlab_bc",
///                    "meteora_bc",
///                    "moonit_bc"
///                  ]
///                }
///              ],
///              "name": "Pair Type"
///            }
///          },
///          "name": "Pair ID"
///        },
///        "side": {
///          "description": "The side of the order; must be \"buy\" or \"sell\"",
///          "type": "string",
///          "enum": [
///            "buy",
///            "sell"
///          ],
///          "name": "Side"
///        },
///        "txPreset": {
///          "type": "object",
///          "required": [
///            "bribe",
///            "key",
///            "maxBaseGas",
///            "method",
///            "priorityGas",
///            "slippage"
///          ],
///          "properties": {
///            "bribe": {
///              "description": "The bribe for the order",
///              "type": "string",
///              "name": "Bribe"
///            },
///            "key": {
///              "description": "The key for the transaction preset",
///              "type": "string",
///              "name": "Preset Key"
///            },
///            "maxBaseGas": {
///              "description": "The maximum base gas for the order",
///              "type": "string",
///              "name": "Max Base Gas"
///            },
///            "method": {
///              "description": "The method for the order; must be \"flashbot\" or \"normal\". Only applies to EVM chains.",
///              "type": "string",
///              "enum": [
///                "flashbot",
///                "normal"
///              ],
///              "name": "Method"
///            },
///            "priorityGas": {
///              "description": "The priority gas for the order",
///              "type": "string",
///              "name": "Priority Gas"
///            },
///            "slippage": {
///              "description": "The slippage for the order",
///              "type": "string",
///              "name": "Slippage"
///            }
///          }
///        },
///        "wallets": {
///          "description": "The wallets that will be used to trade the token; must be an array of 1 wallet for agents",
///          "type": "array",
///          "items": {
///            "type": "object",
///            "required": [
///              "address"
///            ],
///            "properties": {
///              "address": {
///                "type": "string"
///              }
///            }
///          },
///          "maxItems": 1,
///          "minItems": 1,
///          "name": "Wallets"
///        }
///      },
///      "name": "Order"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct PlaceSpotOrderRequest {
    ///The encrypted envelope for the edge vault
    pub envelope: ::std::string::String,
    pub order: PlaceSpotOrderRequestOrder,
}
///The order to place
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "The order to place",
///  "type": "object",
///  "required": [
///    "amount",
///    "exitStrategyId",
///    "pairId",
///    "side",
///    "txPreset",
///    "wallets"
///  ],
///  "properties": {
///    "amount": {
///      "description": "The amount of the order; a discriminated union of native, token, and percentage amounts",
///      "oneOf": [
///        {
///          "description": "The amount of the order in native tokens; must be in base unit amount (eg, wei, lamports, etc.)",
///          "type": "object",
///          "required": [
///            "type",
///            "value"
///          ],
///          "properties": {
///            "type": {
///              "type": "string",
///              "const": "native"
///            },
///            "value": {
///              "description": "The amount of the order in native tokens; stringified; must be in base unit amount (eg, wei, lamports, etc.)",
///              "type": "string",
///              "name": "Value"
///            }
///          },
///          "name": "Native Amount"
///        },
///        {
///          "description": "The amount of the order in tokens",
///          "type": "object",
///          "required": [
///            "type",
///            "value"
///          ],
///          "properties": {
///            "type": {
///              "type": "string",
///              "const": "token"
///            },
///            "value": {
///              "description": "The amount of the order in tokens; stringified; must be in base unit amount (eg, like wei, lamports, etc. would be).\n\nExample: if you want to buy/sell 1000 tokens and the token has 6 decimals, you would pass \"1000000000\".",
///              "type": "string",
///              "name": "Value"
///            }
///          },
///          "name": "Token Amount"
///        },
///        {
///          "description": "The amount of the order as a percentage; must be between 0 and 100",
///          "type": "object",
///          "required": [
///            "type",
///            "value"
///          ],
///          "properties": {
///            "type": {
///              "type": "string",
///              "const": "percentage"
///            },
///            "value": {
///              "description": "The amount of the order as a percentage; stringified; must be between 0 and 100. Only applies to sell orders.",
///              "type": "string",
///              "name": "Value"
///            }
///          },
///          "name": "Percentage Amount"
///        }
///      ],
///      "name": "Amount"
///    },
///    "exitStrategyId": {
///      "description": "The ID of the exit strategy to use for the order (optional)",
///      "anyOf": [
///        {
///          "type": "number"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Exit Strategy ID"
///    },
///    "pairId": {
///      "description": "The ID of the pair to trade the token on",
///      "type": "object",
///      "required": [
///        "chainType",
///        "pairChainId",
///        "pairContractAddress",
///        "pairType"
///      ],
///      "properties": {
///        "chainType": {
///          "description": "The chain type of the pair; must be \"EVM\" or \"SVM\"",
///          "type": "string",
///          "enum": [
///            "EVM",
///            "SVM"
///          ],
///          "name": "Chain Type"
///        },
///        "pairChainId": {
///          "description": "The chain ID of the pair; stringified",
///          "type": "string",
///          "name": "Pair Chain ID"
///        },
///        "pairContractAddress": {
///          "description": "The contract address of the pair",
///          "type": "string",
///          "name": "Pair Contract Address"
///        },
///        "pairType": {
///          "description": "The type of the pair must be one of the following: (params) => {\n    const ctx = initializeContext({ ...params, processors });\n    process(schema, ctx);\n    extractDefs(ctx, schema);\n    return finalize(ctx, schema);\n}, [object Object], union, (...checks) => {\n        return inst.clone(util.mergeDefs(def, {\n            checks: [\n                ...(def.checks ?? []),\n                ...checks.map((ch) => typeof ch === \"function\" ? { _zod: { check: ch, def: { check: \"custom\" }, onattach: [] } } : ch),\n            ],\n        }), {\n            parent: true,\n        });\n    }, (...checks) => {\n        return inst.clone(util.mergeDefs(def, {\n            checks: [\n                ...(def.checks ?? []),\n                ...checks.map((ch) => typeof ch === \"function\" ? { _zod: { check: ch, def: { check: \"custom\" }, onattach: [] } } : ch),\n            ],\n        }), {\n            parent: true,\n        });\n    }, (def, params) => core.clone(inst, def, params), () => inst, (reg, meta) => {\n        reg.add(inst, meta);\n        return inst;\n    }, (data, params) => parse.parse(inst, data, params, { callee: inst.parse }), (data, params) => parse.safeParse(inst, data, params), async (data, params) => parse.parseAsync(inst, data, params, { callee: inst.parseAsync }), async (data, params) => parse.safeParseAsync(inst, data, params), async (data, params) => parse.safeParseAsync(inst, data, params), (data, params) => parse.encode(inst, data, params), (data, params) => parse.decode(inst, data, params), async (data, params) => parse.encodeAsync(inst, data, params), async (data, params) => parse.decodeAsync(inst, data, params), (data, params) => parse.safeEncode(inst, data, params), (data, params) => parse.safeDecode(inst, data, params), async (data, params) => parse.safeEncodeAsync(inst, data, params), async (data, params) => parse.safeDecodeAsync(inst, data, params), (check, params) => inst.check(refine(check, params)), (refinement) => inst.check(superRefine(refinement)), (fn) => inst.check(checks.overwrite(fn)), () => optional(inst), () => exactOptional(inst), () => nullable(inst), () => optional(nullable(inst)), (params) => nonoptional(inst, params), () => array(inst), (arg) => union([inst, arg]), (arg) => intersection(inst, arg), (tx) => pipe(inst, transform(tx)), (def) => _default(inst, def), (def) => prefault(inst, def), (params) => _catch(inst, params), (target) => pipe(inst, target), () => readonly(inst), (description) => {\n        const cl = inst.clone();\n        core.globalRegistry.add(cl, { description });\n        return cl;\n    }, (...args) => {\n        if (args.length === 0) {\n            return core.globalRegistry.get(inst);\n        }\n        const cl = inst.clone();\n        core.globalRegistry.add(cl, args[0]);\n        return cl;\n    }, () => inst.safeParse(undefined).success, () => inst.safeParse(null).success, (fn) => fn(inst), [object Object],[object Object]",
///          "anyOf": [
///            {
///              "type": "string",
///              "enum": [
///                "aerodrome_v2",
///                "aerodrome_v3",
///                "baseswap_v2",
///                "dyorswap_v2",
///                "equalizer_v2",
///                "equalizer_v3",
///                "lynex_v2",
///                "lynex_v3",
///                "monoswap_v2",
///                "monoswap_v3",
///                "pancakeswap_v2",
///                "pancakeswap_v3",
///                "quickswap_v2",
///                "quickswap_v3",
///                "spookyswap_v2",
///                "spookyswap_v3",
///                "sushiswap_v2",
///                "sushiswap_v3",
///                "thruster_v2_2",
///                "thruster_v3",
///                "traderjoe_v2_0",
///                "traderjoe_v2_1",
///                "uniswap_v2",
///                "uniswap_v3",
///                "uniswap_v4",
///                "unknown_v2",
///                "unknown_v3",
///                "unknown_v4",
///                "velodrome_v2",
///                "velodrome_v3",
///                "fourmeme_bc"
///              ]
///            },
///            {
///              "type": "string",
///              "enum": [
///                "meteora_damm_v1",
///                "meteora_damm_v2",
///                "meteora_dlmm",
///                "orca_whirlpools",
///                "pump_amm",
///                "raydium_clmm",
///                "raydium_cpmm",
///                "raydium_v4",
///                "pump_bc",
///                "launchlab_bc",
///                "meteora_bc",
///                "moonit_bc"
///              ]
///            }
///          ],
///          "name": "Pair Type"
///        }
///      },
///      "name": "Pair ID"
///    },
///    "side": {
///      "description": "The side of the order; must be \"buy\" or \"sell\"",
///      "type": "string",
///      "enum": [
///        "buy",
///        "sell"
///      ],
///      "name": "Side"
///    },
///    "txPreset": {
///      "type": "object",
///      "required": [
///        "bribe",
///        "key",
///        "maxBaseGas",
///        "method",
///        "priorityGas",
///        "slippage"
///      ],
///      "properties": {
///        "bribe": {
///          "description": "The bribe for the order",
///          "type": "string",
///          "name": "Bribe"
///        },
///        "key": {
///          "description": "The key for the transaction preset",
///          "type": "string",
///          "name": "Preset Key"
///        },
///        "maxBaseGas": {
///          "description": "The maximum base gas for the order",
///          "type": "string",
///          "name": "Max Base Gas"
///        },
///        "method": {
///          "description": "The method for the order; must be \"flashbot\" or \"normal\". Only applies to EVM chains.",
///          "type": "string",
///          "enum": [
///            "flashbot",
///            "normal"
///          ],
///          "name": "Method"
///        },
///        "priorityGas": {
///          "description": "The priority gas for the order",
///          "type": "string",
///          "name": "Priority Gas"
///        },
///        "slippage": {
///          "description": "The slippage for the order",
///          "type": "string",
///          "name": "Slippage"
///        }
///      }
///    },
///    "wallets": {
///      "description": "The wallets that will be used to trade the token; must be an array of 1 wallet for agents",
///      "type": "array",
///      "items": {
///        "type": "object",
///        "required": [
///          "address"
///        ],
///        "properties": {
///          "address": {
///            "type": "string"
///          }
///        }
///      },
///      "maxItems": 1,
///      "minItems": 1,
///      "name": "Wallets"
///    }
///  },
///  "name": "Order"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct PlaceSpotOrderRequestOrder {
    ///The amount of the order; a discriminated union of native, token, and percentage amounts
    pub amount: PlaceSpotOrderRequestOrderAmount,
    ///The ID of the exit strategy to use for the order (optional)
    #[serde(rename = "exitStrategyId")]
    pub exit_strategy_id: ::std::option::Option<f64>,
    #[serde(rename = "pairId")]
    pub pair_id: PlaceSpotOrderRequestOrderPairId,
    ///The side of the order; must be "buy" or "sell"
    pub side: PlaceSpotOrderRequestOrderSide,
    #[serde(rename = "txPreset")]
    pub tx_preset: PlaceSpotOrderRequestOrderTxPreset,
    ///The wallets that will be used to trade the token; must be an array of 1 wallet for agents
    pub wallets: [PlaceSpotOrderRequestOrderWalletsItem; 1usize],
}
///The amount of the order; a discriminated union of native, token, and percentage amounts
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "The amount of the order; a discriminated union of native, token, and percentage amounts",
///  "oneOf": [
///    {
///      "description": "The amount of the order in native tokens; must be in base unit amount (eg, wei, lamports, etc.)",
///      "type": "object",
///      "required": [
///        "type",
///        "value"
///      ],
///      "properties": {
///        "type": {
///          "type": "string",
///          "const": "native"
///        },
///        "value": {
///          "description": "The amount of the order in native tokens; stringified; must be in base unit amount (eg, wei, lamports, etc.)",
///          "type": "string",
///          "name": "Value"
///        }
///      },
///      "name": "Native Amount"
///    },
///    {
///      "description": "The amount of the order in tokens",
///      "type": "object",
///      "required": [
///        "type",
///        "value"
///      ],
///      "properties": {
///        "type": {
///          "type": "string",
///          "const": "token"
///        },
///        "value": {
///          "description": "The amount of the order in tokens; stringified; must be in base unit amount (eg, like wei, lamports, etc. would be).\n\nExample: if you want to buy/sell 1000 tokens and the token has 6 decimals, you would pass \"1000000000\".",
///          "type": "string",
///          "name": "Value"
///        }
///      },
///      "name": "Token Amount"
///    },
///    {
///      "description": "The amount of the order as a percentage; must be between 0 and 100",
///      "type": "object",
///      "required": [
///        "type",
///        "value"
///      ],
///      "properties": {
///        "type": {
///          "type": "string",
///          "const": "percentage"
///        },
///        "value": {
///          "description": "The amount of the order as a percentage; stringified; must be between 0 and 100. Only applies to sell orders.",
///          "type": "string",
///          "name": "Value"
///        }
///      },
///      "name": "Percentage Amount"
///    }
///  ],
///  "name": "Amount"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(tag = "type", content = "value")]
pub enum PlaceSpotOrderRequestOrderAmount {
    ///The amount of the order in native tokens; must be in base unit amount (eg, wei, lamports, etc.)
    #[serde(rename = "native")]
    Native(::std::string::String),
    ///The amount of the order in tokens
    #[serde(rename = "token")]
    Token(::std::string::String),
    ///The amount of the order as a percentage; must be between 0 and 100
    #[serde(rename = "percentage")]
    Percentage(::std::string::String),
}
///The ID of the pair to trade the token on
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "The ID of the pair to trade the token on",
///  "type": "object",
///  "required": [
///    "chainType",
///    "pairChainId",
///    "pairContractAddress",
///    "pairType"
///  ],
///  "properties": {
///    "chainType": {
///      "description": "The chain type of the pair; must be \"EVM\" or \"SVM\"",
///      "type": "string",
///      "enum": [
///        "EVM",
///        "SVM"
///      ],
///      "name": "Chain Type"
///    },
///    "pairChainId": {
///      "description": "The chain ID of the pair; stringified",
///      "type": "string",
///      "name": "Pair Chain ID"
///    },
///    "pairContractAddress": {
///      "description": "The contract address of the pair",
///      "type": "string",
///      "name": "Pair Contract Address"
///    },
///    "pairType": {
///      "description": "The type of the pair must be one of the following: (params) => {\n    const ctx = initializeContext({ ...params, processors });\n    process(schema, ctx);\n    extractDefs(ctx, schema);\n    return finalize(ctx, schema);\n}, [object Object], union, (...checks) => {\n        return inst.clone(util.mergeDefs(def, {\n            checks: [\n                ...(def.checks ?? []),\n                ...checks.map((ch) => typeof ch === \"function\" ? { _zod: { check: ch, def: { check: \"custom\" }, onattach: [] } } : ch),\n            ],\n        }), {\n            parent: true,\n        });\n    }, (...checks) => {\n        return inst.clone(util.mergeDefs(def, {\n            checks: [\n                ...(def.checks ?? []),\n                ...checks.map((ch) => typeof ch === \"function\" ? { _zod: { check: ch, def: { check: \"custom\" }, onattach: [] } } : ch),\n            ],\n        }), {\n            parent: true,\n        });\n    }, (def, params) => core.clone(inst, def, params), () => inst, (reg, meta) => {\n        reg.add(inst, meta);\n        return inst;\n    }, (data, params) => parse.parse(inst, data, params, { callee: inst.parse }), (data, params) => parse.safeParse(inst, data, params), async (data, params) => parse.parseAsync(inst, data, params, { callee: inst.parseAsync }), async (data, params) => parse.safeParseAsync(inst, data, params), async (data, params) => parse.safeParseAsync(inst, data, params), (data, params) => parse.encode(inst, data, params), (data, params) => parse.decode(inst, data, params), async (data, params) => parse.encodeAsync(inst, data, params), async (data, params) => parse.decodeAsync(inst, data, params), (data, params) => parse.safeEncode(inst, data, params), (data, params) => parse.safeDecode(inst, data, params), async (data, params) => parse.safeEncodeAsync(inst, data, params), async (data, params) => parse.safeDecodeAsync(inst, data, params), (check, params) => inst.check(refine(check, params)), (refinement) => inst.check(superRefine(refinement)), (fn) => inst.check(checks.overwrite(fn)), () => optional(inst), () => exactOptional(inst), () => nullable(inst), () => optional(nullable(inst)), (params) => nonoptional(inst, params), () => array(inst), (arg) => union([inst, arg]), (arg) => intersection(inst, arg), (tx) => pipe(inst, transform(tx)), (def) => _default(inst, def), (def) => prefault(inst, def), (params) => _catch(inst, params), (target) => pipe(inst, target), () => readonly(inst), (description) => {\n        const cl = inst.clone();\n        core.globalRegistry.add(cl, { description });\n        return cl;\n    }, (...args) => {\n        if (args.length === 0) {\n            return core.globalRegistry.get(inst);\n        }\n        const cl = inst.clone();\n        core.globalRegistry.add(cl, args[0]);\n        return cl;\n    }, () => inst.safeParse(undefined).success, () => inst.safeParse(null).success, (fn) => fn(inst), [object Object],[object Object]",
///      "anyOf": [
///        {
///          "type": "string",
///          "enum": [
///            "aerodrome_v2",
///            "aerodrome_v3",
///            "baseswap_v2",
///            "dyorswap_v2",
///            "equalizer_v2",
///            "equalizer_v3",
///            "lynex_v2",
///            "lynex_v3",
///            "monoswap_v2",
///            "monoswap_v3",
///            "pancakeswap_v2",
///            "pancakeswap_v3",
///            "quickswap_v2",
///            "quickswap_v3",
///            "spookyswap_v2",
///            "spookyswap_v3",
///            "sushiswap_v2",
///            "sushiswap_v3",
///            "thruster_v2_2",
///            "thruster_v3",
///            "traderjoe_v2_0",
///            "traderjoe_v2_1",
///            "uniswap_v2",
///            "uniswap_v3",
///            "uniswap_v4",
///            "unknown_v2",
///            "unknown_v3",
///            "unknown_v4",
///            "velodrome_v2",
///            "velodrome_v3",
///            "fourmeme_bc"
///          ]
///        },
///        {
///          "type": "string",
///          "enum": [
///            "meteora_damm_v1",
///            "meteora_damm_v2",
///            "meteora_dlmm",
///            "orca_whirlpools",
///            "pump_amm",
///            "raydium_clmm",
///            "raydium_cpmm",
///            "raydium_v4",
///            "pump_bc",
///            "launchlab_bc",
///            "meteora_bc",
///            "moonit_bc"
///          ]
///        }
///      ],
///      "name": "Pair Type"
///    }
///  },
///  "name": "Pair ID"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct PlaceSpotOrderRequestOrderPairId {
    ///The chain type of the pair; must be "EVM" or "SVM"
    #[serde(rename = "chainType")]
    pub chain_type: PlaceSpotOrderRequestOrderPairIdChainType,
    ///The chain ID of the pair; stringified
    #[serde(rename = "pairChainId")]
    pub pair_chain_id: ::std::string::String,
    ///The contract address of the pair
    #[serde(rename = "pairContractAddress")]
    pub pair_contract_address: ::std::string::String,
    /**The type of the pair must be one of the following: (params) => {
    const ctx = initializeContext({ ...params, processors });
    process(schema, ctx);
    extractDefs(ctx, schema);
    return finalize(ctx, schema);
}, [object Object], union, (...checks) => {
        return inst.clone(util.mergeDefs(def, {
            checks: [
                ...(def.checks ?? []),
                ...checks.map((ch) => typeof ch === "function" ? { _zod: { check: ch, def: { check: "custom" }, onattach: [] } } : ch),
            ],
        }), {
            parent: true,
        });
    }, (...checks) => {
        return inst.clone(util.mergeDefs(def, {
            checks: [
                ...(def.checks ?? []),
                ...checks.map((ch) => typeof ch === "function" ? { _zod: { check: ch, def: { check: "custom" }, onattach: [] } } : ch),
            ],
        }), {
            parent: true,
        });
    }, (def, params) => core.clone(inst, def, params), () => inst, (reg, meta) => {
        reg.add(inst, meta);
        return inst;
    }, (data, params) => parse.parse(inst, data, params, { callee: inst.parse }), (data, params) => parse.safeParse(inst, data, params), async (data, params) => parse.parseAsync(inst, data, params, { callee: inst.parseAsync }), async (data, params) => parse.safeParseAsync(inst, data, params), async (data, params) => parse.safeParseAsync(inst, data, params), (data, params) => parse.encode(inst, data, params), (data, params) => parse.decode(inst, data, params), async (data, params) => parse.encodeAsync(inst, data, params), async (data, params) => parse.decodeAsync(inst, data, params), (data, params) => parse.safeEncode(inst, data, params), (data, params) => parse.safeDecode(inst, data, params), async (data, params) => parse.safeEncodeAsync(inst, data, params), async (data, params) => parse.safeDecodeAsync(inst, data, params), (check, params) => inst.check(refine(check, params)), (refinement) => inst.check(superRefine(refinement)), (fn) => inst.check(checks.overwrite(fn)), () => optional(inst), () => exactOptional(inst), () => nullable(inst), () => optional(nullable(inst)), (params) => nonoptional(inst, params), () => array(inst), (arg) => union([inst, arg]), (arg) => intersection(inst, arg), (tx) => pipe(inst, transform(tx)), (def) => _default(inst, def), (def) => prefault(inst, def), (params) => _catch(inst, params), (target) => pipe(inst, target), () => readonly(inst), (description) => {
        const cl = inst.clone();
        core.globalRegistry.add(cl, { description });
        return cl;
    }, (...args) => {
        if (args.length === 0) {
            return core.globalRegistry.get(inst);
        }
        const cl = inst.clone();
        core.globalRegistry.add(cl, args[0]);
        return cl;
    }, () => inst.safeParse(undefined).success, () => inst.safeParse(null).success, (fn) => fn(inst), [object Object],[object Object]*/
    #[serde(rename = "pairType")]
    pub pair_type: PlaceSpotOrderRequestOrderPairIdPairType,
}
///The chain type of the pair; must be "EVM" or "SVM"
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "The chain type of the pair; must be \"EVM\" or \"SVM\"",
///  "type": "string",
///  "enum": [
///    "EVM",
///    "SVM"
///  ],
///  "name": "Chain Type"
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum PlaceSpotOrderRequestOrderPairIdChainType {
    #[serde(rename = "EVM")]
    Evm,
    #[serde(rename = "SVM")]
    Svm,
}
impl ::std::fmt::Display for PlaceSpotOrderRequestOrderPairIdChainType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Evm => f.write_str("EVM"),
            Self::Svm => f.write_str("SVM"),
        }
    }
}
impl ::std::str::FromStr for PlaceSpotOrderRequestOrderPairIdChainType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "EVM" => Ok(Self::Evm),
            "SVM" => Ok(Self::Svm),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PlaceSpotOrderRequestOrderPairIdChainType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for PlaceSpotOrderRequestOrderPairIdChainType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for PlaceSpotOrderRequestOrderPairIdChainType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
/**The type of the pair must be one of the following: (params) => {
    const ctx = initializeContext({ ...params, processors });
    process(schema, ctx);
    extractDefs(ctx, schema);
    return finalize(ctx, schema);
}, [object Object], union, (...checks) => {
        return inst.clone(util.mergeDefs(def, {
            checks: [
                ...(def.checks ?? []),
                ...checks.map((ch) => typeof ch === "function" ? { _zod: { check: ch, def: { check: "custom" }, onattach: [] } } : ch),
            ],
        }), {
            parent: true,
        });
    }, (...checks) => {
        return inst.clone(util.mergeDefs(def, {
            checks: [
                ...(def.checks ?? []),
                ...checks.map((ch) => typeof ch === "function" ? { _zod: { check: ch, def: { check: "custom" }, onattach: [] } } : ch),
            ],
        }), {
            parent: true,
        });
    }, (def, params) => core.clone(inst, def, params), () => inst, (reg, meta) => {
        reg.add(inst, meta);
        return inst;
    }, (data, params) => parse.parse(inst, data, params, { callee: inst.parse }), (data, params) => parse.safeParse(inst, data, params), async (data, params) => parse.parseAsync(inst, data, params, { callee: inst.parseAsync }), async (data, params) => parse.safeParseAsync(inst, data, params), async (data, params) => parse.safeParseAsync(inst, data, params), (data, params) => parse.encode(inst, data, params), (data, params) => parse.decode(inst, data, params), async (data, params) => parse.encodeAsync(inst, data, params), async (data, params) => parse.decodeAsync(inst, data, params), (data, params) => parse.safeEncode(inst, data, params), (data, params) => parse.safeDecode(inst, data, params), async (data, params) => parse.safeEncodeAsync(inst, data, params), async (data, params) => parse.safeDecodeAsync(inst, data, params), (check, params) => inst.check(refine(check, params)), (refinement) => inst.check(superRefine(refinement)), (fn) => inst.check(checks.overwrite(fn)), () => optional(inst), () => exactOptional(inst), () => nullable(inst), () => optional(nullable(inst)), (params) => nonoptional(inst, params), () => array(inst), (arg) => union([inst, arg]), (arg) => intersection(inst, arg), (tx) => pipe(inst, transform(tx)), (def) => _default(inst, def), (def) => prefault(inst, def), (params) => _catch(inst, params), (target) => pipe(inst, target), () => readonly(inst), (description) => {
        const cl = inst.clone();
        core.globalRegistry.add(cl, { description });
        return cl;
    }, (...args) => {
        if (args.length === 0) {
            return core.globalRegistry.get(inst);
        }
        const cl = inst.clone();
        core.globalRegistry.add(cl, args[0]);
        return cl;
    }, () => inst.safeParse(undefined).success, () => inst.safeParse(null).success, (fn) => fn(inst), [object Object],[object Object]*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "The type of the pair must be one of the following: (params) => {\n    const ctx = initializeContext({ ...params, processors });\n    process(schema, ctx);\n    extractDefs(ctx, schema);\n    return finalize(ctx, schema);\n}, [object Object], union, (...checks) => {\n        return inst.clone(util.mergeDefs(def, {\n            checks: [\n                ...(def.checks ?? []),\n                ...checks.map((ch) => typeof ch === \"function\" ? { _zod: { check: ch, def: { check: \"custom\" }, onattach: [] } } : ch),\n            ],\n        }), {\n            parent: true,\n        });\n    }, (...checks) => {\n        return inst.clone(util.mergeDefs(def, {\n            checks: [\n                ...(def.checks ?? []),\n                ...checks.map((ch) => typeof ch === \"function\" ? { _zod: { check: ch, def: { check: \"custom\" }, onattach: [] } } : ch),\n            ],\n        }), {\n            parent: true,\n        });\n    }, (def, params) => core.clone(inst, def, params), () => inst, (reg, meta) => {\n        reg.add(inst, meta);\n        return inst;\n    }, (data, params) => parse.parse(inst, data, params, { callee: inst.parse }), (data, params) => parse.safeParse(inst, data, params), async (data, params) => parse.parseAsync(inst, data, params, { callee: inst.parseAsync }), async (data, params) => parse.safeParseAsync(inst, data, params), async (data, params) => parse.safeParseAsync(inst, data, params), (data, params) => parse.encode(inst, data, params), (data, params) => parse.decode(inst, data, params), async (data, params) => parse.encodeAsync(inst, data, params), async (data, params) => parse.decodeAsync(inst, data, params), (data, params) => parse.safeEncode(inst, data, params), (data, params) => parse.safeDecode(inst, data, params), async (data, params) => parse.safeEncodeAsync(inst, data, params), async (data, params) => parse.safeDecodeAsync(inst, data, params), (check, params) => inst.check(refine(check, params)), (refinement) => inst.check(superRefine(refinement)), (fn) => inst.check(checks.overwrite(fn)), () => optional(inst), () => exactOptional(inst), () => nullable(inst), () => optional(nullable(inst)), (params) => nonoptional(inst, params), () => array(inst), (arg) => union([inst, arg]), (arg) => intersection(inst, arg), (tx) => pipe(inst, transform(tx)), (def) => _default(inst, def), (def) => prefault(inst, def), (params) => _catch(inst, params), (target) => pipe(inst, target), () => readonly(inst), (description) => {\n        const cl = inst.clone();\n        core.globalRegistry.add(cl, { description });\n        return cl;\n    }, (...args) => {\n        if (args.length === 0) {\n            return core.globalRegistry.get(inst);\n        }\n        const cl = inst.clone();\n        core.globalRegistry.add(cl, args[0]);\n        return cl;\n    }, () => inst.safeParse(undefined).success, () => inst.safeParse(null).success, (fn) => fn(inst), [object Object],[object Object]",
///  "anyOf": [
///    {
///      "type": "string",
///      "enum": [
///        "aerodrome_v2",
///        "aerodrome_v3",
///        "baseswap_v2",
///        "dyorswap_v2",
///        "equalizer_v2",
///        "equalizer_v3",
///        "lynex_v2",
///        "lynex_v3",
///        "monoswap_v2",
///        "monoswap_v3",
///        "pancakeswap_v2",
///        "pancakeswap_v3",
///        "quickswap_v2",
///        "quickswap_v3",
///        "spookyswap_v2",
///        "spookyswap_v3",
///        "sushiswap_v2",
///        "sushiswap_v3",
///        "thruster_v2_2",
///        "thruster_v3",
///        "traderjoe_v2_0",
///        "traderjoe_v2_1",
///        "uniswap_v2",
///        "uniswap_v3",
///        "uniswap_v4",
///        "unknown_v2",
///        "unknown_v3",
///        "unknown_v4",
///        "velodrome_v2",
///        "velodrome_v3",
///        "fourmeme_bc"
///      ]
///    },
///    {
///      "type": "string",
///      "enum": [
///        "meteora_damm_v1",
///        "meteora_damm_v2",
///        "meteora_dlmm",
///        "orca_whirlpools",
///        "pump_amm",
///        "raydium_clmm",
///        "raydium_cpmm",
///        "raydium_v4",
///        "pump_bc",
///        "launchlab_bc",
///        "meteora_bc",
///        "moonit_bc"
///      ]
///    }
///  ],
///  "name": "Pair Type"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct PlaceSpotOrderRequestOrderPairIdPairType {
    #[serde(flatten, default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_0: ::std::option::Option<
        PlaceSpotOrderRequestOrderPairIdPairTypeSubtype0,
    >,
    #[serde(flatten, default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_1: ::std::option::Option<
        PlaceSpotOrderRequestOrderPairIdPairTypeSubtype1,
    >,
}
impl ::std::default::Default for PlaceSpotOrderRequestOrderPairIdPairType {
    fn default() -> Self {
        Self {
            subtype_0: Default::default(),
            subtype_1: Default::default(),
        }
    }
}
///`PlaceSpotOrderRequestOrderPairIdPairTypeSubtype0`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "aerodrome_v2",
///    "aerodrome_v3",
///    "baseswap_v2",
///    "dyorswap_v2",
///    "equalizer_v2",
///    "equalizer_v3",
///    "lynex_v2",
///    "lynex_v3",
///    "monoswap_v2",
///    "monoswap_v3",
///    "pancakeswap_v2",
///    "pancakeswap_v3",
///    "quickswap_v2",
///    "quickswap_v3",
///    "spookyswap_v2",
///    "spookyswap_v3",
///    "sushiswap_v2",
///    "sushiswap_v3",
///    "thruster_v2_2",
///    "thruster_v3",
///    "traderjoe_v2_0",
///    "traderjoe_v2_1",
///    "uniswap_v2",
///    "uniswap_v3",
///    "uniswap_v4",
///    "unknown_v2",
///    "unknown_v3",
///    "unknown_v4",
///    "velodrome_v2",
///    "velodrome_v3",
///    "fourmeme_bc"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum PlaceSpotOrderRequestOrderPairIdPairTypeSubtype0 {
    #[serde(rename = "aerodrome_v2")]
    AerodromeV2,
    #[serde(rename = "aerodrome_v3")]
    AerodromeV3,
    #[serde(rename = "baseswap_v2")]
    BaseswapV2,
    #[serde(rename = "dyorswap_v2")]
    DyorswapV2,
    #[serde(rename = "equalizer_v2")]
    EqualizerV2,
    #[serde(rename = "equalizer_v3")]
    EqualizerV3,
    #[serde(rename = "lynex_v2")]
    LynexV2,
    #[serde(rename = "lynex_v3")]
    LynexV3,
    #[serde(rename = "monoswap_v2")]
    MonoswapV2,
    #[serde(rename = "monoswap_v3")]
    MonoswapV3,
    #[serde(rename = "pancakeswap_v2")]
    PancakeswapV2,
    #[serde(rename = "pancakeswap_v3")]
    PancakeswapV3,
    #[serde(rename = "quickswap_v2")]
    QuickswapV2,
    #[serde(rename = "quickswap_v3")]
    QuickswapV3,
    #[serde(rename = "spookyswap_v2")]
    SpookyswapV2,
    #[serde(rename = "spookyswap_v3")]
    SpookyswapV3,
    #[serde(rename = "sushiswap_v2")]
    SushiswapV2,
    #[serde(rename = "sushiswap_v3")]
    SushiswapV3,
    #[serde(rename = "thruster_v2_2")]
    ThrusterV22,
    #[serde(rename = "thruster_v3")]
    ThrusterV3,
    #[serde(rename = "traderjoe_v2_0")]
    TraderjoeV20,
    #[serde(rename = "traderjoe_v2_1")]
    TraderjoeV21,
    #[serde(rename = "uniswap_v2")]
    UniswapV2,
    #[serde(rename = "uniswap_v3")]
    UniswapV3,
    #[serde(rename = "uniswap_v4")]
    UniswapV4,
    #[serde(rename = "unknown_v2")]
    UnknownV2,
    #[serde(rename = "unknown_v3")]
    UnknownV3,
    #[serde(rename = "unknown_v4")]
    UnknownV4,
    #[serde(rename = "velodrome_v2")]
    VelodromeV2,
    #[serde(rename = "velodrome_v3")]
    VelodromeV3,
    #[serde(rename = "fourmeme_bc")]
    FourmemeBc,
}
impl ::std::fmt::Display for PlaceSpotOrderRequestOrderPairIdPairTypeSubtype0 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::AerodromeV2 => f.write_str("aerodrome_v2"),
            Self::AerodromeV3 => f.write_str("aerodrome_v3"),
            Self::BaseswapV2 => f.write_str("baseswap_v2"),
            Self::DyorswapV2 => f.write_str("dyorswap_v2"),
            Self::EqualizerV2 => f.write_str("equalizer_v2"),
            Self::EqualizerV3 => f.write_str("equalizer_v3"),
            Self::LynexV2 => f.write_str("lynex_v2"),
            Self::LynexV3 => f.write_str("lynex_v3"),
            Self::MonoswapV2 => f.write_str("monoswap_v2"),
            Self::MonoswapV3 => f.write_str("monoswap_v3"),
            Self::PancakeswapV2 => f.write_str("pancakeswap_v2"),
            Self::PancakeswapV3 => f.write_str("pancakeswap_v3"),
            Self::QuickswapV2 => f.write_str("quickswap_v2"),
            Self::QuickswapV3 => f.write_str("quickswap_v3"),
            Self::SpookyswapV2 => f.write_str("spookyswap_v2"),
            Self::SpookyswapV3 => f.write_str("spookyswap_v3"),
            Self::SushiswapV2 => f.write_str("sushiswap_v2"),
            Self::SushiswapV3 => f.write_str("sushiswap_v3"),
            Self::ThrusterV22 => f.write_str("thruster_v2_2"),
            Self::ThrusterV3 => f.write_str("thruster_v3"),
            Self::TraderjoeV20 => f.write_str("traderjoe_v2_0"),
            Self::TraderjoeV21 => f.write_str("traderjoe_v2_1"),
            Self::UniswapV2 => f.write_str("uniswap_v2"),
            Self::UniswapV3 => f.write_str("uniswap_v3"),
            Self::UniswapV4 => f.write_str("uniswap_v4"),
            Self::UnknownV2 => f.write_str("unknown_v2"),
            Self::UnknownV3 => f.write_str("unknown_v3"),
            Self::UnknownV4 => f.write_str("unknown_v4"),
            Self::VelodromeV2 => f.write_str("velodrome_v2"),
            Self::VelodromeV3 => f.write_str("velodrome_v3"),
            Self::FourmemeBc => f.write_str("fourmeme_bc"),
        }
    }
}
impl ::std::str::FromStr for PlaceSpotOrderRequestOrderPairIdPairTypeSubtype0 {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "aerodrome_v2" => Ok(Self::AerodromeV2),
            "aerodrome_v3" => Ok(Self::AerodromeV3),
            "baseswap_v2" => Ok(Self::BaseswapV2),
            "dyorswap_v2" => Ok(Self::DyorswapV2),
            "equalizer_v2" => Ok(Self::EqualizerV2),
            "equalizer_v3" => Ok(Self::EqualizerV3),
            "lynex_v2" => Ok(Self::LynexV2),
            "lynex_v3" => Ok(Self::LynexV3),
            "monoswap_v2" => Ok(Self::MonoswapV2),
            "monoswap_v3" => Ok(Self::MonoswapV3),
            "pancakeswap_v2" => Ok(Self::PancakeswapV2),
            "pancakeswap_v3" => Ok(Self::PancakeswapV3),
            "quickswap_v2" => Ok(Self::QuickswapV2),
            "quickswap_v3" => Ok(Self::QuickswapV3),
            "spookyswap_v2" => Ok(Self::SpookyswapV2),
            "spookyswap_v3" => Ok(Self::SpookyswapV3),
            "sushiswap_v2" => Ok(Self::SushiswapV2),
            "sushiswap_v3" => Ok(Self::SushiswapV3),
            "thruster_v2_2" => Ok(Self::ThrusterV22),
            "thruster_v3" => Ok(Self::ThrusterV3),
            "traderjoe_v2_0" => Ok(Self::TraderjoeV20),
            "traderjoe_v2_1" => Ok(Self::TraderjoeV21),
            "uniswap_v2" => Ok(Self::UniswapV2),
            "uniswap_v3" => Ok(Self::UniswapV3),
            "uniswap_v4" => Ok(Self::UniswapV4),
            "unknown_v2" => Ok(Self::UnknownV2),
            "unknown_v3" => Ok(Self::UnknownV3),
            "unknown_v4" => Ok(Self::UnknownV4),
            "velodrome_v2" => Ok(Self::VelodromeV2),
            "velodrome_v3" => Ok(Self::VelodromeV3),
            "fourmeme_bc" => Ok(Self::FourmemeBc),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PlaceSpotOrderRequestOrderPairIdPairTypeSubtype0 {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for PlaceSpotOrderRequestOrderPairIdPairTypeSubtype0 {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for PlaceSpotOrderRequestOrderPairIdPairTypeSubtype0 {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`PlaceSpotOrderRequestOrderPairIdPairTypeSubtype1`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "meteora_damm_v1",
///    "meteora_damm_v2",
///    "meteora_dlmm",
///    "orca_whirlpools",
///    "pump_amm",
///    "raydium_clmm",
///    "raydium_cpmm",
///    "raydium_v4",
///    "pump_bc",
///    "launchlab_bc",
///    "meteora_bc",
///    "moonit_bc"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum PlaceSpotOrderRequestOrderPairIdPairTypeSubtype1 {
    #[serde(rename = "meteora_damm_v1")]
    MeteoraDammV1,
    #[serde(rename = "meteora_damm_v2")]
    MeteoraDammV2,
    #[serde(rename = "meteora_dlmm")]
    MeteoraDlmm,
    #[serde(rename = "orca_whirlpools")]
    OrcaWhirlpools,
    #[serde(rename = "pump_amm")]
    PumpAmm,
    #[serde(rename = "raydium_clmm")]
    RaydiumClmm,
    #[serde(rename = "raydium_cpmm")]
    RaydiumCpmm,
    #[serde(rename = "raydium_v4")]
    RaydiumV4,
    #[serde(rename = "pump_bc")]
    PumpBc,
    #[serde(rename = "launchlab_bc")]
    LaunchlabBc,
    #[serde(rename = "meteora_bc")]
    MeteoraBc,
    #[serde(rename = "moonit_bc")]
    MoonitBc,
}
impl ::std::fmt::Display for PlaceSpotOrderRequestOrderPairIdPairTypeSubtype1 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::MeteoraDammV1 => f.write_str("meteora_damm_v1"),
            Self::MeteoraDammV2 => f.write_str("meteora_damm_v2"),
            Self::MeteoraDlmm => f.write_str("meteora_dlmm"),
            Self::OrcaWhirlpools => f.write_str("orca_whirlpools"),
            Self::PumpAmm => f.write_str("pump_amm"),
            Self::RaydiumClmm => f.write_str("raydium_clmm"),
            Self::RaydiumCpmm => f.write_str("raydium_cpmm"),
            Self::RaydiumV4 => f.write_str("raydium_v4"),
            Self::PumpBc => f.write_str("pump_bc"),
            Self::LaunchlabBc => f.write_str("launchlab_bc"),
            Self::MeteoraBc => f.write_str("meteora_bc"),
            Self::MoonitBc => f.write_str("moonit_bc"),
        }
    }
}
impl ::std::str::FromStr for PlaceSpotOrderRequestOrderPairIdPairTypeSubtype1 {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "meteora_damm_v1" => Ok(Self::MeteoraDammV1),
            "meteora_damm_v2" => Ok(Self::MeteoraDammV2),
            "meteora_dlmm" => Ok(Self::MeteoraDlmm),
            "orca_whirlpools" => Ok(Self::OrcaWhirlpools),
            "pump_amm" => Ok(Self::PumpAmm),
            "raydium_clmm" => Ok(Self::RaydiumClmm),
            "raydium_cpmm" => Ok(Self::RaydiumCpmm),
            "raydium_v4" => Ok(Self::RaydiumV4),
            "pump_bc" => Ok(Self::PumpBc),
            "launchlab_bc" => Ok(Self::LaunchlabBc),
            "meteora_bc" => Ok(Self::MeteoraBc),
            "moonit_bc" => Ok(Self::MoonitBc),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PlaceSpotOrderRequestOrderPairIdPairTypeSubtype1 {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for PlaceSpotOrderRequestOrderPairIdPairTypeSubtype1 {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for PlaceSpotOrderRequestOrderPairIdPairTypeSubtype1 {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///The side of the order; must be "buy" or "sell"
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "The side of the order; must be \"buy\" or \"sell\"",
///  "type": "string",
///  "enum": [
///    "buy",
///    "sell"
///  ],
///  "name": "Side"
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum PlaceSpotOrderRequestOrderSide {
    #[serde(rename = "buy")]
    Buy,
    #[serde(rename = "sell")]
    Sell,
}
impl ::std::fmt::Display for PlaceSpotOrderRequestOrderSide {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Buy => f.write_str("buy"),
            Self::Sell => f.write_str("sell"),
        }
    }
}
impl ::std::str::FromStr for PlaceSpotOrderRequestOrderSide {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "buy" => Ok(Self::Buy),
            "sell" => Ok(Self::Sell),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PlaceSpotOrderRequestOrderSide {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for PlaceSpotOrderRequestOrderSide {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PlaceSpotOrderRequestOrderSide {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`PlaceSpotOrderRequestOrderTxPreset`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "bribe",
///    "key",
///    "maxBaseGas",
///    "method",
///    "priorityGas",
///    "slippage"
///  ],
///  "properties": {
///    "bribe": {
///      "description": "The bribe for the order",
///      "type": "string",
///      "name": "Bribe"
///    },
///    "key": {
///      "description": "The key for the transaction preset",
///      "type": "string",
///      "name": "Preset Key"
///    },
///    "maxBaseGas": {
///      "description": "The maximum base gas for the order",
///      "type": "string",
///      "name": "Max Base Gas"
///    },
///    "method": {
///      "description": "The method for the order; must be \"flashbot\" or \"normal\". Only applies to EVM chains.",
///      "type": "string",
///      "enum": [
///        "flashbot",
///        "normal"
///      ],
///      "name": "Method"
///    },
///    "priorityGas": {
///      "description": "The priority gas for the order",
///      "type": "string",
///      "name": "Priority Gas"
///    },
///    "slippage": {
///      "description": "The slippage for the order",
///      "type": "string",
///      "name": "Slippage"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct PlaceSpotOrderRequestOrderTxPreset {
    ///The bribe for the order
    pub bribe: ::std::string::String,
    ///The key for the transaction preset
    pub key: ::std::string::String,
    ///The maximum base gas for the order
    #[serde(rename = "maxBaseGas")]
    pub max_base_gas: ::std::string::String,
    ///The method for the order; must be "flashbot" or "normal". Only applies to EVM chains.
    pub method: PlaceSpotOrderRequestOrderTxPresetMethod,
    ///The priority gas for the order
    #[serde(rename = "priorityGas")]
    pub priority_gas: ::std::string::String,
    ///The slippage for the order
    pub slippage: ::std::string::String,
}
///The method for the order; must be "flashbot" or "normal". Only applies to EVM chains.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "The method for the order; must be \"flashbot\" or \"normal\". Only applies to EVM chains.",
///  "type": "string",
///  "enum": [
///    "flashbot",
///    "normal"
///  ],
///  "name": "Method"
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum PlaceSpotOrderRequestOrderTxPresetMethod {
    #[serde(rename = "flashbot")]
    Flashbot,
    #[serde(rename = "normal")]
    Normal,
}
impl ::std::fmt::Display for PlaceSpotOrderRequestOrderTxPresetMethod {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Flashbot => f.write_str("flashbot"),
            Self::Normal => f.write_str("normal"),
        }
    }
}
impl ::std::str::FromStr for PlaceSpotOrderRequestOrderTxPresetMethod {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "flashbot" => Ok(Self::Flashbot),
            "normal" => Ok(Self::Normal),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PlaceSpotOrderRequestOrderTxPresetMethod {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for PlaceSpotOrderRequestOrderTxPresetMethod {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for PlaceSpotOrderRequestOrderTxPresetMethod {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`PlaceSpotOrderRequestOrderWalletsItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "address"
///  ],
///  "properties": {
///    "address": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct PlaceSpotOrderRequestOrderWalletsItem {
    pub address: ::std::string::String,
}
///`PlaceSpotOrderResponseItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "transactions",
///    "wallet"
///  ],
///  "properties": {
///    "transactions": {
///      "description": "Gives a list of transactions for the wallet, because a single spot order request may be executed via multiple transactions",
///      "type": "array",
///      "items": {
///        "anyOf": [
///          {
///            "description": "Gives the hash of the successful transaction",
///            "type": "object",
///            "required": [
///              "error",
///              "hash"
///            ],
///            "properties": {
///              "error": {
///                "type": "null"
///              },
///              "hash": {
///                "type": "string"
///              }
///            },
///            "additionalProperties": false,
///            "name": "Successful Transaction"
///          },
///          {
///            "description": "Gives the error message of the failed transaction",
///            "type": "object",
///            "required": [
///              "error",
///              "hash"
///            ],
///            "properties": {
///              "error": {
///                "type": "string"
///              },
///              "hash": {
///                "type": "null"
///              }
///            },
///            "additionalProperties": false,
///            "name": "Failed Transaction"
///          }
///        ]
///      },
///      "name": "Wallet Transactions"
///    },
///    "wallet": {
///      "type": "string"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct PlaceSpotOrderResponseItem {
    ///Gives a list of transactions for the wallet, because a single spot order request may be executed via multiple transactions
    pub transactions: ::std::vec::Vec<PlaceSpotOrderResponseItemTransactionsItem>,
    pub wallet: ::std::string::String,
}
///`PlaceSpotOrderResponseItemTransactionsItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "anyOf": [
///    {
///      "description": "Gives the hash of the successful transaction",
///      "type": "object",
///      "required": [
///        "error",
///        "hash"
///      ],
///      "properties": {
///        "error": {
///          "type": "null"
///        },
///        "hash": {
///          "type": "string"
///        }
///      },
///      "additionalProperties": false,
///      "name": "Successful Transaction"
///    },
///    {
///      "description": "Gives the error message of the failed transaction",
///      "type": "object",
///      "required": [
///        "error",
///        "hash"
///      ],
///      "properties": {
///        "error": {
///          "type": "string"
///        },
///        "hash": {
///          "type": "null"
///        }
///      },
///      "additionalProperties": false,
///      "name": "Failed Transaction"
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct PlaceSpotOrderResponseItemTransactionsItem {
    #[serde(flatten, default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_0: ::std::option::Option<
        PlaceSpotOrderResponseItemTransactionsItemSubtype0,
    >,
    #[serde(flatten, default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subtype_1: ::std::option::Option<
        PlaceSpotOrderResponseItemTransactionsItemSubtype1,
    >,
}
impl ::std::default::Default for PlaceSpotOrderResponseItemTransactionsItem {
    fn default() -> Self {
        Self {
            subtype_0: Default::default(),
            subtype_1: Default::default(),
        }
    }
}
///Gives the hash of the successful transaction
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Gives the hash of the successful transaction",
///  "type": "object",
///  "required": [
///    "error",
///    "hash"
///  ],
///  "properties": {
///    "error": {
///      "type": "null"
///    },
///    "hash": {
///      "type": "string"
///    }
///  },
///  "additionalProperties": false,
///  "name": "Successful Transaction"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct PlaceSpotOrderResponseItemTransactionsItemSubtype0 {
    pub error: (),
    pub hash: ::std::string::String,
}
///Gives the error message of the failed transaction
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Gives the error message of the failed transaction",
///  "type": "object",
///  "required": [
///    "error",
///    "hash"
///  ],
///  "properties": {
///    "error": {
///      "type": "string"
///    },
///    "hash": {
///      "type": "null"
///    }
///  },
///  "additionalProperties": false,
///  "name": "Failed Transaction"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct PlaceSpotOrderResponseItemTransactionsItemSubtype1 {
    pub error: ::std::string::String,
    pub hash: (),
}
