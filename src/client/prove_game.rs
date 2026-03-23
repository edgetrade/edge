//! Prove game client module for Edge CLI.
//!
//! Provides a type-safe wrapper around the Iris tRPC agent.proveGame endpoint,
//! handling network communication with proper error handling for batch intent
//! execution without blockchain interaction.

use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::client::IrisClient;
use crate::wallet::types::WalletError;

/// Request payload for the prove game API call.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProveGameRequest {
    pub game_session_id: String,
    pub intents: Vec<ExecuteIntent>,
}

/// ExecuteIntent for the prove game - matches the TypeScript protobuf definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecuteIntent {
    pub order_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    pub chain_id: String,
    pub wallet_address: String,
    pub value: String,
    #[serde(with = "serde_bytes")]
    pub data: Vec<u8>,
    #[serde(with = "serde_bytes")]
    pub envelope: Vec<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub svm: Option<SvmParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evm: Option<EvmParameters>,
}

/// SVM-specific parameters for execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SvmParameters {
    #[serde(rename = "recentBlocknumber")]
    pub recent_blocknumber: String,
    #[serde(rename = "recentBlockhash")]
    pub recent_blockhash: String,
    #[serde(rename = "txMethod")]
    pub tx_method: String,
}

/// EVM-specific parameters for execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvmParameters {
    #[serde(rename = "txMethod")]
    pub tx_method: String,
    #[serde(rename = "flashbotPrivateKey", skip_serializing_if = "Option::is_none")]
    pub flashbot_private_key: Option<String>,
    #[serde(rename = "gasLimit", skip_serializing_if = "Option::is_none")]
    pub gas_limit: Option<String>,
    #[serde(rename = "maxBaseGas", skip_serializing_if = "Option::is_none")]
    pub max_base_gas: Option<String>,
    #[serde(rename = "priorityGas", skip_serializing_if = "Option::is_none")]
    pub priority_gas: Option<String>,
}

/// Response from the prove game API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProveGameResponse {
    #[serde(rename = "gameSessionId")]
    pub game_session_id: String,
    pub attempts: Vec<ExecutionAttempt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    pub success: bool,
}

/// Result of a single execution attempt.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionAttempt {
    #[serde(rename = "orderId")]
    pub order_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<ExecutionSuccess>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure: Option<ExecutionFailure>,
    #[serde(rename = "walletAccessed")]
    pub wallet_accessed: bool,
}

/// Successful execution result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionSuccess {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    #[serde(rename = "txHash", skip_serializing_if = "Option::is_none")]
    pub tx_hash: Option<String>,
}

/// Failed execution result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionFailure {
    #[serde(rename = "errorCode")]
    pub error_code: String,
    #[serde(rename = "errorMessage", skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

/// Error type for prove game operations.
#[derive(Debug, Clone, thiserror::Error)]
pub enum ProveGameError {
    #[error("Network error: {0}")]
    Network(String),
    #[error("API error: {0}")]
    Api(String),
    #[error("Serialization error: {0}")]
    Serialization(String),
    #[error("Invalid response: {0}")]
    InvalidResponse(String),
}

impl From<WalletError> for ProveGameError {
    fn from(e: WalletError) -> Self {
        ProveGameError::Api(e.to_string())
    }
}

/// Call the proveGame tRPC endpoint with a batch of sealed intents.
///
/// This function sends a batch of intents to the Iris backend for execution
/// testing. Each intent is processed by the enclave to verify wallet access
/// and constraint validation without submitting to the blockchain.
///
/// # Arguments
/// * `game_session_id` - Unique identifier for this game session
/// * `intents` - Vector of ExecuteIntent messages to test
/// * `client` - The Iris API client
///
/// # Returns
/// * `Ok(ProveGameResponse)` - The execution results for all intents
/// * `Err(ProveGameError)` - If the API call fails
/// ```
pub async fn prove_game(request: ProveGameRequest, client: &IrisClient) -> Result<ProveGameResponse, ProveGameError> {
    // Serialize intents to JSON-compatible format
    let intents_json: Vec<serde_json::Value> = request
        .intents
        .into_iter()
        .map(|intent| {
            json!({
                "orderId": intent.order_id,
                "userId": intent.user_id,
                "agentId": intent.agent_id,
                "chainId": intent.chain_id,
                "walletAddress": intent.wallet_address,
                "value": intent.value,
                "data": STANDARD.encode(&intent.data),
                "envelope": STANDARD.encode(&intent.envelope),
                "svm": intent.svm,
                "evm": intent.evm,
            })
        })
        .collect();

    let input = json!({
        "gameSessionId": request.game_session_id,
        "intents": intents_json,
    });

    let response: ProveGameResponse = client
        .mutation("agent.proveGame", input)
        .await
        .map_err(|e| ProveGameError::Network(e.to_string()))?;

    Ok(response)
}

/// Simplified prove game call with direct parameters.
///
/// This is a convenience wrapper that constructs the request and calls
/// the prove_game function in one step.
///
/// # Arguments
/// * `game_session_id` - Unique identifier for this game session
/// * `intents` - Vector of ExecuteIntent messages to test
/// * `client` - The Iris API client
///
/// # Returns
/// The ProveGameResponse containing results for all intents.
pub async fn prove_game_with_intents(
    game_session_id: String,
    intents: Vec<ExecuteIntent>,
    client: &IrisClient,
) -> Result<ProveGameResponse, ProveGameError> {
    let request = ProveGameRequest {
        game_session_id,
        intents,
    };
    prove_game(request, client).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execution_attempt_success_parsing() {
        let json = r#"{
            "orderId": "order-1",
            "success": {
                "signature": "base64signature",
                "txHash": null
            },
            "failure": null,
            "walletAccessed": true
        }"#;

        let attempt: ExecutionAttempt = serde_json::from_str(json).unwrap();
        assert_eq!(attempt.order_id, "order-1");
        assert!(attempt.success.is_some());
        assert!(attempt.failure.is_none());
        assert!(attempt.wallet_accessed);
    }

    #[test]
    fn test_execution_attempt_failure_parsing() {
        let json = r#"{
            "orderId": "order-2",
            "success": null,
            "failure": {
                "errorCode": "WALLET_NOT_FOUND",
                "errorMessage": "Wallet does not exist"
            },
            "walletAccessed": false
        }"#;

        let attempt: ExecutionAttempt = serde_json::from_str(json).unwrap();
        assert_eq!(attempt.order_id, "order-2");
        assert!(attempt.success.is_none());
        assert!(attempt.failure.is_some());
        assert!(!attempt.wallet_accessed);

        let failure = attempt.failure.unwrap();
        assert_eq!(failure.error_code, "WALLET_NOT_FOUND");
        assert_eq!(failure.error_message, Some("Wallet does not exist".to_string()));
    }

    #[test]
    fn test_prove_game_response_parsing() {
        let json = r#"{
            "gameSessionId": "session-abc",
            "attempts": [
                {
                    "orderId": "order-1",
                    "success": {"signature": "sig1"},
                    "walletAccessed": true
                }
            ],
            "success": true
        }"#;

        let response: ProveGameResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.game_session_id, "session-abc");
        assert_eq!(response.attempts.len(), 1);
        assert!(response.success);
        assert!(response.error.is_none());
    }

    #[test]
    fn test_execute_intent_creation() {
        let intent = ExecuteIntent {
            order_id: "order-123".to_string(),
            user_id: Some("user-456".to_string()),
            agent_id: None,
            chain_id: "1".to_string(),
            wallet_address: "0xabc123".to_string(),
            value: "1000000000000000000".to_string(),
            data: vec![0x01, 0x02, 0x03],
            envelope: vec![0x04, 0x05, 0x06],
            svm: None,
            evm: Some(EvmParameters {
                tx_method: "FLASHBOT".to_string(),
                flashbot_private_key: None,
                gas_limit: Some("100000".to_string()),
                max_base_gas: None,
                priority_gas: None,
            }),
        };

        assert_eq!(intent.order_id, "order-123");
        assert_eq!(intent.chain_id, "1");
        assert_eq!(intent.data, vec![0x01, 0x02, 0x03]);
    }
}
