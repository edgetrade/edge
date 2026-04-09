//! Type-safe domain functions bridging domain types with generated route types.
//!
//! DEPRECATED: These functions have been moved to the actor/handle pattern.
//! Use ClientHandle methods instead.
//!
//! This module is kept for backward compatibility but all implementations
/// now delegate to the actor pattern via ClientHandle.
use base64::Engine;
use base64::engine::general_purpose::STANDARD;

use erato::ChainType;
use erato::messages::envelopes::transport::{
    RotateUserKeyPayload, TransportEnvelope, TransportEnvelopeKey, WalletUpsert,
};

use crate::domains::client::generated::routes::requests::agent_proof_game::{
    self, ProofGameRequest, ProofGameResponse,
};
use crate::domains::client::generated::routes::requests::orders_place_spot_order::{
    PlaceSpotOrderRequest, PlaceSpotOrderResponseItem,
};
use crate::domains::client::generated::routes::requests::{
    agent_create_encrypted_wallet, agent_delete_encrypted_wallet, agent_list_encrypted_wallets,
    agent_rotate_user_encryption_key, orders_place_spot_order,
};
use crate::domains::client::trpc::RouteExecutor;
use crate::domains::enclave::wallet::types::{Wallet, WalletError, WalletList, WalletResult};
use crate::domains::keystore::session::crypto::UsersEncryptionKeys;

/// Create or update an encrypted wallet.
///
/// DEPRECATED: Use ClientHandle::upsert_wallet instead.
pub async fn upsert_wallet(
    wallet: Wallet,
    user_key: &UsersEncryptionKeys,
    client: &impl RouteExecutor,
) -> WalletResult<Wallet> {
    let enclave_keys = crate::domains::client::transport::get_transport_key(client).await?;
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
///
/// DEPRECATED: Use ClientHandle::list_wallets instead.
pub async fn list_wallets(client: &impl RouteExecutor) -> WalletResult<Vec<WalletList>> {
    client
        .execute(&agent_list_encrypted_wallets::ROUTE, &())
        .await
        .map_err(|e| WalletError::StorageFailed(e.to_string()))
        .and_then(|ws| {
            ws.into_iter()
                .map(|w| {
                    let chain_type = ChainType::parse(w.chain_type.as_str())
                        .map_err(|e| WalletError::StorageFailed(e.to_string()))?;
                    Ok(WalletList {
                        chain_type,
                        name: w.name.unwrap_or_default(),
                        address: w.address,
                    })
                })
                .collect::<Result<Vec<_>, WalletError>>()
        })
}

/// Rotate the user encryption key.
///
/// DEPRECATED: Use ClientHandle::rotate_user_encryption_key instead.
pub async fn rotate_user_encryption_key(
    new_key: &UsersEncryptionKeys,
    old_key: &UsersEncryptionKeys,
    client: &impl RouteExecutor,
) -> WalletResult<()> {
    let enclave_keys = crate::domains::client::transport::get_transport_key(client).await?;
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

/// Place a spot order.
///
/// DEPRECATED: Use ClientHandle::place_spot_order instead.
pub async fn place_spot_order(
    request: &PlaceSpotOrderRequest,
    client: &impl RouteExecutor,
) -> WalletResult<Vec<PlaceSpotOrderResponseItem>> {
    client
        .execute(&orders_place_spot_order::ROUTE, request)
        .await
        .map_err(|e| WalletError::Crypto(e.to_string()))
}

/// Conduct the proof game.
///
/// DEPRECATED: Use ClientHandle::proof_game instead.
pub async fn proof_game(request: &ProofGameRequest, client: &impl RouteExecutor) -> WalletResult<ProofGameResponse> {
    client
        .execute(&agent_proof_game::ROUTE, request)
        .await
        .map_err(|e| WalletError::Crypto(e.to_string()))
}

/// Delete an encrypted wallet by address.
///
/// DEPRECATED: Use ClientHandle::delete_wallet instead.
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
