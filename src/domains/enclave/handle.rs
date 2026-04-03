//! Enclave domain handle.
//!
//! Provides the public API for interacting with the enclave domain.
//! Sends messages to the actor and receives responses.

use tokio::sync::mpsc;

use crate::event_bus::EventBus;

use super::actor::EnclaveActor;
use super::actor::{ChainType, EnclaveState, TradeIntent, WalletInfo, WalletMetadata};
use super::errors::{EnclaveError, EnclaveResult};
use super::messages::{EnclaveMessage, EnclaveRequest};

/// Handle for interacting with the enclave domain.
#[derive(Debug, Clone)]
pub struct EnclaveHandle {
    /// Sender for requests to the actor.
    sender: mpsc::Sender<EnclaveRequest>,
}

impl EnclaveHandle {
    /// Create a new enclave handle with receiver and EventBus.
    ///
    /// Used by the orchestrator to wire domains together.
    ///
    /// # Arguments
    /// * `receiver` - The mpsc receiver channel for the actor
    /// * `event_bus` - EventBus for publishing state events
    pub async fn new(receiver: mpsc::Receiver<EnclaveRequest>, event_bus: EventBus) -> EnclaveResult<Self> {
        let (sender, _rx) = mpsc::channel::<EnclaveRequest>(64);
        let actor = EnclaveActor::new(event_bus);

        // Spawn the actor task with the provided receiver
        tokio::spawn(async move {
            actor.run(receiver).await;
        });

        Ok(Self { sender })
    }

    /// Create an EnclaveHandle from an existing sender.
    ///
    /// Used internally when the actor is already spawned.
    pub fn from_sender(sender: mpsc::Sender<EnclaveRequest>) -> Self {
        Self { sender }
    }

    /// Send a message and await a response.
    async fn send_message(&self, message: EnclaveMessage) -> EnclaveResult<EnclaveState> {
        let (reply_to, rx) = tokio::sync::oneshot::channel();
        let request = EnclaveRequest {
            payload: message,
            reply_to,
            trace_ctx: crate::event_bus::TraceContext::new(),
        };

        self.sender
            .send(request)
            .await
            .map_err(|_| EnclaveError::ChannelError)?;

        rx.await.map_err(|_| EnclaveError::ChannelError)?
    }

    /// Create a new wallet.
    ///
    /// # Arguments
    /// * `chain` - The blockchain chain type.
    /// * `name` - Wallet name.
    ///
    /// # Returns
    /// The wallet metadata on success.
    ///
    /// # Errors
    /// Returns an error if the wallet already exists or creation fails.
    pub async fn create_wallet(&self, chain: ChainType, name: String) -> EnclaveResult<WalletMetadata> {
        let state = self
            .send_message(EnclaveMessage::CreateWallet { chain, name })
            .await?;

        // Return the newly created wallet
        state
            .wallets
            .values()
            .last()
            .cloned()
            .ok_or_else(|| EnclaveError::Wallet("Wallet creation failed".to_string()))
    }

    /// Import a wallet from private key.
    ///
    /// # Arguments
    /// * `chain` - The blockchain chain type.
    /// * `name` - Wallet name.
    /// * `private_key` - The private key (will be zeroized after use).
    ///
    /// # Returns
    /// The wallet metadata on success.
    ///
    /// # Errors
    /// Returns an error if the wallet already exists or import fails.
    pub async fn import_wallet(
        &self,
        chain: ChainType,
        name: String,
        private_key: String,
    ) -> EnclaveResult<WalletMetadata> {
        let state = self
            .send_message(EnclaveMessage::ImportWallet {
                chain,
                name,
                private_key,
            })
            .await?;

        // Return the newly imported wallet
        state
            .wallets
            .values()
            .last()
            .cloned()
            .ok_or_else(|| EnclaveError::Wallet("Wallet import failed".to_string()))
    }

    /// Delete a wallet.
    ///
    /// # Arguments
    /// * `name` - The wallet name to delete.
    ///
    /// # Errors
    /// Returns an error if the wallet doesn't exist.
    pub async fn delete_wallet(&self, name: String) -> EnclaveResult<()> {
        self.send_message(EnclaveMessage::DeleteWallet { name })
            .await?;
        Ok(())
    }

    /// List all wallets.
    ///
    /// # Returns
    /// List of wallet info.
    pub async fn list_wallets(&self) -> EnclaveResult<Vec<WalletInfo>> {
        let state = self.send_message(EnclaveMessage::ListWallets).await?;

        let list_wallets: Vec<WalletInfo> = state
            .wallets
            .values()
            .map(|metadata| WalletInfo {
                chain_type: metadata.chain,
                name: metadata.name.clone(),
                address: metadata.address.clone(),
            })
            .collect();

        Ok(list_wallets)
    }

    /// Sign a trade intent with a wallet.
    ///
    /// # Arguments
    /// * `wallet_name` - The wallet name to sign with.
    /// * `intent` - The trade intent to sign.
    ///
    /// # Returns
    /// Encrypted signed payload on success.
    ///
    /// # Errors
    /// Returns an error if the wallet doesn't exist or signing fails.
    pub async fn sign_trade_intent(&self, wallet_name: String, intent: TradeIntent) -> EnclaveResult<Vec<u8>> {
        let _state = self
            .send_message(EnclaveMessage::SignTradeIntent { wallet_name, intent })
            .await?;

        // For now, return empty vec - actual signed payload would be in state
        // This is a placeholder for the full implementation
        Ok(vec![])
    }

    /// Zeroize all key material.
    ///
    /// # Security
    /// This securely wipes all UEK and key material from memory.
    /// Should be called when the keystore is locked.
    pub async fn zeroize_keys(&self) -> EnclaveResult<()> {
        self.send_message(EnclaveMessage::ZeroizeKeys).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    // Note: These tests would require a full EventBus and async runtime
    // They are marked as placeholders for the actual test implementation

    #[test]
    fn test_enclave_handle_creation() {
        // This is a placeholder - actual test would use tokio::test
        // and create an EventBus and EnclaveHandle
    }
}
