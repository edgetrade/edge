//! Type-safe domain functions bridging domain types with generated route types.

use base64::Engine;
use base64::engine::general_purpose::STANDARD;

use tyche_enclave::envelopes::transport::{
    RotateUserKeyPayload, TransportEnvelope, TransportEnvelopeKey, WalletUpsert,
};
use tyche_enclave::types::chain_type::ChainType;

use crate::client::RouteExecutor;
use crate::generated::routes::requests::agent_proof_game::{self, ProofGameRequest, ProofGameResponse};
use crate::generated::routes::requests::{
    agent_create_encrypted_wallet, agent_delete_encrypted_wallet, agent_list_encrypted_wallets,
    agent_rotate_user_encryption_key,
};
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
        .map_err(|e| WalletError::StorageFailed(e.to_string()))?;

    Ok(())
}

/// Conduct the proof game.
pub async fn proof_game(
    // wallet_address: String,
    // wallet_unseal_envelope: Vec<u8>,
    // encrypted_pvt_key: Vec<u8>,
    // unsigned_tx: Vec<u8>,
    // orders: Vec<ProofGameRequestOrdersItem>,
    // user_key: &UsersEncryptionKeys,
    request: &ProofGameRequest,
    client: &impl RouteExecutor,
) -> WalletResult<ProofGameResponse> {
    // let enclave_keys = get_transport_key(client).await?;
    // let key = TransportEnvelopeKey::Unsealing(enclave_keys.deterministic);

    // let encrypted_wallet_blob = WalletUpsert::new(encrypted_pvt_key)
    //     .seal(&key)
    //     .map_err(|e| WalletError::InvalidPrivateKey(e.to_string()))?;

    // let envelope = RotateUserKeyPayload::new(user_key.storage, None)
    //     .seal(&key)
    //     .map_err(|e| WalletError::InvalidPrivateKey(e.to_string()))?;

    // let request = &ProofGameRequest {
    //     chain_id: erato::models::ChainId::ETHEREUM.to_string(),
    //     wallet_address,
    //     unsigned_tx: encode_prefixed(&unsigned_tx),
    //     wallet_envelope: STANDARD.encode(&envelope),
    //     encrypted_wallet_blob: STANDARD.encode(&encrypted_wallet_blob),
    //     orders,
    // };

    client
        .execute(&agent_proof_game::ROUTE, request)
        .await
        .map_err(|e| WalletError::StorageFailed(e.to_string()))
}

/// Delete an encrypted wallet by address.
pub async fn delete_wallet(address: String, client: &impl RouteExecutor) -> WalletResult<()> {
    let request = agent_delete_encrypted_wallet::DeleteEncryptedWalletRequest {
        wallet_address: address,
    };

    client
        .execute(&agent_delete_encrypted_wallet::ROUTE, &request)
        .await
        .map_err(|e| WalletError::StorageFailed(e.to_string()))?;

    Ok(())
}
