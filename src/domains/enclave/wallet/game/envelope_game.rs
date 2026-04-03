//! Game 2: The Vault
//!
//! In this game, the user creates 2 passwords. Each password is used to
//! derive a key via HKDF, and the wallet is encrypted with those keys.
//! The user then chooses ONE password to "seal" the vault. The enclave will
//! test both keys - only the correct password should decrypt the wallet.
//! This demonstrates password-based encryption and key derivation.

use alloy::hex::encode_prefixed;
use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use erato::models::ChainId;
use hkdf::Hkdf;
use sha2::Sha256;
use uuid::Uuid;

use tyche_enclave::envelopes::storage::StorageEnvelope;
use tyche_enclave::envelopes::storage::WalletKey;
use tyche_enclave::envelopes::transport::{
    ExecutionPayload, RotateUserKeyPayload, SealedIntent, TransportEnvelope, TransportEnvelopeKey, WalletUpsert,
};
use tyche_enclave::types::chain_type::ChainType;

use crate::domains::client::IrisClient;
use crate::domains::client::generated::routes::requests::agent_proof_game::{
    ProofGameRequest, ProofGameRequestOrdersItem,
};
use crate::domains::client::proof_game;
use crate::domains::enclave::errors::EnclaveError;
use crate::domains::enclave::errors::EnclaveResult;
use crate::domains::enclave::wallet::game::game_state::generate_session_id;
use crate::domains::keystore::Session;

use super::{
    game_state::{
        GameResultEntry, GameWallet, load_game_state, store_derived_key, store_encrypted_blob, store_game_result,
    },
    messages as game_messages, verification,
};

/// Play Game 2: The Vault.
///
/// Game flow:
/// 1. Get or create a game wallet
/// 2. Prompt user for 2 passwords
/// 3. HKDF derive 2 keys from passwords (store in game.toml)
/// 4. Seal wallet blob with both keys
/// 5. Prompt user for ONE password to test
/// 6. Create 2 unseal orders (one with each key)
/// 7. Call proof_game with both orders
/// 8. Show results: only correct password decrypts wallet
///
/// # Arguments
/// * `client` - The Iris API client
pub async fn play_game(session: &Session, client: &IrisClient) -> EnclaveResult<()> {
    let session_id = generate_session_id();
    let (agent_id, _, transport_key) = super::game_state::get_game_info(session, client)
        .await
        .map_err(|e| EnclaveError::Wallet(e.to_string()))?;

    // Step 1: Get or create game wallet
    let wallet = super::game_state::get_or_create_wallet(true).map_err(EnclaveError::from)?;
    game_messages::game_wallet(&wallet);

    // Step 2: Get intentions from user
    let (key1, key2) = get_passwords_from_user()?;

    // Step 3: Get test value from user
    let test_password = rpassword::prompt_password("Give any password to test against: ")
        .map_err(|e| EnclaveError::Io(e.to_string()))?;
    let final_storage_key = derive_from_password(&test_password)?;

    // Step 4: Create orders
    game_messages::game_sealing();
    let blob1 = create_encrypted_blob(&wallet, "password1", &key1)?;
    let blob2 = create_encrypted_blob(&wallet, "password2", &key2)?;
    let orders: Vec<ProofGameRequestOrdersItem> = vec![
        create_game_order(
            &agent_id,
            &wallet.address,
            blob1,
            &key1,
            &final_storage_key,
            &transport_key,
        )?,
        create_game_order(
            &agent_id,
            &wallet.address,
            blob2,
            &key2,
            &final_storage_key,
            &transport_key,
        )?,
    ];
    game_messages::game_sealed(&orders);

    // Step 5: Play game
    game_messages::status_sending(&orders);
    let response = proof_game(
        &ProofGameRequest {
            chain_id: erato::models::ChainId::ETHEREUM.to_string(),
            wallet_address: wallet.address.clone(),
            unsigned_tx: encode_prefixed("1".to_string().into_bytes()),
            orders,
        },
        client,
    )
    .await
    .map_err(|e| EnclaveError::Wallet(e.to_string()))?;

    // Step 6: Display results
    display_vault_results(&response, &wallet)?;
    let game_result = create_vault_game_result(&response, &session_id)?;
    store_game_result(game_result).map_err(EnclaveError::from)?;

    // Display all stored game results
    let state = load_game_state().map_err(EnclaveError::from)?;
    println!("{}", verification::format_game_results(&state));

    Ok(())
}

/// Get passwords from user.
fn get_passwords_from_user() -> EnclaveResult<([u8; 32], [u8; 32])> {
    game_messages::game_2_select();

    let password1 =
        rpassword::prompt_password("Please enter selection number 1: ").map_err(|e| EnclaveError::Io(e.to_string()))?;

    let password2 =
        rpassword::prompt_password("Please enter selection number 2: ").map_err(|e| EnclaveError::Io(e.to_string()))?;

    if password1.is_empty() || password2.is_empty() {
        return Err(EnclaveError::InvalidInput("Passwords cannot be empty".to_string()));
    }

    Ok((
        get_or_derive_key("password1", &password1)?,
        get_or_derive_key("password2", &password2)?,
    ))
}

/// Get or derive an encryption key from password.
fn get_or_derive_key(password_id: &str, password: &str) -> EnclaveResult<[u8; 32]> {
    // Derive new key using HKDF-SHA256
    let key = derive_from_password(password)?;

    // Store the derived key
    store_derived_key(password_id.to_string(), &key).map_err(EnclaveError::from)?;

    Ok(key)
}

/// Derive a key from a password using HKDF-SHA256.
fn derive_from_password(password: &str) -> EnclaveResult<[u8; 32]> {
    let hkdf = Hkdf::<Sha256>::new(None, password.as_bytes());
    let mut key = [0u8; 32];
    hkdf.expand(b"edge-vault-game", &mut key)
        .map_err(|_| EnclaveError::KeyDerivationFailed("Key derivation failed".to_string()))?;
    Ok(key)
}

/// Create encrypted wallet blobs with both keys.
fn create_encrypted_blob(wallet: &GameWallet, key_id: &str, user_storage_key: &[u8; 32]) -> EnclaveResult<Vec<u8>> {
    let private_key_bytes: [u8; 32] = base64::engine::general_purpose::STANDARD
        .decode(wallet.private_key.clone())
        .map_err(|e| EnclaveError::Wallet(e.to_string()))?
        .try_into()
        .unwrap();

    let encrypted_private_key = WalletKey::new(ChainType::EVM, wallet.address.clone(), private_key_bytes)
        .seal(user_storage_key)
        .map_err(|e| EnclaveError::Wallet(e.to_string()))?;

    // Store blobs
    store_encrypted_blob(key_id.to_string(), encrypted_private_key.clone()).map_err(EnclaveError::from)?;

    Ok(encrypted_private_key)
}

/// Create a single vault intent.
fn create_game_order(
    agent_id: &Uuid,
    wallet_address: &str,
    sealed_wallet: Vec<u8>,
    key_that_encrypts_wallet: &[u8; 32],
    key_sent_to_enclave: &[u8; 32],
    transport_key: &TransportEnvelopeKey,
) -> EnclaveResult<ProofGameRequestOrdersItem> {
    // Create the sealed intent
    let sealed_intent = SealedIntent {
        user_id: None,
        agent_id: Some(agent_id.to_string()),
        chain_id: ChainId::ETHEREUM.to_string(),
        wallet_address: wallet_address.to_string(),
        value: "0".to_string(),
    };

    // Seal the intent payload
    let payload = ExecutionPayload::new(*key_sent_to_enclave, sealed_intent);
    let intent_envelope =
        payload
            .seal(transport_key)
            .map_err(|e: tyche_enclave::envelopes::transport::TransportEnvelopeError| {
                EnclaveError::Wallet(e.to_string())
            })?;

    // Seal the storage envelope for the "upsert"
    let wallet_storage_envelope = WalletUpsert::new(sealed_wallet)
        .seal(transport_key)
        .map_err(|e| EnclaveError::Wallet(e.to_string()))?;

    // Seal the transport envelope for the "upsert"
    let wallet_transport_envelope = RotateUserKeyPayload::new(*key_that_encrypts_wallet, None)
        .seal(transport_key)
        .map_err(|e| EnclaveError::Wallet(e.to_string()))?;

    Ok(ProofGameRequestOrdersItem {
        order_id: Uuid::new_v4(),
        value: 0.0,
        intent_envelope: STANDARD.encode(&intent_envelope),
        wallet_transport_envelope: STANDARD.encode(&wallet_transport_envelope),
        wallet_storage_envelope: STANDARD.encode(&wallet_storage_envelope),
    })
}

/// Display the vault game results.
fn display_vault_results(
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

/// Create a game result entry from the vault response.
fn create_vault_game_result(
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
        game_type: 2,
        success,
        signature,
        enclave_error,
        timestamp: chrono::Utc::now().to_rfc3339(),
    })
}
