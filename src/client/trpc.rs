use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use serde_json::json;

use tyche_enclave::{
    envelopes::transport::{RotateUserKeyPayload, TransportEnvelope, TransportEnvelopeKey, WalletUpsert},
    types::chain_type::ChainType,
};

use crate::wallet::types::{
    CreateEncryptedWalletResponse, ListEncryptedWalletsResponse, Wallet, WalletError, WalletList, WalletResult,
};
use crate::{client::IrisClient, session::crypto::UsersEncryptionKeys};

use super::get_transport_key;

pub async fn upsert_encrypted_wallet(
    wallet: Wallet,
    user_key: &UsersEncryptionKeys,
    client: &IrisClient,
) -> WalletResult<Wallet> {
    let enclave_keys = &get_transport_key(client).await?;
    let key = TransportEnvelopeKey::Unsealing(enclave_keys.deterministic);

    // First we get the encrypted_wallet_material sorted. This is saved in the db after being dual encrypted.
    let encrypted_wallet_blob = WalletUpsert::new(wallet.encrypted_private_key.clone())
        .seal(&key)
        .map_err(|e| WalletError::StorageFailed(e.to_string()))?;

    // Second we get the envelope for the users keys sorted.
    let envelope = RotateUserKeyPayload::new(user_key.storage, None)
        .seal(&key)
        .map_err(|e| WalletError::InvalidPrivateKey(e.to_string()))?;

    let _response: CreateEncryptedWalletResponse = client
        .mutation(
            "agent.createEncryptedWallet",
            json!({
                "name": wallet.name.clone(),
                "address": wallet.address.clone(),
                "blob": STANDARD.encode(encrypted_wallet_blob),
                "envelope": STANDARD.encode(envelope),
            }),
        )
        .await
        .map_err(|e| WalletError::StorageFailed(e.to_string()))?;

    Ok(wallet)
}

pub async fn list_wallets(client: &IrisClient) -> WalletResult<Vec<WalletList>> {
    let response: ListEncryptedWalletsResponse = client
        .query("agent.listEncryptedWallets", json!({}))
        .await
        .map_err(|e| WalletError::StorageFailed(e.to_string()))?;

    // Convert the HashMap<chain_type, address> to Vec<WalletList>
    let wallets: Vec<WalletList> = response
        .0
        .into_iter()
        .map(|(chain_str, entry)| {
            let chain_type = ChainType::parse(&chain_str).map_err(|_| WalletError::ParsingWalletList)?;

            Ok(WalletList {
                chain_type,
                name: entry.name,
                address: entry.address,
            })
        })
        .collect::<Result<Vec<_>, WalletError>>()?;

    Ok(wallets)
}

pub async fn rotate_user_encryption_key(
    new_user_encryption_key: &UsersEncryptionKeys,
    old_user_encryption_key: &UsersEncryptionKeys,
    client: &IrisClient,
) -> WalletResult<()> {
    let enclave_keys = &get_transport_key(client).await?;
    let key = TransportEnvelopeKey::Unsealing(enclave_keys.deterministic);

    let envelope = RotateUserKeyPayload::new(new_user_encryption_key.storage, Some(old_user_encryption_key.storage))
        .seal(&key)
        .map_err(|e| WalletError::InvalidPrivateKey(e.to_string()))?;

    let _response: CreateEncryptedWalletResponse = client
        .mutation(
            "agent.rotateUserEncryptionKey",
            json!({
                "envelope": STANDARD.encode(envelope),
            }),
        )
        .await
        .map_err(|e| WalletError::StorageFailed(e.to_string()))?;

    Ok(())
}

pub async fn delete_wallet(address: String, client: &IrisClient) -> WalletResult<()> {
    let _response: CreateEncryptedWalletResponse = client
        .mutation(
            "agent.deleteEncryptedWallet",
            json!({
                "walletAddress": address
            }),
        )
        .await
        .map_err(|e| WalletError::StorageFailed(e.to_string()))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::wallet::types::WalletEntry;

    use super::*;
    use crate::client::transport::verify_attestation_document;

    #[test]
    fn test_list_wallets_response_conversion() {
        // Test the conversion logic without any API calls
        let mut wallets_map = HashMap::new();
        wallets_map.insert(
            "EVM".to_string(),
            WalletEntry {
                name: "EVM".to_string(),
                address: "0xabc123".to_string(),
            },
        );
        wallets_map.insert(
            "SVM".to_string(),
            WalletEntry {
                name: "SVM".to_string(),
                address: "SolanaAddress123".to_string(),
            },
        );

        let response = ListEncryptedWalletsResponse(wallets_map);

        // Convert the HashMap<chain_type, address> to Vec<WalletList>
        let wallets: Vec<WalletList> = response
            .0
            .into_iter()
            .map(|(chain_str, entry)| {
                let chain_type = ChainType::parse(&chain_str).map_err(|_| WalletError::ParsingWalletList)?;

                Ok(WalletList {
                    chain_type,
                    name: entry.name,
                    address: entry.address,
                })
            })
            .collect::<Result<Vec<_>, WalletError>>()
            .unwrap();

        assert_eq!(wallets.len(), 2);

        // Check EVM wallet
        let evm_wallet = wallets
            .iter()
            .find(|w| matches!(w.chain_type, ChainType::EVM));
        assert!(evm_wallet.is_some());
        assert_eq!(evm_wallet.unwrap().address, "0xabc123");

        // Check SVM wallet
        let svm_wallet = wallets
            .iter()
            .find(|w| matches!(w.chain_type, ChainType::SVM));
        assert!(svm_wallet.is_some());
        assert_eq!(svm_wallet.unwrap().address, "SolanaAddress123");
    }

    #[test]
    fn test_list_wallets_empty_response() {
        let response = ListEncryptedWalletsResponse(HashMap::new());

        let wallets: Vec<WalletList> = response
            .0
            .into_iter()
            .map(|(chain_str, entry)| {
                let chain_type = ChainType::parse(&chain_str).map_err(|_| WalletError::ParsingWalletList)?;

                Ok(WalletList {
                    chain_type,
                    name: entry.name,
                    address: entry.address,
                })
            })
            .collect::<Result<Vec<_>, WalletError>>()
            .unwrap();

        assert!(wallets.is_empty());
    }

    #[test]
    fn test_list_wallets_invalid_chain() {
        let mut wallets_map = HashMap::new();
        wallets_map.insert(
            "INVALID_CHAIN".to_string(),
            WalletEntry {
                name: "INVALID_CHAIN".to_string(),
                address: "0xabc123".to_string(),
            },
        );

        let response = ListEncryptedWalletsResponse(wallets_map);

        let result: Result<Vec<WalletList>, WalletError> = response
            .0
            .into_iter()
            .map(|(chain_str, entry)| {
                let chain_type = ChainType::parse(&chain_str).map_err(|_| WalletError::ParsingWalletList)?;

                Ok(WalletList {
                    chain_type,
                    name: entry.name,
                    address: entry.address,
                })
            })
            .collect();

        assert!(result.is_err());
        match result.unwrap_err() {
            WalletError::ParsingWalletList => (), // Expected
            _ => panic!("Expected ParsingWalletList error"),
        }
    }

    #[test]
    fn test_verify_attestation_empty() {
        let result = verify_attestation_document(&[]);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("empty"));
    }

    #[test]
    fn test_verify_attestation_non_empty() {
        // Currently a placeholder, should succeed with any non-empty data
        let result = verify_attestation_document(b"some attestation data");
        assert!(result.is_ok());
    }
}
