//! Wallet create command for Edge CLI.
//!
//! Implements wallet creation for EVM (secp256k1) and Solana (ed25519)
//! chains. Generates cryptographically secure keys and encrypts them
//! with the user's encryption key.

use erato::ChainType;

use crate::domains::enclave::errors::EnclaveResult;
use crate::domains::enclave::wallet::name;
use crate::domains::enclave::wallet::operations::create_wallet;
use crate::domains::keystore::Session;
use crate::messages;

/// Create a new wallet.
///
/// This function creates a new wallet for the specified chain after
/// ensuring the session is ready. The wallet's private key is encrypted
/// with the user's encryption key before being stored.
///
/// # Arguments
/// * `chain` - The blockchain chain (EVM or SVM)
/// * `name` - Optional wallet name (generates one based on timestamp if not provided)
///
/// # Errors
/// Returns an error if:
/// - Session is not available or cannot be unlocked
/// - Wallet creation fails
pub async fn wallet_create(
    chain: ChainType,
    name: Option<String>,
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

    // Step 4: Create the wallet
    let name = name::ensure_wallet_name(chain, name);
    // TODO: add enclave keys
    let wallet = create_wallet(chain, name, &uek, client)
        .await
        .map_err(|e| crate::domains::enclave::errors::EnclaveError::Wallet(e.to_string()))?;

    // Step 5: Print success message
    messages::success::wallet_created(chain.to_string().as_str(), &wallet.address);
    Ok(wallet.address)
}
