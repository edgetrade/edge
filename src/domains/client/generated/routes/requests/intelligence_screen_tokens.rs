#![allow(clippy::derivable_impls)]

use crate::client::{Route, RouteType};
use std::marker::PhantomData;
/// Route metadata for this procedure
pub const ROUTE: Route<Vec<ScreenTokensRequestItem>, Vec<ScreenTokensResponseItem>> = Route {
    procedure: "intelligence.screenTokens",
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
///Input for screening tokens. All fields are optional.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Input for screening tokens. All fields are optional.",
///  "type": "object",
///  "required": [
///    "filters",
///    "limit",
///    "sortColumn"
///  ],
///  "properties": {
///    "filters": {
///      "description": "Filter object for screening tokens. All fields are optional. Available fields: * pairTypes (array of pair type strings), \n      * isGraduated (boolean), \n      * includeKeywords/excludeKeywords (string arrays matched against name/symbol/address), \n      * minMarketcapUsd/maxMarketcapUsd, \n      * minLiquidityUsd/maxLiquidityUsd, \n      * minVolumeUsd24h/maxVolumeUsd24h, \n      * minVolumeUsd15m/maxVolumeUsd15m, \n      * minTransactionCount24h/maxTransactionCount24h, \n      * minTotalBuys24h/maxTotalBuys24h, \n      * minTotalSells24h/maxTotalSells24h, \n      * minGraduationProgressPercent/maxGraduationProgressPercent (0–100), \n      * minCreationTimestamp/maxCreationTimestamp (unix epoch seconds), \n      * contractAddressEndsInPump (boolean), \n      * hasTwitterUrl/hasWebsiteUrl/hasTelegramUrl/hasAnySocialUrl (boolean), \n      * dexscreenerPaidTokenProfile (boolean), \n      * minHolderCount/maxHolderCount, \n      * minSnipersHoldingPercentage/maxSnipersHoldingPercentage (0–100), \n      * minInsidersHoldingPercentage/maxInsidersHoldingPercentage (0–100), \n      * minTop10HoldingPercentage/maxTop10HoldingPercentage (0–100), \n      * minDeployerHoldingPercentage/maxDeployerHoldingPercentage (0–100), \n      * minDeployerPairs/maxDeployerPairs, \n      * minDeployerGraduatedPairs/maxDeployerGraduatedPairs, \n      * minPairPriceChangePercentage1h/maxPairPriceChangePercentage1h",
///      "name": "Filter object for screening tokens. All fields are optional. Available fields: "
///    },
///    "limit": {
///      "description": "Number of tokens to return (1–100) ordered by recency algorithm. Prefer 2-3 unless doing analysis.",
///      "type": "number",
///      "maximum": 100.0,
///      "minimum": 1.0
///    },
///    "sortColumn": {
///      "description": "Column to sort results by (descending). Must be one of: \"creation_timestamp\" (newest tokens first) or \"graduation_progress_percent\" (closest to graduating first). Required.",
///      "type": "string",
///      "enum": [
///        "creation_timestamp",
///        "graduation_progress_percent"
///      ],
///      "name": "Column to sort results by"
///    }
///  },
///  "name": "Screen Tokens Request Item"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ScreenTokensRequestItem {
    /**Filter object for screening tokens. All fields are optional. Available fields: * pairTypes (array of pair type strings),
     * isGraduated (boolean),
     * includeKeywords/excludeKeywords (string arrays matched against name/symbol/address),
     * minMarketcapUsd/maxMarketcapUsd,
     * minLiquidityUsd/maxLiquidityUsd,
     * minVolumeUsd24h/maxVolumeUsd24h,
     * minVolumeUsd15m/maxVolumeUsd15m,
     * minTransactionCount24h/maxTransactionCount24h,
     * minTotalBuys24h/maxTotalBuys24h,
     * minTotalSells24h/maxTotalSells24h,
     * minGraduationProgressPercent/maxGraduationProgressPercent (0–100),
     * minCreationTimestamp/maxCreationTimestamp (unix epoch seconds),
     * contractAddressEndsInPump (boolean),
     * hasTwitterUrl/hasWebsiteUrl/hasTelegramUrl/hasAnySocialUrl (boolean),
     * dexscreenerPaidTokenProfile (boolean),
     * minHolderCount/maxHolderCount,
     * minSnipersHoldingPercentage/maxSnipersHoldingPercentage (0–100),
     * minInsidersHoldingPercentage/maxInsidersHoldingPercentage (0–100),
     * minTop10HoldingPercentage/maxTop10HoldingPercentage (0–100),
     * minDeployerHoldingPercentage/maxDeployerHoldingPercentage (0–100),
     * minDeployerPairs/maxDeployerPairs,
     * minDeployerGraduatedPairs/maxDeployerGraduatedPairs,
     * minPairPriceChangePercentage1h/maxPairPriceChangePercentage1h*/
    pub filters: ::serde_json::Value,
    pub limit: f64,
    ///Column to sort results by (descending). Must be one of: "creation_timestamp" (newest tokens first) or "graduation_progress_percent" (closest to graduating first). Required.
    #[serde(rename = "sortColumn")]
    pub sort_column: ScreenTokensRequestItemSortColumn,
}
///Column to sort results by (descending). Must be one of: "creation_timestamp" (newest tokens first) or "graduation_progress_percent" (closest to graduating first). Required.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Column to sort results by (descending). Must be one of: \"creation_timestamp\" (newest tokens first) or \"graduation_progress_percent\" (closest to graduating first). Required.",
///  "type": "string",
///  "enum": [
///    "creation_timestamp",
///    "graduation_progress_percent"
///  ],
///  "name": "Column to sort results by"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ScreenTokensRequestItemSortColumn {
    #[serde(rename = "creation_timestamp")]
    CreationTimestamp,
    #[serde(rename = "graduation_progress_percent")]
    GraduationProgressPercent,
}
impl ::std::fmt::Display for ScreenTokensRequestItemSortColumn {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::CreationTimestamp => f.write_str("creation_timestamp"),
            Self::GraduationProgressPercent => f.write_str("graduation_progress_percent"),
        }
    }
}
impl ::std::str::FromStr for ScreenTokensRequestItemSortColumn {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "creation_timestamp" => Ok(Self::CreationTimestamp),
            "graduation_progress_percent" => Ok(Self::GraduationProgressPercent),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ScreenTokensRequestItemSortColumn {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ScreenTokensRequestItemSortColumn {
    type Error = self::error::ConversionError;
    fn try_from(value: &::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ScreenTokensRequestItemSortColumn {
    type Error = self::error::ConversionError;
    fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Response for screening tokens.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Response for screening tokens.",
///  "type": "object",
///  "required": [
///    "chainId",
///    "counterTokenReserves",
///    "creationTimestamp",
///    "deployerGraduatedPairs",
///    "deployerHoldingPercentage",
///    "deployerPairs",
///    "dexscreenerPaidTokenProfile",
///    "doNotIndex",
///    "effectiveSupply",
///    "graduatedFromPairAddress",
///    "graduatedToPairAddress",
///    "graduationProgressPercent",
///    "holderCount",
///    "id",
///    "insidersHoldingPercentage",
///    "liquidityUsd",
///    "logoUrl",
///    "marketcapUsd",
///    "name",
///    "pairContractAddress",
///    "pairPriceChangePercentage1h",
///    "pairType",
///    "snipersHoldingPercentage",
///    "socialTelegramUrl",
///    "socialTwitterUrl",
///    "socialWebsiteUrl",
///    "symbol",
///    "tokenContractAddress",
///    "tokenPriceUsd",
///    "top10HoldingPercentage",
///    "totalBuys24h",
///    "totalQuoteFundRaising",
///    "totalSells24h",
///    "transactionCount24h",
///    "volumeUsd15m",
///    "volumeUsd24h"
///  ],
///  "properties": {
///    "chainId": {
///      "description": "The chain ID of the pair.",
///      "type": "string",
///      "name": "Chain ID"
///    },
///    "counterTokenReserves": {
///      "description": "The reserves of the counter token.",
///      "anyOf": [
///        {
///          "type": "number"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Counter Token Reserves"
///    },
///    "creationTimestamp": {
///      "description": "The timestamp of the creation of the pair.",
///      "type": "number",
///      "name": "Creation Timestamp"
///    },
///    "deployerGraduatedPairs": {
///      "description": "The number of pairs graduated by the deployer.",
///      "anyOf": [
///        {
///          "type": "number"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Deployer Graduated Pairs"
///    },
///    "deployerHoldingPercentage": {
///      "description": "The percentage of the token held by the deployer.",
///      "anyOf": [
///        {
///          "type": "number"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Deployer Holding Percentage"
///    },
///    "deployerPairs": {
///      "description": "The number of pairs created by the deployer.",
///      "anyOf": [
///        {
///          "type": "number"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Deployer Pairs"
///    },
///    "dexscreenerPaidTokenProfile": {
///      "description": "Whether the token has a paid profile on Dexscreener.",
///      "type": "boolean",
///      "name": "Dexscreener Paid Token Profile"
///    },
///    "doNotIndex": {
///      "description": "Whether the token should not be indexed.",
///      "type": "boolean",
///      "name": "Do Not Index"
///    },
///    "effectiveSupply": {
///      "description": "The effective supply of the token.",
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Effective Supply"
///    },
///    "graduatedFromPairAddress": {
///      "description": "The address of the pair that the token graduated from.",
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
///    "graduatedToPairAddress": {
///      "description": "The address of the pair that the token graduated to.",
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Graduated To Pair Address"
///    },
///    "graduationProgressPercent": {
///      "description": "The graduation progress of the pair as a percentage.",
///      "type": "number",
///      "name": "Graduation Progress Percent"
///    },
///    "holderCount": {
///      "description": "The number of holders of the token.",
///      "anyOf": [
///        {
///          "type": "number"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Holder Count"
///    },
///    "id": {
///      "description": "The ID of the pair.",
///      "type": "string",
///      "name": "ID"
///    },
///    "insidersHoldingPercentage": {
///      "description": "The percentage of the token held by insiders.",
///      "anyOf": [
///        {
///          "type": "number"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Insiders Holding Percentage"
///    },
///    "liquidityUsd": {
///      "description": "The liquidity of the pair in USD.",
///      "anyOf": [
///        {
///          "type": "number"
///        },
///        {
///          "type": "null"
///        }
///      ],
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
///      "anyOf": [
///        {
///          "type": "number"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Marketcap USD"
///    },
///    "name": {
///      "description": "The name of the token.",
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Name"
///    },
///    "pairContractAddress": {
///      "description": "The contract address of the pair.",
///      "type": "string",
///      "name": "Pair Contract Address"
///    },
///    "pairPriceChangePercentage1h": {
///      "description": "The price change percentage of the pair over the last hour.",
///      "anyOf": [
///        {
///          "type": "number"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Pair Price Change Percentage 1h"
///    },
///    "pairType": {
///      "description": "The type of the pair.",
///      "type": "string",
///      "name": "Pair Type"
///    },
///    "snipersHoldingPercentage": {
///      "description": "The percentage of the token held by snipers.",
///      "anyOf": [
///        {
///          "type": "number"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Snipers Holding Percentage"
///    },
///    "socialTelegramUrl": {
///      "description": "The URL of the social telegram of the token.",
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Social Telegram URL"
///    },
///    "socialTwitterUrl": {
///      "description": "The URL of the social twitter of the token.",
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Social Twitter URL"
///    },
///    "socialWebsiteUrl": {
///      "description": "The URL of the social website of the token.",
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Social Website URL"
///    },
///    "symbol": {
///      "description": "The symbol of the token.",
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Symbol"
///    },
///    "tokenContractAddress": {
///      "description": "The contract address of the token.",
///      "type": "string",
///      "name": "Token Contract Address"
///    },
///    "tokenPriceUsd": {
///      "description": "The price of the token in USD.",
///      "anyOf": [
///        {
///          "type": "number"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Token Price USD"
///    },
///    "top10HoldingPercentage": {
///      "description": "The percentage of the token held by the top 10 holders.",
///      "anyOf": [
///        {
///          "type": "number"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Top 10 Holding Percentage"
///    },
///    "totalBuys24h": {
///      "description": "The total number of buys of the pair over the last 24 hours.",
///      "type": "number",
///      "name": "Total Buys 24h"
///    },
///    "totalQuoteFundRaising": {
///      "description": "The total amount of quote token used for fund raising.",
///      "anyOf": [
///        {
///          "type": "number"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Total Quote Fund Raising"
///    },
///    "totalSells24h": {
///      "description": "The total number of sells of the pair over the last 24 hours.",
///      "type": "number",
///      "name": "Total Sells 24h"
///    },
///    "transactionCount24h": {
///      "description": "The number of transactions of the pair over the last 24 hours.",
///      "type": "number",
///      "name": "Transaction Count 24h"
///    },
///    "volumeUsd15m": {
///      "description": "The volume of the pair in USD over the last 15 minutes.",
///      "anyOf": [
///        {
///          "type": "number"
///        },
///        {
///          "type": "null"
///        }
///      ],
///      "name": "Volume USD 15m"
///    },
///    "volumeUsd24h": {
///      "description": "The volume of the pair in USD over the last 24 hours.",
///      "type": "number",
///      "name": "Volume USD 24h"
///    }
///  },
///  "additionalProperties": false,
///  "name": "Screen Tokens Response Item"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ScreenTokensResponseItem {
    ///The chain ID of the pair.
    #[serde(rename = "chainId")]
    pub chain_id: ::std::string::String,
    ///The reserves of the counter token.
    #[serde(rename = "counterTokenReserves")]
    pub counter_token_reserves: ::std::option::Option<f64>,
    #[serde(rename = "creationTimestamp")]
    pub creation_timestamp: f64,
    ///The number of pairs graduated by the deployer.
    #[serde(rename = "deployerGraduatedPairs")]
    pub deployer_graduated_pairs: ::std::option::Option<f64>,
    ///The percentage of the token held by the deployer.
    #[serde(rename = "deployerHoldingPercentage")]
    pub deployer_holding_percentage: ::std::option::Option<f64>,
    ///The number of pairs created by the deployer.
    #[serde(rename = "deployerPairs")]
    pub deployer_pairs: ::std::option::Option<f64>,
    ///Whether the token has a paid profile on Dexscreener.
    #[serde(rename = "dexscreenerPaidTokenProfile")]
    pub dexscreener_paid_token_profile: bool,
    ///Whether the token should not be indexed.
    #[serde(rename = "doNotIndex")]
    pub do_not_index: bool,
    ///The effective supply of the token.
    #[serde(rename = "effectiveSupply")]
    pub effective_supply: ::std::option::Option<::std::string::String>,
    ///The address of the pair that the token graduated from.
    #[serde(rename = "graduatedFromPairAddress")]
    pub graduated_from_pair_address: ::std::option::Option<::std::string::String>,
    ///The address of the pair that the token graduated to.
    #[serde(rename = "graduatedToPairAddress")]
    pub graduated_to_pair_address: ::std::option::Option<::std::string::String>,
    #[serde(rename = "graduationProgressPercent")]
    pub graduation_progress_percent: f64,
    ///The number of holders of the token.
    #[serde(rename = "holderCount")]
    pub holder_count: ::std::option::Option<f64>,
    ///The ID of the pair.
    pub id: ::std::string::String,
    ///The percentage of the token held by insiders.
    #[serde(rename = "insidersHoldingPercentage")]
    pub insiders_holding_percentage: ::std::option::Option<f64>,
    ///The liquidity of the pair in USD.
    #[serde(rename = "liquidityUsd")]
    pub liquidity_usd: ::std::option::Option<f64>,
    ///The URL of the logo of the token.
    #[serde(rename = "logoUrl")]
    pub logo_url: ::std::option::Option<::std::string::String>,
    ///The marketcap of the pair in USD.
    #[serde(rename = "marketcapUsd")]
    pub marketcap_usd: ::std::option::Option<f64>,
    ///The name of the token.
    pub name: ::std::option::Option<::std::string::String>,
    ///The contract address of the pair.
    #[serde(rename = "pairContractAddress")]
    pub pair_contract_address: ::std::string::String,
    ///The price change percentage of the pair over the last hour.
    #[serde(rename = "pairPriceChangePercentage1h")]
    pub pair_price_change_percentage1h: ::std::option::Option<f64>,
    ///The type of the pair.
    #[serde(rename = "pairType")]
    pub pair_type: ::std::string::String,
    ///The percentage of the token held by snipers.
    #[serde(rename = "snipersHoldingPercentage")]
    pub snipers_holding_percentage: ::std::option::Option<f64>,
    ///The URL of the social telegram of the token.
    #[serde(rename = "socialTelegramUrl")]
    pub social_telegram_url: ::std::option::Option<::std::string::String>,
    ///The URL of the social twitter of the token.
    #[serde(rename = "socialTwitterUrl")]
    pub social_twitter_url: ::std::option::Option<::std::string::String>,
    ///The URL of the social website of the token.
    #[serde(rename = "socialWebsiteUrl")]
    pub social_website_url: ::std::option::Option<::std::string::String>,
    ///The symbol of the token.
    pub symbol: ::std::option::Option<::std::string::String>,
    ///The contract address of the token.
    #[serde(rename = "tokenContractAddress")]
    pub token_contract_address: ::std::string::String,
    ///The price of the token in USD.
    #[serde(rename = "tokenPriceUsd")]
    pub token_price_usd: ::std::option::Option<f64>,
    ///The percentage of the token held by the top 10 holders.
    #[serde(rename = "top10HoldingPercentage")]
    pub top10_holding_percentage: ::std::option::Option<f64>,
    #[serde(rename = "totalBuys24h")]
    pub total_buys24h: f64,
    ///The total amount of quote token used for fund raising.
    #[serde(rename = "totalQuoteFundRaising")]
    pub total_quote_fund_raising: ::std::option::Option<f64>,
    #[serde(rename = "totalSells24h")]
    pub total_sells24h: f64,
    #[serde(rename = "transactionCount24h")]
    pub transaction_count24h: f64,
    ///The volume of the pair in USD over the last 15 minutes.
    #[serde(rename = "volumeUsd15m")]
    pub volume_usd15m: ::std::option::Option<f64>,
    #[serde(rename = "volumeUsd24h")]
    pub volume_usd24h: f64,
}
