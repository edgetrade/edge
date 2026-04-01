#![allow(clippy::derivable_impls)]

use crate::client::{Route, RouteType};
use std::marker::PhantomData;
/// Route metadata for this procedure
pub const ROUTE: Route<TokenDevTokensRequest, Vec<TokenDevTokensResponseItem>> = Route {
    procedure: "tokens.getDevTokens",
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
///Request to get tokens deployed by a specific address on a given chain
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Request to get tokens deployed by a specific address on a given chain",
///  "type": "object",
///  "required": [
///    "chainId",
///    "deployerAddress"
///  ],
///  "properties": {
///    "chainId": {
///      "description": "The chain ID where the deployer exists (e.g., \"1\" for Ethereum, \"8453\" for Base, \"solana\" for Solana)",
///      "type": "string",
///      "name": "Chain ID"
///    },
///    "deployerAddress": {
///      "description": "The contract address of the token deployer to query tokens for",
///      "type": "string",
///      "name": "Deployer Address"
///    }
///  },
///  "name": "Get Dev Tokens Request"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct TokenDevTokensRequest {
    ///The chain ID where the deployer exists (e.g., "1" for Ethereum, "8453" for Base, "solana" for Solana)
    #[serde(rename = "chainId")]
    pub chain_id: ::std::string::String,
    ///The contract address of the token deployer to query tokens for
    #[serde(rename = "deployerAddress")]
    pub deployer_address: ::std::string::String,
}
///Simplified token information with pair data
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Simplified token information with pair data",
///  "type": "object",
///  "required": [
///    "bestPairAddress",
///    "bestPairCounterToken",
///    "bestPairCreatedAt",
///    "bestPairSymbol",
///    "bestPairType",
///    "createdAt",
///    "decimals",
///    "deployerAddress",
///    "effectiveSupply",
///    "liquidityUsd",
///    "logoUrl",
///    "marketcapUsd",
///    "name",
///    "socialLinks",
///    "symbol",
///    "tokenChainId",
///    "tokenContractAddress",
///    "tokenPriceNativeToken",
///    "tokenPriceUsd",
///    "tradingOpenedAt"
///  ],
///  "properties": {
///    "bestPairAddress": {
///      "description": "Contract address of the best trading pair",
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "bestPairCounterToken": {
///      "description": "Counter token details in the best pair",
///      "anyOf": [
///        {
///          "type": "object",
///          "required": [
///            "decimals",
///            "name",
///            "symbol",
///            "tokenChainId",
///            "tokenContractAddress"
///          ],
///          "properties": {
///            "decimals": {
///              "description": "Number of decimal places for the counter token",
///              "type": "number"
///            },
///            "name": {
///              "description": "Name of the counter token",
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
///              "description": "Symbol of the counter token",
///              "anyOf": [
///                {
///                  "type": "string"
///                },
///                {
///                  "type": "null"
///                }
///              ]
///            },
///            "tokenChainId": {
///              "description": "Chain ID where the counter token exists",
///              "type": "string"
///            },
///            "tokenContractAddress": {
///              "description": "Contract address of the counter token",
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
///    "bestPairCreatedAt": {
///      "description": "ISO timestamp when the best pair was created",
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "bestPairSymbol": {
///      "description": "Symbol of the best trading pair",
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "bestPairType": {
///      "description": "Type of the best trading pair",
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "createdAt": {
///      "description": "ISO timestamp when the token was created",
///      "type": "string"
///    },
///    "decimals": {
///      "description": "Number of decimal places for the token",
///      "type": "number"
///    },
///    "deployerAddress": {
///      "description": "Address of the token deployer",
///      "type": "string"
///    },
///    "effectiveSupply": {
///      "description": "Effective supply as a stringified bigint",
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "liquidityUsd": {
///      "description": "Liquidity in USD",
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
///      "description": "URL to the token logo image",
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
///      "description": "Market capitalization in USD",
///      "anyOf": [
///        {
///          "type": "number"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "name": {
///      "description": "Name of the token",
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "socialLinks": {
///      "description": "Array of social media links",
///      "type": "array",
///      "items": {
///        "type": "object",
///        "required": [
///          "platform",
///          "url"
///        ],
///        "properties": {
///          "platform": {
///            "description": "Social platform name",
///            "type": "string"
///          },
///          "url": {
///            "description": "Social link URL",
///            "type": "string"
///          }
///        },
///        "additionalProperties": false
///      }
///    },
///    "symbol": {
///      "description": "Symbol of the token",
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "tokenChainId": {
///      "description": "Chain ID where the token exists",
///      "type": "string"
///    },
///    "tokenContractAddress": {
///      "description": "Contract address of the token",
///      "type": "string"
///    },
///    "tokenPriceNativeToken": {
///      "description": "Token price in native chain token",
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
///      "description": "Token price in USD",
///      "anyOf": [
///        {
///          "type": "number"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "tradingOpenedAt": {
///      "description": "ISO timestamp when trading opened",
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
///  "additionalProperties": false,
///  "name": "Token Simple"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TokenDevTokensResponseItem {
    ///Contract address of the best trading pair
    #[serde(rename = "bestPairAddress")]
    pub best_pair_address: ::std::option::Option<::std::string::String>,
    ///Counter token details in the best pair
    #[serde(rename = "bestPairCounterToken")]
    pub best_pair_counter_token: ::std::option::Option<
        TokenDevTokensResponseItemBestPairCounterToken,
    >,
    ///ISO timestamp when the best pair was created
    #[serde(rename = "bestPairCreatedAt")]
    pub best_pair_created_at: ::std::option::Option<::std::string::String>,
    ///Symbol of the best trading pair
    #[serde(rename = "bestPairSymbol")]
    pub best_pair_symbol: ::std::option::Option<::std::string::String>,
    ///Type of the best trading pair
    #[serde(rename = "bestPairType")]
    pub best_pair_type: ::std::option::Option<::std::string::String>,
    ///ISO timestamp when the token was created
    #[serde(rename = "createdAt")]
    pub created_at: ::std::string::String,
    pub decimals: f64,
    ///Address of the token deployer
    #[serde(rename = "deployerAddress")]
    pub deployer_address: ::std::string::String,
    ///Effective supply as a stringified bigint
    #[serde(rename = "effectiveSupply")]
    pub effective_supply: ::std::option::Option<::std::string::String>,
    ///Liquidity in USD
    #[serde(rename = "liquidityUsd")]
    pub liquidity_usd: ::std::option::Option<f64>,
    ///URL to the token logo image
    #[serde(rename = "logoUrl")]
    pub logo_url: ::std::option::Option<::std::string::String>,
    ///Market capitalization in USD
    #[serde(rename = "marketcapUsd")]
    pub marketcap_usd: ::std::option::Option<f64>,
    ///Name of the token
    pub name: ::std::option::Option<::std::string::String>,
    ///Array of social media links
    #[serde(rename = "socialLinks")]
    pub social_links: ::std::vec::Vec<TokenDevTokensResponseItemSocialLinksItem>,
    ///Symbol of the token
    pub symbol: ::std::option::Option<::std::string::String>,
    ///Chain ID where the token exists
    #[serde(rename = "tokenChainId")]
    pub token_chain_id: ::std::string::String,
    ///Contract address of the token
    #[serde(rename = "tokenContractAddress")]
    pub token_contract_address: ::std::string::String,
    ///Token price in native chain token
    #[serde(rename = "tokenPriceNativeToken")]
    pub token_price_native_token: ::std::option::Option<f64>,
    ///Token price in USD
    #[serde(rename = "tokenPriceUsd")]
    pub token_price_usd: ::std::option::Option<f64>,
    ///ISO timestamp when trading opened
    #[serde(rename = "tradingOpenedAt")]
    pub trading_opened_at: ::std::option::Option<::std::string::String>,
}
///`TokenDevTokensResponseItemBestPairCounterToken`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "decimals",
///    "name",
///    "symbol",
///    "tokenChainId",
///    "tokenContractAddress"
///  ],
///  "properties": {
///    "decimals": {
///      "description": "Number of decimal places for the counter token",
///      "type": "number"
///    },
///    "name": {
///      "description": "Name of the counter token",
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
///      "description": "Symbol of the counter token",
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "tokenChainId": {
///      "description": "Chain ID where the counter token exists",
///      "type": "string"
///    },
///    "tokenContractAddress": {
///      "description": "Contract address of the counter token",
///      "type": "string"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TokenDevTokensResponseItemBestPairCounterToken {
    pub decimals: f64,
    ///Name of the counter token
    pub name: ::std::option::Option<::std::string::String>,
    ///Symbol of the counter token
    pub symbol: ::std::option::Option<::std::string::String>,
    ///Chain ID where the counter token exists
    #[serde(rename = "tokenChainId")]
    pub token_chain_id: ::std::string::String,
    ///Contract address of the counter token
    #[serde(rename = "tokenContractAddress")]
    pub token_contract_address: ::std::string::String,
}
///`TokenDevTokensResponseItemSocialLinksItem`
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
///      "description": "Social platform name",
///      "type": "string"
///    },
///    "url": {
///      "description": "Social link URL",
///      "type": "string"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TokenDevTokensResponseItemSocialLinksItem {
    ///Social platform name
    pub platform: ::std::string::String,
    ///Social link URL
    pub url: ::std::string::String,
}
