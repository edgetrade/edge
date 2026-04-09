//! Command implementations using domain handles
//!
//! Migrated from commands/key/, commands/wallet/, commands/serve/, commands/subscribe/
//! This module provides command implementations that use domain handles via the App orchestrator
//! instead of direct state access.

use crate::app::cli::{KeyCommand, ServeArgs, Transport, WalletCommand};
use crate::app::orchestrator::App;
use crate::domains::mcp::TransportType;
use crate::domains::trades::actor::{ChainType as TradesChainType, TradeAction};
use crate::error::PoseidonError;

use erato::ChainType as EnclaveChainType;

/// Result type for commands
pub type CommandResult<T> = Result<T, PoseidonError>;

/// Convert Enclave ChainType to Trades ChainType
fn to_trades_chain_type(chain: EnclaveChainType) -> TradesChainType {
    match chain {
        EnclaveChainType::EVM => TradesChainType::Ethereum,
        EnclaveChainType::SVM => TradesChainType::Solana,
    }
}

/// Run key command using keystore domain
///
/// # Arguments
/// * `app` - The App orchestrator
/// * `cmd` - The key command to run
/// * `password` - Optional password for unlock/update operations
///
/// # Returns
/// JSON value with the command result
///
/// # Errors
/// Returns `PoseidonError` if the keystore operation fails
pub async fn run_key_command(app: &App, cmd: KeyCommand, password: Option<String>) -> CommandResult<serde_json::Value> {
    match cmd {
        KeyCommand::Unlock => {
            let password =
                password.ok_or_else(|| PoseidonError::InvalidInput("Password required for unlock".to_string()))?;
            let _result = app.keystore.unlock(password).await?;
            Ok(serde_json::json!({ "status": "unlocked" }))
        }
        KeyCommand::Lock => {
            let _result = app.keystore.lock().await?;
            Ok(serde_json::json!({ "status": "locked" }))
        }
        KeyCommand::Update => {
            // Update requires interactive password input
            Err(PoseidonError::Command(
                "Update command requires interactive mode".to_string(),
            ))
        }
        KeyCommand::Create => {
            // Create key config
            Err(PoseidonError::Command(
                "Create key command requires additional parameters".to_string(),
            ))
        }
        KeyCommand::Delete => {
            // Delete key config
            Err(PoseidonError::Command(
                "Delete key command requires additional parameters".to_string(),
            ))
        }
    }
}

/// Run wallet command using enclave domain
///
/// # Arguments
/// * `app` - The App orchestrator
/// * `cmd` - The wallet command to run
///
/// # Returns
/// JSON value with the command result
///
/// # Errors
/// Returns `PoseidonError` if the enclave operation fails
pub async fn run_wallet_command(app: &App, cmd: WalletCommand) -> CommandResult<serde_json::Value> {
    match cmd {
        WalletCommand::Create { chain_type, name } => {
            let chain = EnclaveChainType::parse(&chain_type).map_err(|e| PoseidonError::InvalidInput(e.to_string()))?;
            let wallet = app
                .enclave
                .create_wallet(chain, name.unwrap_or_default())
                .await?;
            Ok(serde_json::json!({
                "status": "created",
                "wallet": {
                    "name": wallet.name,
                    "address": wallet.address,
                    "chain": format!("{:?}", wallet.chain),
                }
            }))
        }
        WalletCommand::Import {
            chain_type,
            name,
            key_file,
        } => {
            let chain = EnclaveChainType::parse(&chain_type).map_err(|e| PoseidonError::InvalidInput(e.to_string()))?;

            // Read private key from file or prompt
            let private_key = if let Some(file_path) = key_file {
                tokio::fs::read_to_string(&file_path).await?
            } else {
                // In CLI mode, would prompt for input
                // For now, return an error
                return Err(PoseidonError::InvalidInput("Private key file required".to_string()));
            };

            let wallet = app
                .enclave
                .import_wallet(chain, name.unwrap_or_default(), private_key)
                .await?;

            Ok(serde_json::json!({
                "status": "imported",
                "wallet": {
                    "name": wallet.name,
                    "address": wallet.address,
                    "chain": format!("{:?}", wallet.chain),
                }
            }))
        }
        WalletCommand::Delete { address } => {
            app.enclave.delete_wallet(address).await?;
            Ok(serde_json::json!({ "status": "deleted" }))
        }
        WalletCommand::List => {
            let wallets = app.enclave.list_wallets().await?;
            let wallets_json: Vec<_> = wallets
                .into_iter()
                .map(|w| {
                    serde_json::json!({
                        "name": w.name,
                        "address": w.address,
                        "chain": format!("{:?}", w.chain_type),
                    })
                })
                .collect();
            Ok(serde_json::json!({ "wallets": wallets_json }))
        }
        WalletCommand::Prove { game: _ } => {
            // Proving games are interactive and require special handling
            // For now, return not implemented
            Err(PoseidonError::Command(
                "Prove command requires interactive mode".to_string(),
            ))
        }
    }
}

/// Run serve command using mcp domain
///
/// # Arguments
/// * `app` - The App orchestrator
/// * `args` - The serve arguments
///
/// # Returns
/// JSON value with the command result
///
/// # Errors
/// Returns `PoseidonError` if the MCP server fails to start
pub async fn run_serve_command(app: &App, args: ServeArgs) -> CommandResult<serde_json::Value> {
    let transport = match args.transport {
        Transport::Stdio => TransportType::Stdio,
        Transport::Http => TransportType::Http {
            host: args.host.clone(),
            port: args.port,
        },
    };

    app.mcp.start(transport).await?;

    Ok(serde_json::json!({
        "status": "started",
        "transport": match args.transport {
            Transport::Stdio => "stdio",
            Transport::Http => "http",
        },
        "host": args.host,
        "port": args.port,
        "path": args.path,
    }))
}

/// Run subscribe command using alerts domain
///
/// # Arguments
/// * `app` - The App orchestrator
/// * `procedure` - Procedure to subscribe to
/// * `delivery` - Delivery configuration
///
/// # Returns
/// JSON value with the command result
///
/// # Errors
/// Returns `PoseidonError` if the subscription fails
pub async fn run_subscribe_command(
    app: &App,
    procedure: String,
    delivery: crate::domains::alerts::DeliveryConfig,
) -> CommandResult<serde_json::Value> {
    let response = app.alerts.register_delivery(delivery).await?;

    Ok(serde_json::json!({
        "status": "registered",
        "procedure": procedure,
        "result": format!("{:?}", response),
    }))
}

/// Run trade command using trades domain
///
/// # Arguments
/// * `app` - The App orchestrator
/// * `wallet_address` - Wallet address for the trade
/// * `chain` - Blockchain type
/// * `action` - The trade action
///
/// # Returns
/// JSON value with the command result
///
/// # Errors
/// Returns `PoseidonError` if the trade operation fails
pub async fn run_trade_create_command(
    app: &App,
    wallet_address: String,
    chain: EnclaveChainType,
    action: TradeAction,
) -> CommandResult<serde_json::Value> {
    let trades_chain = to_trades_chain_type(chain);
    let intent_id = app
        .trades
        .create_intent(wallet_address, trades_chain, action)
        .await?;

    Ok(serde_json::json!({
        "status": "intent_created",
        "intent_id": intent_id,
    }))
}

/// Run config command using config domain
///
/// # Arguments
/// * `app` - The App orchestrator
/// * `key` - Config key
/// * `value` - Optional value to set
///
/// # Returns
/// JSON value with the command result
///
/// # Errors
/// Returns `PoseidonError` if the config operation fails
pub async fn run_config_get_command(app: &App, key: String) -> CommandResult<serde_json::Value> {
    let value = app.config.get_value(&key).await?;
    Ok(value)
}

/// Set config value
///
/// # Arguments
/// * `app` - The App orchestrator
/// * `key` - Config key
/// * `value` - Value to set
///
/// # Returns
/// JSON value with the command result
pub async fn run_config_set_command(
    app: &App,
    key: String,
    value: serde_json::Value,
) -> CommandResult<serde_json::Value> {
    app.config.set_value(&key, value).await?;
    Ok(serde_json::json!({ "status": "set", "key": key }))
}

/// Reload config from disk
///
/// # Arguments
/// * `app` - The App orchestrator
///
/// # Returns
/// JSON value with the command result
pub async fn run_config_reload_command(app: &App) -> CommandResult<serde_json::Value> {
    app.config.reload().await?;
    Ok(serde_json::json!({ "status": "reloaded" }))
}

/// Get config file path
///
/// # Arguments
/// * `app` - The App orchestrator
///
/// # Returns
/// JSON value with the command result
pub async fn run_config_path_command(app: &App) -> CommandResult<serde_json::Value> {
    let path = app.config.get_config_path().await?;
    Ok(serde_json::json!({
        "path": path.to_string_lossy().to_string(),
    }))
}
