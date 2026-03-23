//! Wallet import command for Edge CLI.
//!
//! Implements wallet import from existing private keys for EVM
//! (hex-encoded secp256k1 keys) and Solana (base58-encoded ed25519 keys).
//! Supports multiple secure input methods.

use tyche_enclave::types::chain_type::ChainType;

use crate::messages;
use crate::session::Session;
use crate::wallet::import::import_wallet;

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
fn read_private_key_file(path: &str) -> messages::success::CommandResult<String> {
    std::fs::read_to_string(path)
        .map(|s| s.trim().to_string())
        .map_err(|e| messages::error::CommandError::Io(format!("Failed to read private key file: {}", e)))
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
    client: &crate::client::IrisClient,
) -> messages::success::CommandResult<()> {
    // Step 1: Ensure session is ready
    crate::commands::key::session_manager::ensure_session_ready("wallet")?;

    // Step 2: Get the UEK from session
    let session = Session::new();
    let uek = session
        .get_user_encryption_key()
        .map_err(|e| messages::error::CommandError::Session(e.to_string()))?
        .ok_or_else(|| messages::error::CommandError::Session("Session unavailable".to_string()))?;

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
        rpassword::prompt_password(prompt).map_err(|e| messages::error::CommandError::Io(e.to_string()))?
    };

    // Step 5: Import the wallet
    let name = super::name::ensure_wallet_name(chain, name);
    // TODO: add enclave keys
    let wallet = import_wallet(&key_input, chain, name, &uek, client)
        .await
        .map_err(messages::error::CommandError::from)?;

    // Step 6: Print success message
    messages::success::wallet_imported(chain.to_string().as_str(), &wallet.address);
    Ok(())
}
