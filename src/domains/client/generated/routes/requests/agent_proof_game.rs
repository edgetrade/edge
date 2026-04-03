#![allow(clippy::derivable_impls)]

use crate::client::{Route, RouteType};
use std::marker::PhantomData;
/// Route metadata for this procedure
pub const ROUTE: Route<ProofGameRequest, ProofGameResponse> = Route {
    procedure: "agent.proofGame",
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
///`ProofGameRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "chain_id",
///    "orders",
///    "unsigned_tx",
///    "wallet_address"
///  ],
///  "properties": {
///    "chain_id": {
///      "type": "string"
///    },
///    "orders": {
///      "type": "array",
///      "items": {
///        "type": "object",
///        "required": [
///          "intent_envelope",
///          "order_id",
///          "value",
///          "wallet_storage_envelope",
///          "wallet_transport_envelope"
///        ],
///        "properties": {
///          "intent_envelope": {
///            "type": "string",
///            "format": "base64",
///            "pattern": "^$|^(?:[0-9a-zA-Z+/]{4})*(?:(?:[0-9a-zA-Z+/]{2}==)|(?:[0-9a-zA-Z+/]{3}=))?$",
///            "contentEncoding": "base64"
///          },
///          "order_id": {
///            "type": "string",
///            "format": "uuid",
///            "pattern": "^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[1-8][0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12}|00000000-0000-0000-0000-000000000000|ffffffff-ffff-ffff-ffff-ffffffffffff)$"
///          },
///          "value": {
///            "type": "number"
///          },
///          "wallet_storage_envelope": {
///            "type": "string",
///            "format": "base64",
///            "pattern": "^$|^(?:[0-9a-zA-Z+/]{4})*(?:(?:[0-9a-zA-Z+/]{2}==)|(?:[0-9a-zA-Z+/]{3}=))?$",
///            "contentEncoding": "base64"
///          },
///          "wallet_transport_envelope": {
///            "type": "string",
///            "format": "base64",
///            "pattern": "^$|^(?:[0-9a-zA-Z+/]{4})*(?:(?:[0-9a-zA-Z+/]{2}==)|(?:[0-9a-zA-Z+/]{3}=))?$",
///            "contentEncoding": "base64"
///          }
///        }
///      }
///    },
///    "unsigned_tx": {
///      "type": "string"
///    },
///    "wallet_address": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ProofGameRequest {
    pub chain_id: ::std::string::String,
    pub orders: ::std::vec::Vec<ProofGameRequestOrdersItem>,
    pub unsigned_tx: ::std::string::String,
    pub wallet_address: ::std::string::String,
}
///`ProofGameRequestOrdersItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "intent_envelope",
///    "order_id",
///    "value",
///    "wallet_storage_envelope",
///    "wallet_transport_envelope"
///  ],
///  "properties": {
///    "intent_envelope": {
///      "type": "string",
///      "format": "base64",
///      "pattern": "^$|^(?:[0-9a-zA-Z+/]{4})*(?:(?:[0-9a-zA-Z+/]{2}==)|(?:[0-9a-zA-Z+/]{3}=))?$",
///      "contentEncoding": "base64"
///    },
///    "order_id": {
///      "type": "string",
///      "format": "uuid",
///      "pattern": "^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[1-8][0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12}|00000000-0000-0000-0000-000000000000|ffffffff-ffff-ffff-ffff-ffffffffffff)$"
///    },
///    "value": {
///      "type": "number"
///    },
///    "wallet_storage_envelope": {
///      "type": "string",
///      "format": "base64",
///      "pattern": "^$|^(?:[0-9a-zA-Z+/]{4})*(?:(?:[0-9a-zA-Z+/]{2}==)|(?:[0-9a-zA-Z+/]{3}=))?$",
///      "contentEncoding": "base64"
///    },
///    "wallet_transport_envelope": {
///      "type": "string",
///      "format": "base64",
///      "pattern": "^$|^(?:[0-9a-zA-Z+/]{4})*(?:(?:[0-9a-zA-Z+/]{2}==)|(?:[0-9a-zA-Z+/]{3}=))?$",
///      "contentEncoding": "base64"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ProofGameRequestOrdersItem {
    pub intent_envelope: ::std::string::String,
    pub order_id: ::uuid::Uuid,
    pub value: f64,
    pub wallet_storage_envelope: ::std::string::String,
    pub wallet_transport_envelope: ::std::string::String,
}
///`ProofGameResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "results"
///  ],
///  "properties": {
///    "results": {
///      "type": "array",
///      "items": {
///        "type": "object",
///        "required": [
///          "order_id"
///        ],
///        "properties": {
///          "enclave_error": {
///            "type": "string"
///          },
///          "order_id": {
///            "type": "string"
///          },
///          "signature": {
///            "type": "string",
///            "format": "base64",
///            "pattern": "^$|^(?:[0-9a-zA-Z+/]{4})*(?:(?:[0-9a-zA-Z+/]{2}==)|(?:[0-9a-zA-Z+/]{3}=))?$",
///            "contentEncoding": "base64"
///          }
///        },
///        "additionalProperties": false
///      }
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ProofGameResponse {
    pub results: ::std::vec::Vec<ProofGameResponseResultsItem>,
}
///`ProofGameResponseResultsItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "order_id"
///  ],
///  "properties": {
///    "enclave_error": {
///      "type": "string"
///    },
///    "order_id": {
///      "type": "string"
///    },
///    "signature": {
///      "type": "string",
///      "format": "base64",
///      "pattern": "^$|^(?:[0-9a-zA-Z+/]{4})*(?:(?:[0-9a-zA-Z+/]{2}==)|(?:[0-9a-zA-Z+/]{3}=))?$",
///      "contentEncoding": "base64"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ProofGameResponseResultsItem {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub enclave_error: ::std::option::Option<::std::string::String>,
    pub order_id: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub signature: ::std::option::Option<::std::string::String>,
}
