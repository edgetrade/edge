#![allow(clippy::derivable_impls)]

use crate::client::{Route, RouteType};
use std::marker::PhantomData;
/// Route metadata for this procedure
pub const ROUTE: Route<PairInfoRequest, PairInfoResponse> = Route {
    procedure: "pairs.getPairDetailed",
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
///Request to get detailed information about a specific liquidity pair
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Request to get detailed information about a specific liquidity pair",
///  "type": "object",
///  "required": [
///    "pairChainId",
///    "pairContractAddress"
///  ],
///  "properties": {
///    "pairChainId": {
///      "description": "The chain ID where the pair exists (e.g., \"1\" for Ethereum, \"8453\" for Base, \"solana\" for Solana)",
///      "type": "string",
///      "name": "Pair Chain ID"
///    },
///    "pairContractAddress": {
///      "description": "The contract address of the liquidity pair to query",
///      "type": "string",
///      "name": "Pair Contract Address"
///    }
///  },
///  "name": "Get Pair Detailed Request"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct PairInfoRequest {
    ///The chain ID where the pair exists (e.g., "1" for Ethereum, "8453" for Base, "solana" for Solana)
    #[serde(rename = "pairChainId")]
    pub pair_chain_id: ::std::string::String,
    ///The contract address of the liquidity pair to query
    #[serde(rename = "pairContractAddress")]
    pub pair_contract_address: ::std::string::String,
}
///`PairInfoResponse`
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
///    "counterTokenReserves",
///    "counterTokenSlot",
///    "counterTokenSymbol",
///    "createdTx",
///    "creationTimestamp",
///    "factoryAddress",
///    "fee",
///    "graduatedFromPairAddress",
///    "isNativeTokenReferencePair",
///    "lastBlockSeen",
///    "liquidityUsd",
///    "marketcapNativeToken",
///    "marketcapUsd",
///    "pairChainId",
///    "pairContractAddress",
///    "pairIndexingType",
///    "pairName",
///    "pairType",
///    "previousTokenPriceUsd",
///    "token0Address",
///    "token0PoolAccount",
///    "token0Reserves",
///    "token1Address",
///    "token1PoolAccount",
///    "token1Reserves",
///    "tokenAddress",
///    "tokenCreatedAt",
///    "tokenDecimals",
///    "tokenEffectiveSupply",
///    "tokenLogoUrl",
///    "tokenName",
///    "tokenOwner",
///    "tokenPriceNativeToken",
///    "tokenPriceUsd",
///    "tokenReserves",
///    "tokenSlot",
///    "tokenSvmProgramId",
///    "tokenSymbol"
///  ],
///  "properties": {
///    "counterTokenAddress": {
///      "description": "The address of the counter token in the pair",
///      "type": "string",
///      "name": "Counter Token Address"
///    },
///    "counterTokenDecimals": {
///      "description": "The decimals of the counter token in the pair",
///      "type": "number",
///      "name": "Counter Token Decimals"
///    },
///    "counterTokenIsNativeToken": {
///      "description": "Whether the counter token is a native token",
///      "type": "boolean",
///      "name": "Counter Token Is Native Token"
///    },
///    "counterTokenReserves": {
///      "description": "The reserves of the counter token in the pair",
///      "type": "string",
///      "name": "Counter Token Reserves"
///    },
///    "counterTokenSlot": {
///      "description": "The slot of the counter token in the pair",
///      "type": "string",
///      "name": "Counter Token Slot"
///    },
///    "counterTokenSymbol": {
///      "description": "The symbol of the counter token in the pair",
///      "type": "string",
///      "name": "Counter Token Symbol"
///    },
///    "createdTx": {
///      "description": "The transaction hash of the pair creation",
///      "type": "string",
///      "name": "Created Transaction Hash"
///    },
///    "creationTimestamp": {
///      "description": "The timestamp of the pair creation",
///      "type": "string",
///      "name": "Creation Timestamp"
///    },
///    "doNotIndex": {
///      "description": "Whether the pair should not be indexed",
///      "type": "boolean",
///      "name": "Do Not Index"
///    },
///    "factoryAddress": {
///      "description": "The address of the factory that created the pair",
///      "type": "string",
///      "name": "Factory Address"
///    },
///    "fee": {
///      "description": "The fee of the pair",
///      "anyOf": [
///        {
///          "type": "number"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Fee"
///    },
///    "graduatedFromPairAddress": {
///      "description": "The address of the pair from which the current pair graduated",
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Graduated From Pair Address"
///    },
///    "isNativeTokenReferencePair": {
///      "description": "Whether the pair is a native token reference pair",
///      "type": "boolean",
///      "name": "Is Native Token Reference Pair"
///    },
///    "lastBlockSeen": {
///      "description": "The last block seen for the pair",
///      "anyOf": [
///        {
///          "type": "number"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Last Block Seen"
///    },
///    "liquidityUsd": {
///      "description": "The liquidity of the pair in USD",
///      "type": "number",
///      "name": "Liquidity USD"
///    },
///    "marketcapNativeToken": {
///      "description": "The marketcap of the pair in native token",
///      "type": "number",
///      "name": "Marketcap Native Token"
///    },
///    "marketcapUsd": {
///      "description": "The marketcap of the pair in USD",
///      "type": "number",
///      "name": "Marketcap USD"
///    },
///    "pairChainId": {
///      "description": "The chain ID where the pair exists (e.g., \"1\" for Ethereum, \"8453\" for Base, \"solana\" for Solana)",
///      "type": "string",
///      "name": "Pair Chain ID"
///    },
///    "pairContractAddress": {
///      "description": "The contract address of the liquidity pair to query",
///      "type": "string",
///      "name": "Pair Contract Address"
///    },
///    "pairIndexingType": {
///      "description": "The type of the pair indexing",
///      "type": "string",
///      "name": "Pair Indexing Type"
///    },
///    "pairName": {
///      "description": "The name of the pair",
///      "type": "string",
///      "name": "Pair Name"
///    },
///    "pairType": {
///      "description": "The type of the pair",
///      "type": "string",
///      "name": "Pair Type"
///    },
///    "previousTokenPriceUsd": {
///      "description": "The price of the token in the pair in USD before the last block",
///      "type": "number",
///      "name": "Previous Token Price USD"
///    },
///    "token0Address": {
///      "description": "The address of the first token in the pair",
///      "type": "string",
///      "name": "Token 0 Address"
///    },
///    "token0PoolAccount": {
///      "description": "The pool account of the first token in the pair",
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Token 0 Pool Account"
///    },
///    "token0Reserves": {
///      "description": "The reserves of the first token in the pair",
///      "type": "string",
///      "name": "Token 0 Reserves"
///    },
///    "token1Address": {
///      "description": "The address of the second token in the pair",
///      "type": "string",
///      "name": "Token 1 Address"
///    },
///    "token1PoolAccount": {
///      "description": "The pool account of the second token in the pair",
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Token 1 Pool Account"
///    },
///    "token1Reserves": {
///      "description": "The reserves of the second token in the pair",
///      "type": "string",
///      "name": "Token 1 Reserves"
///    },
///    "tokenAddress": {
///      "description": "The address of the token in the pair",
///      "type": "string",
///      "name": "Token Address"
///    },
///    "tokenCreatedAt": {
///      "description": "The timestamp of the token creation",
///      "type": "string",
///      "name": "Token Created At"
///    },
///    "tokenDecimals": {
///      "description": "The decimals of the token in the pair",
///      "type": "number",
///      "name": "Token Decimals"
///    },
///    "tokenEffectiveSupply": {
///      "description": "The effective supply of the token in the pair",
///      "type": "string",
///      "name": "Token Effective Supply"
///    },
///    "tokenLogoUrl": {
///      "description": "The logo URL of the token in the pair",
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Token Logo URL"
///    },
///    "tokenName": {
///      "description": "The name of the token in the pair",
///      "type": "string",
///      "name": "Token Name"
///    },
///    "tokenOwner": {
///      "description": "The owner of the token in the pair",
///      "type": "string",
///      "name": "Token Owner"
///    },
///    "tokenPriceNativeToken": {
///      "description": "The price of the token in the pair in native token",
///      "type": "number",
///      "name": "Token Price Native Token"
///    },
///    "tokenPriceUsd": {
///      "description": "The price of the token in the pair in USD",
///      "type": "number",
///      "name": "Token Price USD"
///    },
///    "tokenReserves": {
///      "description": "The reserves of the token in the pair",
///      "type": "string",
///      "name": "Token Reserves"
///    },
///    "tokenSlot": {
///      "description": "The slot of the token in the pair",
///      "type": "string",
///      "name": "Token Slot"
///    },
///    "tokenSvmProgramId": {
///      "description": "The SVM program ID of the token in the pair",
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Token SVM Program ID"
///    },
///    "tokenSymbol": {
///      "description": "The symbol of the token in the pair",
///      "type": "string",
///      "name": "Token Symbol"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct PairInfoResponse {
    ///The address of the counter token in the pair
    #[serde(rename = "counterTokenAddress")]
    pub counter_token_address: ::std::string::String,
    #[serde(rename = "counterTokenDecimals")]
    pub counter_token_decimals: f64,
    ///Whether the counter token is a native token
    #[serde(rename = "counterTokenIsNativeToken")]
    pub counter_token_is_native_token: bool,
    ///The reserves of the counter token in the pair
    #[serde(rename = "counterTokenReserves")]
    pub counter_token_reserves: ::std::string::String,
    ///The slot of the counter token in the pair
    #[serde(rename = "counterTokenSlot")]
    pub counter_token_slot: ::std::string::String,
    ///The symbol of the counter token in the pair
    #[serde(rename = "counterTokenSymbol")]
    pub counter_token_symbol: ::std::string::String,
    ///The transaction hash of the pair creation
    #[serde(rename = "createdTx")]
    pub created_tx: ::std::string::String,
    ///The timestamp of the pair creation
    #[serde(rename = "creationTimestamp")]
    pub creation_timestamp: ::std::string::String,
    ///Whether the pair should not be indexed
    #[serde(
        rename = "doNotIndex",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub do_not_index: ::std::option::Option<bool>,
    ///The address of the factory that created the pair
    #[serde(rename = "factoryAddress")]
    pub factory_address: ::std::string::String,
    ///The fee of the pair
    pub fee: ::std::option::Option<f64>,
    ///The address of the pair from which the current pair graduated
    #[serde(rename = "graduatedFromPairAddress")]
    pub graduated_from_pair_address: ::std::option::Option<::std::string::String>,
    ///Whether the pair is a native token reference pair
    #[serde(rename = "isNativeTokenReferencePair")]
    pub is_native_token_reference_pair: bool,
    ///The last block seen for the pair
    #[serde(rename = "lastBlockSeen")]
    pub last_block_seen: ::std::option::Option<f64>,
    #[serde(rename = "liquidityUsd")]
    pub liquidity_usd: f64,
    #[serde(rename = "marketcapNativeToken")]
    pub marketcap_native_token: f64,
    #[serde(rename = "marketcapUsd")]
    pub marketcap_usd: f64,
    ///The chain ID where the pair exists (e.g., "1" for Ethereum, "8453" for Base, "solana" for Solana)
    #[serde(rename = "pairChainId")]
    pub pair_chain_id: ::std::string::String,
    ///The contract address of the liquidity pair to query
    #[serde(rename = "pairContractAddress")]
    pub pair_contract_address: ::std::string::String,
    ///The type of the pair indexing
    #[serde(rename = "pairIndexingType")]
    pub pair_indexing_type: ::std::string::String,
    ///The name of the pair
    #[serde(rename = "pairName")]
    pub pair_name: ::std::string::String,
    ///The type of the pair
    #[serde(rename = "pairType")]
    pub pair_type: ::std::string::String,
    #[serde(rename = "previousTokenPriceUsd")]
    pub previous_token_price_usd: f64,
    ///The address of the first token in the pair
    #[serde(rename = "token0Address")]
    pub token0_address: ::std::string::String,
    ///The pool account of the first token in the pair
    #[serde(rename = "token0PoolAccount")]
    pub token0_pool_account: ::std::option::Option<::std::string::String>,
    ///The reserves of the first token in the pair
    #[serde(rename = "token0Reserves")]
    pub token0_reserves: ::std::string::String,
    ///The address of the second token in the pair
    #[serde(rename = "token1Address")]
    pub token1_address: ::std::string::String,
    ///The pool account of the second token in the pair
    #[serde(rename = "token1PoolAccount")]
    pub token1_pool_account: ::std::option::Option<::std::string::String>,
    ///The reserves of the second token in the pair
    #[serde(rename = "token1Reserves")]
    pub token1_reserves: ::std::string::String,
    ///The address of the token in the pair
    #[serde(rename = "tokenAddress")]
    pub token_address: ::std::string::String,
    ///The timestamp of the token creation
    #[serde(rename = "tokenCreatedAt")]
    pub token_created_at: ::std::string::String,
    #[serde(rename = "tokenDecimals")]
    pub token_decimals: f64,
    ///The effective supply of the token in the pair
    #[serde(rename = "tokenEffectiveSupply")]
    pub token_effective_supply: ::std::string::String,
    ///The logo URL of the token in the pair
    #[serde(rename = "tokenLogoUrl")]
    pub token_logo_url: ::std::option::Option<::std::string::String>,
    ///The name of the token in the pair
    #[serde(rename = "tokenName")]
    pub token_name: ::std::string::String,
    ///The owner of the token in the pair
    #[serde(rename = "tokenOwner")]
    pub token_owner: ::std::string::String,
    #[serde(rename = "tokenPriceNativeToken")]
    pub token_price_native_token: f64,
    #[serde(rename = "tokenPriceUsd")]
    pub token_price_usd: f64,
    ///The reserves of the token in the pair
    #[serde(rename = "tokenReserves")]
    pub token_reserves: ::std::string::String,
    ///The slot of the token in the pair
    #[serde(rename = "tokenSlot")]
    pub token_slot: ::std::string::String,
    ///The SVM program ID of the token in the pair
    #[serde(rename = "tokenSvmProgramId")]
    pub token_svm_program_id: ::std::option::Option<::std::string::String>,
    ///The symbol of the token in the pair
    #[serde(rename = "tokenSymbol")]
    pub token_symbol: ::std::string::String,
}
