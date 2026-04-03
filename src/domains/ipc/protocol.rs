//! IPC Protocol definitions
//!
//! JSON-RPC protocol for Tauri/CLI-to-daemon communication

pub mod json_rpc {
    use serde::{Deserialize, Serialize};

    /// JSON-RPC Request
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Request {
        /// JSON-RPC version (should be "2.0")
        pub jsonrpc: String,
        /// Request ID (null for notifications)
        pub id: Option<u64>,
        /// Method name
        pub method: String,
        /// Parameters (can be array or object)
        pub params: serde_json::Value,
    }

    /// JSON-RPC Response
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Response {
        /// JSON-RPC version
        pub jsonrpc: String,
        /// Request ID
        pub id: Option<u64>,
        /// Result data (if successful)
        pub result: Option<serde_json::Value>,
        /// Error object (if failed)
        pub error: Option<ErrorObject>,
    }

    /// JSON-RPC Error object
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ErrorObject {
        /// Error code
        pub code: i32,
        /// Error message
        pub message: String,
        /// Additional error data
        pub data: Option<serde_json::Value>,
    }

    /// JSON-RPC Notification (request without ID)
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Notification {
        /// JSON-RPC version
        pub jsonrpc: String,
        /// Method name
        pub method: String,
        /// Parameters
        pub params: serde_json::Value,
    }

    impl Request {
        /// Create a new JSON-RPC request
        pub fn new(id: u64, method: impl Into<String>, params: serde_json::Value) -> Self {
            Self {
                jsonrpc: "2.0".to_string(),
                id: Some(id),
                method: method.into(),
                params,
            }
        }
    }

    impl Response {
        /// Create a successful response
        pub fn success(id: u64, result: serde_json::Value) -> Self {
            Self {
                jsonrpc: "2.0".to_string(),
                id: Some(id),
                result: Some(result),
                error: None,
            }
        }

        /// Create an error response
        pub fn error(id: Option<u64>, code: i32, message: impl Into<String>, data: Option<serde_json::Value>) -> Self {
            Self {
                jsonrpc: "2.0".to_string(),
                id,
                result: None,
                error: Some(ErrorObject {
                    code,
                    message: message.into(),
                    data,
                }),
            }
        }
    }

    impl Notification {
        /// Create a new JSON-RPC notification
        pub fn new(method: impl Into<String>, params: serde_json::Value) -> Self {
            Self {
                jsonrpc: "2.0".to_string(),
                method: method.into(),
                params,
            }
        }
    }

    /// Standard JSON-RPC error codes
    pub mod error_codes {
        /// Parse error (-32700)
        pub const PARSE_ERROR: i32 = -32700;
        /// Invalid request (-32600)
        pub const INVALID_REQUEST: i32 = -32600;
        /// Method not found (-32601)
        pub const METHOD_NOT_FOUND: i32 = -32601;
        /// Invalid params (-32602)
        pub const INVALID_PARAMS: i32 = -32602;
        /// Internal error (-32603)
        pub const INTERNAL_ERROR: i32 = -32603;
        /// Server error range (-32000 to -32099)
        pub const SERVER_ERROR_START: i32 = -32000;
        pub const SERVER_ERROR_END: i32 = -32099;
    }
}

#[cfg(test)]
mod tests {
    use super::json_rpc::*;

    #[test]
    fn test_request_serialization() {
        let req = Request::new(1, "get_config", serde_json::json!({"key": "api.url"}));
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("jsonrpc"));
        assert!(json.contains("get_config"));
    }

    #[test]
    fn test_response_success() {
        let resp = Response::success(1, serde_json::json!({"value": "https://api.example.com"}));
        assert!(resp.result.is_some());
        assert!(resp.error.is_none());
        assert_eq!(resp.id, Some(1));
    }

    #[test]
    fn test_response_error() {
        let resp = Response::error(Some(1), error_codes::METHOD_NOT_FOUND, "Method not found", None);
        assert!(resp.result.is_none());
        assert!(resp.error.is_some());
        let err = resp.error.unwrap();
        assert_eq!(err.code, -32601);
        assert_eq!(err.message, "Method not found");
    }

    #[test]
    fn test_notification() {
        let notif = Notification::new("event", serde_json::json!({"type": "update"}));
        assert_eq!(notif.jsonrpc, "2.0");
        assert_eq!(notif.method, "event");
    }

    #[test]
    fn test_error_codes() {
        assert_eq!(error_codes::PARSE_ERROR, -32700);
        assert_eq!(error_codes::INVALID_REQUEST, -32600);
        assert_eq!(error_codes::METHOD_NOT_FOUND, -32601);
        assert_eq!(error_codes::INVALID_PARAMS, -32602);
        assert_eq!(error_codes::INTERNAL_ERROR, -32603);
    }

    #[test]
    fn test_request_deserialization() {
        let json = r#"{"jsonrpc":"2.0","id":42,"method":"test","params":{"key":"value"}}"#;
        let req: Request = serde_json::from_str(json).unwrap();
        assert_eq!(req.id, Some(42));
        assert_eq!(req.method, "test");
    }
}
