//! Wallet creation for Edge CLI.
//!
//! Implements cryptographically secure wallet generation for EVM
//! (secp256k1) and Solana (ed25519) chains.

use aes_gcm::aead::rand_core;
use ed25519_dalek::{SigningKey as SolanaSigningKey, VerifyingKey as SolanaVerifyingKey};
use k256::ecdsa::{SigningKey as EvmSigningKey, VerifyingKey as EvmVerifyingKey};
use sha3::{Digest as Sha3Digest, Keccak256};

use tyche_enclave::{envelopes::storage::StorageEnvelope, envelopes::storage::WalletKey, types::chain_type::ChainType};

use crate::client::IrisClient;
use crate::client::upsert_encrypted_wallet;
use crate::session::crypto::UsersEncryptionKeys;

use super::types::{Wallet, WalletError, WalletResult};

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
    upsert_encrypted_wallet(wallet, user_key, client).await
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

    let encrypted_private_key = WalletKey::new(ChainType::EVM, address.clone(), signing_key.to_bytes().to_vec())
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
    let encrypted_private_key = WalletKey::new(ChainType::SVM, address.clone(), signing_key.to_bytes().to_vec())
        .seal(&user_key.storage)
        .map_err(|e| WalletError::StorageFailed(e.to_string()))?;

    Ok(Wallet {
        chain: ChainType::SVM,
        address,
        name,
        encrypted_private_key,
    })
}
