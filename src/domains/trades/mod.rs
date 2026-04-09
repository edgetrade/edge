//! Trades domain - Trade intents + execution flow
//!
//! This domain manages trade intents, user confirmation flow,
//! signing orchestration with the enclave, and submission to Iris API.
//!
//! ## Architecture
//!
//! The trades domain follows the actor/handler pattern:
//! - **Actor** (`TradesActor`): Owns state, processes messages
//! - **Handle** (`TradesHandle`): Thin gateway, public API
//! - **Messages** (`TradesMessage`): Command/query enums
//! - **State** (`TradesState`): Domain state structures (in actor.rs)
//! - **Errors** (`TradesError`): Domain-specific errors
//!
//! ## Security Model
//!
//! - Trade intents are stored in `pending_intents` awaiting confirmation
//! - After user confirmation, the enclave signs the intent
//! - Only encrypted signed payloads are stored in `active_trades`
//! - Plaintext private keys never touch the trades domain
//!
//! ## Event Flow
//!
//! 1. **Intent Creation**: `TradeIntentCreated` → pending
//! 2. **User Confirmation**: `TradeIntentConfirmed` → signing
//! 3. **Enclave Signing**: `TradeIntentSigned` → active
//! 4. **Iris Submission**: `TradeSubmitted` → submitted
//! 5. **Confirmation**: `TradeConfirmed` or `TradeFailed` → history
//!
//! ## Dependencies
//!
//! - **enclave**: For secure signing of trade intents
//! - **client**: For submitting trades to Iris API

pub mod actor;
pub mod errors;
pub mod handle;
pub mod messages;

// Re-exports for convenience
pub use actor::{
    ActiveTrade, ChainType, TradeAction, TradeInfo, TradeIntent, TradeRecord, TradeStatus, TradesActor, TradesState,
    run_trades_actor,
};
pub use errors::TradesError;
pub use handle::TradesHandle;
pub use messages::{TradesEvent, TradesMessage, TradesRequest, TradesResponse};

pub mod prelude {
    //! Convenience re-exports for the trades domain
    //!
    //! This module provides a prelude for convenient imports.

    pub use super::{
        ActiveTrade, ChainType, TradeAction, TradeInfo, TradeIntent, TradeRecord, TradeStatus, TradesActor,
        TradesError, TradesEvent, TradesHandle, TradesMessage, TradesRequest, TradesResponse, TradesState,
        run_trades_actor,
    };
}

use std::str::FromStr;

use erato::messages::envelopes::transport::{ExecutionPayload, SealedIntent, TransportEnvelope, TransportEnvelopeKey};

use crate::domains::client::IrisClient;
use crate::domains::client::generated::routes::requests::orders_place_spot_order::{
    PlaceSpotOrderRequest, PlaceSpotOrderRequestOrder, PlaceSpotOrderRequestOrderAmount,
    PlaceSpotOrderRequestOrderSide, PlaceSpotOrderRequestOrderTxPreset, PlaceSpotOrderRequestOrderTxPresetMethod,
};
use crate::domains::client::get_transport_key;
use crate::domains::client::place_spot_order;
use crate::domains::keystore::Session;
use crate::domains::trades::errors::TradesResult;
use base64::{Engine, engine::general_purpose::STANDARD};
use erato::types::ChainId;

/// Place a spot order.
///
/// This function creates a sealed execution payload and places a spot order
/// through the Iris API. It handles envelope creation with the transport key
/// and user encryption key from the session.
///
/// # Arguments
/// * `side` - Order side (buy or sell)
/// * `value` - Order size (amount)
/// * `chain` - Blockchain chain identifier
/// * `token_contract_address` - Token contract address (optional)
/// * `pair_contract_address` - Pair contract address (optional)
/// * `session` - Session for authentication
/// * `client` - Iris client for API communication
///
/// # Returns
/// Ok(()) on success, or TradesError on failure
pub async fn place_spot(
    side: &str,
    value: u128,
    chain: &str,
    token_contract_address: Option<String>,
    pair_contract_address: Option<String>,
    session: &Session,
    client: &IrisClient,
) -> TradesResult<()> {
    // Get transport key for envelope creation
    let enclave_keys = get_transport_key(client)
        .await
        .map_err(|e| TradesError::Client(format!("Failed to get transport key: {}", e)))?;

    let transport_key = TransportEnvelopeKey::unsealing(enclave_keys.ephemeral);

    // Get user encryption key from session
    let user_key = session
        .get_user_encryption_key()
        .map_err(|e| TradesError::Client(format!("Failed to get user encryption key: {}", e)))?
        .ok_or_else(|| TradesError::Client("User encryption key not found".to_string()))?;

    // Get agent ID from session config
    let agent_id = session
        .get_config()
        .map_err(|e| TradesError::Config(format!("Failed to get config: {}", e)))?
        .agent_id;

    // Fallback to loading from config file
    let agent_id = if let Some(id) = agent_id {
        id
    } else {
        let config = crate::domains::config::Config::load(None)
            .map_err(|e| TradesError::Config(format!("Failed to load config: {}", e)))?;
        config
            .agent_id
            .ok_or_else(|| TradesError::Config("Agent ID not found in config".to_string()))?
    };

    // Parse chain ID
    let chain_id = ChainId::from_str(chain).map_err(|_| TradesError::InvalidChain(chain.to_string()))?;

    // Create wallet address (using a default for now - could be from config)
    let wallet_address = "0x0000000000000000000000000000000000000000".to_string();

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
    let order_side = match side {
        "buy" => PlaceSpotOrderRequestOrderSide::Buy,
        "sell" => PlaceSpotOrderRequestOrderSide::Sell,
        _ => {
            return Err(TradesError::InvalidInput(format!(
                "Invalid order side: {}; must be 'buy' or 'sell'",
                side
            )));
        }
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
                key: "a".to_string(),
                method: PlaceSpotOrderRequestOrderTxPresetMethod::Normal,
                bribe: "0".to_string(),
                max_base_gas: "0".to_string(),
                priority_gas: "0".to_string(),
                slippage: "0".to_string(),
            },
            exit_strategy_id: None,
        },
    };

    // Execute the place_spot_order request
    let response = place_spot_order(&request, client)
        .await
        .map_err(|e| TradesError::SubmissionFailed(e.to_string()))?;

    // Validate response - check if any transaction failed
    let has_errors = response
        .iter()
        .any(|item| item.transactions.iter().any(|tx| tx.subtype_1.is_some()));

    if has_errors {
        return Err(TradesError::ExecutionFailed("Failed to place spot order".to_string()));
    }

    // Output success message
    crate::messages::success::successful_order(response);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_exports() {
        // Just verify all types are accessible
        let _: TradesState = TradesState::new();
        let _: ChainType = ChainType::Ethereum;
        let _: ChainType = ChainType::Solana;
    }

    #[test]
    fn test_trade_action_variants() {
        let swap = TradeAction::Swap {
            from_token: "ETH".to_string(),
            to_token: "USDC".to_string(),
            amount: "1.0".to_string(),
        };
        assert!(matches!(swap, TradeAction::Swap { .. }));

        let transfer = TradeAction::Transfer {
            to: "0x1234".to_string(),
            token: "USDC".to_string(),
            amount: "100".to_string(),
        };
        assert!(matches!(transfer, TradeAction::Transfer { .. }));
    }

    #[test]
    fn test_trade_status_variants() {
        assert!(matches!(TradeStatus::Pending, TradeStatus::Pending));
        assert!(matches!(TradeStatus::Signed, TradeStatus::Signed));
        assert!(matches!(
            TradeStatus::Submitted { tx_hash: None },
            TradeStatus::Submitted { .. }
        ));
        assert!(matches!(TradeStatus::Completed, TradeStatus::Completed));
    }
}
