//! Wallet list command for Edge CLI.
//!
//! Lists the current EVM and SVM wallets for the agent.
//! Each agent can have at most one wallet per chain type.

use crate::domains::client::IrisClient;
use crate::domains::client::list_wallets;
use crate::domains::enclave::actor::WalletInfo;
use crate::domains::enclave::errors::EnclaveResult;
use crate::messages;

/// List wallets for the agent.
///
/// This function lists the current wallets:
/// 1. Validates API key is present
/// 2. Fetches wallet data from the API (stubbed for now)
/// 3. Displays EVM and SVM wallet addresses
///
/// # Errors
/// Returns an error if:
/// - API key is not provided
/// - API request fails
pub async fn wallet_list(client: &IrisClient) -> EnclaveResult<()> {
    let wallets = wallet_list_internal(client).await?;

    messages::success::wallets_list_header();
    for wallet in wallets {
        messages::success::wallet_item(wallet.chain_type.to_string().as_str(), &wallet.address, &wallet.name);
    }

    Ok(())
}

/// Internal function to list wallets and return them.
///
/// # Returns
/// The list of wallets.
pub async fn wallet_list_internal(client: &IrisClient) -> EnclaveResult<Vec<WalletInfo>> {
    let wallets = list_wallets(client)
        .await
        .map_err(|e| crate::domains::enclave::errors::EnclaveError::Wallet(e.to_string()))?;

    Ok(wallets
        .into_iter()
        .map(|w| WalletInfo {
            chain_type: w.chain_type,
            name: w.name,
            address: w.address,
        })
        .collect())
}
