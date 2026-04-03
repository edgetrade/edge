//! Enclave domain actor.
//!
//! Owns the enclave state and processes messages from the handle.
//! Implements secure wallet operations with zeroization guarantees.

use std::collections::HashMap;

use tokio::sync::mpsc;
use zeroize::Zeroize;

use crate::domains::enclave::wallet::name;
use crate::domains::enclave::wallet::types::{Wallet, WalletError};
use crate::event_bus::{EventBus, StateEvent};

use super::errors::{EnclaveError, EnclaveResult};
use super::messages::{EnclaveMessage, EnclaveRequest};

/// The enclave actor that owns enclave state.
pub struct EnclaveActor {
    /// The current enclave state.
    state: EnclaveState,
    /// EventBus for publishing state events.
    event_bus: EventBus,
}

impl EnclaveActor {
    /// Create a new enclave actor.
    pub fn new(event_bus: EventBus) -> Self {
        Self {
            state: EnclaveState::new(),
            event_bus,
        }
    }

    /// Emit a StateEvent to the EventBus.
    fn emit_state_event(&self, event: StateEvent) {
        let _ = self.event_bus.publish(event);
    }

    /// Run the actor loop.
    pub async fn run(mut self, mut receiver: mpsc::Receiver<EnclaveRequest>) {
        while let Some(req) = receiver.recv().await {
            let reply = match req.payload {
                EnclaveMessage::CreateWallet { chain, name } => self.create_wallet(chain, name).await,
                EnclaveMessage::ImportWallet {
                    chain,
                    name,
                    private_key,
                } => self.import_wallet(chain, name, private_key).await,
                EnclaveMessage::DeleteWallet { name } => self.delete_wallet(name).await,
                EnclaveMessage::ListWallets => self.list_wallets().await,
                EnclaveMessage::SignTradeIntent { wallet_name, intent } => {
                    self.sign_trade_intent(wallet_name, intent).await
                }
                EnclaveMessage::ZeroizeKeys => self.zeroize_keys().await,
            };
            let _ = req.reply_to.send(reply);
        }
    }

    /// Create a new wallet.
    ///
    /// Security flow:
    /// 1. Generate key pair
    /// 2. Encrypt with UEK
    /// 3. Store encrypted blob
    /// 4. Zeroize original key material
    async fn create_wallet(&mut self, chain: ChainType, name: String) -> EnclaveResult<EnclaveState> {
        // Check if wallet already exists
        if self.state.has_wallet(&name) {
            return Err(EnclaveError::WalletAlreadyExists(name));
        }

        // Generate wallet name if not provided
        let wallet_name = if name.is_empty() {
            name::ensure_wallet_name(chain, None)
        } else {
            name
        };

        // Create wallet
        // TODO: Implement with proper UEK from keystore domain
        let wallet = create_wallet_stub(chain, wallet_name.clone()).map_err(|e| EnclaveError::Wallet(e.to_string()))?;

        // Store metadata with encrypted key
        let metadata = WalletMetadata {
            name: wallet_name.clone(),
            chain: wallet.chain,
            address: wallet.address.clone(),
            encrypted_key: wallet.encrypted_private_key,
        };

        self.state.insert_wallet(metadata);

        // Emit WalletCreated event
        self.emit_state_event(StateEvent::WalletCreated {
            name: wallet_name.clone(),
            chain: chain.to_string(),
        });

        Ok(self.state.clone())
    }

    /// Import a wallet from private key.
    ///
    /// Security flow:
    /// 1. Parse private key
    /// 2. Encrypt with UEK
    /// 3. Store encrypted blob
    /// 4. Zeroize original key material
    async fn import_wallet(
        &mut self,
        chain: ChainType,
        name: String,
        mut private_key: String,
    ) -> EnclaveResult<EnclaveState> {
        // Check if wallet already exists
        if self.state.has_wallet(&name) {
            private_key.zeroize();
            return Err(EnclaveError::WalletAlreadyExists(name));
        }

        // Import wallet
        // TODO: Implement with proper UEK from keystore domain
        let wallet =
            import_wallet_stub(&private_key, chain, name.clone()).map_err(|e| EnclaveError::Wallet(e.to_string()))?;

        // Zeroize private key from memory
        private_key.zeroize();

        // Store metadata with encrypted key
        let metadata = WalletMetadata {
            name: name.clone(),
            chain: wallet.chain,
            address: wallet.address.clone(),
            encrypted_key: wallet.encrypted_private_key,
        };

        self.state.insert_wallet(metadata);

        // Emit WalletImported event
        self.emit_state_event(StateEvent::WalletImported { name: name.clone() });

        Ok(self.state.clone())
    }

    /// Delete a wallet.
    async fn delete_wallet(&mut self, name: String) -> EnclaveResult<EnclaveState> {
        // Check if wallet exists
        if !self.state.has_wallet(&name) {
            return Err(EnclaveError::WalletNotFound(name));
        }

        // Remove wallet from state
        self.state.remove_wallet(&name);

        // Emit WalletDeleted event
        self.emit_state_event(StateEvent::WalletDeleted { name: name.clone() });

        Ok(self.state.clone())
    }

    /// List all wallets.
    async fn list_wallets(&self) -> EnclaveResult<EnclaveState> {
        Ok(self.state.clone())
    }

    /// Sign a trade intent with a wallet.
    ///
    /// Security flow:
    /// 1. Load encrypted blob
    /// 2. Decrypt temporarily with UEK
    /// 3. Sign intent
    /// 4. Zeroize decrypted key
    /// 5. Return encrypted signed payload
    async fn sign_trade_intent(&self, wallet_name: String, intent: TradeIntent) -> EnclaveResult<EnclaveState> {
        // Get wallet metadata
        let _wallet = self
            .state
            .get_wallet(&wallet_name)
            .ok_or_else(|| EnclaveError::WalletNotFound(wallet_name.clone()))?;

        // TODO: Implement actual signing with UEK from keystore
        // 1. Get UEK from keystore domain
        // 2. Decrypt wallet private key using UEK
        // 3. Sign the intent
        // 4. Encrypt signed payload
        // 5. Zeroize decrypted private key

        // For now, emit the event
        self.emit_state_event(StateEvent::TradeIntentCreated {
            id: intent.id,
            wallet: wallet_name.clone(),
        });

        Ok(self.state.clone())
    }

    /// Zeroize all key material.
    async fn zeroize_keys(&mut self) -> EnclaveResult<EnclaveState> {
        // Zeroize state
        self.state.zeroize_keys();

        // Emit KeyMaterialZeroized event
        self.emit_state_event(StateEvent::KeyMaterialZeroized);

        Ok(self.state.clone())
    }
}

/// Start the enclave actor.
pub async fn run_enclave_actor(actor: EnclaveActor, receiver: mpsc::Receiver<EnclaveRequest>) {
    actor.run(receiver).await;
}

// ============================================================================
// State Types (moved from state.rs)
// ============================================================================

/// The enclave state owns all mutable state for the enclave domain.
///
/// Security model:
/// - UEK is only held temporarily during operations, never permanently
/// - Wallet metadata only stores chain, address, and encrypted blob
/// - NO plaintext private keys are ever stored in memory
#[derive(Debug, Clone, Default)]
pub struct EnclaveState {
    /// Wallet metadata only - NO private keys in memory
    pub wallets: HashMap<String, WalletMetadata>,
}

impl EnclaveState {
    /// Create a new empty enclave state.
    pub fn new() -> Self {
        Self {
            wallets: HashMap::new(),
        }
    }

    /// Get the number of wallets.
    pub fn wallet_count(&self) -> usize {
        self.wallets.len()
    }

    /// Check if a wallet exists by name.
    pub fn has_wallet(&self, name: &str) -> bool {
        self.wallets.contains_key(name)
    }

    /// Get wallet metadata by name.
    pub fn get_wallet(&self, name: &str) -> Option<&WalletMetadata> {
        self.wallets.get(name)
    }

    /// Insert a wallet into the state.
    pub fn insert_wallet(&mut self, metadata: WalletMetadata) {
        self.wallets.insert(metadata.name.clone(), metadata);
    }

    /// Remove a wallet from the state.
    pub fn remove_wallet(&mut self, name: &str) -> Option<WalletMetadata> {
        self.wallets.remove(name)
    }

    /// Zeroize all key material.
    pub fn zeroize_keys(&mut self) {
        // Clear wallets (encrypted data)
        self.wallets.clear();
    }
}

/// Wallet metadata - stores only non-sensitive information.
///
/// The encrypted_key field contains the encrypted private key blob.
/// The actual private key is never stored in plaintext.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WalletMetadata {
    /// Human-readable name for the wallet.
    pub name: String,
    /// Blockchain chain type.
    pub chain: ChainType,
    /// Wallet public address.
    pub address: String,
    /// Encrypted private key blob (UEK-encrypted).
    pub encrypted_key: Vec<u8>,
}

/// Chain type for wallets.
///
/// Re-exported from tyche_enclave for convenience.
pub use tyche_enclave::types::chain_type::ChainType;

/// Information about a wallet for listing.
#[derive(Debug, Clone)]
pub struct WalletInfo {
    /// The blockchain chain type.
    pub chain_type: ChainType,
    /// Human-readable name.
    pub name: String,
    /// Wallet address.
    pub address: String,
}

/// Trade intent for signing.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TradeIntent {
    /// Intent ID.
    pub id: u64,
    /// Action type (e.g., "swap", "transfer").
    pub action: String,
    /// Action parameters.
    pub params: serde_json::Value,
}

// ============================================================================
// Stub implementations (TODO: replace with full implementation)
// ============================================================================

/// Create a wallet stub (TODO: implement full version with UEK).
fn create_wallet_stub(chain: ChainType, name: String) -> Result<Wallet, WalletError> {
    // Stub implementation - returns dummy wallet
    let address = match chain {
        ChainType::EVM => "0x0000000000000000000000000000000000000000".to_string(),
        ChainType::SVM => "11111111111111111111111111111111".to_string(),
    };

    Ok(Wallet {
        chain,
        address,
        name,
        encrypted_private_key: vec![],
    })
}

/// Import a wallet stub (TODO: implement full version with UEK).
fn import_wallet_stub(_private_key: &str, chain: ChainType, name: String) -> Result<Wallet, WalletError> {
    // Stub implementation - returns dummy wallet
    let address = match chain {
        ChainType::EVM => "0x0000000000000000000000000000000000000000".to_string(),
        ChainType::SVM => "11111111111111111111111111111111".to_string(),
    };

    Ok(Wallet {
        chain,
        address,
        name,
        encrypted_private_key: vec![],
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wallet_metadata() {
        let metadata = WalletMetadata {
            name: "test".to_string(),
            chain: ChainType::EVM,
            address: "0x1234".to_string(),
            encrypted_key: vec![1, 2, 3],
        };

        assert_eq!(metadata.name, "test");
        assert_eq!(metadata.address, "0x1234");
    }

    #[test]
    fn test_enclave_state_new() {
        let state = EnclaveState::new();
        assert_eq!(state.wallet_count(), 0);
    }

    #[test]
    fn test_enclave_state_wallet_operations() {
        let mut state = EnclaveState::new();

        let metadata = WalletMetadata {
            name: "test".to_string(),
            chain: ChainType::EVM,
            address: "0x5678".to_string(),
            encrypted_key: vec![5, 6, 7, 8],
        };

        // Insert
        state.insert_wallet(metadata.clone());
        assert_eq!(state.wallet_count(), 1);
        assert!(state.has_wallet("test"));

        // Get
        let retrieved = state.get_wallet("test").unwrap();
        assert_eq!(retrieved.address, "0x5678");

        // Remove
        let removed = state.remove_wallet("test").unwrap();
        assert_eq!(removed.name, "test");
        assert_eq!(state.wallet_count(), 0);
        assert!(!state.has_wallet("test"));
    }

    #[test]
    fn test_trade_intent() {
        let intent = TradeIntent {
            id: 123,
            action: "swap".to_string(),
            params: serde_json::json!({"from": "ETH", "to": "USDC"}),
        };

        assert_eq!(intent.id, 123);
        assert_eq!(intent.action, "swap");
    }
}
