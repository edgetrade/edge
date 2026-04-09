//! Enclave domain messages.
//!
//! Defines all command and query messages for the enclave domain,
//! used for communication between handle and actor.

use serde::{Deserialize, Serialize};

use erato::ChainType;

use crate::event_bus::PoseidonRequest;

use super::actor::{EnclaveState, TradeIntent, WalletInfo};
use super::errors::EnclaveError;

/// Messages sent to the enclave actor.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EnclaveMessage {
    /// Create a new wallet.
    CreateWallet {
        /// Blockchain chain type.
        chain: ChainType,
        /// Wallet name.
        name: String,
    },
    /// Import a wallet from private key.
    ImportWallet {
        /// Blockchain chain type.
        chain: ChainType,
        /// Wallet name.
        name: String,
        /// Private key (will be zeroized after use).
        private_key: String,
    },
    /// Delete a wallet by name.
    DeleteWallet {
        /// Wallet name.
        name: String,
    },
    /// List all wallets.
    ListWallets,
    /// Sign a trade intent with a wallet.
    SignTradeIntent {
        /// Wallet name to sign with.
        wallet_name: String,
        /// Trade intent to sign.
        intent: TradeIntent,
    },
    /// Zeroize all key material.
    ZeroizeKeys,
}

/// Request type alias using PoseidonRequest pattern.
pub type EnclaveRequest = PoseidonRequest<EnclaveMessage, EnclaveState, EnclaveError>;

/// Response types for enclave operations.
#[derive(Debug, Clone)]
pub enum EnclaveResponse {
    /// Operation succeeded with no data.
    Success,
    /// Wallet was created.
    WalletCreated {
        /// Wallet address.
        address: String,
        /// Chain type.
        chain: ChainType,
    },
    /// Wallet was imported.
    WalletImported {
        /// Wallet address.
        address: String,
        /// Chain type.
        chain: ChainType,
    },
    /// Wallet was deleted.
    WalletDeleted {
        /// Wallet name.
        name: String,
    },
    /// List of wallets.
    WalletsList(Vec<WalletInfo>),
    /// Trade intent was signed.
    TradeIntentSigned {
        /// Intent ID.
        intent_id: u64,
        /// Signed payload (encrypted).
        signed_payload: Vec<u8>,
    },
    /// Key material was zeroized.
    KeysZeroized,
    /// Operation failed.
    Error(EnclaveError),
}

/// Events emitted by the enclave domain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EnclaveEvent {
    /// Wallet was created.
    WalletCreated {
        /// Wallet name.
        name: String,
        /// Blockchain chain.
        chain: String,
    },
    /// Wallet was imported.
    WalletImported {
        /// Wallet name.
        name: String,
    },
    /// Wallet was deleted.
    WalletDeleted {
        /// Wallet name.
        name: String,
    },
    /// Key material was zeroized.
    KeyMaterialZeroized,
    /// Trade intent was created.
    TradeIntentCreated {
        /// Intent ID.
        id: u64,
        /// Wallet address.
        wallet: String,
    },
    /// Trade intent was signed.
    TradeIntentSigned {
        /// Intent ID.
        id: u64,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enclave_message_variants() {
        let create = EnclaveMessage::CreateWallet {
            chain: ChainType::EVM,
            name: "test".to_string(),
        };
        assert!(matches!(create, EnclaveMessage::CreateWallet { .. }));

        let import = EnclaveMessage::ImportWallet {
            chain: ChainType::SVM,
            name: "imported".to_string(),
            private_key: "key".to_string(),
        };
        assert!(matches!(import, EnclaveMessage::ImportWallet { .. }));

        let delete = EnclaveMessage::DeleteWallet {
            name: "test".to_string(),
        };
        assert!(matches!(delete, EnclaveMessage::DeleteWallet { .. }));

        assert!(matches!(EnclaveMessage::ListWallets, EnclaveMessage::ListWallets));

        let sign = EnclaveMessage::SignTradeIntent {
            wallet_name: "test".to_string(),
            intent: TradeIntent {
                id: 1,
                action: "swap".to_string(),
                params: serde_json::json!({}),
            },
        };
        assert!(matches!(sign, EnclaveMessage::SignTradeIntent { .. }));

        assert!(matches!(EnclaveMessage::ZeroizeKeys, EnclaveMessage::ZeroizeKeys));
    }

    #[test]
    fn test_enclave_response_variants() {
        let created = EnclaveResponse::WalletCreated {
            address: "0x1234".to_string(),
            chain: ChainType::EVM,
        };
        assert!(matches!(created, EnclaveResponse::WalletCreated { .. }));

        let list = EnclaveResponse::WalletsList(vec![]);
        assert!(matches!(list, EnclaveResponse::WalletsList(_)));
    }

    #[test]
    fn test_enclave_event_variants() {
        let event = EnclaveEvent::WalletCreated {
            name: "test".to_string(),
            chain: "EVM".to_string(),
        };
        assert!(matches!(event, EnclaveEvent::WalletCreated { .. }));
    }
}
