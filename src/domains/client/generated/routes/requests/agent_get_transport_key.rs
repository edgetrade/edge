#![allow(clippy::derivable_impls)]

use crate::client::{Route, RouteType};
use std::marker::PhantomData;
/// Route metadata for this procedure
pub const ROUTE: Route<(), GetTransportKeyResponse> = Route {
    procedure: "agent.getTransportKey",
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
///Output containing transport keys for inflight encryption
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Output containing transport keys for inflight encryption",
///  "type": "object",
///  "required": [
///    "agentId",
///    "attestation",
///    "deterministic",
///    "ephemeral"
///  ],
///  "properties": {
///    "agentId": {
///      "description": "The agent ID",
///      "type": "string",
///      "format": "uuid",
///      "pattern": "^([0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[1-8][0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12}|00000000-0000-0000-0000-000000000000|ffffffff-ffff-ffff-ffff-ffffffffffff)$"
///    },
///    "attestation": {
///      "description": "Base64-encoded attestation public key for transport encryption",
///      "type": "string",
///      "format": "base64",
///      "pattern": "^$|^(?:[0-9a-zA-Z+/]{4})*(?:(?:[0-9a-zA-Z+/]{2}==)|(?:[0-9a-zA-Z+/]{3}=))?$",
///      "contentEncoding": "base64"
///    },
///    "deterministic": {
///      "description": "Base64-encoded deterministic public key for transport encryption",
///      "type": "string",
///      "format": "base64",
///      "pattern": "^$|^(?:[0-9a-zA-Z+/]{4})*(?:(?:[0-9a-zA-Z+/]{2}==)|(?:[0-9a-zA-Z+/]{3}=))?$",
///      "contentEncoding": "base64"
///    },
///    "ephemeral": {
///      "description": "Base64-encoded ephemeral public key for transport encryption",
///      "type": "string",
///      "format": "base64",
///      "pattern": "^$|^(?:[0-9a-zA-Z+/]{4})*(?:(?:[0-9a-zA-Z+/]{2}==)|(?:[0-9a-zA-Z+/]{3}=))?$",
///      "contentEncoding": "base64"
///    }
///  },
///  "additionalProperties": false,
///  "name": "Get Transport Key Output"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct GetTransportKeyResponse {
    ///The agent ID
    #[serde(rename = "agentId")]
    pub agent_id: ::uuid::Uuid,
    ///Base64-encoded attestation public key for transport encryption
    pub attestation: ::std::string::String,
    ///Base64-encoded deterministic public key for transport encryption
    pub deterministic: ::std::string::String,
    ///Base64-encoded ephemeral public key for transport encryption
    pub ephemeral: ::std::string::String,
}
