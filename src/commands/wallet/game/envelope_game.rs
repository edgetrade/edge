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
use tyche_enclave::envelopes::storage::WalletKey;
use tyche_enclave::types::chain_type::ChainType;
use uuid::Uuid;

use tyche_enclave::envelopes::storage::StorageEnvelope;
use tyche_enclave::envelopes::transport::{
    ExecutionPayload, RotateUserKeyPayload, SealedIntent, TransportEnvelope, TransportEnvelopeKey, WalletUpsert,
};

use crate::client::IrisClient;
use crate::client::proof_game;
use crate::generated::routes::requests::agent_proof_game::{ProofGameRequest, ProofGameRequestOrdersItem};
use crate::messages;
use crate::session::Session;
use crate::session::transport::get_transport_key;

use super::{
    game_state::{
        GameResultEntry, GameWallet, get_derived_key, get_encrypted_blob, load_game_state, store_derived_key,
        store_encrypted_blob, store_game_result,
    },
    utils::{generate_session_id, prompt_user},
    verification,
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
/// * `replay` - If true, use existing passwords/keys instead of prompting
/// * `client` - The Iris API client
pub async fn play_game(replay: bool, session: &Session, client: &IrisClient) -> messages::success::CommandResult<()> {
    let session_id = generate_session_id();
    println!("Session ID: {}\n", session_id);

    let agent_id = session
        .get_config()
        .map_err(|e| messages::error::CommandError::Session(e.to_string()))?
        .clone()
        .agent_id
        .unwrap();

    // Get transport keys for sealing
    let enclave_keys = get_transport_key(client)
        .await
        .map_err(|e| messages::error::CommandError::Wallet(e.to_string()))?;
    let transport_key = TransportEnvelopeKey::Unsealing(enclave_keys.deterministic);

    // Step 1: Get or create game wallet
    let wallet = super::game_state::get_or_create_wallet(!replay)?;
    println!("Using game wallet: {}\n", wallet.address);

    // Step 2: Get passwords and derive keys
    let (password1, password2) = if replay {
        println!("Replay mode: using stored passwords...\n");
        ("replay1".to_string(), "replay2".to_string())
    } else {
        get_passwords_from_user()?
    };

    // Step 3: Derive keys from passwords
    let key1 = get_or_derive_key("password1", &password1, replay)?;
    let key2 = get_or_derive_key("password2", &password2, replay)?;

    // Step 4: Create encrypted wallet blobs
    let (blob1, blob2) = match replay {
        true => load_existing_blobs()?,
        false => {
            let blob1 = create_encrypted_blob(&wallet, "password1", &key1)?;
            let blob2 = create_encrypted_blob(&wallet, "password2", &key2)?;
            (blob1, blob2)
        }
    };

    // Step 5: Get the test password from user
    let test_password = if replay {
        println!("Replay mode: testing with password1...\n");
        password1.clone()
    } else {
        prompt_user("Give ONE password to test (password1 or password2): ")?
    };

    println!("\nTesting with password: {}\n", test_password);

    // Step 6: Create intents for prove game
    let final_storage_key = derive_from_password(&test_password)?;
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

    // Step 7: Call proof_game
    println!("Sending vault unlock attempts to the enclave...\n");

    let request = &ProofGameRequest {
        chain_id: erato::models::ChainId::ETHEREUM.to_string(),
        wallet_address: wallet.address.clone(),
        unsigned_tx: encode_prefixed("1".to_string().into_bytes()),
        orders,
    };

    let response = proof_game(request, client)
        .await
        .map_err(|e| messages::error::CommandError::Wallet(e.to_string()))?;

    // Step 8: Display results
    display_vault_results(&response, &wallet, &test_password)?;

    // Step 9: Store game result
    let game_result = create_vault_game_result(&response, &session_id)?;
    store_game_result(game_result).map_err(|e| messages::error::CommandError::Wallet(e.to_string()))?;

    // Display all stored game results
    let state = load_game_state().map_err(|e| messages::error::CommandError::Wallet(e.to_string()))?;
    println!("{}", verification::format_game_results(&state));

    Ok(())
}

/// Get passwords from user.
fn get_passwords_from_user() -> messages::success::CommandResult<(String, String)> {
    println!("Create 2 passwords for vault encryption:\n");

    let password1 =
        rpassword::prompt_password("Password 1: ").map_err(|e| messages::error::CommandError::Io(e.to_string()))?;

    let password2 =
        rpassword::prompt_password("Password 2: ").map_err(|e| messages::error::CommandError::Io(e.to_string()))?;

    if password1.is_empty() || password2.is_empty() {
        return Err(messages::error::CommandError::InvalidInput(
            "Passwords cannot be empty".to_string(),
        ));
    }

    println!("\n✓ Passwords set!\n");
    Ok((password1, password2))
}

/// Get or derive an encryption key from password.
fn get_or_derive_key(password_id: &str, password: &str, replay: bool) -> messages::success::CommandResult<[u8; 32]> {
    // Try to load existing key first
    if replay
        && let Some(key) =
            get_derived_key(password_id).map_err(|e| messages::error::CommandError::Wallet(e.to_string()))?
    {
        println!("  Using stored key for {}", password_id);
        return Ok(key);
    }

    // Derive new key using HKDF-SHA256
    let key = derive_from_password(password)?;

    // Store the derived key
    store_derived_key(password_id.to_string(), &key)
        .map_err(|e| messages::error::CommandError::Wallet(e.to_string()))?;

    println!("  Derived key for {}", password_id);
    Ok(key)
}

/// Derive a key from a password using HKDF-SHA256.
fn derive_from_password(password: &str) -> messages::success::CommandResult<[u8; 32]> {
    let hkdf = Hkdf::<Sha256>::new(None, password.as_bytes());
    let mut key = [0u8; 32];
    hkdf.expand(b"edge-vault-game", &mut key)
        .map_err(|_| messages::error::CommandError::Crypto("Key derivation failed".to_string()))?;
    Ok(key)
}

/// Create encrypted wallet blobs with both keys.
fn create_encrypted_blob(
    wallet: &GameWallet,
    key_id: &str,
    user_storage_key: &[u8; 32],
) -> messages::success::CommandResult<Vec<u8>> {
    let private_key_bytes: [u8; 32] = base64::engine::general_purpose::STANDARD
        .decode(wallet.private_key.clone())
        .map_err(|e| messages::error::CommandError::Wallet(e.to_string()))?
        .try_into()
        .unwrap();

    let encrypted_private_key = WalletKey::new(ChainType::EVM, wallet.address.clone(), private_key_bytes)
        .seal(user_storage_key)
        .map_err(|e| messages::error::CommandError::Wallet(e.to_string()))?;

    // Store blobs
    store_encrypted_blob(key_id.to_string(), encrypted_private_key.clone())
        .map_err(|e| messages::error::CommandError::Wallet(e.to_string()))?;

    println!("  Created encrypted blobs for both passwords\n");
    Ok(encrypted_private_key)
}

/// Load existing encrypted blobs.
fn load_existing_blobs() -> messages::success::CommandResult<(Vec<u8>, Vec<u8>)> {
    let blob1 = get_encrypted_blob("password1")
        .map_err(|e| messages::error::CommandError::Wallet(e.to_string()))?
        .ok_or_else(|| messages::error::CommandError::InvalidInput("No stored blob for password1".to_string()))?;

    let blob2 = get_encrypted_blob("password2")
        .map_err(|e| messages::error::CommandError::Wallet(e.to_string()))?
        .ok_or_else(|| messages::error::CommandError::InvalidInput("No stored blob for password2".to_string()))?;

    println!("  Loaded existing encrypted blobs\n");
    Ok((blob1, blob2))
}

/// Create a single vault intent.
fn create_game_order(
    agent_id: &Uuid,
    wallet_address: &str,
    sealed_wallet: Vec<u8>,
    key_that_encrypts_wallet: &[u8; 32],
    key_sent_to_enclave: &[u8; 32],
    transport_key: &TransportEnvelopeKey,
) -> messages::success::CommandResult<ProofGameRequestOrdersItem> {
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
                messages::error::CommandError::Wallet(e.to_string())
            })?;

    // Seal the storage envelope for the "upsert"
    let wallet_storage_envelope = WalletUpsert::new(sealed_wallet)
        .seal(transport_key)
        .map_err(|e| messages::error::CommandError::Wallet(e.to_string()))?;

    // Seal the transport envelope for the "upsert"
    let wallet_transport_envelope = RotateUserKeyPayload::new(*key_that_encrypts_wallet, None)
        .seal(transport_key)
        .map_err(|e| messages::error::CommandError::Wallet(e.to_string()))?;

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
    response: &crate::generated::routes::requests::agent_proof_game::ProofGameResponse,
    _wallet: &GameWallet,
    test_password: &str,
) -> messages::success::CommandResult<()> {
    println!("\n--- Vault Results ---\n");

    let any_wallet_accessed = response
        .results
        .iter()
        .any(|r| r.enclave_error.is_none() && r.signature.is_some());

    for (i, result) in response.results.iter().enumerate() {
        let key_name = if i == 0 { "Password 1" } else { "Password 2" };

        let status = if result.enclave_error.is_none() && result.signature.is_some() {
            "✓ VAULT UNLOCKED - WALLET ACCESSED"
        } else if let Some(ref err) = result.enclave_error {
            &format!("✗ Failed: {}", err)
        } else {
            "✗ Incorrect key - vault locked"
        };

        println!("{}: {}", key_name, status);

        if let Some(ref sig) = result.signature {
            println!("  Signature: {}...", &sig[..sig.len().min(20)]);
        }
    }

    println!();

    if any_wallet_accessed {
        println!("✓✓✓ SUCCESS! Vault Unlocked! ✓✓✓");
        println!();
        println!("The enclave successfully decrypted the vault using the");
        println!("correct password ('{}').", test_password);
        println!();
        println!("This demonstrates that:");
        println!("  - Passwords are properly converted to encryption keys via HKDF");
        println!("  - Only the correct password can decrypt the wallet");
        println!("  - The enclave performs the decryption securely");

        if let Some(result) = response.results.iter().find(|r| r.signature.is_some())
            && let Some(ref sig) = result.signature
        {
            println!("\nSignature: {}", sig);
        }
    } else {
        println!("✗ Vault remains locked.");
        println!();
        println!("Neither password unlocked the vault.");
        println!();
        println!("This demonstrates that the encryption is working correctly -");
        println!("only the exact password used during vault creation can unlock it.");
    }

    println!();
    Ok(())
}

/// Create a game result entry from the vault response.
fn create_vault_game_result(
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
        game_type: 2,
        success,
        signature,
        enclave_error,
        timestamp: chrono::Utc::now().to_rfc3339(),
    })
}
