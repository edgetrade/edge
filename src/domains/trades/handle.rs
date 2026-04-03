//! Trades domain handle
//!
//! Provides the public API for interacting with the trades domain.
//! Sends messages to the actor and receives responses via PoseidonRequest pattern.
//! Thin gateway - delegates all business logic to the TradesActor.

use tokio::sync::mpsc;

use crate::domains::client::IrisClient;
use crate::domains::keystore::Session;
use crate::domains::trades::actor::TradesActor;
use crate::domains::trades::actor::{ChainType, TradeAction, TradeInfo, TradeRecord, TradeStatus};
use crate::domains::trades::errors::{TradesError, TradesResult};
use crate::domains::trades::messages::{TradesMessage, TradesRequest, TradesResponse};
use crate::event_bus::{EventBus, PoseidonRequest, TraceContext};

/// Handle for interacting with the trades domain.
///
/// Provides a thin interface that sends messages to the TradesActor
/// and receives responses via oneshot channels using PoseidonRequest pattern.
#[derive(Debug, Clone)]
pub struct TradesHandle {
    /// Sender for requests to the actor.
    sender: mpsc::Sender<TradesRequest>,
}

impl TradesHandle {
    /// Create a new trades handle with receiver and EventBus.
    ///
    /// Used by the orchestrator to wire domains together.
    ///
    /// # Arguments
    /// * `receiver` - The mpsc receiver channel for the actor
    /// * `event_bus` - EventBus for publishing state events
    pub async fn new(receiver: mpsc::Receiver<TradesRequest>, event_bus: EventBus) -> TradesResult<Self> {
        let (sender, _rx) = mpsc::channel::<TradesRequest>(64);
        let actor = TradesActor::new(event_bus);

        // Spawn the actor task with the provided receiver
        tokio::spawn(async move {
            actor.run(receiver).await;
        });

        Ok(Self { sender })
    }

    /// Create a TradesHandle from an existing sender.
    ///
    /// Used internally when the actor is already spawned.
    pub fn from_sender(sender: mpsc::Sender<TradesRequest>) -> Self {
        Self { sender }
    }

    /// Send a request to the actor and wait for response.
    ///
    /// Internal helper method that wraps the PoseidonRequest pattern.
    /// Creates a oneshot channel, sends the request with trace context,
    /// and awaits the response.
    ///
    /// # Arguments
    /// * `payload` - The TradesMessage to send
    ///
    /// # Returns
    /// The TradesResponse on success, or a TradesError on failure
    async fn send_request(&self, payload: TradesMessage) -> TradesResult<TradesResponse> {
        let (reply_to, rx) = tokio::sync::oneshot::channel();

        let request = PoseidonRequest {
            payload,
            trace_ctx: TraceContext::current(),
            reply_to,
        };

        self.sender
            .send(request)
            .await
            .map_err(|_| TradesError::ChannelSend)?;

        rx.await.map_err(|_| TradesError::ChannelRecv)?
    }

    /// Create a trade intent.
    ///
    /// Creates a new trade intent that awaits user confirmation.
    /// The intent will expire after a timeout (default: 5 minutes).
    ///
    /// # Arguments
    /// * `wallet_address` - Wallet address for the trade
    /// * `chain` - Blockchain chain type (Ethereum, Solana)
    /// * `action` - Trade action (Swap, Transfer, etc.)
    ///
    /// # Returns
    /// The new intent ID on success
    ///
    /// # Errors
    /// Returns TradesError if intent creation fails
    pub async fn create_intent(
        &self,
        wallet_address: String,
        chain: ChainType,
        action: TradeAction,
    ) -> TradesResult<u64> {
        self.send_request(TradesMessage::CreateIntent {
            wallet_address,
            chain,
            action,
            params: serde_json::json!({}),
        })
        .await
        .and_then(|response| match response {
            TradesResponse::IntentCreated { intent_id, .. } => Ok(intent_id),
            TradesResponse::Error(e) => Err(e),
            _ => Err(TradesError::InvalidResponse(
                "Unexpected response type for create_intent".to_string(),
            )),
        })
    }

    /// Create a trade intent with custom parameters.
    ///
    /// Like `create_intent` but allows passing additional parameters.
    ///
    /// # Arguments
    /// * `wallet_address` - Wallet address for the trade
    /// * `chain` - Blockchain chain type
    /// * `action` - Trade action
    /// * `params` - Additional parameters as JSON
    ///
    /// # Returns
    /// The new intent ID on success
    pub async fn create_intent_with_params(
        &self,
        wallet_address: String,
        chain: ChainType,
        action: TradeAction,
        params: serde_json::Value,
    ) -> TradesResult<u64> {
        self.send_request(TradesMessage::CreateIntent {
            wallet_address,
            chain,
            action,
            params,
        })
        .await
        .and_then(|response| match response {
            TradesResponse::IntentCreated { intent_id, .. } => Ok(intent_id),
            TradesResponse::Error(e) => Err(e),
            _ => Err(TradesError::InvalidResponse("Unexpected response type".to_string())),
        })
    }

    /// Confirm a trade intent.
    ///
    /// This is called when the user approves the intent. It:
    /// 1. Requests the enclave to sign the intent
    /// 2. Moves the intent to active_trades with Signed status
    /// 3. Submits the signed trade to Iris API
    /// 4. Emits TradeIntentConfirmed and TradeSubmitted events
    ///
    /// # Arguments
    /// * `intent_id` - The intent ID to confirm
    ///
    /// # Returns
    /// The transaction hash (if available) on success
    ///
    /// # Errors
    /// Returns TradesError if:
    /// - Intent not found
    /// - Intent has expired
    /// - Signing fails
    /// - Submission fails
    pub async fn confirm_intent(&self, intent_id: u64) -> TradesResult<Option<String>> {
        self.send_request(TradesMessage::ConfirmIntent { intent_id })
            .await
            .and_then(|response| match response {
                TradesResponse::IntentConfirmed { tx_hash, .. } => Ok(tx_hash),
                TradesResponse::Error(e) => Err(e),
                _ => Err(TradesError::InvalidResponse(
                    "Unexpected response type for confirm_intent".to_string(),
                )),
            })
    }

    /// Cancel a trade intent.
    ///
    /// Removes the intent from pending and adds it to history as expired.
    /// Can only cancel intents that haven't been confirmed yet.
    ///
    /// # Arguments
    /// * `intent_id` - The intent ID to cancel
    ///
    /// # Errors
    /// Returns TradesError::IntentNotFound if the intent doesn't exist
    pub async fn cancel_intent(&self, intent_id: u64) -> TradesResult<()> {
        self.send_request(TradesMessage::CancelIntent { intent_id })
            .await
            .and_then(|response| match response {
                TradesResponse::IntentCancelled { .. } => Ok(()),
                TradesResponse::Error(e) => Err(e),
                _ => Err(TradesError::InvalidResponse(
                    "Unexpected response type for cancel_intent".to_string(),
                )),
            })
    }

    /// Get trade status.
    ///
    /// Returns the current status from pending, active, or history.
    ///
    /// # Arguments
    /// * `intent_id` - The intent ID to check
    ///
    /// # Returns
    /// The current TradeStatus
    ///
    /// # Errors
    /// Returns TradesError::IntentNotFound if the intent doesn't exist
    pub async fn get_status(&self, intent_id: u64) -> TradesResult<TradeStatus> {
        self.send_request(TradesMessage::GetStatus { intent_id })
            .await
            .and_then(|response| match response {
                TradesResponse::TradeStatus { status, .. } => Ok(status),
                TradesResponse::Error(e) => Err(e),
                _ => Err(TradesError::InvalidResponse(
                    "Unexpected response type for get_status".to_string(),
                )),
            })
    }

    /// Get detailed trade status.
    ///
    /// Like `get_status` but also returns TradeInfo with additional details.
    ///
    /// # Arguments
    /// * `intent_id` - The intent ID to check
    ///
    /// # Returns
    /// Tuple of (TradeStatus, Option<TradeInfo>)
    pub async fn get_status_with_info(&self, intent_id: u64) -> TradesResult<(TradeStatus, Option<TradeInfo>)> {
        self.send_request(TradesMessage::GetStatus { intent_id })
            .await
            .and_then(|response| match response {
                TradesResponse::TradeStatus { status, info, .. } => Ok((status, info)),
                TradesResponse::Error(e) => Err(e),
                _ => Err(TradesError::InvalidResponse(
                    "Unexpected response type for get_status".to_string(),
                )),
            })
    }

    /// Submit a signed trade to Iris API.
    ///
    /// This is typically called automatically after confirm_intent,
    /// but can be called separately if submission failed and needs retry.
    ///
    /// # Arguments
    /// * `intent_id` - The intent ID to submit
    ///
    /// # Returns
    /// The transaction hash (if available) on success
    pub async fn submit_to_iris(&self, intent_id: u64) -> TradesResult<Option<String>> {
        self.send_request(TradesMessage::SubmitToIris { intent_id })
            .await
            .and_then(|response| match response {
                TradesResponse::TradeSubmitted { tx_hash, .. } => Ok(tx_hash),
                TradesResponse::Error(e) => Err(e),
                _ => Err(TradesError::InvalidResponse(
                    "Unexpected response type for submit_to_iris".to_string(),
                )),
            })
    }

    /// List all trades.
    ///
    /// Returns pending intents, active trades, and recent history.
    ///
    /// # Arguments
    /// * `history_limit` - Maximum number of history records to include (default: 10)
    ///
    /// # Returns
    /// Tuple of (pending, active, history)
    pub async fn list_trades(
        &self,
        history_limit: usize,
    ) -> TradesResult<(Vec<TradeInfo>, Vec<TradeInfo>, Vec<TradeRecord>)> {
        self.send_request(TradesMessage::ListTrades { history_limit })
            .await
            .and_then(|response| match response {
                TradesResponse::TradeList {
                    pending,
                    active,
                    history,
                } => Ok((pending, active, history)),
                TradesResponse::Error(e) => Err(e),
                _ => Err(TradesError::InvalidResponse(
                    "Unexpected response type for list_trades".to_string(),
                )),
            })
    }

    /// List pending intents.
    ///
    /// Convenience method that returns only pending intents.
    pub async fn list_pending(&self) -> TradesResult<Vec<TradeInfo>> {
        let (pending, _, _) = self.list_trades(0).await?;
        Ok(pending)
    }

    /// List active trades.
    ///
    /// Convenience method that returns only active trades.
    pub async fn list_active(&self) -> TradesResult<Vec<TradeInfo>> {
        let (_, active, _) = self.list_trades(0).await?;
        Ok(active)
    }

    /// Get trade history.
    ///
    /// Convenience method that returns only history records.
    ///
    /// # Arguments
    /// * `limit` - Maximum number of records to return
    pub async fn get_history(&self, limit: usize) -> TradesResult<Vec<TradeRecord>> {
        let (_, _, history) = self.list_trades(limit).await?;
        Ok(history)
    }

    /// Expire stale intents.
    ///
    /// Checks all pending intents and moves expired ones to history.
    /// This is typically called periodically by a background task.
    ///
    /// # Returns
    /// The number of intents that were expired
    pub async fn expire_stale(&self) -> TradesResult<usize> {
        self.send_request(TradesMessage::ExpireStale)
            .await
            .and_then(|response| match response {
                TradesResponse::StaleExpired { count, .. } => Ok(count),
                TradesResponse::Error(e) => Err(e),
                _ => Err(TradesError::InvalidResponse(
                    "Unexpected response type for expire_stale".to_string(),
                )),
            })
    }

    /// Get the sender channel for direct message sending.
    ///
    /// Used by IPC domain for direct routing to trades domain.
    pub fn sender(&self) -> &mpsc::Sender<TradesRequest> {
        &self.sender
    }

    /// Shutdown the trades domain.
    ///
    /// Sends shutdown message to gracefully terminate the trades actor.
    pub async fn shutdown(&self) -> TradesResult<()> {
        self.send_request(TradesMessage::Shutdown).await.map(|_| ())
    }

    /// Place a spot order.
    ///
    /// Places a spot order with the given parameters through the Iris API.
    ///
    /// # Arguments
    /// * `side` - Order side (buy or sell)
    /// * `value` - Order size (amount)
    /// * `chain` - Blockchain chain identifier
    /// * `token_contract_address` - Token contract address (optional)
    /// * `pair_contract_address` - Pair contract address (optional)
    /// * `wallet_address` - Wallet address for the order
    /// * `session` - Session for authentication
    /// * `client` - Iris client for API communication
    /// * `tx_preset_key` - Transaction preset key (optional, defaults to "a")
    /// * `tx_preset_method` - Transaction preset method (optional, defaults to "normal")
    /// * `tx_preset_bribe` - Transaction bribe (optional, defaults to "0")
    /// * `tx_preset_max_base_gas` - Transaction max base gas (optional, defaults to "0")
    /// * `tx_preset_priority_gas` - Transaction priority gas (optional, defaults to "0")
    /// * `tx_preset_slippage` - Transaction slippage (optional, defaults to "0")
    ///
    /// # Returns
    /// Ok(()) on success, or TradesError on failure
    #[allow(clippy::too_many_arguments)]
    pub async fn place_spot(
        &self,
        side: String,
        value: u128,
        chain: String,
        token_contract_address: Option<String>,
        pair_contract_address: Option<String>,
        wallet_address: String,
        session: Session,
        client: IrisClient,
        tx_preset_key: Option<String>,
        tx_preset_method: Option<String>,
        tx_preset_bribe: Option<String>,
        tx_preset_max_base_gas: Option<String>,
        tx_preset_priority_gas: Option<String>,
        tx_preset_slippage: Option<String>,
    ) -> TradesResult<()> {
        self.send_request(TradesMessage::PlaceSpot {
            side,
            value,
            chain,
            token_contract_address,
            pair_contract_address,
            wallet_address,
            session,
            client,
            tx_preset_key,
            tx_preset_method,
            tx_preset_bribe,
            tx_preset_max_base_gas,
            tx_preset_priority_gas,
            tx_preset_slippage,
        })
        .await
        .and_then(|response| match response {
            TradesResponse::SpotOrderPlaced { result } => result,
            TradesResponse::Error(e) => Err(e),
            _ => Err(TradesError::InvalidResponse(
                "Unexpected response type for place_spot".to_string(),
            )),
        })
    }
}

impl Default for TradesHandle {
    fn default() -> Self {
        // Create a dummy sender for testing
        let (sender, _rx) = mpsc::channel(1);
        Self::from_sender(sender)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trades_handle_new() {
        // Dummy test since new() now requires async and receiver
        let (sender, _rx) = mpsc::channel(1);
        let _handle = TradesHandle::from_sender(sender);
        // Just verify it creates without error
    }

    #[test]
    fn test_trades_handle_default() {
        let _handle: TradesHandle = Default::default();
        // Just verify Default impl works
    }
}
