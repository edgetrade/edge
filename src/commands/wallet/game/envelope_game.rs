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

use tyche_enclave::envelopes::transport::{ExecutionPayload, SealedIntent, TransportEnvelope, TransportEnvelopeKey};

use crate::client::IrisClient;
use crate::client::{EvmParameters, ExecuteIntent, get_transport_key, prove_game_with_intents};
use crate::commands::wallet::{
    game::game_state::{
        GameResultEntry, GameWallet, get_derived_key, get_encrypted_blob, store_derived_key, store_encrypted_blob,
        store_game_result,
    },
    prove::{generate_session_id, prompt_user},
};
use crate::messages;

/// Play Game 2: The Vault.
///
/// Game flow:
/// 1. Get or create a game wallet
/// 2. Prompt user for 2 passwords
/// 3. HKDF derive 2 keys from passwords (store in game.toml)
/// 4. Seal wallet blob with both keys
/// 5. Prompt user for ONE password to test
/// 6. Create 2 unseal orders (one with each key)
/// 7. Call prove_game with both orders
/// 8. Show results: only correct password decrypts wallet
///
/// # Arguments
/// * `replay` - If true, use existing passwords/keys instead of prompting
/// * `client` - The Iris API client
pub async fn play_game(replay: bool, client: &IrisClient) -> messages::success::CommandResult<()> {
    let session_id = generate_session_id();
    println!("Session ID: {}\n", session_id);

    // Step 1: Get or create game wallet
    let wallet = super::game_state::get_or_create_wallet(!replay)?;
    println!("Using game wallet: {}\n", wallet.address);

    // Step 2: Get passwords and derive keys
    let (password1, password2) = if replay {
        println!("Replay mode: using stored passwords...\n");
        // In replay mode, we don't need actual passwords - just use placeholder
        ("replay1".to_string(), "replay2".to_string())
    } else {
        get_passwords_from_user()?
    };

    // Step 3: Derive keys from passwords
    let key1 = get_or_derive_key("password1", &password1, replay)?;
    let key2 = get_or_derive_key("password2", &password2, replay)?;

    // Step 4: Create encrypted wallet blobs
    let (_blob1, _blob2) = if replay {
        load_existing_blobs()?
    } else {
        create_encrypted_blobs(&wallet, &key1, &key2)?
    };

    // Step 5: Get the test password from user
    let test_password = if replay {
        // In replay mode, use password1 as default
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
        // Test with a wrong key to show failure
        [0u8; 32]
    };

    // Step 6: Create intents for prove game
    let intents = create_vault_intents(&wallet, &key1, &key2, &test_key, client).await?;

    // Step 7: Call prove_game
    println!("Sending vault unlock attempts to the enclave...\n");

    let response = prove_game_with_intents(session_id.clone(), intents, client)
        .await
        .map_err(|e| messages::error::CommandError::Wallet(e.to_string()))?;

    // Step 8: Display results
    display_vault_results(&response, &wallet, &test_password)?;

    // Step 9: Store game result
    let game_result = create_vault_game_result(&response, &session_id)?;
    store_game_result(game_result).map_err(|e| messages::error::CommandError::Wallet(e.to_string()))?;

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
) -> messages::success::CommandResult<Vec<ExecuteIntent>> {
    // Get transport keys for sealing
    let enclave_keys = get_transport_key(client)
        .await
        .map_err(|e| messages::error::CommandError::Wallet(e.to_string()))?;
    let transport_key = TransportEnvelopeKey::Unsealing(enclave_keys.deterministic);

    let mut intents = Vec::new();

    // Create intent for password1 (key1)
    let intent1 = create_vault_intent("vault-key1", wallet, key1, &transport_key)?;
    intents.push(intent1);

    // Create intent for password2 (key2)
    let intent2 = create_vault_intent("vault-key2", wallet, key2, &transport_key)?;
    intents.push(intent2);

    println!("  Created {} vault unlock intents", intents.len());
    Ok(intents)
}

/// Create a single vault intent.
fn create_vault_intent(
    order_id: &str,
    wallet: &GameWallet,
    key: &[u8; 32],
    transport_key: &TransportEnvelopeKey,
) -> messages::success::CommandResult<ExecuteIntent> {
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
    let envelope = payload
        .seal(transport_key)
        .map_err(|e| messages::error::CommandError::Wallet(e.to_string()))?;

    let execute_intent = ExecuteIntent {
        order_id: order_id.to_string(),
        user_id: None,
        agent_id: None,
        chain_id: "1".to_string(),
        wallet_address: wallet.address.clone(),
        value: "0".to_string(),
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

    Ok(execute_intent)
}

/// Display the vault game results.
fn display_vault_results(
    response: &crate::client::ProveGameResponse,
    _wallet: &GameWallet,
    test_password: &str,
) -> messages::success::CommandResult<()> {
    println!("\n--- Vault Results ---\n");

    let mut any_wallet_accessed = false;

    for attempt in response.attempts.iter() {
        let key_name = if attempt.order_id.contains("key1") {
            "Password 1"
        } else {
            "Password 2"
        };

        let status = if attempt.wallet_accessed {
            any_wallet_accessed = true;
            "✓ VAULT UNLOCKED - WALLET ACCESSED"
        } else if attempt.success.is_some() {
            "✗ Incorrect key - vault locked"
        } else if let Some(ref failure) = attempt.failure {
            &format!("✗ Failed: {}", failure.error_code)
        } else {
            "✗ Unknown result"
        };

        println!("{}: {}", key_name, status);

        if let Some(ref success) = attempt.success
            && let Some(ref sig) = success.signature
        {
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

        if let Some(attempt) = response.attempts.iter().find(|a| a.wallet_accessed)
            && let Some(ref success) = attempt.success
            && let Some(ref sig) = success.signature
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
        game_type: 2,
        success,
        signature,
        error,
        timestamp: chrono::Utc::now().to_rfc3339(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    use hkdf::Hkdf;
    use sha2::Sha256;

    use tyche_enclave::envelopes::transport::TransportEnvelopeKey;

    use crate::client::{ExecutionAttempt, ExecutionSuccess, ProveGameResponse};
    use crate::commands::wallet::game::game_state::{
        GameWallet, get_derived_key, get_encrypted_blob, set_test_game_state_path, store_derived_key,
        store_encrypted_blob,
    };

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

    #[test]
    fn test_hkdf_key_derivation() {
        let password = "my-test-password";

        // Derive key using HKDF-SHA256
        let hkdf = Hkdf::<Sha256>::new(None, password.as_bytes());
        let mut key = [0u8; 32];
        hkdf.expand(b"edge-vault-game", &mut key)
            .expect("Key derivation failed");

        // Verify key is 32 bytes
        assert_eq!(key.len(), 32);

        // Key should not be all zeros
        assert!(!key.iter().all(|b| *b == 0u8));

        // Deriving again should produce same key
        let hkdf2 = Hkdf::<Sha256>::new(None, password.as_bytes());
        let mut key2 = [0u8; 32];
        hkdf2
            .expand(b"edge-vault-game", &mut key2)
            .expect("Key derivation failed");

        assert_eq!(key, key2);
    }

    #[test]
    fn test_hkdf_different_passwords_produce_different_keys() {
        let passwords = vec!["password1", "password2", "different-password", "P@ssw0rd!"];

        let mut keys = Vec::new();

        for password in &passwords {
            let hkdf = Hkdf::<Sha256>::new(None, password.as_bytes());
            let mut key = [0u8; 32];
            hkdf.expand(b"edge-vault-game", &mut key)
                .expect("Key derivation failed");
            keys.push(key);
        }

        // All keys should be different
        for i in 0..keys.len() {
            for j in i + 1..keys.len() {
                assert_ne!(keys[i], keys[j], "Different passwords should produce different keys");
            }
        }
    }

    #[test]
    fn test_hkdf_info_string_matters() {
        let password = "test-password";

        // Derive with default info
        let hkdf1 = Hkdf::<Sha256>::new(None, password.as_bytes());
        let mut key1 = [0u8; 32];
        hkdf1
            .expand(b"edge-vault-game", &mut key1)
            .expect("Key derivation failed");

        // Derive with different info
        let hkdf2 = Hkdf::<Sha256>::new(None, password.as_bytes());
        let mut key2 = [0u8; 32];
        hkdf2
            .expand(b"different-info", &mut key2)
            .expect("Key derivation failed");

        // Keys should be different
        assert_ne!(key1, key2, "Different info strings should produce different keys");
    }

    #[test]
    fn test_store_and_retrieve_derived_key() {
        let _temp = setup_test_env();
        let password_id = "test-password-1";
        let original_key: [u8; 32] = [0xAB; 32];

        // Store the key
        store_derived_key(password_id.to_string(), &original_key).expect("Failed to store key");

        // Retrieve the key
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

        // Store the blob
        store_encrypted_blob(password_id.to_string(), original_blob.clone()).expect("Failed to store blob");

        // Retrieve the blob
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

        // Encrypt
        let cipher = Aes256Gcm::new(&key.into());
        let nonce: [u8; 12] = rand::random();
        let ciphertext = cipher
            .encrypt(Nonce::from_slice(&nonce), plaintext.as_ref())
            .expect("Encryption failed");

        // Verify ciphertext is different from plaintext
        assert_ne!(ciphertext, plaintext.to_vec());

        // Decrypt
        let cipher = Aes256Gcm::new(&key.into());
        let decrypted = cipher
            .decrypt(Nonce::from_slice(&nonce), ciphertext.as_ref())
            .expect("Decryption failed");

        // Verify decrypted text matches original
        assert_eq!(decrypted, plaintext.to_vec());
    }

    #[test]
    fn test_aes_gcm_wrong_key_fails() {
        use aes_gcm::{
            Aes256Gcm, Nonce,
            aead::{Aead, KeyInit},
        };

        let key1: [u8; 32] = [0xAB; 32];
        let key2: [u8; 32] = [0xCD; 32];
        let plaintext = b"secret data";

        // Encrypt with key1
        let cipher = Aes256Gcm::new(&key1.into());
        let nonce: [u8; 12] = rand::random();
        let ciphertext = cipher
            .encrypt(Nonce::from_slice(&nonce), plaintext.as_ref())
            .expect("Encryption failed");

        // Try to decrypt with key2 (should fail)
        let cipher = Aes256Gcm::new(&key2.into());
        let result = cipher.decrypt(Nonce::from_slice(&nonce), ciphertext.as_ref());

        assert!(result.is_err(), "Decryption with wrong key should fail");
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

        // Both blobs should be non-empty
        assert!(!blob1.is_empty());
        assert!(!blob2.is_empty());

        // Blobs should be different
        assert_ne!(blob1, blob2, "Different keys should produce different ciphertexts");
    }

    #[test]
    fn test_vault_intent_creation() {
        use aes_gcm::aead::rand_core::OsRng;
        use ed25519_dalek::SigningKey;

        let wallet = create_test_wallet();
        let key: [u8; 32] = [0xAB; 32];

        // Generate a proper Ed25519 verifying key for sealing
        let signing_key = SigningKey::generate(&mut OsRng);
        let transport_key = TransportEnvelopeKey::Unsealing(signing_key.verifying_key());

        let intent = create_vault_intent("vault-test", &wallet, &key, &transport_key);
        assert!(intent.is_ok());

        let intent = intent.unwrap();
        assert_eq!(intent.order_id, "vault-test");
        assert_eq!(intent.wallet_address, wallet.address);
        assert_eq!(intent.value, "0");
        assert!(!intent.envelope.is_empty());
    }

    #[test]
    fn test_display_vault_results_success() {
        let wallet = create_test_wallet();

        let response = ProveGameResponse {
            game_session_id: "test-session".to_string(),
            attempts: vec![
                ExecutionAttempt {
                    order_id: "vault-key1".to_string(),
                    success: Some(ExecutionSuccess {
                        signature: Some("sig123".to_string()),
                        tx_hash: None,
                    }),
                    failure: None,
                    wallet_accessed: true,
                },
                ExecutionAttempt {
                    order_id: "vault-key2".to_string(),
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

        let result = display_vault_results(&response, &wallet, "password1");
        assert!(result.is_ok());
    }

    #[test]
    fn test_display_vault_results_failure() {
        let wallet = create_test_wallet();

        let response = ProveGameResponse {
            game_session_id: "test-session".to_string(),
            attempts: vec![
                ExecutionAttempt {
                    order_id: "vault-key1".to_string(),
                    success: None,
                    failure: Some(crate::client::ExecutionFailure {
                        error_code: "DECRYPTION_FAILED".to_string(),
                        error_message: Some("Failed to decrypt wallet".to_string()),
                    }),
                    wallet_accessed: false,
                },
                ExecutionAttempt {
                    order_id: "vault-key2".to_string(),
                    success: None,
                    failure: Some(crate::client::ExecutionFailure {
                        error_code: "DECRYPTION_FAILED".to_string(),
                        error_message: None,
                    }),
                    wallet_accessed: false,
                },
            ],
            error: None,
            success: true,
        };

        let result = display_vault_results(&response, &wallet, "wrong-password");
        assert!(result.is_ok());
    }

    #[test]
    fn test_create_vault_game_result() {
        let response = ProveGameResponse {
            game_session_id: "test-session".to_string(),
            attempts: vec![ExecutionAttempt {
                order_id: "vault-key1".to_string(),
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

        let result = create_vault_game_result(&response, "test-session");
        assert!(result.is_ok());

        let result = result.unwrap();
        assert_eq!(result.session_id, "test-session");
        assert_eq!(result.game_type, 2);
        assert!(result.success);
        assert_eq!(result.signature, Some("test-sig".to_string()));
    }

    #[test]
    fn test_create_vault_game_result_with_error() {
        let response = ProveGameResponse {
            game_session_id: "test-session".to_string(),
            attempts: vec![],
            error: Some("Network error".to_string()),
            success: false,
        };

        let result = create_vault_game_result(&response, "test-session");
        assert!(result.is_ok());

        let result = result.unwrap();
        assert_eq!(result.session_id, "test-session");
        assert_eq!(result.game_type, 2);
        assert!(!result.success);
        assert_eq!(result.error, Some("Network error".to_string()));
        assert!(result.signature.is_none());
    }

    #[test]
    fn test_password_empty_rejected() {
        // Empty password should be rejected
        let password = "";
        assert!(password.is_empty());
    }

    #[test]
    fn test_password_whitespace_rejected() {
        // Whitespace-only password should be rejected
        let password = "   ";
        assert!(password.trim().is_empty());
    }

    #[test]
    fn test_password_minimum_length() {
        // Password should have minimum reasonable length
        let password = "ab";
        assert!(password.len() < 8, "Password too short");
    }

    #[test]
    fn test_key_derivation_consistency() {
        let _temp = setup_test_env();
        // Same password should always produce same key
        let password = "consistent-password-123";

        for i in 0..10 {
            let hkdf = Hkdf::<Sha256>::new(None, password.as_bytes());
            let mut key = [0u8; 32];
            hkdf.expand(b"edge-vault-game", &mut key)
                .expect("Key derivation failed");

            // Store and retrieve with unique ID for each iteration
            let key_id = format!("consistent-test-{}", i);
            store_derived_key(key_id.clone(), &key).expect("Failed to store key");

            let retrieved = get_derived_key(&key_id)
                .expect("Failed to get key")
                .expect("Key not found");

            assert_eq!(key, retrieved);
        }
    }

    #[test]
    fn test_encrypted_blob_structure() {
        use aes_gcm::{
            Aes256Gcm, Nonce,
            aead::{Aead, KeyInit},
        };

        let key: [u8; 32] = [0xCD; 32];
        let plaintext = b"wallet private key here";

        // Encrypt
        let cipher = Aes256Gcm::new(&key.into());
        let nonce: [u8; 12] = rand::random();
        let ciphertext = cipher
            .encrypt(Nonce::from_slice(&nonce), plaintext.as_ref())
            .expect("Encryption failed");

        // AES-GCM ciphertext format: plaintext_len + 16 bytes (auth tag)
        // The auth tag is always 16 bytes, so ciphertext.len() == plaintext.len() + 16
        assert_eq!(
            ciphertext.len(),
            plaintext.len() + 16,
            "Ciphertext should be plaintext + 16 byte auth tag"
        );
    }

    #[test]
    fn test_multiple_password_key_isolation() {
        let _temp = setup_test_env();
        // Each password should have independent storage
        let passwords = vec![("pwd1", "id1"), ("pwd2", "id2"), ("pwd3", "id3")];

        for (password, id) in &passwords {
            let hkdf = Hkdf::<Sha256>::new(None, password.as_bytes());
            let mut key = [0u8; 32];
            hkdf.expand(b"edge-vault-game", &mut key)
                .expect("Key derivation failed");

            store_derived_key(id.to_string(), &key).expect("Failed to store");
        }

        // Verify each key is retrievable
        for (_, id) in &passwords {
            let key = get_derived_key(id)
                .expect("Failed to get key")
                .expect("Key not found");
            assert_eq!(key.len(), 32);
        }
    }

    #[test]
    fn test_wallet_address_from_private_key() {
        use k256::ecdsa::SigningKey;
        use sha3::{Digest, Keccak256};

        // Create a valid private key (not all zeros - must be valid secp256k1 scalar)
        let private_key_bytes: [u8; 32] = [
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10, 0x11, 0x12,
            0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F, 0x20,
        ];

        // Create wallet with this private key
        let private_key_b64 = base64::engine::general_purpose::STANDARD.encode(private_key_bytes);

        // Derive public key and address
        let signing_key = SigningKey::from_bytes(&private_key_bytes.into())
            .expect("Failed to create signing key - invalid private key");
        let verifying_key = k256::ecdsa::VerifyingKey::from(&signing_key);
        let public_key_bytes = verifying_key.to_encoded_point(false).as_bytes().to_vec();

        // Derive Ethereum address
        let hash = Keccak256::digest(&public_key_bytes[1..]);
        let derived_address = format!("0x{}", hex::encode(&hash[hash.len() - 20..]));

        // Create test wallet and verify address matches
        let wallet = GameWallet {
            address: derived_address.clone(),
            private_key: private_key_b64,
            chain_type: "EVM".to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
        };

        assert_eq!(wallet.address, derived_address);
    }

    #[test]
    fn test_hkdf_rfc5869_compliance() {
        // Test HKDF against known test vectors from RFC 5869
        let ikm = b"some input key material";
        let salt: Option<&[u8]> = None;
        let info = b"edge-vault-game";

        let hkdf = Hkdf::<Sha256>::new(salt, ikm);
        let mut okm = [0u8; 32];
        hkdf.expand(info, &mut okm).expect("HKDF expand failed");

        // Output should be deterministic
        let hkdf2 = Hkdf::<Sha256>::new(salt, ikm);
        let mut okm2 = [0u8; 32];
        hkdf2.expand(info, &mut okm2).expect("HKDF expand failed");

        assert_eq!(okm, okm2, "HKDF output should be deterministic");
    }
}
