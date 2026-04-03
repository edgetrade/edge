#![allow(clippy::derivable_impls)]

use crate::client::{Route, RouteType};
use std::marker::PhantomData;
/// Route metadata for this procedure
pub const ROUTE: Route<SearchTokensRequest, Vec<SearchTokensResponseItem>> = Route {
    procedure: "intelligence.searchTokens",
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
///`Schema0`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "anyOf": [
///    {
///      "type": "string"
///    },
///    {
///      "type": "number"
///    },
///    {
///      "type": "boolean"
///    },
///    {
///      "type": "null"
///    },
///    {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/__schema0"
///      }
///    },
///    {
///      "type": "object",
///      "additionalProperties": {
///        "$ref": "#/$defs/__schema0"
///      },
///      "propertyNames": {
///        "type": "string"
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum Schema0 {
    String(::std::string::String),
    Number(f64),
    Boolean(bool),
    Null,
    Array(::std::vec::Vec<Schema0>),
    Object(::std::collections::HashMap<::std::string::String, Schema0>),
}
impl ::std::convert::From<f64> for Schema0 {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}
impl ::std::convert::From<bool> for Schema0 {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}
impl ::std::convert::From<::std::vec::Vec<Schema0>> for Schema0 {
    fn from(value: ::std::vec::Vec<Schema0>) -> Self {
        Self::Array(value)
    }
}
impl ::std::convert::From<::std::collections::HashMap<::std::string::String, Schema0>> for Schema0 {
    fn from(value: ::std::collections::HashMap<::std::string::String, Schema0>) -> Self {
        Self::Object(value)
    }
}
///Input for searching tokens.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Input for searching tokens.",
///  "type": "object",
///  "properties": {
///    "chainId": {
///      "description": "Narrow to a specific chain. Omit to search all chains.",
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Chain ID"
///    },
///    "hasGraduated": {
///      "description": "Filter to graduated (true) or pre-graduation (false) tokens only. Omit for all.",
///      "anyOf": [
///        {
///          "type": "boolean"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Has Graduated"
///    },
///    "pairTypes": {
///      "description": "Filter to specific DEX pair types. Omit for all. Available types: [object Object], [object Object]",
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
///      ],
///      "name": "Pair Types"
///    },
///    "search": {
///      "description": "Name, symbol, or contract address to search for. Be as specific as possible — broad terms return large result sets. Prefer exact contract addresses or symbols.",
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Search"
///    },
///    "weightings": {
///      "description": "Scoring weights (0–1) for ranking results across: marketcap, liquidity, volume, hasLogo, recency, nameSimilarity. All six fields are required if provided.",
///      "anyOf": [
///        {
///          "type": "object",
///          "required": [
///            "hasLogo",
///            "liquidity",
///            "marketcap",
///            "nameSimilarity",
///            "recency",
///            "volume"
///          ],
///          "properties": {
///            "hasLogo": {
///              "description": "Weighting for hasLogo (0–1).",
///              "type": "number",
///              "maximum": 1.0,
///              "minimum": 0.0,
///              "name": "Has Logo"
///            },
///            "liquidity": {
///              "description": "Weighting for liquidity (0–1).",
///              "type": "number",
///              "maximum": 1.0,
///              "minimum": 0.0,
///              "name": "Liquidity"
///            },
///            "marketcap": {
///              "description": "Weighting for marketcap (0–1).",
///              "type": "number",
///              "maximum": 1.0,
///              "minimum": 0.0,
///              "name": "Marketcap"
///            },
///            "nameSimilarity": {
///              "description": "Weighting for nameSimilarity (0–1).",
///              "type": "number",
///              "maximum": 1.0,
///              "minimum": 0.0,
///              "name": "Name Similarity"
///            },
///            "recency": {
///              "description": "Weighting for recency (0–1).",
///              "type": "number",
///              "maximum": 1.0,
///              "minimum": 0.0,
///              "name": "Recency"
///            },
///            "volume": {
///              "description": "Weighting for volume (0–1).",
///              "type": "number",
///              "maximum": 1.0,
///              "minimum": 0.0,
///              "name": "Volume"
///            }
///          }
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Weightings"
///    }
///  },
///  "name": "Search Tokens Request"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SearchTokensRequest {
    ///Narrow to a specific chain. Omit to search all chains.
    #[serde(rename = "chainId", default, skip_serializing_if = "::std::option::Option::is_none")]
    pub chain_id: ::std::option::Option<::std::string::String>,
    ///Filter to graduated (true) or pre-graduation (false) tokens only. Omit for all.
    #[serde(
        rename = "hasGraduated",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub has_graduated: ::std::option::Option<bool>,
    ///Filter to specific DEX pair types. Omit for all. Available types: [object Object], [object Object]
    #[serde(
        rename = "pairTypes",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub pair_types: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ///Name, symbol, or contract address to search for. Be as specific as possible — broad terms return large result sets. Prefer exact contract addresses or symbols.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub search: ::std::option::Option<::std::string::String>,
    ///Scoring weights (0–1) for ranking results across: marketcap, liquidity, volume, hasLogo, recency, nameSimilarity. All six fields are required if provided.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub weightings: ::std::option::Option<SearchTokensRequestWeightings>,
}
impl ::std::default::Default for SearchTokensRequest {
    fn default() -> Self {
        Self {
            chain_id: Default::default(),
            has_graduated: Default::default(),
            pair_types: Default::default(),
            search: Default::default(),
            weightings: Default::default(),
        }
    }
}
///`SearchTokensRequestWeightings`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "hasLogo",
///    "liquidity",
///    "marketcap",
///    "nameSimilarity",
///    "recency",
///    "volume"
///  ],
///  "properties": {
///    "hasLogo": {
///      "description": "Weighting for hasLogo (0–1).",
///      "type": "number",
///      "maximum": 1.0,
///      "minimum": 0.0,
///      "name": "Has Logo"
///    },
///    "liquidity": {
///      "description": "Weighting for liquidity (0–1).",
///      "type": "number",
///      "maximum": 1.0,
///      "minimum": 0.0,
///      "name": "Liquidity"
///    },
///    "marketcap": {
///      "description": "Weighting for marketcap (0–1).",
///      "type": "number",
///      "maximum": 1.0,
///      "minimum": 0.0,
///      "name": "Marketcap"
///    },
///    "nameSimilarity": {
///      "description": "Weighting for nameSimilarity (0–1).",
///      "type": "number",
///      "maximum": 1.0,
///      "minimum": 0.0,
///      "name": "Name Similarity"
///    },
///    "recency": {
///      "description": "Weighting for recency (0–1).",
///      "type": "number",
///      "maximum": 1.0,
///      "minimum": 0.0,
///      "name": "Recency"
///    },
///    "volume": {
///      "description": "Weighting for volume (0–1).",
///      "type": "number",
///      "maximum": 1.0,
///      "minimum": 0.0,
///      "name": "Volume"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SearchTokensRequestWeightings {
    #[serde(rename = "hasLogo")]
    pub has_logo: f64,
    pub liquidity: f64,
    pub marketcap: f64,
    #[serde(rename = "nameSimilarity")]
    pub name_similarity: f64,
    pub recency: f64,
    pub volume: f64,
}
///Response for searching tokens.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Response for searching tokens.",
///  "type": "object",
///  "required": [
///    "chainId",
///    "counterTokenSymbol",
///    "id",
///    "liquidityUsd",
///    "logoUrl",
///    "marketcapUsd",
///    "nameSimilarity",
///    "pairContractAddress",
///    "pairCreatedAt",
///    "pairType",
///    "scores",
///    "social",
///    "tokenContractAddress",
///    "tokenCreatedAt",
///    "tokenName",
///    "tokenSymbol",
///    "volumeUsd1H"
///  ],
///  "properties": {
///    "chainId": {
///      "description": "The chain ID of the token.",
///      "type": "string",
///      "name": "Chain ID"
///    },
///    "counterTokenSymbol": {
///      "description": "The symbol of the counter token.",
///      "type": "string",
///      "name": "Counter Token Symbol"
///    },
///    "id": {
///      "description": "The ID of the token.",
///      "type": "string",
///      "name": "ID"
///    },
///    "liquidityUsd": {
///      "description": "The liquidity of the pair in USD.",
///      "type": "number",
///      "name": "Liquidity USD"
///    },
///    "logoUrl": {
///      "description": "The URL of the logo of the token.",
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Logo URL"
///    },
///    "marketcapUsd": {
///      "description": "The marketcap of the pair in USD.",
///      "type": "number",
///      "name": "Marketcap USD"
///    },
///    "nameSimilarity": {
///      "description": "The similarity of the name of the token.",
///      "type": "number",
///      "name": "Name Similarity"
///    },
///    "pairContractAddress": {
///      "description": "The contract address of the pair.",
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Pair Contract Address"
///    },
///    "pairCreatedAt": {
///      "description": "The timestamp of the creation of the pair.",
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
///      "name": "Pair Created At"
///    },
///    "pairType": {
///      "description": "The type of the pair.",
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Pair Type"
///    },
///    "scores": {
///      "description": "The scores of the token.",
///      "type": "object",
///      "required": [
///        "hasLogo",
///        "liquidity",
///        "marketcap",
///        "nameSimilarity",
///        "recency",
///        "total",
///        "volume"
///      ],
///      "properties": {
///        "hasLogo": {
///          "description": "Whether the token has a logo.",
///          "type": "number",
///          "name": "Has Logo"
///        },
///        "liquidity": {
///          "description": "The liquidity of the token.",
///          "type": "number",
///          "name": "Liquidity"
///        },
///        "marketcap": {
///          "description": "The marketcap of the token.",
///          "type": "number",
///          "name": "Marketcap"
///        },
///        "nameSimilarity": {
///          "description": "The similarity of the name of the token.",
///          "type": "number",
///          "name": "Name Similarity"
///        },
///        "recency": {
///          "description": "The recency of the token.",
///          "type": "number",
///          "name": "Recency"
///        },
///        "total": {
///          "description": "The total of the token.",
///          "type": "number",
///          "name": "Total"
///        },
///        "volume": {
///          "description": "The volume of the token.",
///          "type": "number",
///          "name": "Volume"
///        }
///      },
///      "additionalProperties": false,
///      "name": "Scores"
///    },
///    "social": {
///      "description": "The social of the token.",
///      "anyOf": [
///        {
///          "$ref": "#/$defs/__schema0"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Social"
///    },
///    "tokenContractAddress": {
///      "description": "The contract address of the token.",
///      "type": "string",
///      "name": "Token Contract Address"
///    },
///    "tokenCreatedAt": {
///      "description": "The timestamp of the creation of the token.",
///      "type": "string",
///      "format": "date-time",
///      "pattern": "^(?:(?:\\d\\d[2468][048]|\\d\\d[13579][26]|\\d\\d0[48]|[02468][048]00|[13579][26]00)-02-29|\\d{4}-(?:(?:0[13578]|1[02])-(?:0[1-9]|[12]\\d|3[01])|(?:0[469]|11)-(?:0[1-9]|[12]\\d|30)|(?:02)-(?:0[1-9]|1\\d|2[0-8])))T(?:(?:[01]\\d|2[0-3]):[0-5]\\d(?::[0-5]\\d(?:\\.\\d+)?)?(?:Z))$",
///      "name": "Token Created At"
///    },
///    "tokenName": {
///      "description": "The name of the token.",
///      "type": "string",
///      "name": "Token Name"
///    },
///    "tokenSymbol": {
///      "description": "The symbol of the token.",
///      "type": "string",
///      "name": "Token Symbol"
///    },
///    "volumeUsd1H": {
///      "description": "The volume of the pair in USD over the last hour.",
///      "type": "number",
///      "name": "Volume USD 1H"
///    }
///  },
///  "additionalProperties": false,
///  "name": "Search Tokens Response Item"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct SearchTokensResponseItem {
    ///The chain ID of the token.
    #[serde(rename = "chainId")]
    pub chain_id: ::std::string::String,
    ///The symbol of the counter token.
    #[serde(rename = "counterTokenSymbol")]
    pub counter_token_symbol: ::std::string::String,
    ///The ID of the token.
    pub id: ::std::string::String,
    #[serde(rename = "liquidityUsd")]
    pub liquidity_usd: f64,
    ///The URL of the logo of the token.
    #[serde(rename = "logoUrl")]
    pub logo_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "marketcapUsd")]
    pub marketcap_usd: f64,
    #[serde(rename = "nameSimilarity")]
    pub name_similarity: f64,
    ///The contract address of the pair.
    #[serde(rename = "pairContractAddress")]
    pub pair_contract_address: ::std::option::Option<::std::string::String>,
    ///The timestamp of the creation of the pair.
    #[serde(rename = "pairCreatedAt")]
    pub pair_created_at: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    ///The type of the pair.
    #[serde(rename = "pairType")]
    pub pair_type: ::std::option::Option<::std::string::String>,
    pub scores: SearchTokensResponseItemScores,
    ///The social of the token.
    pub social: ::std::option::Option<Schema0>,
    ///The contract address of the token.
    #[serde(rename = "tokenContractAddress")]
    pub token_contract_address: ::std::string::String,
    ///The timestamp of the creation of the token.
    #[serde(rename = "tokenCreatedAt")]
    pub token_created_at: ::chrono::DateTime<::chrono::offset::Utc>,
    ///The name of the token.
    #[serde(rename = "tokenName")]
    pub token_name: ::std::string::String,
    ///The symbol of the token.
    #[serde(rename = "tokenSymbol")]
    pub token_symbol: ::std::string::String,
    #[serde(rename = "volumeUsd1H")]
    pub volume_usd1_h: f64,
}
///The scores of the token.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "The scores of the token.",
///  "type": "object",
///  "required": [
///    "hasLogo",
///    "liquidity",
///    "marketcap",
///    "nameSimilarity",
///    "recency",
///    "total",
///    "volume"
///  ],
///  "properties": {
///    "hasLogo": {
///      "description": "Whether the token has a logo.",
///      "type": "number",
///      "name": "Has Logo"
///    },
///    "liquidity": {
///      "description": "The liquidity of the token.",
///      "type": "number",
///      "name": "Liquidity"
///    },
///    "marketcap": {
///      "description": "The marketcap of the token.",
///      "type": "number",
///      "name": "Marketcap"
///    },
///    "nameSimilarity": {
///      "description": "The similarity of the name of the token.",
///      "type": "number",
///      "name": "Name Similarity"
///    },
///    "recency": {
///      "description": "The recency of the token.",
///      "type": "number",
///      "name": "Recency"
///    },
///    "total": {
///      "description": "The total of the token.",
///      "type": "number",
///      "name": "Total"
///    },
///    "volume": {
///      "description": "The volume of the token.",
///      "type": "number",
///      "name": "Volume"
///    }
///  },
///  "additionalProperties": false,
///  "name": "Scores"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct SearchTokensResponseItemScores {
    #[serde(rename = "hasLogo")]
    pub has_logo: f64,
    pub liquidity: f64,
    pub marketcap: f64,
    #[serde(rename = "nameSimilarity")]
    pub name_similarity: f64,
    pub recency: f64,
    pub total: f64,
    pub volume: f64,
}
