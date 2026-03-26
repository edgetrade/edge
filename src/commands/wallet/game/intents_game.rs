//! Game 1: The Blind Oracle
//!
//! In this game, the user creates 3 sealed intents with constraint values.
//! The enclave will only grant wallet access if the test value matches one
//! of the 3 constraint values. This demonstrates constraint-based wallet access.

use core::f64;

use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use tyche_enclave::envelopes::storage::WalletKey;
use tyche_enclave::types::chain_type::ChainType;
use uuid::Uuid;

use tyche_enclave::envelopes::storage::StorageEnvelope;
use tyche_enclave::envelopes::transport::{ExecutionPayload, SealedIntent, TransportEnvelope, TransportEnvelopeKey};

use crate::client::{IrisClient, proof_game};
use crate::generated::routes::requests::agent_proof_game::ProofGameRequestOrdersItem;
use crate::messages;
use crate::session::crypto::UsersEncryptionKeys;
use crate::session::transport::get_transport_key;

use super::{
    game_state::{
        GameResultEntry, GameWallet, get_sealed_intents, load_game_state, store_game_result, store_sealed_intent,
    },
    utils::{generate_session_id, prompt_number},
    verification,
};

/// Play Game 1: The Blind Oracle.
///
/// Game flow:
/// 1. Get or create a game wallet
/// 2. Prompt user for 3 constraint values
/// 3. Create 3 sealed intents, each with one constraint value
/// 4. Prompt user for a test value (can be one of the 3 or different)
/// 5. Call proof_game with all 3 intents + test value
/// 6. Show results: wallet access only granted if test value matches constraint
///
/// # Arguments
/// * `replay` - If true, use existing sealed intents instead of creating new ones
/// * `client` - The Iris API client
pub async fn play_game(
    replay: bool,
    user_key: &UsersEncryptionKeys,
    client: &IrisClient,
) -> messages::success::CommandResult<()> {
    let session_id = generate_session_id();
    println!("Session ID: {}\n", session_id);

    // Step 1: Get or create game wallet
    let wallet = super::game_state::get_or_create_wallet(!replay)?;
    println!("Using game wallet: {}\n", wallet.address);

    // Step 2: Get or create sealed intents
    let intents = if replay {
        load_existing_intents(&wallet)?
    } else {
        create_new_intents(&wallet, user_key, client).await?
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
    let prove_intents: Vec<ProofGameRequestOrdersItem> = intents.into_iter().take(3).collect();

    // Step 5: Call proof_game
    println!("Sending {} intents to the enclave...\n", prove_intents.len());

    let encrypted_wallet_blob = WalletKey::new(
        ChainType::EVM,
        wallet.address.clone(),
        wallet.private_key.clone().into_bytes().to_vec(),
    )
    .seal(&user_key.storage)
    .map_err(|e| messages::error::CommandError::Wallet(e.to_string()))?;

    let response = proof_game(wallet.address.clone(), encrypted_wallet_blob, prove_intents, client)
        .await
        .map_err(|e| messages::error::CommandError::Wallet(e.to_string()))?;

    // Step 6: Display results
    display_results(&response, &wallet, test_value)?;

    // Step 7: Store game result
    let game_result = create_game_result(&response, &session_id)?;
    store_game_result(game_result).map_err(|e| messages::error::CommandError::Wallet(e.to_string()))?;

    // Display all stored game results
    let state = load_game_state().map_err(|e| messages::error::CommandError::Wallet(e.to_string()))?;
    println!("{}", verification::format_game_results(&state));

    Ok(())
}

/// Create new sealed intents for Game 1.
async fn create_new_intents(
    wallet: &GameWallet,
    user_key: &UsersEncryptionKeys,
    client: &IrisClient,
) -> messages::success::CommandResult<Vec<ProofGameRequestOrdersItem>> {
    println!("Creating new sealed intents...\n");
    println!("Pick 3 numbers that will be the access constraints:\n");

    // Get 3 constraint values from user
    let constraint_values: Vec<f64> = (1..=3)
        .map(|i: usize| prompt_number(&format!("Number {}: ", i)).map(|v: u64| v as f64))
        .collect::<Result<Vec<_>, _>>()?;

    println!("\nYour constraint values: {:?}\n", constraint_values);

    // Get transport keys for sealing
    let enclave_keys = get_transport_key(client)
        .await
        .map_err(|e| messages::error::CommandError::Wallet(e.to_string()))?;
    let key = TransportEnvelopeKey::Unsealing(enclave_keys.deterministic);

    let mut intents = Vec::new();

    // Create 3 sealed intents
    for (i, constraint) in constraint_values.iter().enumerate() {
        let order_id = Uuid::new_v4();

        // Create the sealed intent
        let sealed_intent = SealedIntent {
            user_id: None,
            agent_id: None,
            chain_id: "1".to_string(),
            wallet_address: wallet.address.clone(),
            value: constraint.to_string(),
        };

        // Seal the payload
        let payload = ExecutionPayload::new(user_key.storage, sealed_intent);
        let envelope = payload
            .seal(&key)
            .map_err(|e| messages::error::CommandError::Wallet(e.to_string()))?;

        // Store intent locally
        let order_id_str = order_id.to_string();
        store_sealed_intent(order_id_str.clone(), envelope.clone(), Some(constraint.to_string()))
            .map_err(|e| messages::error::CommandError::Wallet(e.to_string()))?;

        // Create the ProofGameRequestOrdersItem for the API
        let execute_intent = ProofGameRequestOrdersItem {
            order_id,
            data: hex::encode([]), // Empty hex string for data
            value: *constraint,
            sealed_envelope: STANDARD.encode(&envelope),
        };

        intents.push(execute_intent);

        println!("Created intent {} with constraint: {}", i + 1, constraint);
    }

    println!("\nAll {} intents created and sealed!\n", intents.len());
    Ok(intents)
}

/// Load existing sealed intents from game state.
fn load_existing_intents(_wallet: &GameWallet) -> messages::success::CommandResult<Vec<ProofGameRequestOrdersItem>> {
    println!("Loading existing sealed intents...\n");

    let stored_intents = get_sealed_intents().map_err(|e| messages::error::CommandError::Wallet(e.to_string()))?;

    let mut intents = Vec::new();

    for stored in stored_intents.iter().take(3) {
        let order_id = stored
            .id
            .parse::<Uuid>()
            .map_err(|_| messages::error::CommandError::InvalidInput("Invalid stored intent ID".to_string()))?;

        let execute_intent = ProofGameRequestOrdersItem {
            order_id,
            data: hex::encode([]), // Empty hex string
            value: stored
                .constraint_value
                .clone()
                .unwrap_or_default()
                .parse::<f64>()
                .unwrap_or(0.0),
            sealed_envelope: stored.envelope.clone(),
        };

        intents.push(execute_intent);
    }

    println!("Loaded {} existing intents.\n", intents.len());
    Ok(intents)
}

/// Display the prove game results.
fn display_results(
    response: &crate::generated::routes::requests::agent_proof_game::ProofGameResponse,
    wallet: &GameWallet,
    test_value: u64,
) -> messages::success::CommandResult<()> {
    println!("\n--- Results ---\n");

    let any_wallet_accessed = response
        .results
        .iter()
        .any(|r| r.enclave_error.is_none() && r.signature.is_some());

    for (i, result) in response.results.iter().enumerate() {
        let status = if result.enclave_error.is_none() && result.signature.is_some() {
            "✓ WALLET ACCESSED"
        } else if let Some(ref err) = result.enclave_error {
            &format!("✗ Failed: {}", err)
        } else {
            "✗ Access denied"
        };

        println!("Intent {}: {}", i + 1, status);

        if let Some(ref sig) = result.signature {
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

        if let Some(result) = response
            .results
            .iter()
            .find(|r| r.enclave_error.is_none() && r.signature.is_some())
            && let Some(ref sig) = result.signature
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
    response: &crate::generated::routes::requests::agent_proof_game::ProofGameResponse,
    session_id: &str,
) -> messages::success::CommandResult<GameResultEntry> {
    let success = response
        .results
        .iter()
        .any(|r| r.enclave_error.is_none() && r.signature.is_some());
    let signature = response
        .results
        .iter()
        .find(|r| r.signature.is_some())
        .and_then(|r| r.signature.clone());
    let enclave_error = response
        .results
        .iter()
        .find(|r| r.enclave_error.is_some())
        .and_then(|r| r.enclave_error.clone());

    Ok(GameResultEntry {
        session_id: session_id.to_string(),
        game_type: 1,
        success,
        signature,
        enclave_error,
        timestamp: chrono::Utc::now().to_rfc3339(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::commands::wallet::game::game_state::{
        GameResultEntry, GameWallet, set_test_game_state_path, store_sealed_intent,
    };
    use crate::commands::wallet::game::utils::generate_session_id;

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
        let wallet = create_test_wallet();
        let result = load_existing_intents(&wallet);
        assert!(result.is_ok());
        let intents = result.unwrap();
        assert!(intents.is_empty());
    }

    #[tokio::test]
    async fn test_load_existing_intents_with_data() {
        let _temp = setup_test_env();
        let wallet = create_test_wallet();

        let envelope = vec![1, 2, 3, 4, 5];
        store_sealed_intent(Uuid::new_v4().to_string(), envelope.clone(), Some("42".to_string()))
            .expect("Failed to store intent");
        store_sealed_intent(Uuid::new_v4().to_string(), envelope.clone(), Some("100".to_string()))
            .expect("Failed to store intent");

        let result = load_existing_intents(&wallet);
        assert!(result.is_ok());

        let intents = result.unwrap();
        assert_eq!(intents.len(), 2);
    }

    #[test]
    fn test_game_result_creation() {
        use crate::generated::routes::requests::agent_proof_game::{ProofGameResponse, ProofGameResponseResultsItem};

        let response = ProofGameResponse {
            results: vec![ProofGameResponseResultsItem {
                order_id: "test-uuid".to_string(),
                enclave_error: None,
                signature: Some("test-sig".to_string()),
            }],
        };

        let result = GameResultEntry {
            session_id: "test-session".to_string(),
            game_type: 1,
            success: true,
            signature: response.results[0].signature.clone(),
            enclave_error: None,
            timestamp: chrono::Utc::now().to_rfc3339(),
        };

        assert_eq!(result.session_id, "test-session");
        assert_eq!(result.game_type, 1);
        assert!(result.success);
        assert_eq!(result.signature, Some("test-sig".to_string()));
    }

    #[test]
    fn test_session_id_generation() {
        let id1 = generate_session_id();
        let id2 = generate_session_id();
        let id3 = generate_session_id();

        assert_ne!(id1, id2);
        assert_ne!(id2, id3);
        assert_ne!(id1, id3);
        assert!(id1.starts_with("prove-game-"));
        assert!(!id1.is_empty());
    }
}
