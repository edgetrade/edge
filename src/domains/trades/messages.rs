//! Trades domain messages
//!
//! Defines all command and query messages for the trades domain,
//! used for communication between handle and actor via PoseidonRequest pattern.

use serde::{Deserialize, Serialize};

use crate::domains::client::IrisClient;
use crate::domains::keystore::Session;
use crate::domains::trades::actor::{ChainType, TradeAction, TradeInfo, TradeRecord, TradeStatus};
use crate::domains::trades::errors::TradesError;
use crate::event_bus::PoseidonRequest;

/// Messages sent to the TradesActor.
#[derive(Debug, Clone)]
pub enum TradesMessage {
    /// Create a trade intent
    CreateIntent {
        /// Wallet address for the trade
        wallet_address: String,
        /// Blockchain chain type
        chain: ChainType,
        /// Trade action
        action: TradeAction,
        /// Additional parameters
        params: serde_json::Value,
    },

    /// Confirm a trade intent (user approval + sign)
    ConfirmIntent {
        /// Intent ID to confirm
        intent_id: u64,
    },

    /// Cancel a trade intent
    CancelIntent {
        /// Intent ID to cancel
        intent_id: u64,
    },

    /// Get trade status
    GetStatus {
        /// Intent ID to check
        intent_id: u64,
    },

    /// Submit signed trade to Iris API
    SubmitToIris {
        /// Intent ID to submit
        intent_id: u64,
    },

    /// Handle external confirmation (from Iris API)
    ExternalConfirmation {
        /// Intent ID
        intent_id: u64,
        /// Transaction hash
        tx_hash: Option<String>,
    },

    /// Handle external failure (from Iris API)
    ExternalFailure {
        /// Intent ID
        intent_id: u64,
        /// Error message
        error: String,
    },

    /// List all trades (pending, active, recent history)
    ListTrades {
        /// Maximum number of history records to include
        history_limit: usize,
    },

    /// Expire stale intents
    ExpireStale,

    /// Shutdown the trades domain
    Shutdown,

    /// Place a spot order
    PlaceSpot {
        /// Order side (buy or sell)
        side: String,
        /// Order value (amount)
        value: u128,
        /// Blockchain chain identifier
        chain: String,
        /// Token contract address (optional)
        token_contract_address: Option<String>,
        /// Pair contract address (optional)
        pair_contract_address: Option<String>,
        /// Wallet address for the order
        wallet_address: String,
        /// Session for authentication
        session: Session,
        /// Iris client for API communication
        client: IrisClient,
        /// Transaction preset key (optional, defaults to "a")
        tx_preset_key: Option<String>,
        /// Transaction preset method (optional, defaults to Normal)
        tx_preset_method: Option<String>,
        /// Transaction bribe (optional, defaults to "0")
        tx_preset_bribe: Option<String>,
        /// Transaction max base gas (optional, defaults to "0")
        tx_preset_max_base_gas: Option<String>,
        /// Transaction priority gas (optional, defaults to "0")
        tx_preset_priority_gas: Option<String>,
        /// Transaction slippage (optional, defaults to "0")
        tx_preset_slippage: Option<String>,
    },
}

/// Response types for trades operations.
#[derive(Debug, Clone)]
pub enum TradesResponse {
    /// Intent created successfully
    IntentCreated {
        /// The new intent ID
        intent_id: u64,
        /// Wallet address
        wallet_address: String,
        /// Expiration time (seconds from now)
        expires_in_secs: u64,
    },

    /// Intent confirmed and signed
    IntentConfirmed {
        /// Intent ID
        intent_id: u64,
        /// Transaction hash (if already submitted)
        tx_hash: Option<String>,
    },

    /// Intent cancelled
    IntentCancelled {
        /// Intent ID
        intent_id: u64,
    },

    /// Trade status response
    TradeStatus {
        /// Intent ID
        intent_id: u64,
        /// Current status
        status: TradeStatus,
        /// Additional info
        info: Option<TradeInfo>,
    },

    /// Trade submitted to Iris
    TradeSubmitted {
        /// Intent ID
        intent_id: u64,
        /// Transaction hash
        tx_hash: Option<String>,
    },

    /// Trade confirmed on-chain
    TradeConfirmed {
        /// Intent ID
        intent_id: u64,
    },

    /// Trade failed
    TradeFailed {
        /// Intent ID
        intent_id: u64,
        /// Error message
        error: String,
    },

    /// List of trades
    TradeList {
        /// Pending intents
        pending: Vec<TradeInfo>,
        /// Active trades
        active: Vec<TradeInfo>,
        /// Recent history
        history: Vec<TradeRecord>,
    },

    /// Stale intents expired
    StaleExpired {
        /// Number of intents expired
        count: usize,
        /// IDs of expired intents
        ids: Vec<u64>,
    },

    /// Success (generic)
    Success,

    /// Error occurred
    Error(TradesError),

    /// Spot order placed successfully
    SpotOrderPlaced {
        /// Result of the spot order placement
        result: Result<(), TradesError>,
    },
}

/// Request type using PoseidonRequest pattern.
///
/// TradesRequest wraps TradesMessage with trace context and reply channel.
/// This enables request/response communication with telemetry support.
pub type TradesRequest = PoseidonRequest<TradesMessage, TradesResponse, TradesError>;

/// Events emitted by the trades domain.
///
/// These are published to the EventBus for other domains to observe.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TradesEvent {
    /// Trade intent was created
    IntentCreated {
        /// Intent ID
        id: u64,
        /// Wallet address
        wallet: String,
        /// Chain type
        chain: String,
    },

    /// Trade intent was confirmed
    IntentConfirmed {
        /// Intent ID
        id: u64,
    },

    /// Trade was submitted to Iris API
    Submitted {
        /// Intent ID
        id: u64,
        /// Transaction hash
        tx_hash: Option<String>,
    },

    /// Trade was confirmed on-chain
    Confirmed {
        /// Intent ID
        id: u64,
        /// Transaction hash
        tx_hash: Option<String>,
    },

    /// Trade failed
    Failed {
        /// Intent ID
        id: u64,
        /// Error message
        error: String,
    },

    /// Intent expired
    Expired {
        /// Intent ID
        id: u64,
    },

    /// Intent was cancelled
    Cancelled {
        /// Intent ID
        id: u64,
    },
}

impl TradesEvent {
    /// Convert to StateEvent for EventBus publication.
    pub fn to_state_event(&self) -> crate::event_bus::StateEvent {
        use crate::event_bus::StateEvent;

        match self {
            TradesEvent::IntentCreated { id, wallet, chain: _ } => StateEvent::TradeIntentCreated {
                id: *id,
                wallet: wallet.clone(),
            },
            TradesEvent::IntentConfirmed { id } => StateEvent::TradeIntentConfirmed { id: *id },
            TradesEvent::Submitted { id, tx_hash } => StateEvent::TradeSubmitted {
                id: *id,
                tx_hash: tx_hash.clone(),
            },
            TradesEvent::Confirmed { id, .. } => StateEvent::TradeConfirmed { id: *id },
            TradesEvent::Failed { id, error } => StateEvent::TradeFailed {
                id: *id,
                error: error.clone(),
            },
            TradesEvent::Expired { id } => StateEvent::TradeExpired { id: *id },
            TradesEvent::Cancelled { id } => {
                // Map cancelled to expired for StateEvent
                StateEvent::TradeExpired { id: *id }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trades_message_variants() {
        let create = TradesMessage::CreateIntent {
            wallet_address: "0x1234".to_string(),
            chain: ChainType::Ethereum,
            action: TradeAction::Swap {
                from_token: "ETH".to_string(),
                to_token: "USDC".to_string(),
                amount: "1.0".to_string(),
            },
            params: serde_json::json!({}),
        };
        assert!(matches!(create, TradesMessage::CreateIntent { .. }));

        let confirm = TradesMessage::ConfirmIntent { intent_id: 42 };
        assert!(matches!(confirm, TradesMessage::ConfirmIntent { intent_id: 42 }));

        let cancel = TradesMessage::CancelIntent { intent_id: 42 };
        assert!(matches!(cancel, TradesMessage::CancelIntent { .. }));

        let get_status = TradesMessage::GetStatus { intent_id: 42 };
        assert!(matches!(get_status, TradesMessage::GetStatus { .. }));

        let submit = TradesMessage::SubmitToIris { intent_id: 42 };
        assert!(matches!(submit, TradesMessage::SubmitToIris { .. }));
    }

    #[test]
    fn test_trades_response_variants() {
        let created = TradesResponse::IntentCreated {
            intent_id: 1,
            wallet_address: "0x1234".to_string(),
            expires_in_secs: 300,
        };
        assert!(matches!(created, TradesResponse::IntentCreated { .. }));

        let confirmed = TradesResponse::IntentConfirmed {
            intent_id: 1,
            tx_hash: None,
        };
        assert!(matches!(confirmed, TradesResponse::IntentConfirmed { .. }));

        let list = TradesResponse::TradeList {
            pending: vec![],
            active: vec![],
            history: vec![],
        };
        assert!(matches!(list, TradesResponse::TradeList { .. }));
    }

    #[test]
    fn test_trades_event_to_state_event() {
        let event = TradesEvent::IntentCreated {
            id: 1,
            wallet: "0x1234".to_string(),
            chain: "ethereum".to_string(),
        };
        let state_event = event.to_state_event();
        assert!(matches!(
            state_event,
            crate::event_bus::StateEvent::TradeIntentCreated { .. }
        ));

        let event = TradesEvent::Confirmed {
            id: 1,
            tx_hash: Some("0xabc".to_string()),
        };
        let state_event = event.to_state_event();
        assert!(matches!(
            state_event,
            crate::event_bus::StateEvent::TradeConfirmed { .. }
        ));
    }
}
