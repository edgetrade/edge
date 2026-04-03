//! Wallet operations for Edge CLI.
//!
//! Implements cryptographically secure wallet generation for EVM
//! (secp256k1) and Solana (ed25519) chains.

use aes_gcm::aead::rand_core;
use ed25519_dalek::{SigningKey as SolanaSigningKey, VerifyingKey as SolanaVerifyingKey};
use k256::ecdsa::{SigningKey as EvmSigningKey, VerifyingKey as EvmVerifyingKey};
use sha3::{Digest as Sha3Digest, Keccak256};

use tyche_enclave::{envelopes::storage::StorageEnvelope, envelopes::storage::WalletKey, types::chain_type::ChainType};

use crate::domains::client::{IrisClient, upsert_wallet};
use crate::domains::enclave::crypto::UsersEncryptionKeys;
use crate::domains::enclave::wallet::types::{Wallet, WalletError, WalletResult};

/// Create a new wallet with a randomly generated key.
///
/// Generates a new wallet for the specified chain and encrypts the private
/// key with the user encryption key. Note: This function does NOT store the
/// wallet in the backend. Call `upsert_encrypted_wallet` separately to store it.
///
/// # Arguments
/// * `chain` - The blockchain chain type (ChainType::EVM or ChainType::SVM)
/// * `user_key` - The user's encryption key (from authenticated session)
/// * `name` - Wallet name
///
/// # Returns
/// A `Wallet` struct with the encrypted private key and derived address.
pub async fn create_wallet(
    chain: ChainType,
    name: String,
    user_key: &UsersEncryptionKeys,
    client: &IrisClient,
) -> WalletResult<Wallet> {
    let wallet = match chain {
        ChainType::EVM => create_evm_wallet(user_key, name)?,
        ChainType::SVM => create_svm_wallet(user_key, name)?,
    };
    upsert_wallet(wallet, user_key, client).await
}

fn create_evm_wallet(user_key: &UsersEncryptionKeys, name: String) -> WalletResult<Wallet> {
    // Generate random secp256k1 keypair
    let signing_key = EvmSigningKey::random(&mut rand_core::OsRng);

    // Derive Ethereum address: keccak256(public_key)[12..32]
    let verifying_key = EvmVerifyingKey::from(&signing_key);
    let public_key_bytes = verifying_key.to_encoded_point(false).as_bytes().to_vec();

    if public_key_bytes.len() != 65 || public_key_bytes[0] != 0x04 {
        return Err(WalletError::AddressDerivationFailed(
            "Invalid public key format".to_string(),
        ));
    }

    let hash = Keccak256::digest(&public_key_bytes[1..]);
    let address = format!("0x{}", hex::encode(&hash[hash.len() - 20..]));

    let encrypted_private_key = WalletKey::new(ChainType::EVM, address.clone(), signing_key.to_bytes().into())
        .seal(&user_key.storage)
        .map_err(|e| WalletError::StorageFailed(e.to_string()))?;

    Ok(Wallet {
        chain: ChainType::EVM,
        address,
        name,
        encrypted_private_key,
    })
}

fn create_svm_wallet(user_key: &UsersEncryptionKeys, name: String) -> WalletResult<Wallet> {
    // Generate random ed25519 keypair
    let signing_key = SolanaSigningKey::generate(&mut rand_core::OsRng);

    // Derive Solana address (base58-encoded public key)
    let verifying_key = SolanaVerifyingKey::from(&signing_key);
    let address = bs58::encode(verifying_key.as_bytes()).into_string();

    // Encrypt private key (32-byte seed)
    let encrypted_private_key = WalletKey::new(ChainType::SVM, address.clone(), signing_key.to_bytes())
        .seal(&user_key.storage)
        .map_err(|e| WalletError::StorageFailed(e.to_string()))?;

    Ok(Wallet {
        chain: ChainType::SVM,
        address,
        name,
        encrypted_private_key,
    })
}

/// Import a wallet from a private key string.
///
/// The chain type must be explicitly specified to avoid ambiguity.
/// Note: This function does NOT store the wallet in the backend.
/// Call `upsert_encrypted_wallet` separately to store it.
///
/// # Arguments
/// * `private_key` - The encoded private key (hex for EVM, base58 for SVM)
/// * `chain` - The chain type (ChainType::EVM or ChainType::SVM)
/// * `user_key` - The user's encryption key
/// * `name` - Wallet name
///
/// # Returns
/// A `Wallet` with the imported and encrypted private key.
pub async fn import_wallet(
    private_key: &str,
    chain: ChainType,
    name: String,
    user_key: &UsersEncryptionKeys,
    client: &IrisClient,
) -> WalletResult<Wallet> {
    let wallet = match chain {
        ChainType::EVM => import_evm(private_key, user_key, name)?,
        ChainType::SVM => import_svm(private_key, user_key, name)?,
    };
    upsert_wallet(wallet, user_key, client).await
}

/// Import an EVM wallet from a hex-encoded private key.
fn import_evm(private_key_hex: &str, user_key: &UsersEncryptionKeys, name: String) -> WalletResult<Wallet> {
    // Parse hex (with or without 0x prefix)
    let hex_str = if private_key_hex.starts_with("0x") || private_key_hex.starts_with("0X") {
        &private_key_hex[2..]
    } else {
        private_key_hex
    };

    let bytes =
        hex::decode(hex_str).map_err(|e| WalletError::InvalidPrivateKey(format!("Invalid hex encoding: {}", e)))?;

    if bytes.len() != 32 {
        return Err(WalletError::InvalidPrivateKey(format!(
            "Invalid key length: expected 32 bytes, got {}",
            bytes.len()
        )));
    }

    let private_key_bytes: [u8; 32] = bytes.try_into().unwrap();

    // Create signing key and derive address
    let signing_key = EvmSigningKey::from_bytes(&private_key_bytes.into())
        .map_err(|e| WalletError::InvalidPrivateKey(format!("Invalid secp256k1 private key: {}", e)))?;

    // Derive Ethereum address: keccak256(public_key)[12..32]
    let verifying_key = k256::ecdsa::VerifyingKey::from(&signing_key);
    let public_key_bytes = verifying_key.to_encoded_point(false).as_bytes().to_vec();

    if public_key_bytes.len() != 65 || public_key_bytes[0] != 0x04 {
        return Err(WalletError::AddressDerivationFailed(
            "Invalid public key format".to_string(),
        ));
    }

    let hash = Keccak256::digest(&public_key_bytes[1..]);
    let address = format!("0x{}", hex::encode(&hash[hash.len() - 20..]));

    // Encrypt private key
    let encrypted_private_key = WalletKey::new(ChainType::EVM, address.clone(), private_key_bytes)
        .seal(&user_key.storage)
        .map_err(|e| WalletError::StorageFailed(e.to_string()))?;

    Ok(Wallet {
        chain: ChainType::EVM,
        address: address.to_lowercase(),
        name,
        encrypted_private_key,
    })
}

/// Import a Solana wallet from a base58-encoded private key.
fn import_svm(private_key_bs58: &str, user_key: &UsersEncryptionKeys, name: String) -> WalletResult<Wallet> {
    // Decode base58
    let bytes = bs58::decode(private_key_bs58)
        .into_vec()
        .map_err(|e| WalletError::InvalidPrivateKey(format!("Invalid base58 encoding: {}", e)))?;

    // Solana keys: 32 bytes (seed), 64 bytes (seed+pubkey), or 88 bytes (JSON keypair)
    if bytes.len() != 32 && bytes.len() != 64 && bytes.len() != 88 {
        return Err(WalletError::InvalidPrivateKey(format!(
            "Invalid key length: expected 32, 64, or 88 bytes, got {}",
            bytes.len()
        )));
    }

    // Extract 32-byte seed
    let seed_bytes: [u8; 32] = bytes[..32]
        .try_into()
        .map_err(|_| WalletError::InvalidPrivateKey("Could not extract 32-byte seed".to_string()))?;

    // Create signing key and derive address
    let signing_key = SolanaSigningKey::from_bytes(&seed_bytes);
    let address = bs58::encode(ed25519_dalek::VerifyingKey::from(&signing_key).as_bytes()).into_string();

    // Encrypt the seed
    let encrypted_private_key = WalletKey::new(ChainType::SVM, address.clone(), signing_key.to_bytes())
        .seal(&user_key.storage)
        .map_err(|e| WalletError::StorageFailed(e.to_string()))?;

    Ok(Wallet {
        chain: ChainType::SVM,
        address,
        name,
        encrypted_private_key,
    })
}
