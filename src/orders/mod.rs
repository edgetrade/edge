use std::str::FromStr;

use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use erato::models::chain_type::{ChainType, get_chain_type};
use thiserror::Error;

use erato::models::ChainId;
use tyche_enclave::envelopes::transport::{ExecutionPayload, SealedIntent, TransportEnvelope, TransportEnvelopeKey};
use uuid::Uuid;

use crate::client::place_spot_order;
use crate::generated::routes::requests::orders_place_spot_order::{
    self, PlaceSpotOrderRequestOrderPairId, PlaceSpotOrderRequestOrderPairIdChainType,
    PlaceSpotOrderRequestOrderPairIdPairType, PlaceSpotOrderRequestOrderTxPreset,
    PlaceSpotOrderRequestOrderTxPresetMethod, PlaceSpotOrderRequestOrderWalletsItem,
};
use crate::session::transport::get_transport_key as get_transport_key_session;
use crate::{Config, Session, messages};

#[derive(Error, Debug)]
pub enum OrdersError {
    #[error("Invalid chain ID: {0}")]
    InvalidChainId(String),
    #[error("Invalid order side: {0}; must be 'buy' or 'sell'")]
    InvalidOrderSide(String),
    #[error("Agent ID not found")]
    AgentIdNotFound,
    #[error("User key not found")]
    UserKeyNotFound,
    #[error("Transport key not found")]
    TransportKeyNotFound,
    #[error("Execution error: {0}")]
    ExecutionError(String),
    #[error("Encryption error: {0}")]
    Serialization(String),
}

async fn get_transport_key(client: &crate::client::IrisClient) -> Result<TransportEnvelopeKey, OrdersError> {
    let key = get_transport_key_session(client)
        .await
        .map_err(|_| OrdersError::TransportKeyNotFound)?;

    Ok(TransportEnvelopeKey::Unsealing(key.ephemeral))
}

fn get_agent_id(session: &Session) -> Result<Uuid, OrdersError> {
    let maybe_agent_id = session
        .get_config()
        .map_err(|_| OrdersError::AgentIdNotFound)?
        .clone()
        .agent_id;

    if let Some(agent_id) = maybe_agent_id {
        return Ok(agent_id);
    }

    let config = Config::load(None).map_err(|_| OrdersError::AgentIdNotFound)?;
    let agent_id = config.agent_id;
    if agent_id.is_none() {
        return Err(OrdersError::AgentIdNotFound);
    };

    Ok(agent_id.unwrap())
}

fn get_chain_id(chain: &str) -> Result<ChainId, OrdersError> {
    ChainId::from_str(chain).map_err(|_| OrdersError::InvalidChainId(chain.to_string()))
}

pub async fn create_intent_envelope(
    cid: &str,
    wallet_address: String,
    value: &u128,
    session: &Session,
    client: &crate::client::IrisClient,
) -> Result<Vec<u8>, OrdersError> {
    let transport_key = get_transport_key(client).await?;

    let user_key = match session.get_user_encryption_key() {
        Ok(Some(key)) => key,
        _ => return Err(OrdersError::UserKeyNotFound),
    };

    let chain_id = get_chain_id(cid)?.to_string();
    let agent_id = Some(get_agent_id(session)?.to_string());

    let sealed_intent = SealedIntent {
        user_id: None,
        agent_id,
        chain_id,
        wallet_address,
        value: value.to_string(),
    };

    ExecutionPayload::new(user_key.storage, sealed_intent)
        .seal(&transport_key)
        .map_err(|e| OrdersError::Serialization(e.to_string()))
}

pub async fn place_spot(
    side: &str,
    value: &u128,
    chain: &str,
    pair: &str,
    session: &Session,
    client: &crate::client::IrisClient,
) -> Result<(), OrdersError> {
    let wallet_address = "0x0000000000000000000000000000000000000000".to_string();

    let envelope = create_intent_envelope(chain, wallet_address.clone(), value, session, client).await?;

    let side = match side {
        "buy" => orders_place_spot_order::PlaceSpotOrderRequestOrderSide::Buy,
        "sell" => orders_place_spot_order::PlaceSpotOrderRequestOrderSide::Sell,
        _ => return Err(OrdersError::InvalidOrderSide(side.to_string())),
    };

    let chain_id = get_chain_id(chain)?;
    let ctype = get_chain_type(chain_id);
    let chain_type = match ctype {
        ChainType::EVM => PlaceSpotOrderRequestOrderPairIdChainType::Evm,
        ChainType::SVM => PlaceSpotOrderRequestOrderPairIdChainType::Svm,
    };

    let request = orders_place_spot_order::PlaceSpotOrderRequest {
        envelope: STANDARD.encode(&envelope),
        order: orders_place_spot_order::PlaceSpotOrderRequestOrder {
            amount: orders_place_spot_order::PlaceSpotOrderRequestOrderAmount::Native(value.to_string()),
            side,
            exit_strategy_id: None,
            pair_id: PlaceSpotOrderRequestOrderPairId {
                chain_type,
                pair_type: PlaceSpotOrderRequestOrderPairIdPairType::default(),
                pair_chain_id: chain_id.to_string(),
                pair_contract_address: pair.to_string(),
            },
            tx_preset: PlaceSpotOrderRequestOrderTxPreset {
                key: "a".to_string(),
                method: PlaceSpotOrderRequestOrderTxPresetMethod::Normal,
                bribe: "0".to_string(),
                max_base_gas: "0".to_string(),
                priority_gas: "0".to_string(),
                slippage: "0".to_string(),
            },
            wallets: [PlaceSpotOrderRequestOrderWalletsItem {
                address: wallet_address,
            }],
        },
    };

    let response = place_spot_order(&request, client).await?;
    match response
        .iter()
        .any(|item| item.transactions.iter().any(|tx| tx.subtype_1.is_some()))
    {
        true => return Err(OrdersError::ExecutionError("Failed to place spot order".to_string())),
        false => {
            messages::success::successful_order(response);
        }
    }
    Ok(())
}
