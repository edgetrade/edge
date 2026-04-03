#![allow(clippy::derivable_impls)]

use crate::client::{Route, RouteType};
use std::marker::PhantomData;
/// Route metadata for this procedure
pub const ROUTE: Route<TokenInfoWithPricingRequest, TokenInfoWithPricingResponse> = Route {
    procedure: "tokens.getTokenSimple",
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
///Request to get token information with optional pair details
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Request to get token information with optional pair details",
///  "type": "object",
///  "required": [
///    "chainId",
///    "tokenContractAddress",
///    "useBestPair"
///  ],
///  "properties": {
///    "chainId": {
///      "description": "The chain ID where the token exists (e.g., \"1\" for Ethereum, \"8453\" for Base, \"solana\" for Solana)",
///      "type": "string",
///      "name": "Chain ID"
///    },
///    "pairContractAddress": {
///      "description": "Optional pair contract address for additional context",
///      "type": "string",
///      "name": "Pair Contract Address"
///    },
///    "tokenContractAddress": {
///      "description": "The contract address of the token to query",
///      "type": "string",
///      "name": "Token Contract Address"
///    },
///    "useBestPair": {
///      "description": "Whether to use the best available pair for the token",
///      "type": "boolean",
///      "name": "Use Best Pair"
///    }
///  },
///  "name": "Get Token Simple Request"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct TokenInfoWithPricingRequest {
    ///The chain ID where the token exists (e.g., "1" for Ethereum, "8453" for Base, "solana" for Solana)
    #[serde(rename = "chainId")]
    pub chain_id: ::std::string::String,
    ///Optional pair contract address for additional context
    #[serde(
        rename = "pairContractAddress",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub pair_contract_address: ::std::option::Option<::std::string::String>,
    ///The contract address of the token to query
    #[serde(rename = "tokenContractAddress")]
    pub token_contract_address: ::std::string::String,
    ///Whether to use the best available pair for the token
    #[serde(rename = "useBestPair")]
    pub use_best_pair: bool,
}
///Response containing token information with optional pair details
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Response containing token information with optional pair details",
///  "type": "object",
///  "required": [
///    "pair",
///    "token"
///  ],
///  "properties": {
///    "pair": {
///      "anyOf": [
///        {
///          "type": "object",
///          "required": [
///            "counterTokenAddress",
///            "counterTokenDecimals",
///            "counterTokenIsNativeToken",
///            "counterTokenName",
///            "counterTokenReserves",
///            "counterTokenSymbol",
///            "createdAt",
///            "exchangeSymbol",
///            "fundraisingTarget",
///            "graduatedFromPair",
///            "isNativeTokenReferencePair",
///            "liquidityUsd",
///            "logoUrl",
///            "marketcapUsd",
///            "migratedToPair",
///            "pairChainId",
///            "pairContractAddress",
///            "pairSymbol",
///            "pairType",
///            "tokenAddress",
///            "tokenCreatedAt",
///            "tokenDecimals",
///            "tokenDeployerAddress",
///            "tokenEffectiveSupply",
///            "tokenName",
///            "tokenPriceNativeToken",
///            "tokenPriceUsd",
///            "tokenReserves",
///            "tokenSlot",
///            "tokenSymbol",
///            "vault0Address",
///            "vault1Address"
///          ],
///          "properties": {
///            "counterTokenAddress": {
///              "type": "string"
///            },
///            "counterTokenDecimals": {
///              "type": "number"
///            },
///            "counterTokenIsNativeToken": {
///              "type": "boolean"
///            },
///            "counterTokenName": {
///              "type": "string"
///            },
///            "counterTokenReserves": {
///              "anyOf": [
///                {
///                  "type": "string"
///                },
///                {
///                  "type": "null"
///                }
///              ]
///            },
///            "counterTokenSymbol": {
///              "type": "string"
///            },
///            "createdAt": {
///              "type": "string"
///            },
///            "exchangeSymbol": {
///              "type": "string"
///            },
///            "fundraisingTarget": {
///              "anyOf": [
///                {
///                  "type": "string"
///                },
///                {
///                  "type": "null"
///                }
///              ]
///            },
///            "graduatedFromPair": {
///              "anyOf": [
///                {
///                  "type": "object",
///                  "required": [
///                    "date",
///                    "pairChainId",
///                    "pairContractAddress",
///                    "pairType"
///                  ],
///                  "properties": {
///                    "date": {
///                      "anyOf": [
///                        {
///                          "type": "string"
///                        },
///                        {
///                          "type": "null"
///                        }
///                      ]
///                    },
///                    "pairChainId": {
///                      "type": "string"
///                    },
///                    "pairContractAddress": {
///                      "type": "string"
///                    },
///                    "pairType": {
///                      "type": "string"
///                    }
///                  },
///                  "additionalProperties": false
///                },
///                {
///                  "type": "null"
///                }
///              ]
///            },
///            "isNativeTokenReferencePair": {
///              "type": "boolean"
///            },
///            "liquidityUsd": {
///              "anyOf": [
///                {
///                  "type": "number"
///                },
///                {
///                  "type": "null"
///                }
///              ]
///            },
///            "logoUrl": {
///              "anyOf": [
///                {
///                  "type": "string"
///                },
///                {
///                  "type": "null"
///                }
///              ]
///            },
///            "marketcapUsd": {
///              "anyOf": [
///                {
///                  "type": "number"
///                },
///                {
///                  "type": "null"
///                }
///              ]
///            },
///            "migratedToPair": {
///              "anyOf": [
///                {
///                  "type": "object",
///                  "required": [
///                    "date",
///                    "pairChainId",
///                    "pairContractAddress",
///                    "pairType"
///                  ],
///                  "properties": {
///                    "date": {
///                      "anyOf": [
///                        {
///                          "type": "string"
///                        },
///                        {
///                          "type": "null"
///                        }
///                      ]
///                    },
///                    "pairChainId": {
///                      "type": "string"
///                    },
///                    "pairContractAddress": {
///                      "type": "string"
///                    },
///                    "pairType": {
///                      "type": "string"
///                    }
///                  },
///                  "additionalProperties": false
///                },
///                {
///                  "type": "null"
///                }
///              ]
///            },
///            "pairChainId": {
///              "type": "string"
///            },
///            "pairContractAddress": {
///              "type": "string"
///            },
///            "pairSymbol": {
///              "type": "string"
///            },
///            "pairType": {
///              "type": "string"
///            },
///            "tokenAddress": {
///              "type": "string"
///            },
///            "tokenCreatedAt": {
///              "type": "string"
///            },
///            "tokenDecimals": {
///              "type": "number"
///            },
///            "tokenDeployerAddress": {
///              "anyOf": [
///                {
///                  "type": "string"
///                },
///                {
///                  "type": "null"
///                }
///              ]
///            },
///            "tokenEffectiveSupply": {
///              "anyOf": [
///                {
///                  "type": "string"
///                },
///                {
///                  "type": "null"
///                }
///              ]
///            },
///            "tokenName": {
///              "type": "string"
///            },
///            "tokenPriceNativeToken": {
///              "anyOf": [
///                {
///                  "type": "number"
///                },
///                {
///                  "type": "null"
///                }
///              ]
///            },
///            "tokenPriceUsd": {
///              "anyOf": [
///                {
///                  "type": "number"
///                },
///                {
///                  "type": "null"
///                }
///              ]
///            },
///            "tokenReserves": {
///              "anyOf": [
///                {
///                  "type": "string"
///                },
///                {
///                  "type": "null"
///                }
///              ]
///            },
///            "tokenSlot": {
///              "type": "string"
///            },
///            "tokenSymbol": {
///              "type": "string"
///            },
///            "vault0Address": {
///              "anyOf": [
///                {
///                  "type": "string"
///                },
///                {
///                  "type": "null"
///                }
///              ]
///            },
///            "vault1Address": {
///              "anyOf": [
///                {
///                  "type": "string"
///                },
///                {
///                  "type": "null"
///                }
///              ]
///            }
///          },
///          "additionalProperties": false
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "token": {
///      "type": "object",
///      "required": [
///        "details",
///        "summary",
///        "tokenChainId",
///        "tokenContractAddress",
///        "tokenSymbol"
///      ],
///      "properties": {
///        "details": {
///          "type": "object",
///          "required": [
///            "contract",
///            "deployer",
///            "holderDistribution",
///            "social",
///            "transactions"
///          ],
///          "properties": {
///            "contract": {
///              "anyOf": [
///                {
///                  "type": "object",
///                  "required": [
///                    "byteCode",
///                    "sourceCodeS3Key",
///                    "verified"
///                  ],
///                  "properties": {
///                    "byteCode": {
///                      "anyOf": [
///                        {
///                          "type": "string"
///                        },
///                        {
///                          "type": "null"
///                        }
///                      ]
///                    },
///                    "refresh": {
///                      "anyOf": [
///                        {
///                          "type": "object",
///                          "required": [
///                            "backoffRatio",
///                            "nextRefresh",
///                            "previousAttempt"
///                          ],
///                          "properties": {
///                            "backoffRatio": {
///                              "type": "number"
///                            },
///                            "nextRefresh": {
///                              "type": "string"
///                            },
///                            "previousAttempt": {
///                              "type": "string"
///                            }
///                          },
///                          "additionalProperties": false
///                        },
///                        {
///                          "type": "null"
///                        }
///                      ]
///                    },
///                    "sourceCodeS3Key": {
///                      "anyOf": [
///                        {
///                          "type": "string"
///                        },
///                        {
///                          "type": "null"
///                        }
///                      ]
///                    },
///                    "verified": {
///                      "anyOf": [
///                        {
///                          "type": "boolean"
///                        },
///                        {
///                          "type": "null"
///                        }
///                      ]
///                    }
///                  },
///                  "additionalProperties": false
///                },
///                {
///                  "type": "null"
///                }
///              ]
///            },
///            "deployer": {
///              "anyOf": [
///                {
///                  "type": "object",
///                  "required": [
///                    "address",
///                    "createdTx",
///                    "renounced"
///                  ],
///                  "properties": {
///                    "address": {
///                      "type": "string"
///                    },
///                    "createdTx": {
///                      "type": "string"
///                    },
///                    "renounced": {
///                      "anyOf": [
///                        {
///                          "type": "boolean"
///                        },
///                        {
///                          "type": "null"
///                        }
///                      ]
///                    }
///                  },
///                  "additionalProperties": false
///                },
///                {
///                  "type": "null"
///                }
///              ]
///            },
///            "holderDistribution": {
///              "anyOf": [
///                {
///                  "type": "object",
///                  "properties": {
///                    "insidersBalance": {
///                      "anyOf": [
///                        {
///                          "type": "string"
///                        },
///                        {
///                          "type": "null"
///                        }
///                      ]
///                    },
///                    "snipersBalance": {
///                      "anyOf": [
///                        {
///                          "type": "string"
///                        },
///                        {
///                          "type": "null"
///                        }
///                      ]
///                    },
///                    "top100Balance": {
///                      "anyOf": [
///                        {
///                          "type": "string"
///                        },
///                        {
///                          "type": "null"
///                        }
///                      ]
///                    },
///                    "top10Balance": {
///                      "anyOf": [
///                        {
///                          "type": "string"
///                        },
///                        {
///                          "type": "null"
///                        }
///                      ]
///                    },
///                    "top20Balance": {
///                      "anyOf": [
///                        {
///                          "type": "string"
///                        },
///                        {
///                          "type": "null"
///                        }
///                      ]
///                    },
///                    "top50Balance": {
///                      "anyOf": [
///                        {
///                          "type": "string"
///                        },
///                        {
///                          "type": "null"
///                        }
///                      ]
///                    }
///                  },
///                  "additionalProperties": false
///                },
///                {
///                  "type": "null"
///                }
///              ]
///            },
///            "safetyReports": {
///              "anyOf": [
///                {
///                  "type": "array",
///                  "items": {
///                    "type": "string"
///                  }
///                },
///                {
///                  "type": "null"
///                }
///              ]
///            },
///            "social": {
///              "anyOf": [
///                {
///                  "type": "object",
///                  "properties": {
///                    "dexscreener": {
///                      "anyOf": [
///                        {
///                          "type": "object",
///                          "properties": {
///                            "bannerUrl": {
///                              "type": "string"
///                            },
///                            "description": {
///                              "type": "string"
///                            },
///                            "links": {
///                              "type": "array",
///                              "items": {
///                                "type": "object",
///                                "required": [
///                                  "platform",
///                                  "url"
///                                ],
///                                "properties": {
///                                  "platform": {
///                                    "type": "string"
///                                  },
///                                  "url": {
///                                    "type": "string"
///                                  }
///                                },
///                                "additionalProperties": false
///                              }
///                            },
///                            "logoUrl": {
///                              "type": "string"
///                            },
///                            "refresh": {
///                              "type": "object",
///                              "required": [
///                                "backoffRatio",
///                                "nextRefresh",
///                                "previousAttempt"
///                              ],
///                              "properties": {
///                                "backoffRatio": {
///                                  "type": "number"
///                                },
///                                "nextRefresh": {
///                                  "type": "string"
///                                },
///                                "previousAttempt": {
///                                  "type": "string"
///                                }
///                              },
///                              "additionalProperties": false
///                            }
///                          },
///                          "additionalProperties": false
///                        },
///                        {
///                          "type": "null"
///                        }
///                      ]
///                    },
///                    "dexscreenerPaid": {
///                      "anyOf": [
///                        {
///                          "type": "object",
///                          "required": [
///                            "orders"
///                          ],
///                          "properties": {
///                            "orders": {
///                              "type": "array",
///                              "items": {
///                                "type": "object",
///                                "required": [
///                                  "paymentTimestamp",
///                                  "status",
///                                  "type"
///                                ],
///                                "properties": {
///                                  "paymentTimestamp": {
///                                    "type": "number"
///                                  },
///                                  "status": {
///                                    "type": "string"
///                                  },
///                                  "type": {
///                                    "type": "string"
///                                  }
///                                },
///                                "additionalProperties": false
///                              }
///                            },
///                            "refresh": {
///                              "type": "object",
///                              "required": [
///                                "backoffRatio",
///                                "nextRefresh",
///                                "previousAttempt"
///                              ],
///                              "properties": {
///                                "backoffRatio": {
///                                  "type": "number"
///                                },
///                                "nextRefresh": {
///                                  "type": "string"
///                                },
///                                "previousAttempt": {
///                                  "type": "string"
///                                }
///                              },
///                              "additionalProperties": false
///                            }
///                          },
///                          "additionalProperties": false
///                        },
///                        {
///                          "type": "null"
///                        }
///                      ]
///                    },
///                    "gecko": {
///                      "anyOf": [
///                        {
///                          "type": "object",
///                          "properties": {
///                            "description": {
///                              "anyOf": [
///                                {
///                                  "type": "string"
///                                },
///                                {
///                                  "type": "null"
///                                }
///                              ]
///                            },
///                            "links": {
///                              "type": "array",
///                              "items": {
///                                "type": "object",
///                                "required": [
///                                  "platform",
///                                  "url"
///                                ],
///                                "properties": {
///                                  "platform": {
///                                    "type": "string"
///                                  },
///                                  "url": {
///                                    "type": "string"
///                                  }
///                                },
///                                "additionalProperties": false
///                              }
///                            },
///                            "logoUrl": {
///                              "type": "object",
///                              "required": [
///                                "large",
///                                "small",
///                                "thumb"
///                              ],
///                              "properties": {
///                                "large": {
///                                  "type": "string"
///                                },
///                                "small": {
///                                  "type": "string"
///                                },
///                                "thumb": {
///                                  "type": "string"
///                                }
///                              },
///                              "additionalProperties": false
///                            },
///                            "refresh": {
///                              "type": "object",
///                              "required": [
///                                "backoffRatio",
///                                "nextRefresh",
///                                "previousAttempt"
///                              ],
///                              "properties": {
///                                "backoffRatio": {
///                                  "type": "number"
///                                },
///                                "nextRefresh": {
///                                  "type": "string"
///                                },
///                                "previousAttempt": {
///                                  "type": "string"
///                                }
///                              },
///                              "additionalProperties": false
///                            }
///                          },
///                          "additionalProperties": false
///                        },
///                        {
///                          "type": "null"
///                        }
///                      ]
///                    },
///                    "metaplex": {
///                      "anyOf": [
///                        {
///                          "type": "object",
///                          "properties": {
///                            "description": {
///                              "type": "string"
///                            },
///                            "links": {
///                              "type": "array",
///                              "items": {
///                                "type": "object",
///                                "required": [
///                                  "platform",
///                                  "url"
///                                ],
///                                "properties": {
///                                  "platform": {
///                                    "type": "string"
///                                  },
///                                  "url": {
///                                    "type": "string"
///                                  }
///                                },
///                                "additionalProperties": false
///                              }
///                            },
///                            "logoUrl": {
///                              "type": "string"
///                            },
///                            "name": {
///                              "type": "string"
///                            },
///                            "refresh": {
///                              "type": "object",
///                              "required": [
///                                "backoffRatio",
///                                "nextRefresh",
///                                "previousAttempt"
///                              ],
///                              "properties": {
///                                "backoffRatio": {
///                                  "type": "number"
///                                },
///                                "nextRefresh": {
///                                  "type": "string"
///                                },
///                                "previousAttempt": {
///                                  "type": "string"
///                                }
///                              },
///                              "additionalProperties": false
///                            },
///                            "sellerFeeBasisPoints": {
///                              "type": "number"
///                            },
///                            "symbol": {
///                              "type": "string"
///                            },
///                            "uri": {
///                              "type": "string"
///                            }
///                          },
///                          "additionalProperties": false
///                        },
///                        {
///                          "type": "null"
///                        }
///                      ]
///                    },
///                    "twitterProfile": {
///                      "anyOf": [
///                        {
///                          "type": "object",
///                          "properties": {
///                            "communities": {
///                              "anyOf": [
///                                {
///                                  "type": "array",
///                                  "items": {
///                                    "type": "object",
///                                    "required": [
///                                      "id",
///                                      "name"
///                                    ],
///                                    "properties": {
///                                      "access": {
///                                        "type": "string"
///                                      },
///                                      "createdAt": {
///                                        "type": "string"
///                                      },
///                                      "description": {
///                                        "type": "string"
///                                      },
///                                      "id": {
///                                        "type": "string"
///                                      },
///                                      "joinPolicy": {
///                                        "type": "string"
///                                      },
///                                      "memberCount": {
///                                        "type": "number"
///                                      },
///                                      "name": {
///                                        "type": "string"
///                                      }
///                                    },
///                                    "additionalProperties": false
///                                  }
///                                },
///                                {
///                                  "type": "null"
///                                }
///                              ]
///                            },
///                            "profiles": {
///                              "anyOf": [
///                                {
///                                  "type": "array",
///                                  "items": {
///                                    "type": "object",
///                                    "required": [
///                                      "handle"
///                                    ],
///                                    "properties": {
///                                      "bannerImage": {
///                                        "type": "string"
///                                      },
///                                      "dateJoined": {
///                                        "type": "string"
///                                      },
///                                      "description": {
///                                        "type": "string"
///                                      },
///                                      "followers": {
///                                        "type": "number"
///                                      },
///                                      "following": {
///                                        "type": "number"
///                                      },
///                                      "handle": {
///                                        "type": "string"
///                                      },
///                                      "logoImage": {
///                                        "type": "string"
///                                      },
///                                      "name": {
///                                        "type": "string"
///                                      },
///                                      "verifiedType": {
///                                        "type": "string"
///                                      }
///                                    },
///                                    "additionalProperties": false
///                                  }
///                                },
///                                {
///                                  "type": "null"
///                                }
///                              ]
///                            },
///                            "refresh": {
///                              "type": "object",
///                              "required": [
///                                "backoffRatio",
///                                "nextRefresh",
///                                "previousAttempt"
///                              ],
///                              "properties": {
///                                "backoffRatio": {
///                                  "type": "number"
///                                },
///                                "nextRefresh": {
///                                  "type": "string"
///                                },
///                                "previousAttempt": {
///                                  "type": "string"
///                                }
///                              },
///                              "additionalProperties": false
///                            }
///                          },
///                          "additionalProperties": false
///                        },
///                        {
///                          "type": "null"
///                        }
///                      ]
///                    }
///                  },
///                  "additionalProperties": false
///                },
///                {
///                  "type": "null"
///                }
///              ]
///            },
///            "transactions": {
///              "anyOf": [
///                {
///                  "type": "object",
///                  "required": [
///                    "buyTax",
///                    "maxPerTx",
///                    "maxPerWallet",
///                    "refresh",
///                    "sellTax",
///                    "txTax"
///                  ],
///                  "properties": {
///                    "buyTax": {
///                      "type": "number"
///                    },
///                    "maxPerTx": {
///                      "type": "string"
///                    },
///                    "maxPerWallet": {
///                      "type": "string"
///                    },
///                    "refresh": {
///                      "type": "object",
///                      "required": [
///                        "nextRefresh",
///                        "previousAttempt"
///                      ],
///                      "properties": {
///                        "nextRefresh": {
///                          "type": "string"
///                        },
///                        "previousAttempt": {
///                          "type": "string"
///                        }
///                      },
///                      "additionalProperties": false
///                    },
///                    "sellTax": {
///                      "type": "number"
///                    },
///                    "txTax": {
///                      "type": "number"
///                    }
///                  },
///                  "additionalProperties": false
///                },
///                {
///                  "type": "null"
///                }
///              ]
///            }
///          },
///          "additionalProperties": false
///        },
///        "summary": {
///          "type": "object",
///          "required": [
///            "athTokenPriceUsd",
///            "atlTokenPriceUsd",
///            "bestPairAddress",
///            "creationTimestamp",
///            "decimals",
///            "effectiveSupply",
///            "holderCount",
///            "marketcapUsd",
///            "marketcapUsdAth",
///            "marketcapUsdAtl",
///            "name",
///            "svmProgramId",
///            "symbol",
///            "tokenPriceUsd",
///            "totalSupply"
///          ],
///          "properties": {
///            "athTokenPriceUsd": {
///              "type": "number"
///            },
///            "atlTokenPriceUsd": {
///              "type": "number"
///            },
///            "bannerUrl": {
///              "anyOf": [
///                {
///                  "type": "string"
///                },
///                {
///                  "type": "null"
///                }
///              ]
///            },
///            "bestPairAddress": {
///              "anyOf": [
///                {
///                  "type": "string"
///                },
///                {
///                  "type": "null"
///                }
///              ]
///            },
///            "bio": {
///              "anyOf": [
///                {
///                  "type": "string"
///                },
///                {
///                  "type": "null"
///                }
///              ]
///            },
///            "creationTimestamp": {
///              "type": "string"
///            },
///            "decimals": {
///              "type": "number"
///            },
///            "description": {
///              "anyOf": [
///                {
///                  "type": "string"
///                },
///                {
///                  "type": "null"
///                }
///              ]
///            },
///            "effectiveSupply": {
///              "type": "string"
///            },
///            "holderCount": {
///              "type": "number"
///            },
///            "lastBlockSeen": {
///              "anyOf": [
///                {
///                  "type": "number"
///                },
///                {
///                  "type": "null"
///                }
///              ]
///            },
///            "logoUrl": {
///              "anyOf": [
///                {
///                  "type": "string"
///                },
///                {
///                  "type": "null"
///                }
///              ]
///            },
///            "marketcapUsd": {
///              "type": "number"
///            },
///            "marketcapUsdAth": {
///              "type": "number"
///            },
///            "marketcapUsdAtl": {
///              "type": "number"
///            },
///            "name": {
///              "type": "string"
///            },
///            "svmProgramId": {
///              "anyOf": [
///                {
///                  "type": "string"
///                },
///                {
///                  "type": "null"
///                }
///              ]
///            },
///            "symbol": {
///              "type": "string"
///            },
///            "tokenPriceUsd": {
///              "type": "number"
///            },
///            "totalSupply": {
///              "type": "string"
///            }
///          },
///          "additionalProperties": false
///        },
///        "tokenChainId": {
///          "type": "string"
///        },
///        "tokenContractAddress": {
///          "type": "string"
///        },
///        "tokenSymbol": {
///          "anyOf": [
///            {
///              "type": "string"
///            },
///            {
///              "type": "null"
///            }
///          ]
///        }
///      },
///      "additionalProperties": false
///    }
///  },
///  "additionalProperties": false,
///  "name": "Get Token Simple Response"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TokenInfoWithPricingResponse {
    pub pair: ::std::option::Option<TokenInfoWithPricingResponsePair>,
    pub token: TokenInfoWithPricingResponseToken,
}
///`TokenInfoWithPricingResponsePair`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "counterTokenAddress",
///    "counterTokenDecimals",
///    "counterTokenIsNativeToken",
///    "counterTokenName",
///    "counterTokenReserves",
///    "counterTokenSymbol",
///    "createdAt",
///    "exchangeSymbol",
///    "fundraisingTarget",
///    "graduatedFromPair",
///    "isNativeTokenReferencePair",
///    "liquidityUsd",
///    "logoUrl",
///    "marketcapUsd",
///    "migratedToPair",
///    "pairChainId",
///    "pairContractAddress",
///    "pairSymbol",
///    "pairType",
///    "tokenAddress",
///    "tokenCreatedAt",
///    "tokenDecimals",
///    "tokenDeployerAddress",
///    "tokenEffectiveSupply",
///    "tokenName",
///    "tokenPriceNativeToken",
///    "tokenPriceUsd",
///    "tokenReserves",
///    "tokenSlot",
///    "tokenSymbol",
///    "vault0Address",
///    "vault1Address"
///  ],
///  "properties": {
///    "counterTokenAddress": {
///      "type": "string"
///    },
///    "counterTokenDecimals": {
///      "type": "number"
///    },
///    "counterTokenIsNativeToken": {
///      "type": "boolean"
///    },
///    "counterTokenName": {
///      "type": "string"
///    },
///    "counterTokenReserves": {
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "counterTokenSymbol": {
///      "type": "string"
///    },
///    "createdAt": {
///      "type": "string"
///    },
///    "exchangeSymbol": {
///      "type": "string"
///    },
///    "fundraisingTarget": {
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "graduatedFromPair": {
///      "anyOf": [
///        {
///          "type": "object",
///          "required": [
///            "date",
///            "pairChainId",
///            "pairContractAddress",
///            "pairType"
///          ],
///          "properties": {
///            "date": {
///              "anyOf": [
///                {
///                  "type": "string"
///                },
///                {
///                  "type": "null"
///                }
///              ]
///            },
///            "pairChainId": {
///              "type": "string"
///            },
///            "pairContractAddress": {
///              "type": "string"
///            },
///            "pairType": {
///              "type": "string"
///            }
///          },
///          "additionalProperties": false
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "isNativeTokenReferencePair": {
///      "type": "boolean"
///    },
///    "liquidityUsd": {
///      "anyOf": [
///        {
///          "type": "number"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "logoUrl": {
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "marketcapUsd": {
///      "anyOf": [
///        {
///          "type": "number"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "migratedToPair": {
///      "anyOf": [
///        {
///          "type": "object",
///          "required": [
///            "date",
///            "pairChainId",
///            "pairContractAddress",
///            "pairType"
///          ],
///          "properties": {
///            "date": {
///              "anyOf": [
///                {
///                  "type": "string"
///                },
///                {
///                  "type": "null"
///                }
///              ]
///            },
///            "pairChainId": {
///              "type": "string"
///            },
///            "pairContractAddress": {
///              "type": "string"
///            },
///            "pairType": {
///              "type": "string"
///            }
///          },
///          "additionalProperties": false
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "pairChainId": {
///      "type": "string"
///    },
///    "pairContractAddress": {
///      "type": "string"
///    },
///    "pairSymbol": {
///      "type": "string"
///    },
///    "pairType": {
///      "type": "string"
///    },
///    "tokenAddress": {
///      "type": "string"
///    },
///    "tokenCreatedAt": {
///      "type": "string"
///    },
///    "tokenDecimals": {
///      "type": "number"
///    },
///    "tokenDeployerAddress": {
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "tokenEffectiveSupply": {
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "tokenName": {
///      "type": "string"
///    },
///    "tokenPriceNativeToken": {
///      "anyOf": [
///        {
///          "type": "number"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "tokenPriceUsd": {
///      "anyOf": [
///        {
///          "type": "number"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "tokenReserves": {
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "tokenSlot": {
///      "type": "string"
///    },
///    "tokenSymbol": {
///      "type": "string"
///    },
///    "vault0Address": {
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "vault1Address": {
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TokenInfoWithPricingResponsePair {
    #[serde(rename = "counterTokenAddress")]
    pub counter_token_address: ::std::string::String,
    #[serde(rename = "counterTokenDecimals")]
    pub counter_token_decimals: f64,
    #[serde(rename = "counterTokenIsNativeToken")]
    pub counter_token_is_native_token: bool,
    #[serde(rename = "counterTokenName")]
    pub counter_token_name: ::std::string::String,
    #[serde(rename = "counterTokenReserves")]
    pub counter_token_reserves: ::std::option::Option<::std::string::String>,
    #[serde(rename = "counterTokenSymbol")]
    pub counter_token_symbol: ::std::string::String,
    #[serde(rename = "createdAt")]
    pub created_at: ::std::string::String,
    #[serde(rename = "exchangeSymbol")]
    pub exchange_symbol: ::std::string::String,
    #[serde(rename = "fundraisingTarget")]
    pub fundraising_target: ::std::option::Option<::std::string::String>,
    #[serde(rename = "graduatedFromPair")]
    pub graduated_from_pair: ::std::option::Option<TokenInfoWithPricingResponsePairGraduatedFromPair>,
    #[serde(rename = "isNativeTokenReferencePair")]
    pub is_native_token_reference_pair: bool,
    #[serde(rename = "liquidityUsd")]
    pub liquidity_usd: ::std::option::Option<f64>,
    #[serde(rename = "logoUrl")]
    pub logo_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "marketcapUsd")]
    pub marketcap_usd: ::std::option::Option<f64>,
    #[serde(rename = "migratedToPair")]
    pub migrated_to_pair: ::std::option::Option<TokenInfoWithPricingResponsePairMigratedToPair>,
    #[serde(rename = "pairChainId")]
    pub pair_chain_id: ::std::string::String,
    #[serde(rename = "pairContractAddress")]
    pub pair_contract_address: ::std::string::String,
    #[serde(rename = "pairSymbol")]
    pub pair_symbol: ::std::string::String,
    #[serde(rename = "pairType")]
    pub pair_type: ::std::string::String,
    #[serde(rename = "tokenAddress")]
    pub token_address: ::std::string::String,
    #[serde(rename = "tokenCreatedAt")]
    pub token_created_at: ::std::string::String,
    #[serde(rename = "tokenDecimals")]
    pub token_decimals: f64,
    #[serde(rename = "tokenDeployerAddress")]
    pub token_deployer_address: ::std::option::Option<::std::string::String>,
    #[serde(rename = "tokenEffectiveSupply")]
    pub token_effective_supply: ::std::option::Option<::std::string::String>,
    #[serde(rename = "tokenName")]
    pub token_name: ::std::string::String,
    #[serde(rename = "tokenPriceNativeToken")]
    pub token_price_native_token: ::std::option::Option<f64>,
    #[serde(rename = "tokenPriceUsd")]
    pub token_price_usd: ::std::option::Option<f64>,
    #[serde(rename = "tokenReserves")]
    pub token_reserves: ::std::option::Option<::std::string::String>,
    #[serde(rename = "tokenSlot")]
    pub token_slot: ::std::string::String,
    #[serde(rename = "tokenSymbol")]
    pub token_symbol: ::std::string::String,
    #[serde(rename = "vault0Address")]
    pub vault0_address: ::std::option::Option<::std::string::String>,
    #[serde(rename = "vault1Address")]
    pub vault1_address: ::std::option::Option<::std::string::String>,
}
///`TokenInfoWithPricingResponsePairGraduatedFromPair`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "date",
///    "pairChainId",
///    "pairContractAddress",
///    "pairType"
///  ],
///  "properties": {
///    "date": {
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "pairChainId": {
///      "type": "string"
///    },
///    "pairContractAddress": {
///      "type": "string"
///    },
///    "pairType": {
///      "type": "string"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TokenInfoWithPricingResponsePairGraduatedFromPair {
    pub date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "pairChainId")]
    pub pair_chain_id: ::std::string::String,
    #[serde(rename = "pairContractAddress")]
    pub pair_contract_address: ::std::string::String,
    #[serde(rename = "pairType")]
    pub pair_type: ::std::string::String,
}
///`TokenInfoWithPricingResponsePairMigratedToPair`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "date",
///    "pairChainId",
///    "pairContractAddress",
///    "pairType"
///  ],
///  "properties": {
///    "date": {
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "pairChainId": {
///      "type": "string"
///    },
///    "pairContractAddress": {
///      "type": "string"
///    },
///    "pairType": {
///      "type": "string"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TokenInfoWithPricingResponsePairMigratedToPair {
    pub date: ::std::option::Option<::std::string::String>,
    #[serde(rename = "pairChainId")]
    pub pair_chain_id: ::std::string::String,
    #[serde(rename = "pairContractAddress")]
    pub pair_contract_address: ::std::string::String,
    #[serde(rename = "pairType")]
    pub pair_type: ::std::string::String,
}
///`TokenInfoWithPricingResponseToken`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "details",
///    "summary",
///    "tokenChainId",
///    "tokenContractAddress",
///    "tokenSymbol"
///  ],
///  "properties": {
///    "details": {
///      "type": "object",
///      "required": [
///        "contract",
///        "deployer",
///        "holderDistribution",
///        "social",
///        "transactions"
///      ],
///      "properties": {
///        "contract": {
///          "anyOf": [
///            {
///              "type": "object",
///              "required": [
///                "byteCode",
///                "sourceCodeS3Key",
///                "verified"
///              ],
///              "properties": {
///                "byteCode": {
///                  "anyOf": [
///                    {
///                      "type": "string"
///                    },
///                    {
///                      "type": "null"
///                    }
///                  ]
///                },
///                "refresh": {
///                  "anyOf": [
///                    {
///                      "type": "object",
///                      "required": [
///                        "backoffRatio",
///                        "nextRefresh",
///                        "previousAttempt"
///                      ],
///                      "properties": {
///                        "backoffRatio": {
///                          "type": "number"
///                        },
///                        "nextRefresh": {
///                          "type": "string"
///                        },
///                        "previousAttempt": {
///                          "type": "string"
///                        }
///                      },
///                      "additionalProperties": false
///                    },
///                    {
///                      "type": "null"
///                    }
///                  ]
///                },
///                "sourceCodeS3Key": {
///                  "anyOf": [
///                    {
///                      "type": "string"
///                    },
///                    {
///                      "type": "null"
///                    }
///                  ]
///                },
///                "verified": {
///                  "anyOf": [
///                    {
///                      "type": "boolean"
///                    },
///                    {
///                      "type": "null"
///                    }
///                  ]
///                }
///              },
///              "additionalProperties": false
///            },
///            {
///              "type": "null"
///            }
///          ]
///        },
///        "deployer": {
///          "anyOf": [
///            {
///              "type": "object",
///              "required": [
///                "address",
///                "createdTx",
///                "renounced"
///              ],
///              "properties": {
///                "address": {
///                  "type": "string"
///                },
///                "createdTx": {
///                  "type": "string"
///                },
///                "renounced": {
///                  "anyOf": [
///                    {
///                      "type": "boolean"
///                    },
///                    {
///                      "type": "null"
///                    }
///                  ]
///                }
///              },
///              "additionalProperties": false
///            },
///            {
///              "type": "null"
///            }
///          ]
///        },
///        "holderDistribution": {
///          "anyOf": [
///            {
///              "type": "object",
///              "properties": {
///                "insidersBalance": {
///                  "anyOf": [
///                    {
///                      "type": "string"
///                    },
///                    {
///                      "type": "null"
///                    }
///                  ]
///                },
///                "snipersBalance": {
///                  "anyOf": [
///                    {
///                      "type": "string"
///                    },
///                    {
///                      "type": "null"
///                    }
///                  ]
///                },
///                "top100Balance": {
///                  "anyOf": [
///                    {
///                      "type": "string"
///                    },
///                    {
///                      "type": "null"
///                    }
///                  ]
///                },
///                "top10Balance": {
///                  "anyOf": [
///                    {
///                      "type": "string"
///                    },
///                    {
///                      "type": "null"
///                    }
///                  ]
///                },
///                "top20Balance": {
///                  "anyOf": [
///                    {
///                      "type": "string"
///                    },
///                    {
///                      "type": "null"
///                    }
///                  ]
///                },
///                "top50Balance": {
///                  "anyOf": [
///                    {
///                      "type": "string"
///                    },
///                    {
///                      "type": "null"
///                    }
///                  ]
///                }
///              },
///              "additionalProperties": false
///            },
///            {
///              "type": "null"
///            }
///          ]
///        },
///        "safetyReports": {
///          "anyOf": [
///            {
///              "type": "array",
///              "items": {
///                "type": "string"
///              }
///            },
///            {
///              "type": "null"
///            }
///          ]
///        },
///        "social": {
///          "anyOf": [
///            {
///              "type": "object",
///              "properties": {
///                "dexscreener": {
///                  "anyOf": [
///                    {
///                      "type": "object",
///                      "properties": {
///                        "bannerUrl": {
///                          "type": "string"
///                        },
///                        "description": {
///                          "type": "string"
///                        },
///                        "links": {
///                          "type": "array",
///                          "items": {
///                            "type": "object",
///                            "required": [
///                              "platform",
///                              "url"
///                            ],
///                            "properties": {
///                              "platform": {
///                                "type": "string"
///                              },
///                              "url": {
///                                "type": "string"
///                              }
///                            },
///                            "additionalProperties": false
///                          }
///                        },
///                        "logoUrl": {
///                          "type": "string"
///                        },
///                        "refresh": {
///                          "type": "object",
///                          "required": [
///                            "backoffRatio",
///                            "nextRefresh",
///                            "previousAttempt"
///                          ],
///                          "properties": {
///                            "backoffRatio": {
///                              "type": "number"
///                            },
///                            "nextRefresh": {
///                              "type": "string"
///                            },
///                            "previousAttempt": {
///                              "type": "string"
///                            }
///                          },
///                          "additionalProperties": false
///                        }
///                      },
///                      "additionalProperties": false
///                    },
///                    {
///                      "type": "null"
///                    }
///                  ]
///                },
///                "dexscreenerPaid": {
///                  "anyOf": [
///                    {
///                      "type": "object",
///                      "required": [
///                        "orders"
///                      ],
///                      "properties": {
///                        "orders": {
///                          "type": "array",
///                          "items": {
///                            "type": "object",
///                            "required": [
///                              "paymentTimestamp",
///                              "status",
///                              "type"
///                            ],
///                            "properties": {
///                              "paymentTimestamp": {
///                                "type": "number"
///                              },
///                              "status": {
///                                "type": "string"
///                              },
///                              "type": {
///                                "type": "string"
///                              }
///                            },
///                            "additionalProperties": false
///                          }
///                        },
///                        "refresh": {
///                          "type": "object",
///                          "required": [
///                            "backoffRatio",
///                            "nextRefresh",
///                            "previousAttempt"
///                          ],
///                          "properties": {
///                            "backoffRatio": {
///                              "type": "number"
///                            },
///                            "nextRefresh": {
///                              "type": "string"
///                            },
///                            "previousAttempt": {
///                              "type": "string"
///                            }
///                          },
///                          "additionalProperties": false
///                        }
///                      },
///                      "additionalProperties": false
///                    },
///                    {
///                      "type": "null"
///                    }
///                  ]
///                },
///                "gecko": {
///                  "anyOf": [
///                    {
///                      "type": "object",
///                      "properties": {
///                        "description": {
///                          "anyOf": [
///                            {
///                              "type": "string"
///                            },
///                            {
///                              "type": "null"
///                            }
///                          ]
///                        },
///                        "links": {
///                          "type": "array",
///                          "items": {
///                            "type": "object",
///                            "required": [
///                              "platform",
///                              "url"
///                            ],
///                            "properties": {
///                              "platform": {
///                                "type": "string"
///                              },
///                              "url": {
///                                "type": "string"
///                              }
///                            },
///                            "additionalProperties": false
///                          }
///                        },
///                        "logoUrl": {
///                          "type": "object",
///                          "required": [
///                            "large",
///                            "small",
///                            "thumb"
///                          ],
///                          "properties": {
///                            "large": {
///                              "type": "string"
///                            },
///                            "small": {
///                              "type": "string"
///                            },
///                            "thumb": {
///                              "type": "string"
///                            }
///                          },
///                          "additionalProperties": false
///                        },
///                        "refresh": {
///                          "type": "object",
///                          "required": [
///                            "backoffRatio",
///                            "nextRefresh",
///                            "previousAttempt"
///                          ],
///                          "properties": {
///                            "backoffRatio": {
///                              "type": "number"
///                            },
///                            "nextRefresh": {
///                              "type": "string"
///                            },
///                            "previousAttempt": {
///                              "type": "string"
///                            }
///                          },
///                          "additionalProperties": false
///                        }
///                      },
///                      "additionalProperties": false
///                    },
///                    {
///                      "type": "null"
///                    }
///                  ]
///                },
///                "metaplex": {
///                  "anyOf": [
///                    {
///                      "type": "object",
///                      "properties": {
///                        "description": {
///                          "type": "string"
///                        },
///                        "links": {
///                          "type": "array",
///                          "items": {
///                            "type": "object",
///                            "required": [
///                              "platform",
///                              "url"
///                            ],
///                            "properties": {
///                              "platform": {
///                                "type": "string"
///                              },
///                              "url": {
///                                "type": "string"
///                              }
///                            },
///                            "additionalProperties": false
///                          }
///                        },
///                        "logoUrl": {
///                          "type": "string"
///                        },
///                        "name": {
///                          "type": "string"
///                        },
///                        "refresh": {
///                          "type": "object",
///                          "required": [
///                            "backoffRatio",
///                            "nextRefresh",
///                            "previousAttempt"
///                          ],
///                          "properties": {
///                            "backoffRatio": {
///                              "type": "number"
///                            },
///                            "nextRefresh": {
///                              "type": "string"
///                            },
///                            "previousAttempt": {
///                              "type": "string"
///                            }
///                          },
///                          "additionalProperties": false
///                        },
///                        "sellerFeeBasisPoints": {
///                          "type": "number"
///                        },
///                        "symbol": {
///                          "type": "string"
///                        },
///                        "uri": {
///                          "type": "string"
///                        }
///                      },
///                      "additionalProperties": false
///                    },
///                    {
///                      "type": "null"
///                    }
///                  ]
///                },
///                "twitterProfile": {
///                  "anyOf": [
///                    {
///                      "type": "object",
///                      "properties": {
///                        "communities": {
///                          "anyOf": [
///                            {
///                              "type": "array",
///                              "items": {
///                                "type": "object",
///                                "required": [
///                                  "id",
///                                  "name"
///                                ],
///                                "properties": {
///                                  "access": {
///                                    "type": "string"
///                                  },
///                                  "createdAt": {
///                                    "type": "string"
///                                  },
///                                  "description": {
///                                    "type": "string"
///                                  },
///                                  "id": {
///                                    "type": "string"
///                                  },
///                                  "joinPolicy": {
///                                    "type": "string"
///                                  },
///                                  "memberCount": {
///                                    "type": "number"
///                                  },
///                                  "name": {
///                                    "type": "string"
///                                  }
///                                },
///                                "additionalProperties": false
///                              }
///                            },
///                            {
///                              "type": "null"
///                            }
///                          ]
///                        },
///                        "profiles": {
///                          "anyOf": [
///                            {
///                              "type": "array",
///                              "items": {
///                                "type": "object",
///                                "required": [
///                                  "handle"
///                                ],
///                                "properties": {
///                                  "bannerImage": {
///                                    "type": "string"
///                                  },
///                                  "dateJoined": {
///                                    "type": "string"
///                                  },
///                                  "description": {
///                                    "type": "string"
///                                  },
///                                  "followers": {
///                                    "type": "number"
///                                  },
///                                  "following": {
///                                    "type": "number"
///                                  },
///                                  "handle": {
///                                    "type": "string"
///                                  },
///                                  "logoImage": {
///                                    "type": "string"
///                                  },
///                                  "name": {
///                                    "type": "string"
///                                  },
///                                  "verifiedType": {
///                                    "type": "string"
///                                  }
///                                },
///                                "additionalProperties": false
///                              }
///                            },
///                            {
///                              "type": "null"
///                            }
///                          ]
///                        },
///                        "refresh": {
///                          "type": "object",
///                          "required": [
///                            "backoffRatio",
///                            "nextRefresh",
///                            "previousAttempt"
///                          ],
///                          "properties": {
///                            "backoffRatio": {
///                              "type": "number"
///                            },
///                            "nextRefresh": {
///                              "type": "string"
///                            },
///                            "previousAttempt": {
///                              "type": "string"
///                            }
///                          },
///                          "additionalProperties": false
///                        }
///                      },
///                      "additionalProperties": false
///                    },
///                    {
///                      "type": "null"
///                    }
///                  ]
///                }
///              },
///              "additionalProperties": false
///            },
///            {
///              "type": "null"
///            }
///          ]
///        },
///        "transactions": {
///          "anyOf": [
///            {
///              "type": "object",
///              "required": [
///                "buyTax",
///                "maxPerTx",
///                "maxPerWallet",
///                "refresh",
///                "sellTax",
///                "txTax"
///              ],
///              "properties": {
///                "buyTax": {
///                  "type": "number"
///                },
///                "maxPerTx": {
///                  "type": "string"
///                },
///                "maxPerWallet": {
///                  "type": "string"
///                },
///                "refresh": {
///                  "type": "object",
///                  "required": [
///                    "nextRefresh",
///                    "previousAttempt"
///                  ],
///                  "properties": {
///                    "nextRefresh": {
///                      "type": "string"
///                    },
///                    "previousAttempt": {
///                      "type": "string"
///                    }
///                  },
///                  "additionalProperties": false
///                },
///                "sellTax": {
///                  "type": "number"
///                },
///                "txTax": {
///                  "type": "number"
///                }
///              },
///              "additionalProperties": false
///            },
///            {
///              "type": "null"
///            }
///          ]
///        }
///      },
///      "additionalProperties": false
///    },
///    "summary": {
///      "type": "object",
///      "required": [
///        "athTokenPriceUsd",
///        "atlTokenPriceUsd",
///        "bestPairAddress",
///        "creationTimestamp",
///        "decimals",
///        "effectiveSupply",
///        "holderCount",
///        "marketcapUsd",
///        "marketcapUsdAth",
///        "marketcapUsdAtl",
///        "name",
///        "svmProgramId",
///        "symbol",
///        "tokenPriceUsd",
///        "totalSupply"
///      ],
///      "properties": {
///        "athTokenPriceUsd": {
///          "type": "number"
///        },
///        "atlTokenPriceUsd": {
///          "type": "number"
///        },
///        "bannerUrl": {
///          "anyOf": [
///            {
///              "type": "string"
///            },
///            {
///              "type": "null"
///            }
///          ]
///        },
///        "bestPairAddress": {
///          "anyOf": [
///            {
///              "type": "string"
///            },
///            {
///              "type": "null"
///            }
///          ]
///        },
///        "bio": {
///          "anyOf": [
///            {
///              "type": "string"
///            },
///            {
///              "type": "null"
///            }
///          ]
///        },
///        "creationTimestamp": {
///          "type": "string"
///        },
///        "decimals": {
///          "type": "number"
///        },
///        "description": {
///          "anyOf": [
///            {
///              "type": "string"
///            },
///            {
///              "type": "null"
///            }
///          ]
///        },
///        "effectiveSupply": {
///          "type": "string"
///        },
///        "holderCount": {
///          "type": "number"
///        },
///        "lastBlockSeen": {
///          "anyOf": [
///            {
///              "type": "number"
///            },
///            {
///              "type": "null"
///            }
///          ]
///        },
///        "logoUrl": {
///          "anyOf": [
///            {
///              "type": "string"
///            },
///            {
///              "type": "null"
///            }
///          ]
///        },
///        "marketcapUsd": {
///          "type": "number"
///        },
///        "marketcapUsdAth": {
///          "type": "number"
///        },
///        "marketcapUsdAtl": {
///          "type": "number"
///        },
///        "name": {
///          "type": "string"
///        },
///        "svmProgramId": {
///          "anyOf": [
///            {
///              "type": "string"
///            },
///            {
///              "type": "null"
///            }
///          ]
///        },
///        "symbol": {
///          "type": "string"
///        },
///        "tokenPriceUsd": {
///          "type": "number"
///        },
///        "totalSupply": {
///          "type": "string"
///        }
///      },
///      "additionalProperties": false
///    },
///    "tokenChainId": {
///      "type": "string"
///    },
///    "tokenContractAddress": {
///      "type": "string"
///    },
///    "tokenSymbol": {
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TokenInfoWithPricingResponseToken {
    pub details: TokenInfoWithPricingResponseTokenDetails,
    pub summary: TokenInfoWithPricingResponseTokenSummary,
    #[serde(rename = "tokenChainId")]
    pub token_chain_id: ::std::string::String,
    #[serde(rename = "tokenContractAddress")]
    pub token_contract_address: ::std::string::String,
    #[serde(rename = "tokenSymbol")]
    pub token_symbol: ::std::option::Option<::std::string::String>,
}
///`TokenInfoWithPricingResponseTokenDetails`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "contract",
///    "deployer",
///    "holderDistribution",
///    "social",
///    "transactions"
///  ],
///  "properties": {
///    "contract": {
///      "anyOf": [
///        {
///          "type": "object",
///          "required": [
///            "byteCode",
///            "sourceCodeS3Key",
///            "verified"
///          ],
///          "properties": {
///            "byteCode": {
///              "anyOf": [
///                {
///                  "type": "string"
///                },
///                {
///                  "type": "null"
///                }
///              ]
///            },
///            "refresh": {
///              "anyOf": [
///                {
///                  "type": "object",
///                  "required": [
///                    "backoffRatio",
///                    "nextRefresh",
///                    "previousAttempt"
///                  ],
///                  "properties": {
///                    "backoffRatio": {
///                      "type": "number"
///                    },
///                    "nextRefresh": {
///                      "type": "string"
///                    },
///                    "previousAttempt": {
///                      "type": "string"
///                    }
///                  },
///                  "additionalProperties": false
///                },
///                {
///                  "type": "null"
///                }
///              ]
///            },
///            "sourceCodeS3Key": {
///              "anyOf": [
///                {
///                  "type": "string"
///                },
///                {
///                  "type": "null"
///                }
///              ]
///            },
///            "verified": {
///              "anyOf": [
///                {
///                  "type": "boolean"
///                },
///                {
///                  "type": "null"
///                }
///              ]
///            }
///          },
///          "additionalProperties": false
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "deployer": {
///      "anyOf": [
///        {
///          "type": "object",
///          "required": [
///            "address",
///            "createdTx",
///            "renounced"
///          ],
///          "properties": {
///            "address": {
///              "type": "string"
///            },
///            "createdTx": {
///              "type": "string"
///            },
///            "renounced": {
///              "anyOf": [
///                {
///                  "type": "boolean"
///                },
///                {
///                  "type": "null"
///                }
///              ]
///            }
///          },
///          "additionalProperties": false
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "holderDistribution": {
///      "anyOf": [
///        {
///          "type": "object",
///          "properties": {
///            "insidersBalance": {
///              "anyOf": [
///                {
///                  "type": "string"
///                },
///                {
///                  "type": "null"
///                }
///              ]
///            },
///            "snipersBalance": {
///              "anyOf": [
///                {
///                  "type": "string"
///                },
///                {
///                  "type": "null"
///                }
///              ]
///            },
///            "top100Balance": {
///              "anyOf": [
///                {
///                  "type": "string"
///                },
///                {
///                  "type": "null"
///                }
///              ]
///            },
///            "top10Balance": {
///              "anyOf": [
///                {
///                  "type": "string"
///                },
///                {
///                  "type": "null"
///                }
///              ]
///            },
///            "top20Balance": {
///              "anyOf": [
///                {
///                  "type": "string"
///                },
///                {
///                  "type": "null"
///                }
///              ]
///            },
///            "top50Balance": {
///              "anyOf": [
///                {
///                  "type": "string"
///                },
///                {
///                  "type": "null"
///                }
///              ]
///            }
///          },
///          "additionalProperties": false
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "safetyReports": {
///      "anyOf": [
///        {
///          "type": "array",
///          "items": {
///            "type": "string"
///          }
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "social": {
///      "anyOf": [
///        {
///          "type": "object",
///          "properties": {
///            "dexscreener": {
///              "anyOf": [
///                {
///                  "type": "object",
///                  "properties": {
///                    "bannerUrl": {
///                      "type": "string"
///                    },
///                    "description": {
///                      "type": "string"
///                    },
///                    "links": {
///                      "type": "array",
///                      "items": {
///                        "type": "object",
///                        "required": [
///                          "platform",
///                          "url"
///                        ],
///                        "properties": {
///                          "platform": {
///                            "type": "string"
///                          },
///                          "url": {
///                            "type": "string"
///                          }
///                        },
///                        "additionalProperties": false
///                      }
///                    },
///                    "logoUrl": {
///                      "type": "string"
///                    },
///                    "refresh": {
///                      "type": "object",
///                      "required": [
///                        "backoffRatio",
///                        "nextRefresh",
///                        "previousAttempt"
///                      ],
///                      "properties": {
///                        "backoffRatio": {
///                          "type": "number"
///                        },
///                        "nextRefresh": {
///                          "type": "string"
///                        },
///                        "previousAttempt": {
///                          "type": "string"
///                        }
///                      },
///                      "additionalProperties": false
///                    }
///                  },
///                  "additionalProperties": false
///                },
///                {
///                  "type": "null"
///                }
///              ]
///            },
///            "dexscreenerPaid": {
///              "anyOf": [
///                {
///                  "type": "object",
///                  "required": [
///                    "orders"
///                  ],
///                  "properties": {
///                    "orders": {
///                      "type": "array",
///                      "items": {
///                        "type": "object",
///                        "required": [
///                          "paymentTimestamp",
///                          "status",
///                          "type"
///                        ],
///                        "properties": {
///                          "paymentTimestamp": {
///                            "type": "number"
///                          },
///                          "status": {
///                            "type": "string"
///                          },
///                          "type": {
///                            "type": "string"
///                          }
///                        },
///                        "additionalProperties": false
///                      }
///                    },
///                    "refresh": {
///                      "type": "object",
///                      "required": [
///                        "backoffRatio",
///                        "nextRefresh",
///                        "previousAttempt"
///                      ],
///                      "properties": {
///                        "backoffRatio": {
///                          "type": "number"
///                        },
///                        "nextRefresh": {
///                          "type": "string"
///                        },
///                        "previousAttempt": {
///                          "type": "string"
///                        }
///                      },
///                      "additionalProperties": false
///                    }
///                  },
///                  "additionalProperties": false
///                },
///                {
///                  "type": "null"
///                }
///              ]
///            },
///            "gecko": {
///              "anyOf": [
///                {
///                  "type": "object",
///                  "properties": {
///                    "description": {
///                      "anyOf": [
///                        {
///                          "type": "string"
///                        },
///                        {
///                          "type": "null"
///                        }
///                      ]
///                    },
///                    "links": {
///                      "type": "array",
///                      "items": {
///                        "type": "object",
///                        "required": [
///                          "platform",
///                          "url"
///                        ],
///                        "properties": {
///                          "platform": {
///                            "type": "string"
///                          },
///                          "url": {
///                            "type": "string"
///                          }
///                        },
///                        "additionalProperties": false
///                      }
///                    },
///                    "logoUrl": {
///                      "type": "object",
///                      "required": [
///                        "large",
///                        "small",
///                        "thumb"
///                      ],
///                      "properties": {
///                        "large": {
///                          "type": "string"
///                        },
///                        "small": {
///                          "type": "string"
///                        },
///                        "thumb": {
///                          "type": "string"
///                        }
///                      },
///                      "additionalProperties": false
///                    },
///                    "refresh": {
///                      "type": "object",
///                      "required": [
///                        "backoffRatio",
///                        "nextRefresh",
///                        "previousAttempt"
///                      ],
///                      "properties": {
///                        "backoffRatio": {
///                          "type": "number"
///                        },
///                        "nextRefresh": {
///                          "type": "string"
///                        },
///                        "previousAttempt": {
///                          "type": "string"
///                        }
///                      },
///                      "additionalProperties": false
///                    }
///                  },
///                  "additionalProperties": false
///                },
///                {
///                  "type": "null"
///                }
///              ]
///            },
///            "metaplex": {
///              "anyOf": [
///                {
///                  "type": "object",
///                  "properties": {
///                    "description": {
///                      "type": "string"
///                    },
///                    "links": {
///                      "type": "array",
///                      "items": {
///                        "type": "object",
///                        "required": [
///                          "platform",
///                          "url"
///                        ],
///                        "properties": {
///                          "platform": {
///                            "type": "string"
///                          },
///                          "url": {
///                            "type": "string"
///                          }
///                        },
///                        "additionalProperties": false
///                      }
///                    },
///                    "logoUrl": {
///                      "type": "string"
///                    },
///                    "name": {
///                      "type": "string"
///                    },
///                    "refresh": {
///                      "type": "object",
///                      "required": [
///                        "backoffRatio",
///                        "nextRefresh",
///                        "previousAttempt"
///                      ],
///                      "properties": {
///                        "backoffRatio": {
///                          "type": "number"
///                        },
///                        "nextRefresh": {
///                          "type": "string"
///                        },
///                        "previousAttempt": {
///                          "type": "string"
///                        }
///                      },
///                      "additionalProperties": false
///                    },
///                    "sellerFeeBasisPoints": {
///                      "type": "number"
///                    },
///                    "symbol": {
///                      "type": "string"
///                    },
///                    "uri": {
///                      "type": "string"
///                    }
///                  },
///                  "additionalProperties": false
///                },
///                {
///                  "type": "null"
///                }
///              ]
///            },
///            "twitterProfile": {
///              "anyOf": [
///                {
///                  "type": "object",
///                  "properties": {
///                    "communities": {
///                      "anyOf": [
///                        {
///                          "type": "array",
///                          "items": {
///                            "type": "object",
///                            "required": [
///                              "id",
///                              "name"
///                            ],
///                            "properties": {
///                              "access": {
///                                "type": "string"
///                              },
///                              "createdAt": {
///                                "type": "string"
///                              },
///                              "description": {
///                                "type": "string"
///                              },
///                              "id": {
///                                "type": "string"
///                              },
///                              "joinPolicy": {
///                                "type": "string"
///                              },
///                              "memberCount": {
///                                "type": "number"
///                              },
///                              "name": {
///                                "type": "string"
///                              }
///                            },
///                            "additionalProperties": false
///                          }
///                        },
///                        {
///                          "type": "null"
///                        }
///                      ]
///                    },
///                    "profiles": {
///                      "anyOf": [
///                        {
///                          "type": "array",
///                          "items": {
///                            "type": "object",
///                            "required": [
///                              "handle"
///                            ],
///                            "properties": {
///                              "bannerImage": {
///                                "type": "string"
///                              },
///                              "dateJoined": {
///                                "type": "string"
///                              },
///                              "description": {
///                                "type": "string"
///                              },
///                              "followers": {
///                                "type": "number"
///                              },
///                              "following": {
///                                "type": "number"
///                              },
///                              "handle": {
///                                "type": "string"
///                              },
///                              "logoImage": {
///                                "type": "string"
///                              },
///                              "name": {
///                                "type": "string"
///                              },
///                              "verifiedType": {
///                                "type": "string"
///                              }
///                            },
///                            "additionalProperties": false
///                          }
///                        },
///                        {
///                          "type": "null"
///                        }
///                      ]
///                    },
///                    "refresh": {
///                      "type": "object",
///                      "required": [
///                        "backoffRatio",
///                        "nextRefresh",
///                        "previousAttempt"
///                      ],
///                      "properties": {
///                        "backoffRatio": {
///                          "type": "number"
///                        },
///                        "nextRefresh": {
///                          "type": "string"
///                        },
///                        "previousAttempt": {
///                          "type": "string"
///                        }
///                      },
///                      "additionalProperties": false
///                    }
///                  },
///                  "additionalProperties": false
///                },
///                {
///                  "type": "null"
///                }
///              ]
///            }
///          },
///          "additionalProperties": false
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "transactions": {
///      "anyOf": [
///        {
///          "type": "object",
///          "required": [
///            "buyTax",
///            "maxPerTx",
///            "maxPerWallet",
///            "refresh",
///            "sellTax",
///            "txTax"
///          ],
///          "properties": {
///            "buyTax": {
///              "type": "number"
///            },
///            "maxPerTx": {
///              "type": "string"
///            },
///            "maxPerWallet": {
///              "type": "string"
///            },
///            "refresh": {
///              "type": "object",
///              "required": [
///                "nextRefresh",
///                "previousAttempt"
///              ],
///              "properties": {
///                "nextRefresh": {
///                  "type": "string"
///                },
///                "previousAttempt": {
///                  "type": "string"
///                }
///              },
///              "additionalProperties": false
///            },
///            "sellTax": {
///              "type": "number"
///            },
///            "txTax": {
///              "type": "number"
///            }
///          },
///          "additionalProperties": false
///        },
///        {
///          "type": "null"
///        }
///      ]
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TokenInfoWithPricingResponseTokenDetails {
    pub contract: ::std::option::Option<TokenInfoWithPricingResponseTokenDetailsContract>,
    pub deployer: ::std::option::Option<TokenInfoWithPricingResponseTokenDetailsDeployer>,
    #[serde(rename = "holderDistribution")]
    pub holder_distribution: ::std::option::Option<TokenInfoWithPricingResponseTokenDetailsHolderDistribution>,
    #[serde(
        rename = "safetyReports",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub safety_reports: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub social: ::std::option::Option<TokenInfoWithPricingResponseTokenDetailsSocial>,
    pub transactions: ::std::option::Option<TokenInfoWithPricingResponseTokenDetailsTransactions>,
}
///`TokenInfoWithPricingResponseTokenDetailsContract`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "byteCode",
///    "sourceCodeS3Key",
///    "verified"
///  ],
///  "properties": {
///    "byteCode": {
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "refresh": {
///      "anyOf": [
///        {
///          "type": "object",
///          "required": [
///            "backoffRatio",
///            "nextRefresh",
///            "previousAttempt"
///          ],
///          "properties": {
///            "backoffRatio": {
///              "type": "number"
///            },
///            "nextRefresh": {
///              "type": "string"
///            },
///            "previousAttempt": {
///              "type": "string"
///            }
///          },
///          "additionalProperties": false
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "sourceCodeS3Key": {
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "verified": {
///      "anyOf": [
///        {
///          "type": "boolean"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TokenInfoWithPricingResponseTokenDetailsContract {
    #[serde(rename = "byteCode")]
    pub byte_code: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub refresh: ::std::option::Option<TokenInfoWithPricingResponseTokenDetailsContractRefresh>,
    #[serde(rename = "sourceCodeS3Key")]
    pub source_code_s3_key: ::std::option::Option<::std::string::String>,
    pub verified: ::std::option::Option<bool>,
}
///`TokenInfoWithPricingResponseTokenDetailsContractRefresh`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "backoffRatio",
///    "nextRefresh",
///    "previousAttempt"
///  ],
///  "properties": {
///    "backoffRatio": {
///      "type": "number"
///    },
///    "nextRefresh": {
///      "type": "string"
///    },
///    "previousAttempt": {
///      "type": "string"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TokenInfoWithPricingResponseTokenDetailsContractRefresh {
    #[serde(rename = "backoffRatio")]
    pub backoff_ratio: f64,
    #[serde(rename = "nextRefresh")]
    pub next_refresh: ::std::string::String,
    #[serde(rename = "previousAttempt")]
    pub previous_attempt: ::std::string::String,
}
///`TokenInfoWithPricingResponseTokenDetailsDeployer`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "address",
///    "createdTx",
///    "renounced"
///  ],
///  "properties": {
///    "address": {
///      "type": "string"
///    },
///    "createdTx": {
///      "type": "string"
///    },
///    "renounced": {
///      "anyOf": [
///        {
///          "type": "boolean"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TokenInfoWithPricingResponseTokenDetailsDeployer {
    pub address: ::std::string::String,
    #[serde(rename = "createdTx")]
    pub created_tx: ::std::string::String,
    pub renounced: ::std::option::Option<bool>,
}
///`TokenInfoWithPricingResponseTokenDetailsHolderDistribution`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "insidersBalance": {
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "snipersBalance": {
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "top100Balance": {
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "top10Balance": {
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "top20Balance": {
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "top50Balance": {
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TokenInfoWithPricingResponseTokenDetailsHolderDistribution {
    #[serde(
        rename = "insidersBalance",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub insiders_balance: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "snipersBalance",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub snipers_balance: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "top100Balance",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub top100_balance: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "top10Balance",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub top10_balance: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "top20Balance",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub top20_balance: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "top50Balance",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub top50_balance: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for TokenInfoWithPricingResponseTokenDetailsHolderDistribution {
    fn default() -> Self {
        Self {
            insiders_balance: Default::default(),
            snipers_balance: Default::default(),
            top100_balance: Default::default(),
            top10_balance: Default::default(),
            top20_balance: Default::default(),
            top50_balance: Default::default(),
        }
    }
}
///`TokenInfoWithPricingResponseTokenDetailsSocial`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "dexscreener": {
///      "anyOf": [
///        {
///          "type": "object",
///          "properties": {
///            "bannerUrl": {
///              "type": "string"
///            },
///            "description": {
///              "type": "string"
///            },
///            "links": {
///              "type": "array",
///              "items": {
///                "type": "object",
///                "required": [
///                  "platform",
///                  "url"
///                ],
///                "properties": {
///                  "platform": {
///                    "type": "string"
///                  },
///                  "url": {
///                    "type": "string"
///                  }
///                },
///                "additionalProperties": false
///              }
///            },
///            "logoUrl": {
///              "type": "string"
///            },
///            "refresh": {
///              "type": "object",
///              "required": [
///                "backoffRatio",
///                "nextRefresh",
///                "previousAttempt"
///              ],
///              "properties": {
///                "backoffRatio": {
///                  "type": "number"
///                },
///                "nextRefresh": {
///                  "type": "string"
///                },
///                "previousAttempt": {
///                  "type": "string"
///                }
///              },
///              "additionalProperties": false
///            }
///          },
///          "additionalProperties": false
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "dexscreenerPaid": {
///      "anyOf": [
///        {
///          "type": "object",
///          "required": [
///            "orders"
///          ],
///          "properties": {
///            "orders": {
///              "type": "array",
///              "items": {
///                "type": "object",
///                "required": [
///                  "paymentTimestamp",
///                  "status",
///                  "type"
///                ],
///                "properties": {
///                  "paymentTimestamp": {
///                    "type": "number"
///                  },
///                  "status": {
///                    "type": "string"
///                  },
///                  "type": {
///                    "type": "string"
///                  }
///                },
///                "additionalProperties": false
///              }
///            },
///            "refresh": {
///              "type": "object",
///              "required": [
///                "backoffRatio",
///                "nextRefresh",
///                "previousAttempt"
///              ],
///              "properties": {
///                "backoffRatio": {
///                  "type": "number"
///                },
///                "nextRefresh": {
///                  "type": "string"
///                },
///                "previousAttempt": {
///                  "type": "string"
///                }
///              },
///              "additionalProperties": false
///            }
///          },
///          "additionalProperties": false
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "gecko": {
///      "anyOf": [
///        {
///          "type": "object",
///          "properties": {
///            "description": {
///              "anyOf": [
///                {
///                  "type": "string"
///                },
///                {
///                  "type": "null"
///                }
///              ]
///            },
///            "links": {
///              "type": "array",
///              "items": {
///                "type": "object",
///                "required": [
///                  "platform",
///                  "url"
///                ],
///                "properties": {
///                  "platform": {
///                    "type": "string"
///                  },
///                  "url": {
///                    "type": "string"
///                  }
///                },
///                "additionalProperties": false
///              }
///            },
///            "logoUrl": {
///              "type": "object",
///              "required": [
///                "large",
///                "small",
///                "thumb"
///              ],
///              "properties": {
///                "large": {
///                  "type": "string"
///                },
///                "small": {
///                  "type": "string"
///                },
///                "thumb": {
///                  "type": "string"
///                }
///              },
///              "additionalProperties": false
///            },
///            "refresh": {
///              "type": "object",
///              "required": [
///                "backoffRatio",
///                "nextRefresh",
///                "previousAttempt"
///              ],
///              "properties": {
///                "backoffRatio": {
///                  "type": "number"
///                },
///                "nextRefresh": {
///                  "type": "string"
///                },
///                "previousAttempt": {
///                  "type": "string"
///                }
///              },
///              "additionalProperties": false
///            }
///          },
///          "additionalProperties": false
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "metaplex": {
///      "anyOf": [
///        {
///          "type": "object",
///          "properties": {
///            "description": {
///              "type": "string"
///            },
///            "links": {
///              "type": "array",
///              "items": {
///                "type": "object",
///                "required": [
///                  "platform",
///                  "url"
///                ],
///                "properties": {
///                  "platform": {
///                    "type": "string"
///                  },
///                  "url": {
///                    "type": "string"
///                  }
///                },
///                "additionalProperties": false
///              }
///            },
///            "logoUrl": {
///              "type": "string"
///            },
///            "name": {
///              "type": "string"
///            },
///            "refresh": {
///              "type": "object",
///              "required": [
///                "backoffRatio",
///                "nextRefresh",
///                "previousAttempt"
///              ],
///              "properties": {
///                "backoffRatio": {
///                  "type": "number"
///                },
///                "nextRefresh": {
///                  "type": "string"
///                },
///                "previousAttempt": {
///                  "type": "string"
///                }
///              },
///              "additionalProperties": false
///            },
///            "sellerFeeBasisPoints": {
///              "type": "number"
///            },
///            "symbol": {
///              "type": "string"
///            },
///            "uri": {
///              "type": "string"
///            }
///          },
///          "additionalProperties": false
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "twitterProfile": {
///      "anyOf": [
///        {
///          "type": "object",
///          "properties": {
///            "communities": {
///              "anyOf": [
///                {
///                  "type": "array",
///                  "items": {
///                    "type": "object",
///                    "required": [
///                      "id",
///                      "name"
///                    ],
///                    "properties": {
///                      "access": {
///                        "type": "string"
///                      },
///                      "createdAt": {
///                        "type": "string"
///                      },
///                      "description": {
///                        "type": "string"
///                      },
///                      "id": {
///                        "type": "string"
///                      },
///                      "joinPolicy": {
///                        "type": "string"
///                      },
///                      "memberCount": {
///                        "type": "number"
///                      },
///                      "name": {
///                        "type": "string"
///                      }
///                    },
///                    "additionalProperties": false
///                  }
///                },
///                {
///                  "type": "null"
///                }
///              ]
///            },
///            "profiles": {
///              "anyOf": [
///                {
///                  "type": "array",
///                  "items": {
///                    "type": "object",
///                    "required": [
///                      "handle"
///                    ],
///                    "properties": {
///                      "bannerImage": {
///                        "type": "string"
///                      },
///                      "dateJoined": {
///                        "type": "string"
///                      },
///                      "description": {
///                        "type": "string"
///                      },
///                      "followers": {
///                        "type": "number"
///                      },
///                      "following": {
///                        "type": "number"
///                      },
///                      "handle": {
///                        "type": "string"
///                      },
///                      "logoImage": {
///                        "type": "string"
///                      },
///                      "name": {
///                        "type": "string"
///                      },
///                      "verifiedType": {
///                        "type": "string"
///                      }
///                    },
///                    "additionalProperties": false
///                  }
///                },
///                {
///                  "type": "null"
///                }
///              ]
///            },
///            "refresh": {
///              "type": "object",
///              "required": [
///                "backoffRatio",
///                "nextRefresh",
///                "previousAttempt"
///              ],
///              "properties": {
///                "backoffRatio": {
///                  "type": "number"
///                },
///                "nextRefresh": {
///                  "type": "string"
///                },
///                "previousAttempt": {
///                  "type": "string"
///                }
///              },
///              "additionalProperties": false
///            }
///          },
///          "additionalProperties": false
///        },
///        {
///          "type": "null"
///        }
///      ]
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TokenInfoWithPricingResponseTokenDetailsSocial {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub dexscreener: ::std::option::Option<TokenInfoWithPricingResponseTokenDetailsSocialDexscreener>,
    #[serde(
        rename = "dexscreenerPaid",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub dexscreener_paid: ::std::option::Option<TokenInfoWithPricingResponseTokenDetailsSocialDexscreenerPaid>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub gecko: ::std::option::Option<TokenInfoWithPricingResponseTokenDetailsSocialGecko>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub metaplex: ::std::option::Option<TokenInfoWithPricingResponseTokenDetailsSocialMetaplex>,
    #[serde(
        rename = "twitterProfile",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub twitter_profile: ::std::option::Option<TokenInfoWithPricingResponseTokenDetailsSocialTwitterProfile>,
}
impl ::std::default::Default for TokenInfoWithPricingResponseTokenDetailsSocial {
    fn default() -> Self {
        Self {
            dexscreener: Default::default(),
            dexscreener_paid: Default::default(),
            gecko: Default::default(),
            metaplex: Default::default(),
            twitter_profile: Default::default(),
        }
    }
}
///`TokenInfoWithPricingResponseTokenDetailsSocialDexscreener`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "bannerUrl": {
///      "type": "string"
///    },
///    "description": {
///      "type": "string"
///    },
///    "links": {
///      "type": "array",
///      "items": {
///        "type": "object",
///        "required": [
///          "platform",
///          "url"
///        ],
///        "properties": {
///          "platform": {
///            "type": "string"
///          },
///          "url": {
///            "type": "string"
///          }
///        },
///        "additionalProperties": false
///      }
///    },
///    "logoUrl": {
///      "type": "string"
///    },
///    "refresh": {
///      "type": "object",
///      "required": [
///        "backoffRatio",
///        "nextRefresh",
///        "previousAttempt"
///      ],
///      "properties": {
///        "backoffRatio": {
///          "type": "number"
///        },
///        "nextRefresh": {
///          "type": "string"
///        },
///        "previousAttempt": {
///          "type": "string"
///        }
///      },
///      "additionalProperties": false
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TokenInfoWithPricingResponseTokenDetailsSocialDexscreener {
    #[serde(
        rename = "bannerUrl",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub banner_url: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub links: ::std::vec::Vec<TokenInfoWithPricingResponseTokenDetailsSocialDexscreenerLinksItem>,
    #[serde(rename = "logoUrl", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub logo_url: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub refresh: ::std::option::Option<TokenInfoWithPricingResponseTokenDetailsSocialDexscreenerRefresh>,
}
impl ::std::default::Default for TokenInfoWithPricingResponseTokenDetailsSocialDexscreener {
    fn default() -> Self {
        Self {
            banner_url: Default::default(),
            description: Default::default(),
            links: Default::default(),
            logo_url: Default::default(),
            refresh: Default::default(),
        }
    }
}
///`TokenInfoWithPricingResponseTokenDetailsSocialDexscreenerLinksItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "platform",
///    "url"
///  ],
///  "properties": {
///    "platform": {
///      "type": "string"
///    },
///    "url": {
///      "type": "string"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TokenInfoWithPricingResponseTokenDetailsSocialDexscreenerLinksItem {
    pub platform: ::std::string::String,
    pub url: ::std::string::String,
}
///`TokenInfoWithPricingResponseTokenDetailsSocialDexscreenerPaid`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "orders"
///  ],
///  "properties": {
///    "orders": {
///      "type": "array",
///      "items": {
///        "type": "object",
///        "required": [
///          "paymentTimestamp",
///          "status",
///          "type"
///        ],
///        "properties": {
///          "paymentTimestamp": {
///            "type": "number"
///          },
///          "status": {
///            "type": "string"
///          },
///          "type": {
///            "type": "string"
///          }
///        },
///        "additionalProperties": false
///      }
///    },
///    "refresh": {
///      "type": "object",
///      "required": [
///        "backoffRatio",
///        "nextRefresh",
///        "previousAttempt"
///      ],
///      "properties": {
///        "backoffRatio": {
///          "type": "number"
///        },
///        "nextRefresh": {
///          "type": "string"
///        },
///        "previousAttempt": {
///          "type": "string"
///        }
///      },
///      "additionalProperties": false
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TokenInfoWithPricingResponseTokenDetailsSocialDexscreenerPaid {
    pub orders: ::std::vec::Vec<TokenInfoWithPricingResponseTokenDetailsSocialDexscreenerPaidOrdersItem>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub refresh: ::std::option::Option<TokenInfoWithPricingResponseTokenDetailsSocialDexscreenerPaidRefresh>,
}
///`TokenInfoWithPricingResponseTokenDetailsSocialDexscreenerPaidOrdersItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "paymentTimestamp",
///    "status",
///    "type"
///  ],
///  "properties": {
///    "paymentTimestamp": {
///      "type": "number"
///    },
///    "status": {
///      "type": "string"
///    },
///    "type": {
///      "type": "string"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TokenInfoWithPricingResponseTokenDetailsSocialDexscreenerPaidOrdersItem {
    #[serde(rename = "paymentTimestamp")]
    pub payment_timestamp: f64,
    pub status: ::std::string::String,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
///`TokenInfoWithPricingResponseTokenDetailsSocialDexscreenerPaidRefresh`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "backoffRatio",
///    "nextRefresh",
///    "previousAttempt"
///  ],
///  "properties": {
///    "backoffRatio": {
///      "type": "number"
///    },
///    "nextRefresh": {
///      "type": "string"
///    },
///    "previousAttempt": {
///      "type": "string"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TokenInfoWithPricingResponseTokenDetailsSocialDexscreenerPaidRefresh {
    #[serde(rename = "backoffRatio")]
    pub backoff_ratio: f64,
    #[serde(rename = "nextRefresh")]
    pub next_refresh: ::std::string::String,
    #[serde(rename = "previousAttempt")]
    pub previous_attempt: ::std::string::String,
}
///`TokenInfoWithPricingResponseTokenDetailsSocialDexscreenerRefresh`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "backoffRatio",
///    "nextRefresh",
///    "previousAttempt"
///  ],
///  "properties": {
///    "backoffRatio": {
///      "type": "number"
///    },
///    "nextRefresh": {
///      "type": "string"
///    },
///    "previousAttempt": {
///      "type": "string"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TokenInfoWithPricingResponseTokenDetailsSocialDexscreenerRefresh {
    #[serde(rename = "backoffRatio")]
    pub backoff_ratio: f64,
    #[serde(rename = "nextRefresh")]
    pub next_refresh: ::std::string::String,
    #[serde(rename = "previousAttempt")]
    pub previous_attempt: ::std::string::String,
}
///`TokenInfoWithPricingResponseTokenDetailsSocialGecko`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "description": {
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "links": {
///      "type": "array",
///      "items": {
///        "type": "object",
///        "required": [
///          "platform",
///          "url"
///        ],
///        "properties": {
///          "platform": {
///            "type": "string"
///          },
///          "url": {
///            "type": "string"
///          }
///        },
///        "additionalProperties": false
///      }
///    },
///    "logoUrl": {
///      "type": "object",
///      "required": [
///        "large",
///        "small",
///        "thumb"
///      ],
///      "properties": {
///        "large": {
///          "type": "string"
///        },
///        "small": {
///          "type": "string"
///        },
///        "thumb": {
///          "type": "string"
///        }
///      },
///      "additionalProperties": false
///    },
///    "refresh": {
///      "type": "object",
///      "required": [
///        "backoffRatio",
///        "nextRefresh",
///        "previousAttempt"
///      ],
///      "properties": {
///        "backoffRatio": {
///          "type": "number"
///        },
///        "nextRefresh": {
///          "type": "string"
///        },
///        "previousAttempt": {
///          "type": "string"
///        }
///      },
///      "additionalProperties": false
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TokenInfoWithPricingResponseTokenDetailsSocialGecko {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub links: ::std::vec::Vec<TokenInfoWithPricingResponseTokenDetailsSocialGeckoLinksItem>,
    #[serde(rename = "logoUrl", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub logo_url: ::std::option::Option<TokenInfoWithPricingResponseTokenDetailsSocialGeckoLogoUrl>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub refresh: ::std::option::Option<TokenInfoWithPricingResponseTokenDetailsSocialGeckoRefresh>,
}
impl ::std::default::Default for TokenInfoWithPricingResponseTokenDetailsSocialGecko {
    fn default() -> Self {
        Self {
            description: Default::default(),
            links: Default::default(),
            logo_url: Default::default(),
            refresh: Default::default(),
        }
    }
}
///`TokenInfoWithPricingResponseTokenDetailsSocialGeckoLinksItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "platform",
///    "url"
///  ],
///  "properties": {
///    "platform": {
///      "type": "string"
///    },
///    "url": {
///      "type": "string"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TokenInfoWithPricingResponseTokenDetailsSocialGeckoLinksItem {
    pub platform: ::std::string::String,
    pub url: ::std::string::String,
}
///`TokenInfoWithPricingResponseTokenDetailsSocialGeckoLogoUrl`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "large",
///    "small",
///    "thumb"
///  ],
///  "properties": {
///    "large": {
///      "type": "string"
///    },
///    "small": {
///      "type": "string"
///    },
///    "thumb": {
///      "type": "string"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TokenInfoWithPricingResponseTokenDetailsSocialGeckoLogoUrl {
    pub large: ::std::string::String,
    pub small: ::std::string::String,
    pub thumb: ::std::string::String,
}
///`TokenInfoWithPricingResponseTokenDetailsSocialGeckoRefresh`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "backoffRatio",
///    "nextRefresh",
///    "previousAttempt"
///  ],
///  "properties": {
///    "backoffRatio": {
///      "type": "number"
///    },
///    "nextRefresh": {
///      "type": "string"
///    },
///    "previousAttempt": {
///      "type": "string"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TokenInfoWithPricingResponseTokenDetailsSocialGeckoRefresh {
    #[serde(rename = "backoffRatio")]
    pub backoff_ratio: f64,
    #[serde(rename = "nextRefresh")]
    pub next_refresh: ::std::string::String,
    #[serde(rename = "previousAttempt")]
    pub previous_attempt: ::std::string::String,
}
///`TokenInfoWithPricingResponseTokenDetailsSocialMetaplex`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "description": {
///      "type": "string"
///    },
///    "links": {
///      "type": "array",
///      "items": {
///        "type": "object",
///        "required": [
///          "platform",
///          "url"
///        ],
///        "properties": {
///          "platform": {
///            "type": "string"
///          },
///          "url": {
///            "type": "string"
///          }
///        },
///        "additionalProperties": false
///      }
///    },
///    "logoUrl": {
///      "type": "string"
///    },
///    "name": {
///      "type": "string"
///    },
///    "refresh": {
///      "type": "object",
///      "required": [
///        "backoffRatio",
///        "nextRefresh",
///        "previousAttempt"
///      ],
///      "properties": {
///        "backoffRatio": {
///          "type": "number"
///        },
///        "nextRefresh": {
///          "type": "string"
///        },
///        "previousAttempt": {
///          "type": "string"
///        }
///      },
///      "additionalProperties": false
///    },
///    "sellerFeeBasisPoints": {
///      "type": "number"
///    },
///    "symbol": {
///      "type": "string"
///    },
///    "uri": {
///      "type": "string"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TokenInfoWithPricingResponseTokenDetailsSocialMetaplex {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub links: ::std::vec::Vec<TokenInfoWithPricingResponseTokenDetailsSocialMetaplexLinksItem>,
    #[serde(rename = "logoUrl", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub logo_url: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub refresh: ::std::option::Option<TokenInfoWithPricingResponseTokenDetailsSocialMetaplexRefresh>,
    #[serde(
        rename = "sellerFeeBasisPoints",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub seller_fee_basis_points: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub symbol: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub uri: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for TokenInfoWithPricingResponseTokenDetailsSocialMetaplex {
    fn default() -> Self {
        Self {
            description: Default::default(),
            links: Default::default(),
            logo_url: Default::default(),
            name: Default::default(),
            refresh: Default::default(),
            seller_fee_basis_points: Default::default(),
            symbol: Default::default(),
            uri: Default::default(),
        }
    }
}
///`TokenInfoWithPricingResponseTokenDetailsSocialMetaplexLinksItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "platform",
///    "url"
///  ],
///  "properties": {
///    "platform": {
///      "type": "string"
///    },
///    "url": {
///      "type": "string"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TokenInfoWithPricingResponseTokenDetailsSocialMetaplexLinksItem {
    pub platform: ::std::string::String,
    pub url: ::std::string::String,
}
///`TokenInfoWithPricingResponseTokenDetailsSocialMetaplexRefresh`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "backoffRatio",
///    "nextRefresh",
///    "previousAttempt"
///  ],
///  "properties": {
///    "backoffRatio": {
///      "type": "number"
///    },
///    "nextRefresh": {
///      "type": "string"
///    },
///    "previousAttempt": {
///      "type": "string"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TokenInfoWithPricingResponseTokenDetailsSocialMetaplexRefresh {
    #[serde(rename = "backoffRatio")]
    pub backoff_ratio: f64,
    #[serde(rename = "nextRefresh")]
    pub next_refresh: ::std::string::String,
    #[serde(rename = "previousAttempt")]
    pub previous_attempt: ::std::string::String,
}
///`TokenInfoWithPricingResponseTokenDetailsSocialTwitterProfile`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "communities": {
///      "anyOf": [
///        {
///          "type": "array",
///          "items": {
///            "type": "object",
///            "required": [
///              "id",
///              "name"
///            ],
///            "properties": {
///              "access": {
///                "type": "string"
///              },
///              "createdAt": {
///                "type": "string"
///              },
///              "description": {
///                "type": "string"
///              },
///              "id": {
///                "type": "string"
///              },
///              "joinPolicy": {
///                "type": "string"
///              },
///              "memberCount": {
///                "type": "number"
///              },
///              "name": {
///                "type": "string"
///              }
///            },
///            "additionalProperties": false
///          }
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "profiles": {
///      "anyOf": [
///        {
///          "type": "array",
///          "items": {
///            "type": "object",
///            "required": [
///              "handle"
///            ],
///            "properties": {
///              "bannerImage": {
///                "type": "string"
///              },
///              "dateJoined": {
///                "type": "string"
///              },
///              "description": {
///                "type": "string"
///              },
///              "followers": {
///                "type": "number"
///              },
///              "following": {
///                "type": "number"
///              },
///              "handle": {
///                "type": "string"
///              },
///              "logoImage": {
///                "type": "string"
///              },
///              "name": {
///                "type": "string"
///              },
///              "verifiedType": {
///                "type": "string"
///              }
///            },
///            "additionalProperties": false
///          }
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "refresh": {
///      "type": "object",
///      "required": [
///        "backoffRatio",
///        "nextRefresh",
///        "previousAttempt"
///      ],
///      "properties": {
///        "backoffRatio": {
///          "type": "number"
///        },
///        "nextRefresh": {
///          "type": "string"
///        },
///        "previousAttempt": {
///          "type": "string"
///        }
///      },
///      "additionalProperties": false
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TokenInfoWithPricingResponseTokenDetailsSocialTwitterProfile {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub communities: ::std::option::Option<
        ::std::vec::Vec<TokenInfoWithPricingResponseTokenDetailsSocialTwitterProfileCommunitiesItem>,
    >,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub profiles: ::std::option::Option<
        ::std::vec::Vec<TokenInfoWithPricingResponseTokenDetailsSocialTwitterProfileProfilesItem>,
    >,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub refresh: ::std::option::Option<TokenInfoWithPricingResponseTokenDetailsSocialTwitterProfileRefresh>,
}
impl ::std::default::Default for TokenInfoWithPricingResponseTokenDetailsSocialTwitterProfile {
    fn default() -> Self {
        Self {
            communities: Default::default(),
            profiles: Default::default(),
            refresh: Default::default(),
        }
    }
}
///`TokenInfoWithPricingResponseTokenDetailsSocialTwitterProfileCommunitiesItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "id",
///    "name"
///  ],
///  "properties": {
///    "access": {
///      "type": "string"
///    },
///    "createdAt": {
///      "type": "string"
///    },
///    "description": {
///      "type": "string"
///    },
///    "id": {
///      "type": "string"
///    },
///    "joinPolicy": {
///      "type": "string"
///    },
///    "memberCount": {
///      "type": "number"
///    },
///    "name": {
///      "type": "string"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TokenInfoWithPricingResponseTokenDetailsSocialTwitterProfileCommunitiesItem {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub access: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "createdAt",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub created_at: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    pub id: ::std::string::String,
    #[serde(
        rename = "joinPolicy",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub join_policy: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "memberCount",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub member_count: ::std::option::Option<f64>,
    pub name: ::std::string::String,
}
///`TokenInfoWithPricingResponseTokenDetailsSocialTwitterProfileProfilesItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "handle"
///  ],
///  "properties": {
///    "bannerImage": {
///      "type": "string"
///    },
///    "dateJoined": {
///      "type": "string"
///    },
///    "description": {
///      "type": "string"
///    },
///    "followers": {
///      "type": "number"
///    },
///    "following": {
///      "type": "number"
///    },
///    "handle": {
///      "type": "string"
///    },
///    "logoImage": {
///      "type": "string"
///    },
///    "name": {
///      "type": "string"
///    },
///    "verifiedType": {
///      "type": "string"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TokenInfoWithPricingResponseTokenDetailsSocialTwitterProfileProfilesItem {
    #[serde(
        rename = "bannerImage",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub banner_image: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "dateJoined",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub date_joined: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub followers: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub following: ::std::option::Option<f64>,
    pub handle: ::std::string::String,
    #[serde(
        rename = "logoImage",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub logo_image: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "verifiedType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub verified_type: ::std::option::Option<::std::string::String>,
}
///`TokenInfoWithPricingResponseTokenDetailsSocialTwitterProfileRefresh`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "backoffRatio",
///    "nextRefresh",
///    "previousAttempt"
///  ],
///  "properties": {
///    "backoffRatio": {
///      "type": "number"
///    },
///    "nextRefresh": {
///      "type": "string"
///    },
///    "previousAttempt": {
///      "type": "string"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TokenInfoWithPricingResponseTokenDetailsSocialTwitterProfileRefresh {
    #[serde(rename = "backoffRatio")]
    pub backoff_ratio: f64,
    #[serde(rename = "nextRefresh")]
    pub next_refresh: ::std::string::String,
    #[serde(rename = "previousAttempt")]
    pub previous_attempt: ::std::string::String,
}
///`TokenInfoWithPricingResponseTokenDetailsTransactions`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "buyTax",
///    "maxPerTx",
///    "maxPerWallet",
///    "refresh",
///    "sellTax",
///    "txTax"
///  ],
///  "properties": {
///    "buyTax": {
///      "type": "number"
///    },
///    "maxPerTx": {
///      "type": "string"
///    },
///    "maxPerWallet": {
///      "type": "string"
///    },
///    "refresh": {
///      "type": "object",
///      "required": [
///        "nextRefresh",
///        "previousAttempt"
///      ],
///      "properties": {
///        "nextRefresh": {
///          "type": "string"
///        },
///        "previousAttempt": {
///          "type": "string"
///        }
///      },
///      "additionalProperties": false
///    },
///    "sellTax": {
///      "type": "number"
///    },
///    "txTax": {
///      "type": "number"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TokenInfoWithPricingResponseTokenDetailsTransactions {
    #[serde(rename = "buyTax")]
    pub buy_tax: f64,
    #[serde(rename = "maxPerTx")]
    pub max_per_tx: ::std::string::String,
    #[serde(rename = "maxPerWallet")]
    pub max_per_wallet: ::std::string::String,
    pub refresh: TokenInfoWithPricingResponseTokenDetailsTransactionsRefresh,
    #[serde(rename = "sellTax")]
    pub sell_tax: f64,
    #[serde(rename = "txTax")]
    pub tx_tax: f64,
}
///`TokenInfoWithPricingResponseTokenDetailsTransactionsRefresh`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "nextRefresh",
///    "previousAttempt"
///  ],
///  "properties": {
///    "nextRefresh": {
///      "type": "string"
///    },
///    "previousAttempt": {
///      "type": "string"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TokenInfoWithPricingResponseTokenDetailsTransactionsRefresh {
    #[serde(rename = "nextRefresh")]
    pub next_refresh: ::std::string::String,
    #[serde(rename = "previousAttempt")]
    pub previous_attempt: ::std::string::String,
}
///`TokenInfoWithPricingResponseTokenSummary`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "athTokenPriceUsd",
///    "atlTokenPriceUsd",
///    "bestPairAddress",
///    "creationTimestamp",
///    "decimals",
///    "effectiveSupply",
///    "holderCount",
///    "marketcapUsd",
///    "marketcapUsdAth",
///    "marketcapUsdAtl",
///    "name",
///    "svmProgramId",
///    "symbol",
///    "tokenPriceUsd",
///    "totalSupply"
///  ],
///  "properties": {
///    "athTokenPriceUsd": {
///      "type": "number"
///    },
///    "atlTokenPriceUsd": {
///      "type": "number"
///    },
///    "bannerUrl": {
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "bestPairAddress": {
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "bio": {
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "creationTimestamp": {
///      "type": "string"
///    },
///    "decimals": {
///      "type": "number"
///    },
///    "description": {
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "effectiveSupply": {
///      "type": "string"
///    },
///    "holderCount": {
///      "type": "number"
///    },
///    "lastBlockSeen": {
///      "anyOf": [
///        {
///          "type": "number"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "logoUrl": {
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "marketcapUsd": {
///      "type": "number"
///    },
///    "marketcapUsdAth": {
///      "type": "number"
///    },
///    "marketcapUsdAtl": {
///      "type": "number"
///    },
///    "name": {
///      "type": "string"
///    },
///    "svmProgramId": {
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "symbol": {
///      "type": "string"
///    },
///    "tokenPriceUsd": {
///      "type": "number"
///    },
///    "totalSupply": {
///      "type": "string"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TokenInfoWithPricingResponseTokenSummary {
    #[serde(rename = "athTokenPriceUsd")]
    pub ath_token_price_usd: f64,
    #[serde(rename = "atlTokenPriceUsd")]
    pub atl_token_price_usd: f64,
    #[serde(
        rename = "bannerUrl",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub banner_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "bestPairAddress")]
    pub best_pair_address: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub bio: ::std::option::Option<::std::string::String>,
    #[serde(rename = "creationTimestamp")]
    pub creation_timestamp: ::std::string::String,
    pub decimals: f64,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "effectiveSupply")]
    pub effective_supply: ::std::string::String,
    #[serde(rename = "holderCount")]
    pub holder_count: f64,
    #[serde(
        rename = "lastBlockSeen",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub last_block_seen: ::std::option::Option<f64>,
    #[serde(rename = "logoUrl", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub logo_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "marketcapUsd")]
    pub marketcap_usd: f64,
    #[serde(rename = "marketcapUsdAth")]
    pub marketcap_usd_ath: f64,
    #[serde(rename = "marketcapUsdAtl")]
    pub marketcap_usd_atl: f64,
    pub name: ::std::string::String,
    #[serde(rename = "svmProgramId")]
    pub svm_program_id: ::std::option::Option<::std::string::String>,
    pub symbol: ::std::string::String,
    #[serde(rename = "tokenPriceUsd")]
    pub token_price_usd: f64,
    #[serde(rename = "totalSupply")]
    pub total_supply: ::std::string::String,
}
