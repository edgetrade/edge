//! Wallet import command for Edge CLI.
//!
//! Implements wallet import from existing private keys for EVM
//! (hex-encoded secp256k1 keys) and Solana (base58-encoded ed25519 keys).
//! Supports multiple secure input methods.

use erato::ChainType;

use crate::domains::enclave::errors::EnclaveResult;
use crate::domains::enclave::wallet::name;
use crate::domains::enclave::wallet::operations::import_wallet;
use crate::domains::keystore::Session;
use crate::messages;

/// Read a private key from a file.
///
/// # Arguments
/// * `path` - Path to the file containing the private key
///
/// # Returns
/// The private key as a string
///
/// # Errors
/// Returns an error if the file cannot be read
fn read_private_key_file(path: &str) -> EnclaveResult<String> {
    std::fs::read_to_string(path)
        .map(|s| s.trim().to_string())
        .map_err(|e| {
            crate::domains::enclave::errors::EnclaveError::Io(format!("Failed to read private key file: {}", e))
        })
}

/// Import a wallet from private key file or manual input.
///
/// This function imports a wallet for the specified chain.
///
/// # Arguments
/// * `chain` - The blockchain chain (ETHEREUM or SOLANA)
/// * `name` - Optional wallet name
/// * `key_file` - Optional path to file containing the private key
///
/// # Errors
/// Returns an error if:
/// - Session is not ready
/// - Private key is invalid
/// - Wallet import fails
pub async fn wallet_import(
    chain: ChainType,
    name: Option<String>,
    key_file: Option<String>,
    session: &Session,
    client: &crate::domains::client::IrisClient,
) -> EnclaveResult<String> {
    // Step 2: Get the UEK from session
    let uek = session
        .get_user_encryption_key()
        .map_err(|e| crate::domains::enclave::errors::EnclaveError::Wallet(e.to_string()))?
        .ok_or(crate::domains::enclave::errors::EnclaveError::Wallet(
            "No active session found".to_string(),
        ))?;

    // Step 3: Print progress message
    messages::success::wallet_importing();

    // Step 4: Get the private key from file or prompt user
    // TODO: zeroize
    let key_input = if let Some(file_path) = key_file {
        read_private_key_file(&file_path)?
    } else {
        let prompt = match chain {
            ChainType::EVM => "Enter your EVM private key (hex format, with or without 0x prefix): ",
            ChainType::SVM => "Enter your SVM private key (base58 format): ",
        };
        rpassword::prompt_password(prompt)
            .map_err(|e| crate::domains::enclave::errors::EnclaveError::Io(e.to_string()))?
    };

    // Step 5: Import the wallet
    let name = name::ensure_wallet_name(chain, name);
    // TODO: add enclave keys
    let wallet = import_wallet(&key_input, chain, name, &uek, client)
        .await
        .map_err(|e| crate::domains::enclave::errors::EnclaveError::Wallet(e.to_string()))?;

    // Step 6: Print success message
    messages::success::wallet_imported(chain.to_string().as_str(), &wallet.address);
    Ok(wallet.address)
}
