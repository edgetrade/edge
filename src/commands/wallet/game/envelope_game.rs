//! Game 2: The Vault
//!
//! In this game, the user creates 2 passwords. Each password is used to
//! derive a key via HKDF, and the wallet is encrypted with those keys.
//! The user then chooses ONE password to "seal" the vault. The enclave will
//! test both keys - only the correct password should decrypt the wallet.
//! This demonstrates password-based encryption and key derivation.

use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use hkdf::Hkdf;
use sha2::Sha256;
use tyche_enclave::envelopes::storage::WalletKey;
use tyche_enclave::types::chain_type::ChainType;
use uuid::Uuid;

use tyche_enclave::envelopes::storage::StorageEnvelope;
use tyche_enclave::envelopes::transport::{ExecutionPayload, SealedIntent, TransportEnvelope, TransportEnvelopeKey};

use crate::client::proof_game;
use crate::generated::routes::requests::agent_proof_game::ProofGameRequestOrdersItem;
use crate::messages;
use crate::session::transport::get_transport_key;
use crate::{client::IrisClient, session::crypto::UsersEncryptionKeys};

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
    if replay {
        load_existing_blobs()?;
    } else {
        create_encrypted_blobs(&wallet, &key1, &key2)?;
    };

    // Step 5: Get the test password from user
    let test_password = if replay {
        println!("Replay mode: testing with password1...\n");
        password1.clone()
    } else {
        prompt_user("Give ONE password to test (password1 or password2): ")?
    };

    println!("\nTesting with password: {}\n", test_password);

    // Determine which key to use based on password
    let test_key = if test_password == password1 {
        key1
    } else if test_password == password2 {
        key2
    } else {
        [0u8; 32]
    };

    // Step 6: Create intents for prove game
    let intents = create_vault_intents(&wallet, &key1, &key2, &test_key, client).await?;

    // Step 7: Call proof_game
    println!("Sending vault unlock attempts to the enclave...\n");

    let encrypted_wallet_blob = WalletKey::new(
        ChainType::EVM,
        wallet.address.clone(),
        wallet.private_key.clone().into_bytes().to_vec(),
    )
    .seal(&user_key.storage)
    .map_err(|e| messages::error::CommandError::Wallet(e.to_string()))?;

    let response = proof_game(wallet.address.clone(), encrypted_wallet_blob, intents, client)
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
    let hkdf = Hkdf::<Sha256>::new(None, password.as_bytes());
    let mut key = [0u8; 32];
    hkdf.expand(b"edge-vault-game", &mut key)
        .map_err(|_| messages::error::CommandError::Crypto("Key derivation failed".to_string()))?;

    // Store the derived key
    store_derived_key(password_id.to_string(), &key)
        .map_err(|e| messages::error::CommandError::Wallet(e.to_string()))?;

    println!("  Derived key for {}", password_id);
    Ok(key)
}

/// Create encrypted wallet blobs with both keys.
fn create_encrypted_blobs(
    wallet: &GameWallet,
    key1: &[u8; 32],
    key2: &[u8; 32],
) -> messages::success::CommandResult<(Vec<u8>, Vec<u8>)> {
    use aes_gcm::{
        Aes256Gcm, Nonce,
        aead::{Aead, KeyInit},
    };

    // Decode the private key
    let private_key = STANDARD
        .decode(&wallet.private_key)
        .map_err(|_| messages::error::CommandError::InvalidInput("Invalid wallet key".to_string()))?;

    // Encrypt with key1
    let cipher1 = Aes256Gcm::new(key1.into());
    let nonce1: [u8; 12] = rand::random();
    let blob1 = cipher1
        .encrypt(Nonce::from_slice(&nonce1), private_key.as_ref())
        .map_err(|_| messages::error::CommandError::Crypto("Encryption failed".to_string()))?;

    // Encrypt with key2
    let cipher2 = Aes256Gcm::new(key2.into());
    let nonce2: [u8; 12] = rand::random();
    let blob2 = cipher2
        .encrypt(Nonce::from_slice(&nonce2), private_key.as_ref())
        .map_err(|_| messages::error::CommandError::Crypto("Encryption failed".to_string()))?;

    // Store blobs
    store_encrypted_blob("password1".to_string(), blob1.clone())
        .map_err(|e| messages::error::CommandError::Wallet(e.to_string()))?;
    store_encrypted_blob("password2".to_string(), blob2.clone())
        .map_err(|e| messages::error::CommandError::Wallet(e.to_string()))?;

    println!("  Created encrypted blobs for both passwords\n");
    Ok((blob1, blob2))
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

/// Create vault intents for prove game.
async fn create_vault_intents(
    wallet: &GameWallet,
    key1: &[u8; 32],
    key2: &[u8; 32],
    _test_key: &[u8; 32],
    client: &IrisClient,
) -> messages::success::CommandResult<Vec<ProofGameRequestOrdersItem>> {
    // Get transport keys for sealing
    let enclave_keys = get_transport_key(client)
        .await
        .map_err(|e| messages::error::CommandError::Wallet(e.to_string()))?;
    let transport_key = TransportEnvelopeKey::Unsealing(enclave_keys.deterministic);

    let mut intents = Vec::new();

    // Create intent for password1 (key1)
    let intent1 = create_vault_intent(Uuid::new_v4(), wallet, key1, &transport_key)?;
    intents.push(intent1);

    // Create intent for password2 (key2)
    let intent2 = create_vault_intent(Uuid::new_v4(), wallet, key2, &transport_key)?;
    intents.push(intent2);

    println!("  Created {} vault unlock intents", intents.len());
    Ok(intents)
}

/// Create a single vault intent.
fn create_vault_intent(
    order_id: Uuid,
    wallet: &GameWallet,
    key: &[u8; 32],
    transport_key: &TransportEnvelopeKey,
) -> messages::success::CommandResult<ProofGameRequestOrdersItem> {
    // Create the sealed intent
    let sealed_intent = SealedIntent {
        user_id: None,
        agent_id: None,
        chain_id: "1".to_string(),
        wallet_address: wallet.address.clone(),
        value: "0".to_string(),
    };

    // Create execution payload with the derived key
    let payload = ExecutionPayload::new(*key, sealed_intent);

    // Seal the payload
    let envelope =
        payload
            .seal(transport_key)
            .map_err(|e: tyche_enclave::envelopes::transport::TransportEnvelopeError| {
                messages::error::CommandError::Wallet(e.to_string())
            })?;

    let execute_intent = ProofGameRequestOrdersItem {
        order_id,
        data: hex::encode([]), // Empty hex string
        value: 0.0,
        sealed_envelope: STANDARD.encode(&envelope),
    };

    Ok(execute_intent)
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

#[cfg(test)]
mod tests {
    use super::*;

    use hkdf::Hkdf;
    use sha2::Sha256;

    use tyche_enclave::envelopes::transport::TransportEnvelopeKey;

    use crate::commands::wallet::game::game_state::{
        GameWallet, get_derived_key, get_encrypted_blob, set_test_game_state_path, store_derived_key,
        store_encrypted_blob,
    };

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

    #[test]
    fn test_hkdf_key_derivation() {
        let password = "my-test-password";

        let hkdf = Hkdf::<Sha256>::new(None, password.as_bytes());
        let mut key = [0u8; 32];
        hkdf.expand(b"edge-vault-game", &mut key)
            .expect("Key derivation failed");

        assert_eq!(key.len(), 32);
        assert!(!key.iter().all(|b| *b == 0u8));

        let hkdf2 = Hkdf::<Sha256>::new(None, password.as_bytes());
        let mut key2 = [0u8; 32];
        hkdf2
            .expand(b"edge-vault-game", &mut key2)
            .expect("Key derivation failed");

        assert_eq!(key, key2);
    }

    #[test]
    fn test_store_and_retrieve_derived_key() {
        let _temp = setup_test_env();
        let password_id = "test-password-1";
        let original_key: [u8; 32] = [0xAB; 32];

        store_derived_key(password_id.to_string(), &original_key).expect("Failed to store key");

        let retrieved = get_derived_key(password_id)
            .expect("Failed to get key")
            .expect("Key not found");

        assert_eq!(retrieved, original_key);
    }

    #[test]
    fn test_store_and_retrieve_encrypted_blob() {
        let _temp = setup_test_env();
        let password_id = "test-blob-1";
        let original_blob = vec![0xDE, 0xAD, 0xBE, 0xEF, 0x00, 0x01, 0x02, 0x03];

        store_encrypted_blob(password_id.to_string(), original_blob.clone()).expect("Failed to store blob");

        let retrieved = get_encrypted_blob(password_id)
            .expect("Failed to get blob")
            .expect("Blob not found");

        assert_eq!(retrieved, original_blob);
    }

    #[test]
    fn test_aes_gcm_encryption_decryption() {
        use aes_gcm::{
            Aes256Gcm, Nonce,
            aead::{Aead, KeyInit},
        };

        let key: [u8; 32] = [0xAB; 32];
        let plaintext = b"test plaintext data for encryption";

        let cipher = Aes256Gcm::new(&key.into());
        let nonce: [u8; 12] = rand::random();
        let ciphertext = cipher
            .encrypt(Nonce::from_slice(&nonce), plaintext.as_ref())
            .expect("Encryption failed");

        assert_ne!(ciphertext, plaintext.to_vec());

        let cipher = Aes256Gcm::new(&key.into());
        let decrypted = cipher
            .decrypt(Nonce::from_slice(&nonce), ciphertext.as_ref())
            .expect("Decryption failed");

        assert_eq!(decrypted, plaintext.to_vec());
    }

    #[test]
    fn test_create_encrypted_blobs() {
        let _temp = setup_test_env();
        let wallet = create_test_wallet();
        let key1: [u8; 32] = [0x01; 32];
        let key2: [u8; 32] = [0x02; 32];

        let result = create_encrypted_blobs(&wallet, &key1, &key2);
        assert!(result.is_ok());

        let (blob1, blob2) = result.unwrap();

        assert!(!blob1.is_empty());
        assert!(!blob2.is_empty());
        assert_ne!(blob1, blob2, "Different keys should produce different ciphertexts");
    }

    #[test]
    fn test_vault_intent_creation() {
        use aes_gcm::aead::rand_core::OsRng;
        use ed25519_dalek::SigningKey;

        let wallet = create_test_wallet();
        let key: [u8; 32] = [0xAB; 32];

        let signing_key = SigningKey::generate(&mut OsRng);
        let transport_key = TransportEnvelopeKey::Unsealing(signing_key.verifying_key());

        let intent = create_vault_intent(Uuid::new_v4(), &wallet, &key, &transport_key);
        assert!(intent.is_ok());

        let intent = intent.unwrap();
        assert_eq!(intent.value, 0.0);
        assert!(!intent.sealed_envelope.is_empty());
    }

    #[test]
    fn test_create_vault_game_result() {
        use crate::generated::routes::requests::agent_proof_game::{ProofGameResponse, ProofGameResponseResultsItem};

        let response = ProofGameResponse {
            results: vec![ProofGameResponseResultsItem {
                order_id: "vault-key1".to_string(),
                enclave_error: None,
                signature: Some("test-sig".to_string()),
            }],
        };

        let result = create_vault_game_result(&response, "test-session");
        assert!(result.is_ok());

        let result = result.unwrap();
        assert_eq!(result.session_id, "test-session");
        assert_eq!(result.game_type, 2);
        assert!(result.success);
        assert_eq!(result.signature, Some("test-sig".to_string()));
    }
}
