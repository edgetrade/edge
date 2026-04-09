//! Game 1: The Blind Oracle
//!
//! In this game, the user creates 3 sealed intents with constraint values.
//! The enclave will only grant wallet access if the test value matches one
//! of the 3 constraint values. This demonstrates constraint-based wallet access.

use core::f64;

use alloy::hex::encode_prefixed;
use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use uuid::Uuid;

use erato::messages::envelopes::{
    storage::StorageEnvelope,
    storage::WalletKey,
    transport::{
        ExecutionPayload, RotateUserKeyPayload, SealedIntent, TransportEnvelope, TransportEnvelopeKey, WalletUpsert,
    },
};
use erato::{ChainId, ChainType};

use crate::domains::client::generated::routes::requests::agent_proof_game::{
    ProofGameRequest, ProofGameRequestOrdersItem,
};
use crate::domains::client::{IrisClient, proof_game};
use crate::domains::enclave::errors::EnclaveError;
use crate::domains::enclave::errors::EnclaveResult;
use crate::domains::keystore::Session;
use crate::domains::keystore::session::crypto::UsersEncryptionKeys;
use crate::messages::prompt_number;
use crate::messages::prompt_secret_number;

use super::{
    game_state::{GameResultEntry, GameWallet, load_game_state, store_game_result, store_sealed_intent},
    messages as game_messages, verification,
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
/// * `client` - The Iris API client
pub async fn play_game(session: &Session, client: &IrisClient) -> EnclaveResult<()> {
    let session_id = super::game_state::generate_session_id();
    let (agent_id, user_key, transport_key) = super::game_state::get_game_info(session, client)
        .await
        .map_err(|e| EnclaveError::Wallet(e.to_string()))?;

    // Step 1: Get or create game wallet
    let wallet = super::game_state::get_or_create_wallet(true).map_err(EnclaveError::from)?;
    game_messages::game_wallet(&wallet);

    // Step 2: Get intentions from user
    let mut orders = create_new_orders(&wallet, &agent_id, &user_key, &transport_key).await?;
    if orders.is_empty() {
        return Err(EnclaveError::InvalidInput("No orders given. Exiting.".to_string()));
    }

    // Step 3: Get test value from user
    let test_value = prompt_secret_number("Give ANY number (this will be tested against the constraints): ")
        .map_err(|e| EnclaveError::InvalidInput(e.to_string()))?;
    for intent in orders.iter_mut() {
        intent.value = test_value as f64;
    }

    // Step 4: Play game
    game_messages::status_sending(&orders);
    let response = proof_game(
        &ProofGameRequest {
            chain_id: erato::types::ChainId::ETHEREUM.to_string(),
            wallet_address: wallet.address.clone(),
            unsigned_tx: encode_prefixed(test_value.to_be_bytes()),
            orders: orders.into_iter().take(3).collect(),
        },
        client,
    )
    .await
    .map_err(|e| EnclaveError::Wallet(e.to_string()))?;

    // Step 5: Display results
    display_results(&response, &wallet)?;
    let game_result = create_game_result(&response, &session_id)?;
    store_game_result(game_result).map_err(EnclaveError::from)?;

    // Display all stored game results
    let state = load_game_state().map_err(EnclaveError::from)?;
    println!("{}", verification::format_game_results(&state));
    Ok(())
}

/// Create new sealed intents for Game 1.
async fn create_new_orders(
    wallet: &GameWallet,
    agent_id: &Uuid,
    user_key: &UsersEncryptionKeys,
    transport_key: &TransportEnvelopeKey,
) -> EnclaveResult<Vec<ProofGameRequestOrdersItem>> {
    game_messages::game_1_select();

    let intentions: Vec<f64> = (1..=3)
        .map(|i: usize| {
            prompt_number(&format!("Please enter selection number {}: ", i))
                .map_err(|e| EnclaveError::InvalidInput(e.to_string()))
                .map(|v: u64| v as f64)
        })
        .collect::<Result<Vec<_>, _>>()?;

    let private_key_bytes: [u8; 32] = base64::engine::general_purpose::STANDARD
        .decode(wallet.private_key.clone())
        .map_err(|e| EnclaveError::Wallet(e.to_string()))?
        .try_into()
        .unwrap();

    let sealed_wallet = WalletKey::new(ChainType::EVM, wallet.address.clone(), private_key_bytes)
        .seal(&user_key.storage)
        .map_err(|e| EnclaveError::Wallet(e.to_string()))?;

    // Seal the storage envelope for the "upsert"
    let wallet_storage_envelope = WalletUpsert::new(sealed_wallet)
        .seal(transport_key)
        .map_err(|e| EnclaveError::Wallet(e.to_string()))?;

    // Seal the transport envelope for the "upsert"
    let wallet_transport_envelope = RotateUserKeyPayload::new(user_key.storage, None)
        .seal(transport_key)
        .map_err(|e| EnclaveError::Wallet(e.to_string()))?;

    let mut orders = Vec::new();

    // Create 3 sealed orders
    for intent in intentions.iter() {
        let order_id = Uuid::new_v4();

        // Create the sealed intent
        let sealed_intent = SealedIntent {
            user_id: None,
            agent_id: Some(agent_id.to_string()),
            chain_id: ChainId::ETHEREUM.to_string(),
            wallet_address: wallet.address.clone(),
            value: intent.to_string(),
        };

        // Seal the payload
        let payload = ExecutionPayload::new(user_key.storage, sealed_intent);
        let intent_envelope = payload
            .seal(transport_key)
            .map_err(|e| EnclaveError::Wallet(e.to_string()))?;

        // Store intent locally
        let order_id_str = order_id.to_string();
        store_sealed_intent(order_id_str.clone(), intent_envelope.clone(), Some(intent.to_string()))
            .map_err(EnclaveError::from)?;

        // Create the ProofGameRequestOrdersItem for the API
        let execute_intent = ProofGameRequestOrdersItem {
            order_id,
            value: 0 as f64, // this is just a placeholder. we overwrite later when we know
            intent_envelope: STANDARD.encode(&intent_envelope),
            wallet_storage_envelope: STANDARD.encode(&wallet_storage_envelope),
            wallet_transport_envelope: STANDARD.encode(&wallet_transport_envelope),
        };

        orders.push(execute_intent);
    }

    game_messages::game_sealed(&orders);
    Ok(orders)
}

/// Display the prove game results.
fn display_results(
    response: &crate::domains::client::generated::routes::requests::agent_proof_game::ProofGameResponse,
    wallet: &GameWallet,
) -> EnclaveResult<()> {
    game_messages::game_result_title();

    for (i, result) in response.results.iter().enumerate() {
        game_messages::game_order_result(i, result);
    }

    let any_wallet_accessed = response
        .results
        .iter()
        .any(|r| r.enclave_error.is_none() && r.signature.is_some());

    if any_wallet_accessed {
        game_messages::game_wallet_succeeded(wallet, response);
    } else {
        game_messages::game_wallet_failed(wallet);
    }
    Ok(())
}

/// Create a game result entry from the response.
fn create_game_result(
    response: &crate::domains::client::generated::routes::requests::agent_proof_game::ProofGameResponse,
    session_id: &str,
) -> EnclaveResult<GameResultEntry> {
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
