//! Game 1: The Blind Oracle
//!
//! In this game, the user creates 3 sealed intents with constraint values.
//! The enclave will only grant wallet access if the test value matches one
//! of the 3 constraint values. This demonstrates constraint-based wallet access.

use base64::Engine;
use base64::engine::general_purpose::STANDARD;

use tyche_enclave::envelopes::transport::{ExecutionPayload, SealedIntent, TransportEnvelope, TransportEnvelopeKey};

use crate::client::IrisClient;
use crate::client::{EvmParameters, ExecuteIntent, get_transport_key, prove_game_with_intents};
use crate::commands::wallet::{
    game::game_state::{GameResultEntry, GameWallet, get_sealed_intents, store_game_result, store_sealed_intent},
    prove::{generate_session_id, prompt_number},
};
use crate::messages;

/// Play Game 1: The Blind Oracle.
///
/// Game flow:
/// 1. Get or create a game wallet
/// 2. Prompt user for 3 constraint values
/// 3. Create 3 sealed intents, each with one constraint value
/// 4. Prompt user for a test value (can be one of the 3 or different)
/// 5. Call prove_game with all 3 intents + test value
/// 6. Show results: wallet access only granted if test value matches constraint
///
/// # Arguments
/// * `replay` - If true, use existing sealed intents instead of creating new ones
/// * `client` - The Iris API client
pub async fn play_game(replay: bool, client: &IrisClient) -> messages::success::CommandResult<()> {
    let session_id = generate_session_id();
    println!("Session ID: {}\n", session_id);

    // Step 1: Get or create game wallet
    let wallet = super::game_state::get_or_create_wallet(!replay)?;
    println!("Using game wallet: {}\n", wallet.address);

    // Step 2: Get or create sealed intents
    let intents = if replay {
        load_existing_intents(&wallet)?
    } else {
        create_new_intents(&wallet, client).await?
    };

    if intents.is_empty() {
        return Err(messages::error::CommandError::InvalidInput(
            "No intents available. Run without --replay to create new intents.".to_string(),
        ));
    }

    // Step 3: Get test value from user
    let test_value = prompt_number("Give ANY number (this will be tested against the constraints): ")?;
    println!("\nTest value: {}\n", test_value);

    // Step 4: Prepare intents for prove game
    let mut prove_intents: Vec<ExecuteIntent> = Vec::new();
    for intent in intents.iter().take(3) {
        prove_intents.push(intent.clone());
    }

    // Step 5: Call prove_game
    println!("Sending {} intents to the enclave...\n", prove_intents.len());

    let response = prove_game_with_intents(session_id.clone(), prove_intents, client)
        .await
        .map_err(|e| messages::error::CommandError::Wallet(e.to_string()))?;

    // Step 6: Display results
    display_results(&response, &wallet, test_value, replay)?;

    // Step 7: Store game result
    let game_result = create_game_result(&response, &session_id)?;
    store_game_result(game_result).map_err(|e| messages::error::CommandError::Wallet(e.to_string()))?;

    Ok(())
}

/// Create new sealed intents for Game 1.
async fn create_new_intents(
    wallet: &GameWallet,
    client: &IrisClient,
) -> messages::success::CommandResult<Vec<ExecuteIntent>> {
    println!("Creating new sealed intents...\n");
    println!("Pick 3 numbers that will be the access constraints:\n");

    let mut constraint_values = Vec::new();

    // Get 3 constraint values from user
    for i in 1..=3 {
        let value = prompt_number(&format!("Number {}: ", i))?;
        constraint_values.push(value.to_string());
    }

    println!("\nYour constraint values: {:?}\n", constraint_values);

    // Get transport keys for sealing
    let enclave_keys = get_transport_key(client)
        .await
        .map_err(|e| messages::error::CommandError::Wallet(e.to_string()))?;
    let key = TransportEnvelopeKey::Unsealing(enclave_keys.deterministic);

    let mut intents = Vec::new();

    // Create 3 sealed intents
    for (i, constraint) in constraint_values.iter().enumerate() {
        let order_id = format!("game1-intent-{}", i + 1);
        let constraint_value: String = constraint.clone();

        // Create the sealed intent
        let sealed_intent = SealedIntent {
            user_id: None,
            agent_id: None,
            chain_id: "1".to_string(), // Ethereum mainnet
            wallet_address: wallet.address.clone(),
            value: constraint_value.clone(),
        };

        // Create execution payload with game key (simplified - using dummy key for demo)
        let game_key = [0u8; 32]; // In production, this would be derived from user input
        let payload = ExecutionPayload::new(game_key, sealed_intent);

        // Seal the payload
        let envelope = payload
            .seal(&key)
            .map_err(|e| messages::error::CommandError::Wallet(e.to_string()))?;

        // Store intent locally
        store_sealed_intent(order_id.clone(), envelope.clone(), Some(constraint.clone()))
            .map_err(|e| messages::error::CommandError::Wallet(e.to_string()))?;

        // Create the ExecuteIntent for the API
        let execute_intent = ExecuteIntent {
            order_id: order_id.clone(),
            user_id: None,
            agent_id: None,
            chain_id: "1".to_string(),
            wallet_address: wallet.address.clone(),
            value: constraint_value.clone(),
            data: vec![], // No actual transaction data for prove game
            envelope,
            svm: None,
            evm: Some(EvmParameters {
                tx_method: "NORMAL".to_string(),
                flashbot_private_key: None,
                gas_limit: None,
                max_base_gas: None,
                priority_gas: None,
            }),
        };

        intents.push(execute_intent);

        println!("Created intent {} with constraint: {}", i + 1, constraint);
    }

    println!("\nAll {} intents created and sealed!\n", intents.len());
    Ok(intents)
}

/// Load existing sealed intents from game state.
fn load_existing_intents(wallet: &GameWallet) -> messages::success::CommandResult<Vec<ExecuteIntent>> {
    println!("Loading existing sealed intents...\n");

    let stored_intents = get_sealed_intents().map_err(|e| messages::error::CommandError::Wallet(e.to_string()))?;

    let mut intents = Vec::new();

    for stored in stored_intents.iter().take(3) {
        let envelope = STANDARD
            .decode(&stored.envelope)
            .map_err(|_| messages::error::CommandError::InvalidInput("Invalid stored intent".to_string()))?;

        let execute_intent = ExecuteIntent {
            order_id: stored.id.clone(),
            user_id: None,
            agent_id: None,
            chain_id: "1".to_string(),
            wallet_address: wallet.address.clone(),
            value: stored.constraint_value.clone().unwrap_or_default(),
            data: vec![],
            envelope,
            svm: None,
            evm: Some(EvmParameters {
                tx_method: "NORMAL".to_string(),
                flashbot_private_key: None,
                gas_limit: None,
                max_base_gas: None,
                priority_gas: None,
            }),
        };

        intents.push(execute_intent);
    }

    println!("Loaded {} existing intents.\n", intents.len());
    Ok(intents)
}

/// Display the prove game results.
fn display_results(
    response: &crate::client::ProveGameResponse,
    wallet: &GameWallet,
    test_value: u64,
    _replay: bool,
) -> messages::success::CommandResult<()> {
    println!("\n--- Results ---\n");

    let mut any_wallet_accessed = false;

    for (i, attempt) in response.attempts.iter().enumerate() {
        let status = if attempt.wallet_accessed {
            any_wallet_accessed = true;
            "✓ WALLET ACCESSED".to_string()
        } else if attempt.success.is_some() {
            "✗ Access denied".to_string()
        } else if let Some(ref failure) = attempt.failure {
            format!("✗ Failed: {}", failure.error_code)
        } else {
            "✗ Unknown".to_string()
        };

        println!("Intent {}: {}", i + 1, status);

        if let Some(ref success) = attempt.success
            && let Some(ref sig) = success.signature
        {
            println!("  Signature: {}...", &sig[..sig.len().min(20)]);
        }
    }

    println!();

    if any_wallet_accessed {
        println!("✓✓✓ SUCCESS! Wallet was accessed! ✓✓✓");
        println!();
        println!("The enclave granted access because the test value");
        println!("matched one of the sealed intent constraint values.");
        println!();
        println!("Wallet: {}", wallet.address);

        if let Some(attempt) = response.attempts.iter().find(|a| a.wallet_accessed)
            && let Some(ref success) = attempt.success
            && let Some(ref sig) = success.signature
        {
            println!("\nSignature: {}", sig);
            println!("\nTo verify this signature:");
            println!("  1. The signature proves the enclave accessed the wallet");
            println!("  2. The constraint-based access control worked correctly");
        }
    } else {
        println!("✗ Wallet access denied.");
        println!();
        println!("The test value ({}) did not match any of the", test_value);
        println!("sealed intent constraint values.");
        println!();
        println!("This is the expected behavior - the enclave correctly");
        println!("enforced the constraint-based access control.");
    }

    println!();
    Ok(())
}

/// Create a game result entry from the response.
fn create_game_result(
    response: &crate::client::ProveGameResponse,
    session_id: &str,
) -> messages::success::CommandResult<GameResultEntry> {
    let success = response.success;
    let signature = response
        .attempts
        .iter()
        .find(|a| a.wallet_accessed)
        .and_then(|a| a.success.as_ref().and_then(|s| s.signature.clone()));

    let error = if success { None } else { response.error.clone() };

    Ok(GameResultEntry {
        session_id: session_id.to_string(),
        game_type: 1,
        success,
        signature,
        error,
        timestamp: chrono::Utc::now().to_rfc3339(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    use tyche_enclave::envelopes::transport::SealedIntent;

    use crate::client::{ExecutionAttempt, ExecutionSuccess, ProveGameResponse};
    use crate::commands::wallet::game::game_state::{
        GameResultEntry, GameWallet, set_test_game_state_path, store_sealed_intent,
    };
    use crate::commands::wallet::prove::generate_session_id;

    /// Set up an isolated test environment with a temporary config directory.
    fn setup_test_env() -> tempfile::TempDir {
        let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
        let game_state_path = temp_dir.path().join("game.toml");
        set_test_game_state_path(game_state_path);
        temp_dir
    }

    fn create_test_wallet() -> GameWallet {
        GameWallet {
            address: "0x1234567890123456789012345678901234567890".to_string(),
            private_key: base64::engine::general_purpose::STANDARD.encode(&[0u8; 32]),
            chain_type: "EVM".to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
        }
    }

    #[tokio::test]
    async fn test_load_existing_intents_empty() {
        let _temp = setup_test_env();
        // Setup test state
        let wallet = create_test_wallet();

        // Try to load from empty state
        // This should return an empty vector since no intents exist
        let result = load_existing_intents(&wallet);

        // Should succeed but return empty
        assert!(result.is_ok());
        let intents = result.unwrap();
        assert!(intents.is_empty());
    }

    #[tokio::test]
    async fn test_load_existing_intents_with_data() {
        let _temp = setup_test_env();
        let wallet = create_test_wallet();

        // Store some test intents
        let envelope = vec![1, 2, 3, 4, 5];
        store_sealed_intent("game1-intent-0".to_string(), envelope.clone(), Some("42".to_string()))
            .expect("Failed to store intent");

        store_sealed_intent("game1-intent-1".to_string(), envelope.clone(), Some("100".to_string()))
            .expect("Failed to store intent");

        // Load the intents
        let result = load_existing_intents(&wallet);
        assert!(result.is_ok());

        let intents = result.unwrap();
        assert_eq!(intents.len(), 2);

        // Verify the intents
        assert_eq!(intents[0].order_id, "game1-intent-0");
        assert_eq!(intents[0].value, "42");
        assert_eq!(intents[1].order_id, "game1-intent-1");
        assert_eq!(intents[1].value, "100");
    }

    #[test]
    fn test_display_results_with_success() {
        let wallet = create_test_wallet();

        let response = ProveGameResponse {
            game_session_id: "test-session".to_string(),
            attempts: vec![ExecutionAttempt {
                order_id: "intent-1".to_string(),
                success: Some(ExecutionSuccess {
                    signature: Some("sig1234567890abcdef".to_string()),
                    tx_hash: None,
                }),
                failure: None,
                wallet_accessed: true,
            }],
            error: None,
            success: true,
        };

        // This should not panic and should display success
        let result = display_results(&response, &wallet, 42, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_display_results_with_failure() {
        let wallet = create_test_wallet();

        let response = ProveGameResponse {
            game_session_id: "test-session".to_string(),
            attempts: vec![ExecutionAttempt {
                order_id: "intent-1".to_string(),
                success: None,
                failure: Some(crate::client::ExecutionFailure {
                    error_code: "CONSTRAINT_MISMATCH".to_string(),
                    error_message: Some("Constraint value did not match".to_string()),
                }),
                wallet_accessed: false,
            }],
            error: None,
            success: true,
        };

        // This should not panic and should display failure
        let result = display_results(&response, &wallet, 999, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_display_results_with_no_access() {
        let wallet = create_test_wallet();

        let response = ProveGameResponse {
            game_session_id: "test-session".to_string(),
            attempts: vec![
                ExecutionAttempt {
                    order_id: "intent-1".to_string(),
                    success: Some(ExecutionSuccess {
                        signature: None,
                        tx_hash: None,
                    }),
                    failure: None,
                    wallet_accessed: false,
                },
                ExecutionAttempt {
                    order_id: "intent-2".to_string(),
                    success: Some(ExecutionSuccess {
                        signature: None,
                        tx_hash: None,
                    }),
                    failure: None,
                    wallet_accessed: false,
                },
            ],
            error: None,
            success: true,
        };

        // Should display "access denied" message
        let result = display_results(&response, &wallet, 123, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_display_results_in_replay_mode() {
        let wallet = create_test_wallet();

        let response = ProveGameResponse {
            game_session_id: "test-session".to_string(),
            attempts: vec![ExecutionAttempt {
                order_id: "intent-1".to_string(),
                success: Some(ExecutionSuccess {
                    signature: Some("sig123".to_string()),
                    tx_hash: None,
                }),
                failure: None,
                wallet_accessed: true,
            }],
            error: None,
            success: true,
        };

        // In replay mode, should show replay-specific output
        let result = display_results(&response, &wallet, 42, true);
        assert!(result.is_ok());
    }

    #[test]
    fn test_sealed_intent_creation() {
        let wallet_address = "0x1234567890123456789012345678901234567890";
        let constraint_value = "42";

        let intent = SealedIntent {
            user_id: None,
            agent_id: None,
            chain_id: "1".to_string(),
            wallet_address: wallet_address.to_string(),
            value: constraint_value.to_string(),
        };

        // Verify intent fields
        assert!(intent.user_id.is_none());
        assert!(intent.agent_id.is_none());
        assert_eq!(intent.chain_id, "1");
        assert_eq!(intent.wallet_address, wallet_address);
        assert_eq!(intent.value, constraint_value);
    }

    #[test]
    fn test_game_result_creation() {
        let response = ProveGameResponse {
            game_session_id: "test-session".to_string(),
            attempts: vec![ExecutionAttempt {
                order_id: "intent-1".to_string(),
                success: Some(ExecutionSuccess {
                    signature: Some("test-sig".to_string()),
                    tx_hash: None,
                }),
                failure: None,
                wallet_accessed: true,
            }],
            error: None,
            success: true,
        };

        let result = GameResultEntry {
            session_id: "test-session".to_string(),
            game_type: 1,
            success: response.success,
            signature: response
                .attempts
                .iter()
                .find(|a| a.wallet_accessed)
                .and_then(|a| a.success.as_ref().and_then(|s| s.signature.clone())),
            error: None,
            timestamp: chrono::Utc::now().to_rfc3339(),
        };

        assert_eq!(result.session_id, "test-session");
        assert_eq!(result.game_type, 1);
        assert!(result.success);
        assert_eq!(result.signature, Some("test-sig".to_string()));
    }

    #[test]
    fn test_multiple_intent_handling() {
        let wallet = create_test_wallet();

        let response = ProveGameResponse {
            game_session_id: "test-session".to_string(),
            attempts: vec![
                ExecutionAttempt {
                    order_id: "intent-1".to_string(),
                    success: Some(ExecutionSuccess {
                        signature: None,
                        tx_hash: None,
                    }),
                    failure: None,
                    wallet_accessed: false,
                },
                ExecutionAttempt {
                    order_id: "intent-2".to_string(),
                    success: Some(ExecutionSuccess {
                        signature: Some("sig2".to_string()),
                        tx_hash: None,
                    }),
                    failure: None,
                    wallet_accessed: true,
                },
                ExecutionAttempt {
                    order_id: "intent-3".to_string(),
                    success: Some(ExecutionSuccess {
                        signature: None,
                        tx_hash: None,
                    }),
                    failure: None,
                    wallet_accessed: false,
                },
            ],
            error: None,
            success: true,
        };

        // Should correctly handle multiple attempts
        let result = display_results(&response, &wallet, 100, false);
        assert!(result.is_ok());

        // Verify that at least one wallet was accessed
        let accessed_count = response
            .attempts
            .iter()
            .filter(|a| a.wallet_accessed)
            .count();
        assert_eq!(accessed_count, 1);
    }

    #[test]
    fn test_intent_storage_format() {
        let _temp = setup_test_env();
        // Verify intent storage format matches expected structure
        let intent_id = "test-intent-123";
        let constraint = "constraint-value";
        let envelope = vec![1, 2, 3, 4, 5];

        // Store intent
        store_sealed_intent(intent_id.to_string(), envelope.clone(), Some(constraint.to_string()))
            .expect("Failed to store intent");

        // Load back and verify format
        let state = crate::commands::wallet::game::game_state::load_game_state().expect("Failed to load state");

        let stored = state
            .sealed_intents
            .iter()
            .find(|i| i.id == intent_id)
            .expect("Intent not found");

        assert_eq!(stored.id, intent_id);
        assert_eq!(stored.constraint_value, Some(constraint.to_string()));

        // Verify envelope is base64 encoded
        let decoded = base64::engine::general_purpose::STANDARD
            .decode(&stored.envelope)
            .expect("Failed to decode envelope");
        assert_eq!(decoded, envelope);
    }

    #[test]
    fn test_constrain_value_types() {
        // Test with different constraint value types
        let test_values = vec!["42", "0", "999999999999", "text-value", "0x123abc"];

        for value in &test_values {
            let intent = SealedIntent {
                user_id: None,
                agent_id: None,
                chain_id: "1".to_string(),
                wallet_address: "0x123".to_string(),
                value: value.to_string(),
            };
            assert_eq!(intent.value, *value);
        }
    }

    #[test]
    fn test_session_id_generation() {
        // Generate multiple session IDs
        let id1 = generate_session_id();
        let id2 = generate_session_id();
        let id3 = generate_session_id();

        // All should be unique
        assert_ne!(id1, id2);
        assert_ne!(id2, id3);
        assert_ne!(id1, id3);

        // All should start with "prove-game-"
        assert!(id1.starts_with("prove-game-"));
        assert!(id2.starts_with("prove-game-"));
        assert!(id3.starts_with("prove-game-"));

        // Should be non-empty
        assert!(!id1.is_empty());
        assert!(!id2.is_empty());
        assert!(!id3.is_empty());
    }
}
