//! Wallet delete command for Edge CLI.
//!
//! Deletes a wallet for the specified chain type (EVM or SVM).

use crate::client::IrisClient;
use crate::client::delete_wallet;
use crate::messages;

/// Delete a wallet for the specified chain.
///
/// This function deletes a wallet:
/// 1. Validates API key is present
/// 2. Validates chain type
/// 3. Sends delete request to the API (stubbed for now)
///
/// # Arguments
/// * `chain` - The blockchain chain (ETHEREUM or SOLANA)
///
/// # Errors
/// Returns an error if:
/// - API key is not provided
/// - Chain type is invalid
/// - API request fails
pub async fn wallet_delete(address: String, client: &IrisClient) -> messages::success::CommandResult<()> {
    delete_wallet(address.clone(), client).await?;
    messages::success::wallet_deleted(&address);
    Ok(())
}
