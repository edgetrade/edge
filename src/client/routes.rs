//! Type-safe domain functions bridging domain types with generated route types.

use base64::Engine;
use base64::engine::general_purpose::STANDARD;

use tyche_enclave::envelopes::transport::{
    RotateUserKeyPayload, TransportEnvelope, TransportEnvelopeKey, WalletUpsert,
};
use tyche_enclave::types::chain_type::ChainType;

use crate::client::RouteExecutor;
use crate::generated::routes::requests::agent_proof_game::{self, ProofGameRequest, ProofGameResponse};
use crate::generated::routes::requests::orders_place_spot_order::{PlaceSpotOrderRequest, PlaceSpotOrderResponseItem};
use crate::generated::routes::requests::{
    agent_create_encrypted_wallet, agent_delete_encrypted_wallet, agent_list_encrypted_wallets,
    agent_rotate_user_encryption_key, orders_place_spot_order,
};
use crate::orders::OrdersError;
use crate::session::crypto::UsersEncryptionKeys;
use crate::session::transport::get_transport_key;
use crate::wallet::types::{Wallet, WalletError, WalletList, WalletResult};

/// Create or update an encrypted wallet.
pub async fn upsert_wallet(
    wallet: Wallet,
    user_key: &UsersEncryptionKeys,
    client: &impl RouteExecutor,
) -> WalletResult<Wallet> {
    let enclave_keys = get_transport_key(client).await?;
    let key = TransportEnvelopeKey::Unsealing(enclave_keys.deterministic);

    let encrypted_blob = WalletUpsert::new(wallet.encrypted_private_key.clone())
        .seal(&key)
        .map_err(|e| WalletError::StorageFailed(e.to_string()))?;

    let envelope = RotateUserKeyPayload::new(user_key.storage, None)
        .seal(&key)
        .map_err(|e| WalletError::InvalidPrivateKey(e.to_string()))?;

    let request = agent_create_encrypted_wallet::CreateEncryptedWalletRequest {
        name: wallet.name.clone(),
        address: wallet.address.clone(),
        blob: STANDARD.encode(&encrypted_blob),
        envelope: STANDARD.encode(&envelope),
    };

    client
        .execute(&agent_create_encrypted_wallet::ROUTE, &request)
        .await
        .map_err(|e| WalletError::StorageFailed(e.to_string()))?;

    Ok(wallet)
}

/// List all encrypted wallets for the current agent.
pub async fn list_wallets(client: &impl RouteExecutor) -> WalletResult<Vec<WalletList>> {
    let response: agent_list_encrypted_wallets::ListEncryptedWalletsResponse = client
        .execute(&agent_list_encrypted_wallets::ROUTE, &())
        .await
        .map_err(|e| WalletError::StorageFailed(e.to_string()))?;

    let mut wallets = Vec::new();

    if let Ok(evm) =
        serde_json::from_value::<agent_list_encrypted_wallets::ListEncryptedWalletsResponseExtraValue>(response.evm)
    {
        wallets.push(WalletList {
            chain_type: ChainType::EVM,
            name: evm.name,
            address: evm.address,
        });
    }

    if let Ok(svm) =
        serde_json::from_value::<agent_list_encrypted_wallets::ListEncryptedWalletsResponseExtraValue>(response.svm)
    {
        wallets.push(WalletList {
            chain_type: ChainType::SVM,
            name: svm.name,
            address: svm.address,
        });
    }

    Ok(wallets)
}

/// Rotate the user encryption key.
pub async fn rotate_user_encryption_key(
    new_key: &UsersEncryptionKeys,
    old_key: &UsersEncryptionKeys,
    client: &impl RouteExecutor,
) -> WalletResult<()> {
    let enclave_keys = get_transport_key(client).await?;
    let key = TransportEnvelopeKey::Unsealing(enclave_keys.deterministic);

    let envelope = RotateUserKeyPayload::new(new_key.storage, Some(old_key.storage))
        .seal(&key)
        .map_err(|e| WalletError::InvalidPrivateKey(e.to_string()))?;

    let request = agent_rotate_user_encryption_key::RotateUserEncryptionKeyRequest {
        envelope: STANDARD.encode(&envelope),
    };

    client
        .execute(&agent_rotate_user_encryption_key::ROUTE, &request)
        .await
        .map_err(|e| WalletError::Serialization(e.to_string()))?;

    Ok(())
}

pub async fn place_spot_order(
    request: &PlaceSpotOrderRequest,
    client: &impl RouteExecutor,
) -> Result<Vec<PlaceSpotOrderResponseItem>, OrdersError> {
    client
        .execute(&orders_place_spot_order::ROUTE, request)
        .await
        .map_err(|e| OrdersError::ExecutionError(e.to_string()))
}

/// Conduct the proof game.
pub async fn proof_game(request: &ProofGameRequest, client: &impl RouteExecutor) -> WalletResult<ProofGameResponse> {
    client
        .execute(&agent_proof_game::ROUTE, request)
        .await
        .map_err(|e| WalletError::Crypto(e.to_string()))
}

/// Delete an encrypted wallet by address.
pub async fn delete_wallet(address: String, client: &impl RouteExecutor) -> WalletResult<()> {
    let request = agent_delete_encrypted_wallet::DeleteEncryptedWalletRequest {
        wallet_address: address,
    };

    client
        .execute(&agent_delete_encrypted_wallet::ROUTE, &request)
        .await
        .map_err(|e| WalletError::WalletNotFound(e.to_string()))?;

    Ok(())
}
