use crate::domains::client::RouteExecutor;
use crate::domains::client::generated::routes::requests::orders_place_spot_order::{
    self, PlaceSpotOrderRequest, PlaceSpotOrderRequestOrder, PlaceSpotOrderRequestOrderAmount,
    PlaceSpotOrderRequestOrderSide, PlaceSpotOrderRequestOrderTxPreset, PlaceSpotOrderRequestOrderTxPresetMethod,
};
use crate::domains::trades::errors::{TradesError, TradesResult};
use crate::domains::trades::messages::{TradesEvent, TradesMessage, TradesRequest, TradesResponse};
use crate::event_bus::EventBus;
use crate::messages;
use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use erato::models::ChainId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;
use std::time::{Duration, Instant};
use tokio::sync::mpsc;
use tyche_enclave::envelopes::transport::{ExecutionPayload, SealedIntent, TransportEnvelope, TransportEnvelopeKey};
use uuid::Uuid;

/// The trades actor that owns trades state.
pub struct TradesActor {
    /// Current trades state (intents, active trades, history)
    state: TradesState,
    /// EventBus for publishing state events
    event_bus: EventBus,
    /// Handle to enclave domain for signing
    enclave: Option<crate::domains::enclave::EnclaveHandle>,

    /// Default intent expiration duration
    intent_expiration: Duration,
}

impl TradesActor {
    /// Create a new trades actor.
    ///
    /// # Arguments
    /// * `event_bus` - EventBus for publishing state events
    pub fn new(event_bus: EventBus) -> Self {
        Self {
            state: TradesState::new(),
            event_bus,
            enclave: None,
            intent_expiration: Duration::from_secs(300), // 5 minutes default
        }
    }

    /// Emit a TradesEvent to the EventBus.
    fn emit_event(&self, event: TradesEvent) {
        let state_event = event.to_state_event();
        if let Err(_e) = self.event_bus.publish(state_event) {
            // EventBus publish error is non-critical
        }
    }

    /// Run the actor message loop.
    ///
    /// Processes incoming TradesRequest messages and sends responses
    /// via the oneshot reply channel.
    ///
    /// # Arguments
    /// * `receiver` - Channel receiver for TradesRequest messages
    pub async fn run(mut self, mut receiver: mpsc::Receiver<TradesRequest>) {
        while let Some(req) = receiver.recv().await {
            let reply = self.handle_message(req.payload).await;
            let _ = req.reply_to.send(reply);
        }
    }

    /// Handle incoming messages.
    async fn handle_message(&mut self, payload: TradesMessage) -> Result<TradesResponse, TradesError> {
        match payload {
            TradesMessage::CreateIntent {
                wallet_address,
                chain,
                action,
                params,
            } => {
                self.create_intent(wallet_address, chain, action, params)
                    .await
            }
            TradesMessage::ConfirmIntent { intent_id } => self.confirm_intent(intent_id).await,
            TradesMessage::CancelIntent { intent_id } => self.cancel_intent(intent_id).await,
            TradesMessage::GetStatus { intent_id } => self.get_status(intent_id).await,
            TradesMessage::SubmitToIris { intent_id } => self.submit_to_iris(intent_id).await,
            TradesMessage::ExternalConfirmation { intent_id, tx_hash } => {
                self.handle_external_confirmation(intent_id, tx_hash).await
            }
            TradesMessage::ExternalFailure { intent_id, error } => self.handle_external_failure(intent_id, error).await,
            TradesMessage::ListTrades { history_limit } => self.list_trades(history_limit).await,
            TradesMessage::ExpireStale => self.expire_stale().await,
            TradesMessage::Shutdown => Ok(TradesResponse::Success),
            TradesMessage::PlaceSpot {
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
            } => {
                self.place_spot(
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
                )
                .await
            }
        }
    }

    /// Create a trade intent.
    ///
    /// # Arguments
    /// * `wallet_address` - Wallet address for the trade
    /// * `chain` - Blockchain chain type
    /// * `action` - Trade action (Swap, Transfer, etc.)
    /// * `params` - Additional parameters
    ///
    /// # Emits
    /// * `StateEvent::TradeIntentCreated` - On successful creation
    async fn create_intent(
        &mut self,
        wallet_address: String,
        chain: ChainType,
        action: TradeAction,
        params: serde_json::Value,
    ) -> TradesResult<TradesResponse> {
        let id = self.state.next_id();
        let now = Instant::now();
        let expires_at = now + self.intent_expiration;

        let intent = TradeIntent {
            id,
            wallet_address: wallet_address.clone(),
            chain,
            action,
            params,
            created_at: now,
            expires_at,
        };

        self.state.insert_pending(intent);

        // Emit TradeIntentCreated event
        self.emit_event(TradesEvent::IntentCreated {
            id,
            wallet: wallet_address.clone(),
            chain: chain.to_string(),
        });

        Ok(TradesResponse::IntentCreated {
            intent_id: id,
            wallet_address,
            expires_in_secs: self.intent_expiration.as_secs(),
        })
    }

    /// Confirm a trade intent.
    ///
    /// This is called when the user approves the intent. It:
    /// 1. Gets the intent from pending
    /// 2. Requests enclave to sign (returns encrypted payload)
    /// 3. Moves to active_trades with Signed status
    /// 4. Emits TradeIntentConfirmed event
    /// 5. Submits to Iris API
    /// 6. Emits TradeSubmitted event
    ///
    /// # Arguments
    /// * `intent_id` - The intent ID to confirm
    ///
    /// # Emits
    /// * `StateEvent::TradeIntentConfirmed` - On successful confirmation/signing
    /// * `StateEvent::TradeSubmitted` - After Iris API submission
    async fn confirm_intent(&mut self, intent_id: u64) -> TradesResult<TradesResponse> {
        // Get the pending intent
        let intent = self
            .state
            .remove_pending(intent_id)
            .ok_or(TradesError::IntentNotFound(intent_id))?;

        // Check if expired
        if intent.is_expired(Instant::now()) {
            // Add to history as expired
            let record = TradeRecord {
                id: intent.id,
                wallet_address: intent.wallet_address.clone(),
                action: intent.action.clone(),
                status: TradeStatus::Expired,
                created_at: intent.created_at,
                completed_at: Some(Instant::now()),
            };
            self.state.add_to_history(record);

            self.emit_event(TradesEvent::Expired { id: intent_id });
            return Err(TradesError::IntentExpired(intent_id));
        }

        // Request enclave to sign the intent
        // For now, simulate signing with an empty encrypted payload
        // In production, this would call the enclave domain
        let signed_payload = match &self.enclave {
            Some(_enclave) => {
                // TODO: Call enclave.sign_trade_intent() when available
                // For now, simulate successful signing
                vec![1u8; 64] // Dummy encrypted payload
            }
            None => {
                // No enclave available, simulate signing
                vec![1u8; 64]
            }
        };

        // Create active trade with signed payload
        let active_trade = ActiveTrade {
            intent: intent.clone(),
            signed_payload,
            status: TradeStatus::Signed,
            tx_hash: None,
        };

        self.state.insert_active(active_trade);

        // Emit TradeIntentConfirmed event
        self.emit_event(TradesEvent::IntentConfirmed { id: intent_id });

        // Automatically submit to Iris API
        match self.submit_to_iris(intent_id).await {
            Ok(submit_response) => {
                // Extract tx_hash from submission response
                if let TradesResponse::TradeSubmitted { tx_hash, .. } = &submit_response {
                    return Ok(TradesResponse::IntentConfirmed {
                        intent_id,
                        tx_hash: tx_hash.clone(),
                    });
                }
                Ok(TradesResponse::IntentConfirmed {
                    intent_id,
                    tx_hash: None,
                })
            }
            Err(_e) => {
                // Submission failed, but intent was confirmed and signed
                // Return success for confirmation, caller can check status separately
                Ok(TradesResponse::IntentConfirmed {
                    intent_id,
                    tx_hash: None,
                })
            }
        }
    }

    /// Cancel a trade intent.
    ///
    /// Removes from pending and adds to history as expired.
    ///
    /// # Arguments
    /// * `intent_id` - The intent ID to cancel
    ///
    /// # Emits
    /// * `StateEvent::TradeExpired` - On successful cancellation
    async fn cancel_intent(&mut self, intent_id: u64) -> TradesResult<TradesResponse> {
        if let Some(intent) = self.state.remove_pending(intent_id) {
            let record = TradeRecord {
                id: intent.id,
                wallet_address: intent.wallet_address,
                action: intent.action,
                status: TradeStatus::Expired,
                created_at: intent.created_at,
                completed_at: Some(Instant::now()),
            };
            self.state.add_to_history(record);

            self.emit_event(TradesEvent::Cancelled { id: intent_id });
            Ok(TradesResponse::IntentCancelled { intent_id })
        } else {
            Err(TradesError::IntentNotFound(intent_id))
        }
    }

    /// Get trade status.
    ///
    /// Returns status from pending, active, or history.
    ///
    /// # Arguments
    /// * `intent_id` - The intent ID to check
    async fn get_status(&self, intent_id: u64) -> TradesResult<TradesResponse> {
        // Check pending
        if let Some(intent) = self.state.get_pending(intent_id) {
            let info = TradeInfo {
                id: intent.id,
                wallet: intent.wallet_address.clone(),
                chain: intent.chain.to_string(),
                action: intent.action.clone(),
                status: TradeStatus::Pending,
            };
            return Ok(TradesResponse::TradeStatus {
                intent_id,
                status: TradeStatus::Pending,
                info: Some(info),
            });
        }

        // Check active
        if let Some(trade) = self.state.get_active(intent_id) {
            let info = TradeInfo {
                id: trade.intent.id,
                wallet: trade.intent.wallet_address.clone(),
                chain: trade.intent.chain.to_string(),
                action: trade.intent.action.clone(),
                status: trade.status.clone(),
            };
            return Ok(TradesResponse::TradeStatus {
                intent_id,
                status: trade.status.clone(),
                info: Some(info),
            });
        }

        // Check history
        if let Some(record) = self.state.find_in_history(intent_id) {
            let info = TradeInfo {
                id: record.id,
                wallet: record.wallet_address.clone(),
                chain: record.action.to_string(), // Simplified
                action: record.action.clone(),
                status: record.status.clone(),
            };
            return Ok(TradesResponse::TradeStatus {
                intent_id,
                status: record.status.clone(),
                info: Some(info),
            });
        }

        Err(TradesError::IntentNotFound(intent_id))
    }

    /// Submit signed trade to Iris API.
    ///
    /// # Arguments
    /// * `intent_id` - The intent ID to submit
    ///
    /// # Emits
    /// * `StateEvent::TradeSubmitted` - On successful submission
    async fn submit_to_iris(&mut self, intent_id: u64) -> TradesResult<TradesResponse> {
        let _trade = self
            .state
            .get_active(intent_id)
            .ok_or(TradesError::IntentNotFound(intent_id))?;

        // TODO: Actual Iris API submission logic
        // For now, simulate a successful submission
        let tx_hash = format!("0x{}", hex::encode(intent_id.to_le_bytes()));

        // Update trade status
        // We need to remove and re-insert because get_active returns immutable reference
        if let Some(mut _trade) = self.state.remove_active(intent_id) {
            _trade.status = TradeStatus::Submitted {
                tx_hash: Some(tx_hash.clone()),
            };
            _trade.tx_hash = Some(tx_hash.clone());
            self.state.insert_active(_trade);
        }

        // Emit TradeSubmitted event
        self.emit_event(TradesEvent::Submitted {
            id: intent_id,
            tx_hash: Some(tx_hash.clone()),
        });

        Ok(TradesResponse::TradeSubmitted {
            intent_id,
            tx_hash: Some(tx_hash),
        })
    }

    /// Handle external confirmation from Iris API.
    ///
    /// Called when Iris API reports trade confirmation on-chain.
    ///
    /// # Arguments
    /// * `intent_id` - The intent ID
    /// * `tx_hash` - Optional transaction hash
    ///
    /// # Emits
    /// * `StateEvent::TradeConfirmed` - On confirmation
    async fn handle_external_confirmation(
        &mut self,
        intent_id: u64,
        tx_hash: Option<String>,
    ) -> TradesResult<TradesResponse> {
        if let Some(trade) = self.state.remove_active(intent_id) {
            let record = TradeRecord {
                id: trade.intent.id,
                wallet_address: trade.intent.wallet_address,
                action: trade.intent.action,
                status: TradeStatus::Completed,
                created_at: trade.intent.created_at,
                completed_at: Some(Instant::now()),
            };
            self.state.add_to_history(record);

            self.emit_event(TradesEvent::Confirmed { id: intent_id, tx_hash });
            Ok(TradesResponse::TradeConfirmed { intent_id })
        } else {
            Err(TradesError::IntentNotFound(intent_id))
        }
    }

    /// Handle external failure from Iris API.
    ///
    /// Called when Iris API reports trade failure.
    ///
    /// # Arguments
    /// * `intent_id` - The intent ID
    /// * `error` - Error message
    ///
    /// # Emits
    /// * `StateEvent::TradeFailed` - On failure
    async fn handle_external_failure(&mut self, intent_id: u64, error: String) -> TradesResult<TradesResponse> {
        if let Some(trade) = self.state.remove_active(intent_id) {
            let record = TradeRecord {
                id: trade.intent.id,
                wallet_address: trade.intent.wallet_address,
                action: trade.intent.action,
                status: TradeStatus::Failed { error: error.clone() },
                created_at: trade.intent.created_at,
                completed_at: Some(Instant::now()),
            };
            self.state.add_to_history(record);

            self.emit_event(TradesEvent::Failed {
                id: intent_id,
                error: error.clone(),
            });
            Ok(TradesResponse::TradeFailed { intent_id, error })
        } else {
            Err(TradesError::IntentNotFound(intent_id))
        }
    }

    /// List all trades.
    ///
    /// Returns pending intents, active trades, and recent history.
    ///
    /// # Arguments
    /// * `history_limit` - Maximum number of history records to include
    async fn list_trades(&self, history_limit: usize) -> TradesResult<TradesResponse> {
        let pending: Vec<TradeInfo> = self
            .state
            .pending_intents
            .values()
            .map(|intent| TradeInfo {
                id: intent.id,
                wallet: intent.wallet_address.clone(),
                chain: intent.chain.to_string(),
                action: intent.action.clone(),
                status: TradeStatus::Pending,
            })
            .collect();

        let active: Vec<TradeInfo> = self
            .state
            .active_trades
            .values()
            .map(|trade| TradeInfo {
                id: trade.intent.id,
                wallet: trade.intent.wallet_address.clone(),
                chain: trade.intent.chain.to_string(),
                action: trade.intent.action.clone(),
                status: trade.status.clone(),
            })
            .collect();

        let history: Vec<TradeRecord> = self
            .state
            .history
            .iter()
            .rev() // Most recent first
            .take(history_limit)
            .cloned()
            .collect();

        Ok(TradesResponse::TradeList {
            pending,
            active,
            history,
        })
    }

    /// Expire stale intents.
    ///
    /// Checks pending intents and moves expired ones to history.
    ///
    /// # Emits
    /// * `StateEvent::TradeExpired` - For each expired intent
    async fn expire_stale(&mut self) -> TradesResult<TradesResponse> {
        let now = Instant::now();
        let expired_ids = self.state.expire_intents(now);

        for id in &expired_ids {
            self.emit_event(TradesEvent::Expired { id: *id });
        }

        Ok(TradesResponse::StaleExpired {
            count: expired_ids.len(),
            ids: expired_ids,
        })
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
    #[allow(clippy::too_many_arguments)]
    async fn place_spot(
        &mut self,
        side: String,
        value: u128,
        chain: String,
        token_contract_address: Option<String>,
        pair_contract_address: Option<String>,
        wallet_address: String,
        session: crate::domains::keystore::Session,
        client: crate::domains::client::IrisClient,
        tx_preset_key: Option<String>,
        tx_preset_method: Option<String>,
        tx_preset_bribe: Option<String>,
        tx_preset_max_base_gas: Option<String>,
        tx_preset_priority_gas: Option<String>,
        tx_preset_slippage: Option<String>,
    ) -> TradesResult<TradesResponse> {
        // Get transport key for envelope creation
        let transport_key = Self::get_transport_key(&client).await?;

        // Get user encryption key from session
        let user_key = session
            .get_user_encryption_key()
            .map_err(|e| TradesError::Client(format!("Failed to get user encryption key: {}", e)))?
            .ok_or_else(|| TradesError::Client("User encryption key not found".to_string()))?;

        // Get agent ID from session or config
        let agent_id = Self::get_agent_id(&session)?;

        // Parse chain ID
        let chain_id = ChainId::from_str(&chain).map_err(|_| TradesError::InvalidChain(chain.clone()))?;

        // Create the sealed intent
        let sealed_intent = SealedIntent {
            user_id: None,
            agent_id: Some(agent_id.to_string()),
            chain_id: chain_id.to_string(),
            wallet_address: wallet_address.clone(),
            value: value.to_string(),
        };

        // Create and seal the execution payload
        let envelope = ExecutionPayload::new(user_key.storage, sealed_intent)
            .seal(&transport_key)
            .map_err(|e| TradesError::Serialization(e.to_string()))?;

        // Parse order side
        let order_side = match side.as_str() {
            "buy" => PlaceSpotOrderRequestOrderSide::Buy,
            "sell" => PlaceSpotOrderRequestOrderSide::Sell,
            _ => {
                return Err(TradesError::InvalidInput(format!(
                    "Invalid order side: {}; must be 'buy' or 'sell'",
                    side
                )));
            }
        };

        // Parse tx_preset method
        let tx_preset_method_enum = match tx_preset_method.as_deref().unwrap_or("normal") {
            "flashbot" => PlaceSpotOrderRequestOrderTxPresetMethod::Flashbot,
            "normal" => PlaceSpotOrderRequestOrderTxPresetMethod::Normal,
            _ => PlaceSpotOrderRequestOrderTxPresetMethod::Normal,
        };

        // Build the request
        let request = PlaceSpotOrderRequest {
            envelope: STANDARD.encode(&envelope),
            order: PlaceSpotOrderRequestOrder {
                chain_id: chain_id.to_string(),
                pair_contract_address,
                token_contract_address,
                amount: PlaceSpotOrderRequestOrderAmount::Native(value.to_string()),
                side: order_side,
                tx_preset: PlaceSpotOrderRequestOrderTxPreset {
                    key: tx_preset_key.unwrap_or_else(|| "a".to_string()),
                    method: tx_preset_method_enum,
                    bribe: tx_preset_bribe.unwrap_or_else(|| "0".to_string()),
                    max_base_gas: tx_preset_max_base_gas.unwrap_or_else(|| "0".to_string()),
                    priority_gas: tx_preset_priority_gas.unwrap_or_else(|| "0".to_string()),
                    slippage: tx_preset_slippage.unwrap_or_else(|| "0".to_string()),
                },
                exit_strategy_id: None,
            },
        };

        // Execute the place_spot_order request
        let response = client
            .execute(&orders_place_spot_order::ROUTE, &request)
            .await
            .map_err(|e| TradesError::SubmissionFailed(e.to_string()))?;

        // Validate response - check if any transaction failed
        let has_errors = response
            .iter()
            .any(|item| item.transactions.iter().any(|tx| tx.subtype_1.is_some()));

        let has_success = response
            .iter()
            .any(|item| item.transactions.iter().any(|tx| tx.subtype_0.is_some()));

        if has_errors {
            return Err(TradesError::ExecutionFailed("Failed to place spot order".to_string()));
        }

        if !has_success {
            return Err(TradesError::ExecutionFailed(
                "No successful transactions in response".to_string(),
            ));
        }

        // Output success message
        messages::success::successful_order(response);

        Ok(TradesResponse::SpotOrderPlaced { result: Ok(()) })
    }

    /// Get transport key for envelope creation.
    async fn get_transport_key(client: &crate::domains::client::IrisClient) -> TradesResult<TransportEnvelopeKey> {
        let enclave_keys = crate::domains::client::get_transport_key(client)
            .await
            .map_err(|e| TradesError::Client(format!("Failed to get transport key: {}", e)))?;

        Ok(TransportEnvelopeKey::unsealing(enclave_keys.ephemeral))
    }

    /// Get agent ID from session or config.
    fn get_agent_id(session: &crate::domains::keystore::Session) -> TradesResult<Uuid> {
        // Try to get from session config
        let maybe_agent_id = session
            .get_config()
            .map_err(|e| TradesError::Config(format!("Failed to get config: {}", e)))?
            .agent_id;

        if let Some(agent_id) = maybe_agent_id {
            return Ok(agent_id);
        }

        // Fallback to loading from config file
        let config = crate::domains::config::Config::load(None)
            .map_err(|e| TradesError::Config(format!("Failed to load config: {}", e)))?;

        let agent_id = config
            .agent_id
            .ok_or_else(|| TradesError::Config("Agent ID not found in config".to_string()))?;

        Ok(agent_id)
    }
}
pub async fn run_trades_actor(actor: TradesActor, receiver: mpsc::Receiver<TradesRequest>) {
    actor.run(receiver).await;
}

// ============================================================================
// State Types (moved from state.rs)
// ============================================================================

/// The trades state owns all mutable state for the trades domain.
///
/// Security model:
/// - Pending intents are stored awaiting user confirmation
/// - Active trades store encrypted signed payloads (never plaintext keys)
/// - History maintains a record of completed/cancelled trades
#[derive(Debug, Clone, Default)]
pub struct TradesState {
    /// Pending trade intents awaiting user confirmation
    pub pending_intents: HashMap<u64, TradeIntent>,
    /// Active trade sessions (confirmed, awaiting settlement)
    pub active_trades: HashMap<u64, ActiveTrade>,
    /// Trade history (completed/cancelled/expired)
    pub history: Vec<TradeRecord>,
    /// Next intent ID counter
    pub next_intent_id: u64,
}

impl TradesState {
    /// Create a new empty trades state.
    pub fn new() -> Self {
        Self {
            pending_intents: HashMap::new(),
            active_trades: HashMap::new(),
            history: Vec::new(),
            next_intent_id: 1,
        }
    }

    /// Generate the next intent ID.
    pub fn next_id(&mut self) -> u64 {
        let id = self.next_intent_id;
        self.next_intent_id += 1;
        id
    }

    /// Get the number of pending intents.
    pub fn pending_count(&self) -> usize {
        self.pending_intents.len()
    }

    /// Get the number of active trades.
    pub fn active_count(&self) -> usize {
        self.active_trades.len()
    }

    /// Get the total history count.
    pub fn history_count(&self) -> usize {
        self.history.len()
    }

    /// Get a pending intent by ID.
    pub fn get_pending(&self, id: u64) -> Option<&TradeIntent> {
        self.pending_intents.get(&id)
    }

    /// Get an active trade by ID.
    pub fn get_active(&self, id: u64) -> Option<&ActiveTrade> {
        self.active_trades.get(&id)
    }

    /// Insert a pending intent.
    pub fn insert_pending(&mut self, intent: TradeIntent) {
        self.pending_intents.insert(intent.id, intent);
    }

    /// Remove a pending intent.
    pub fn remove_pending(&mut self, id: u64) -> Option<TradeIntent> {
        self.pending_intents.remove(&id)
    }

    /// Insert an active trade.
    pub fn insert_active(&mut self, trade: ActiveTrade) {
        self.active_trades.insert(trade.intent.id, trade);
    }

    /// Remove an active trade.
    pub fn remove_active(&mut self, id: u64) -> Option<ActiveTrade> {
        self.active_trades.remove(&id)
    }

    /// Add a record to history.
    pub fn add_to_history(&mut self, record: TradeRecord) {
        self.history.push(record);
    }

    /// Find a trade record in history by ID.
    pub fn find_in_history(&self, id: u64) -> Option<&TradeRecord> {
        self.history.iter().find(|r| r.id == id)
    }

    /// Check for expired intents and move them to history.
    ///
    /// Returns the IDs of intents that were expired.
    pub fn expire_intents(&mut self, now: Instant) -> Vec<u64> {
        let expired_ids: Vec<u64> = self
            .pending_intents
            .iter()
            .filter(|(_, intent)| intent.expires_at < now)
            .map(|(id, _)| *id)
            .collect();

        for id in expired_ids.clone() {
            if let Some(intent) = self.pending_intents.remove(&id) {
                let record = TradeRecord {
                    id: intent.id,
                    wallet_address: intent.wallet_address.clone(),
                    action: intent.action.clone(),
                    status: TradeStatus::Expired,
                    created_at: intent.created_at,
                    completed_at: Some(now),
                };
                self.history.push(record);
            }
        }

        expired_ids
    }
}

/// Trade intent awaiting user confirmation.
#[derive(Debug, Clone)]
pub struct TradeIntent {
    /// Intent ID
    pub id: u64,
    /// Wallet address for the trade
    pub wallet_address: String,
    /// Blockchain chain type
    pub chain: ChainType,
    /// Trade action (Swap, Transfer, etc.)
    pub action: TradeAction,
    /// Additional parameters
    pub params: serde_json::Value,
    /// Creation timestamp
    pub created_at: Instant,
    /// Expiration timestamp
    pub expires_at: Instant,
}

impl TradeIntent {
    /// Check if the intent has expired.
    pub fn is_expired(&self, now: Instant) -> bool {
        self.expires_at < now
    }
}

/// Active trade session with signed payload.
///
/// Security: Only stores encrypted signed payloads, never plaintext keys.
#[derive(Debug, Clone)]
pub struct ActiveTrade {
    /// The original trade intent
    pub intent: TradeIntent,
    /// Encrypted signed payload (from enclave)
    pub signed_payload: Vec<u8>,
    /// Current trade status
    pub status: TradeStatus,
    /// Transaction hash if submitted
    pub tx_hash: Option<String>,
}

/// Trade record for history.
#[derive(Debug, Clone)]
pub struct TradeRecord {
    /// Trade ID
    pub id: u64,
    /// Wallet address
    pub wallet_address: String,
    /// Trade action
    pub action: TradeAction,
    /// Final status
    pub status: TradeStatus,
    /// Creation timestamp
    pub created_at: Instant,
    /// Completion timestamp (if completed)
    pub completed_at: Option<Instant>,
}

/// Blockchain chain type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ChainType {
    /// Ethereum Virtual Machine (EVM) chains
    Ethereum,
    /// Solana Virtual Machine (SVM) chains
    Solana,
}

impl fmt::Display for ChainType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChainType::Ethereum => write!(f, "ethereum"),
            ChainType::Solana => write!(f, "solana"),
        }
    }
}

impl std::str::FromStr for ChainType {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "ethereum" | "eth" | "evm" => Ok(ChainType::Ethereum),
            "solana" | "sol" | "svm" => Ok(ChainType::Solana),
            _ => Err(format!("Unknown chain type: {}", s)),
        }
    }
}

/// Trade action types.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TradeAction {
    /// Swap tokens
    Swap {
        /// Source token
        from_token: String,
        /// Destination token
        to_token: String,
        /// Amount to swap
        amount: String,
    },
    /// Transfer tokens
    Transfer {
        /// Destination address
        to: String,
        /// Token to transfer
        token: String,
        /// Amount to transfer
        amount: String,
    },
}

impl fmt::Display for TradeAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TradeAction::Swap {
                from_token,
                to_token,
                amount,
            } => {
                write!(f, "Swap {} {} -> {}", amount, from_token, to_token)
            }
            TradeAction::Transfer { to, token, amount } => {
                write!(f, "Transfer {} {} to {}", amount, token, to)
            }
        }
    }
}

/// Trade status lifecycle.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TradeStatus {
    /// Intent created, awaiting user confirmation
    Pending,
    /// Intent confirmed, awaiting signature from enclave
    Confirmed,
    /// Trade signed by enclave (encrypted payload ready)
    Signed,
    /// Trade submitted to Iris API
    Submitted {
        /// Transaction hash if available
        tx_hash: Option<String>,
    },
    /// Trade confirmed on-chain
    Completed,
    /// Trade failed
    Failed {
        /// Error message
        error: String,
    },
    /// Intent expired without confirmation
    Expired,
}

impl TradeStatus {
    /// Check if the trade is in a terminal state.
    pub fn is_terminal(&self) -> bool {
        matches!(
            self,
            TradeStatus::Completed | TradeStatus::Failed { .. } | TradeStatus::Expired
        )
    }

    /// Check if the trade is awaiting user action.
    pub fn is_pending_user(&self) -> bool {
        matches!(self, TradeStatus::Pending)
    }

    /// Check if the trade is in progress.
    pub fn is_in_progress(&self) -> bool {
        matches!(
            self,
            TradeStatus::Confirmed | TradeStatus::Signed | TradeStatus::Submitted { .. }
        )
    }
}

impl fmt::Display for TradeStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TradeStatus::Pending => write!(f, "pending"),
            TradeStatus::Confirmed => write!(f, "confirmed"),
            TradeStatus::Signed => write!(f, "signed"),
            TradeStatus::Submitted { tx_hash } => {
                if let Some(hash) = tx_hash {
                    write!(f, "submitted (tx: {})", hash)
                } else {
                    write!(f, "submitted")
                }
            }
            TradeStatus::Completed => write!(f, "completed"),
            TradeStatus::Failed { error } => write!(f, "failed: {}", error),
            TradeStatus::Expired => write!(f, "expired"),
        }
    }
}

/// Trade information for list responses.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeInfo {
    /// Trade intent ID
    pub id: u64,
    /// Wallet address
    pub wallet: String,
    /// Chain type
    pub chain: String,
    /// Trade action
    pub action: TradeAction,
    /// Current status
    pub status: TradeStatus,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_trades_state_new() {
        let state = TradesState::new();
        assert_eq!(state.pending_count(), 0);
        assert_eq!(state.active_count(), 0);
        assert_eq!(state.history_count(), 0);
        assert_eq!(state.next_intent_id, 1);
    }

    #[test]
    fn test_next_id() {
        let mut state = TradesState::new();
        assert_eq!(state.next_id(), 1);
        assert_eq!(state.next_id(), 2);
        assert_eq!(state.next_id(), 3);
        assert_eq!(state.next_intent_id, 4);
    }

    #[test]
    fn test_trade_intent_expiration() {
        let now = Instant::now();
        let intent = TradeIntent {
            id: 1,
            wallet_address: "0x1234".to_string(),
            chain: ChainType::Ethereum,
            action: TradeAction::Swap {
                from_token: "ETH".to_string(),
                to_token: "USDC".to_string(),
                amount: "1.0".to_string(),
            },
            params: serde_json::json!({}),
            created_at: now,
            expires_at: now + Duration::from_secs(300),
        };

        assert!(!intent.is_expired(now));
        assert!(intent.is_expired(now + Duration::from_secs(301)));
    }

    #[test]
    fn test_chain_type_display() {
        assert_eq!(ChainType::Ethereum.to_string(), "ethereum");
        assert_eq!(ChainType::Solana.to_string(), "solana");
    }

    #[test]
    fn test_chain_type_from_str() {
        assert_eq!(ChainType::from_str("ethereum").unwrap(), ChainType::Ethereum);
        assert_eq!(ChainType::from_str("ETH").unwrap(), ChainType::Ethereum);
        assert_eq!(ChainType::from_str("evm").unwrap(), ChainType::Ethereum);
        assert_eq!(ChainType::from_str("solana").unwrap(), ChainType::Solana);
        assert_eq!(ChainType::from_str("SOL").unwrap(), ChainType::Solana);
        assert!(ChainType::from_str("unknown").is_err());
    }

    #[test]
    fn test_trade_action_display() {
        let swap = TradeAction::Swap {
            from_token: "ETH".to_string(),
            to_token: "USDC".to_string(),
            amount: "1.0".to_string(),
        };
        assert!(swap.to_string().contains("Swap"));

        let transfer = TradeAction::Transfer {
            to: "0x5678".to_string(),
            token: "USDC".to_string(),
            amount: "100".to_string(),
        };
        assert!(transfer.to_string().contains("Transfer"));
    }

    #[test]
    fn test_trade_status_variants() {
        assert!(TradeStatus::Pending.is_pending_user());
        assert!(!TradeStatus::Completed.is_pending_user());

        assert!(TradeStatus::Completed.is_terminal());
        assert!(
            TradeStatus::Failed {
                error: "test".to_string()
            }
            .is_terminal()
        );
        assert!(TradeStatus::Expired.is_terminal());
        assert!(!TradeStatus::Pending.is_terminal());
    }

    #[test]
    fn test_state_expire_intents() {
        let mut state = TradesState::new();
        let now = Instant::now();

        // Add a pending intent that expires in the future
        let intent = TradeIntent {
            id: 1,
            wallet_address: "0x1234".to_string(),
            chain: ChainType::Ethereum,
            action: TradeAction::Swap {
                from_token: "ETH".to_string(),
                to_token: "USDC".to_string(),
                amount: "1.0".to_string(),
            },
            params: serde_json::json!({}),
            created_at: now,
            expires_at: now + Duration::from_secs(60),
        };
        state.insert_pending(intent);

        // Add another intent that has already expired
        let expired_intent = TradeIntent {
            id: 2,
            wallet_address: "0x5678".to_string(),
            chain: ChainType::Solana,
            action: TradeAction::Transfer {
                to: "0x9999".to_string(),
                token: "SOL".to_string(),
                amount: "5.0".to_string(),
            },
            params: serde_json::json!({}),
            created_at: now - Duration::from_secs(120),
            expires_at: now - Duration::from_secs(60),
        };
        state.insert_pending(expired_intent);

        assert_eq!(state.pending_count(), 2);

        // Check for expired intents
        let expired = state.expire_intents(now);
        assert_eq!(expired.len(), 1);
        assert_eq!(expired[0], 2);
        assert_eq!(state.pending_count(), 1);
        assert_eq!(state.history_count(), 1);
    }
}
