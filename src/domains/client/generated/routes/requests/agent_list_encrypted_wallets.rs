#![allow(clippy::derivable_impls)]

use crate::client::{Route, RouteType};
use std::marker::PhantomData;
/// Route metadata for this procedure
pub const ROUTE: Route<(), Vec<ListEncryptedWalletsResponseItem>> = Route {
    procedure: "agent.listEncryptedWallets",
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
///`ListEncryptedWalletsResponseItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "address",
///    "chainType",
///    "isArchived",
///    "kind",
///    "name",
///    "selectedChains"
///  ],
///  "properties": {
///    "address": {
///      "type": "string"
///    },
///    "chainType": {
///      "type": "string"
///    },
///    "isArchived": {
///      "type": "boolean"
///    },
///    "kind": {
///      "anyOf": [
///        {
///          "type": "string",
///          "const": "v0"
///        },
///        {
///          "type": "string",
///          "const": "v1"
///        }
///      ]
///    },
///    "name": {
///      "anyOf": [
///        {
///          "type": "string"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "selectedChains": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ListEncryptedWalletsResponseItem {
    pub address: ::std::string::String,
    #[serde(rename = "chainType")]
    pub chain_type: ::std::string::String,
    #[serde(rename = "isArchived")]
    pub is_archived: bool,
    pub kind: ListEncryptedWalletsResponseItemKind,
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "selectedChains")]
    pub selected_chains: ::std::vec::Vec<::std::string::String>,
}
///`ListEncryptedWalletsResponseItemKind`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "anyOf": [
///    {
///      "type": "string",
///      "const": "v0"
///    },
///    {
///      "type": "string",
///      "const": "v1"
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ListEncryptedWalletsResponseItemKind {
    #[serde(rename = "v0")]
    V0,
    #[serde(rename = "v1")]
    V1,
}
impl ::std::fmt::Display for ListEncryptedWalletsResponseItemKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::V0 => f.write_str("v0"),
            Self::V1 => f.write_str("v1"),
        }
    }
}
impl ::std::str::FromStr for ListEncryptedWalletsResponseItemKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "v0" => Ok(Self::V0),
            "v1" => Ok(Self::V1),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ListEncryptedWalletsResponseItemKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ListEncryptedWalletsResponseItemKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ListEncryptedWalletsResponseItemKind {
    type Error = self::error::ConversionError;
    fn try_from(value: ::std::string::String) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
