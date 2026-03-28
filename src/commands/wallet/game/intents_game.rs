//! Game 1: The Blind Oracle
//!
//! In this game, the user creates 3 sealed intents with constraint values.
//! The enclave will only grant wallet access if the test value matches one
//! of the 3 constraint values. This demonstrates constraint-based wallet access.

use core::f64;

use alloy::hex::encode_prefixed;
use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use erato::models::ChainId;
use tyche_enclave::envelopes::storage::WalletKey;
use tyche_enclave::types::chain_type::ChainType;
use uuid::Uuid;

use tyche_enclave::envelopes::storage::StorageEnvelope;
use tyche_enclave::envelopes::transport::{
    ExecutionPayload, RotateUserKeyPayload, SealedIntent, TransportEnvelope, TransportEnvelopeKey, WalletUpsert,
};

use crate::client::{IrisClient, proof_game};
use crate::config::Config;
use crate::generated::routes::requests::agent_proof_game::{ProofGameRequest, ProofGameRequestOrdersItem};
use crate::messages;
use crate::session::Session;
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
    session: &Session,
    client: &IrisClient,
) -> messages::success::CommandResult<()> {
    let session_id = generate_session_id();
    println!("Session ID: {}\n", session_id);

    let mut aid = session
        .get_config()
        .map_err(|e| messages::error::CommandError::Session(e.to_string()))?
        .clone()
        .agent_id;

    if aid.is_none() {
        get_transport_key(client).await?;
        let config = Config::load().map_err(|e| messages::error::CommandError::Session(e.to_string()))?;
        let agent_id = config.agent_id;
        if agent_id.is_none() {
            return Err(messages::error::CommandError::InvalidInput(
                "Agent ID not found. Please set the agent ID in the session config.".to_string(),
            ));
        };
        aid = Some(agent_id.unwrap());
    }

    let agent_id = aid.unwrap();

    // Step 1: Get or create game wallet
    let wallet = super::game_state::get_or_create_wallet(!replay)?;
    println!("Using game wallet: {}\n", wallet.address);

    // Step 2: Get or create sealed orders
    let mut orders = if replay {
        load_existing_orders(&wallet, user_key, client).await?
    } else {
        create_new_orders(&wallet, &agent_id, user_key, client).await?
    };

    if orders.is_empty() {
        return Err(messages::error::CommandError::InvalidInput(
            "No orders available. Run without --replay to create new orders.".to_string(),
        ));
    }

    // Step 3: Get test value from user
    let test_value = prompt_number("Give ANY number (this will be tested against the constraints): ")?;
    println!("\nTest value: {}\n", test_value);

    for intent in orders.iter_mut() {
        intent.value = test_value as f64;
    }

    // Step 4: Call proof_game
    println!("Sending {} orders to the enclave...\n", orders.len());
    let request = &ProofGameRequest {
        chain_id: erato::models::ChainId::ETHEREUM.to_string(),
        wallet_address: wallet.address.clone(),
        unsigned_tx: encode_prefixed(test_value.to_be_bytes()),
        orders: orders.into_iter().take(3).collect(),
    };

    let response = proof_game(request, client)
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
async fn create_new_orders(
    wallet: &GameWallet,
    agent_id: &Uuid,
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
    let transport_key = TransportEnvelopeKey::Unsealing(enclave_keys.deterministic);

    let private_key_bytes: [u8; 32] = base64::engine::general_purpose::STANDARD
        .decode(wallet.private_key.clone())
        .map_err(|e| messages::error::CommandError::Wallet(e.to_string()))?
        .try_into()
        .unwrap();

    let sealed_wallet = WalletKey::new(ChainType::EVM, wallet.address.clone(), private_key_bytes)
        .seal(&user_key.storage)
        .map_err(|e| messages::error::CommandError::Wallet(e.to_string()))?;

    // Seal the storage envelope for the "upsert"
    let wallet_storage_envelope = WalletUpsert::new(sealed_wallet)
        .seal(&transport_key)
        .map_err(|e| messages::error::CommandError::Wallet(e.to_string()))?;

    // Seal the transport envelope for the "upsert"
    let wallet_transport_envelope = RotateUserKeyPayload::new(user_key.storage, None)
        .seal(&transport_key)
        .map_err(|e| messages::error::CommandError::Wallet(e.to_string()))?;

    let mut orders = Vec::new();

    // Create 3 sealed orders
    for (i, constraint) in constraint_values.iter().enumerate() {
        let order_id = Uuid::new_v4();

        // Create the sealed intent
        let sealed_intent = SealedIntent {
            user_id: None,
            agent_id: Some(agent_id.to_string()),
            chain_id: ChainId::ETHEREUM.to_string(),
            wallet_address: wallet.address.clone(),
            value: constraint.to_string(),
        };

        // Seal the payload
        let payload = ExecutionPayload::new(user_key.storage, sealed_intent);
        let intent_envelope = payload
            .seal(&transport_key)
            .map_err(|e| messages::error::CommandError::Wallet(e.to_string()))?;

        // Store intent locally
        let order_id_str = order_id.to_string();
        store_sealed_intent(
            order_id_str.clone(),
            intent_envelope.clone(),
            Some(constraint.to_string()),
        )
        .map_err(|e| messages::error::CommandError::Wallet(e.to_string()))?;

        // Create the ProofGameRequestOrdersItem for the API
        let execute_intent = ProofGameRequestOrdersItem {
            order_id,
            value: 0 as f64, // this is just a placeholder. we overwrite later when we know
            intent_envelope: STANDARD.encode(&intent_envelope),
            wallet_storage_envelope: STANDARD.encode(&wallet_storage_envelope),
            wallet_transport_envelope: STANDARD.encode(&wallet_transport_envelope),
        };

        orders.push(execute_intent);

        println!("Created intent {} with constraint: {}", i + 1, constraint);
    }

    println!("\nAll {} orders created and sealed!\n", orders.len());
    Ok(orders)
}

/// Load existing sealed intents from game state.
async fn load_existing_orders(
    wallet: &GameWallet,
    user_key: &UsersEncryptionKeys,
    client: &IrisClient,
) -> messages::success::CommandResult<Vec<ProofGameRequestOrdersItem>> {
    println!("Loading existing sealed intents...\n");

    // Get transport keys for sealing
    let enclave_keys = get_transport_key(client)
        .await
        .map_err(|e| messages::error::CommandError::Wallet(e.to_string()))?;
    let transport_key = TransportEnvelopeKey::Unsealing(enclave_keys.deterministic);

    let private_key_bytes: [u8; 32] = base64::engine::general_purpose::STANDARD
        .decode(wallet.private_key.clone())
        .map_err(|e| messages::error::CommandError::Wallet(e.to_string()))?
        .try_into()
        .unwrap();

    let sealed_wallet = WalletKey::new(ChainType::EVM, wallet.address.clone(), private_key_bytes)
        .seal(&user_key.storage)
        .map_err(|e| messages::error::CommandError::Wallet(e.to_string()))?;

    // Seal the storage envelope for the "upsert"
    let wallet_storage_envelope = WalletUpsert::new(sealed_wallet)
        .seal(&transport_key)
        .map_err(|e| messages::error::CommandError::Wallet(e.to_string()))?;

    // Seal the transport envelope for the "upsert"
    let wallet_transport_envelope = RotateUserKeyPayload::new(user_key.storage, None)
        .seal(&transport_key)
        .map_err(|e| messages::error::CommandError::Wallet(e.to_string()))?;

    let stored_intents = get_sealed_intents().map_err(|e| messages::error::CommandError::Wallet(e.to_string()))?;

    let mut intents = Vec::new();

    for stored in stored_intents.iter().take(3) {
        let order_id = stored
            .id
            .parse::<Uuid>()
            .map_err(|_| messages::error::CommandError::InvalidInput("Invalid stored intent ID".to_string()))?;

        let execute_intent = ProofGameRequestOrdersItem {
            order_id,
            value: stored
                .constraint_value
                .clone()
                .unwrap_or_default()
                .parse::<f64>()
                .unwrap_or(0.0),
            intent_envelope: STANDARD.encode(stored.envelope.clone()),
            wallet_storage_envelope: STANDARD.encode(&wallet_storage_envelope),
            wallet_transport_envelope: STANDARD.encode(&wallet_transport_envelope),
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
