//! App Orchestrator
//!
//! Coordinates all 8 domains and provides the unified entry point.
//! Manages domain lifecycle, direct mpsc channel wiring, and CLI/daemon modes.
//!
//! MIGRATED FROM: app/runner.rs - App struct and run function
//! REFACTORED: Now uses domain handles with actor/handler pattern

use std::path::PathBuf;

use tokio::signal::unix::{SignalKind, signal};
use tokio::sync::mpsc;

// Domain imports - using public exports
use crate::domains::alerts::{AlertsHandle, AlertsRequest};
use crate::domains::client::{ClientHandle, ClientRequest};
use crate::domains::config::{ConfigHandle, ConfigRequest};
use crate::domains::enclave::{EnclaveHandle, EnclaveRequest};
use crate::domains::ipc::{DomainGatewayRegistry, IpcDomainRequest, IpcHandle, IpcListener};
use crate::domains::keystore::{BackendType, KeystoreHandle, KeystoreRequest};
use crate::domains::mcp::{McpHandle, McpRequest, TransportType};
use crate::domains::trades::{TradesHandle, TradesRequest};
use crate::event_bus::EventBus;

use erato::ChainType;

/// App handle - orchestrates all domains
///
/// This is the central orchestrator that:
/// - Creates mpsc channels for direct domain-to-domain communication
/// - Initializes all 8 domain actors with their receivers
/// - Provides access to all domain handles for CLI/daemon/IPC use
/// - Manages graceful shutdown
#[derive(Clone)]
pub struct App {
    /// Config domain handle
    pub config: ConfigHandle,
    /// Keystore domain handle
    pub keystore: KeystoreHandle,
    /// Enclave domain handle
    pub enclave: EnclaveHandle,
    /// Client domain handle
    pub client: ClientHandle,
    /// Trades domain handle
    pub trades: TradesHandle,
    /// MCP domain handle
    pub mcp: McpHandle,
    /// Alerts domain handle
    pub alerts: AlertsHandle,
    /// IPC domain handle
    pub ipc: IpcHandle,
    /// EventBus for inter-domain communication
    pub event_bus: EventBus,
    /// Domain gateway registry for direct mpsc routing
    pub domain_gateways: DomainGatewayRegistry,
}

impl App {
    /// Initialize all domains with direct mpsc channel wiring
    ///
    /// Creates mpsc channels for each domain, initializes all 8 domain handles
    /// in dependency order, and wires them together for direct communication.
    ///
    /// # Arguments
    /// * `config_path` - Optional path to config file (uses default if None)
    /// * `cli_iris_url` - Optional URL from CLI flag (takes highest priority)
    ///
    /// # Returns
    /// * `App` - The orchestrated app with all domains initialized
    ///
    /// # Errors
    /// Returns `AppError` if any domain fails to initialize
    ///
    /// # Domain Dependency Order
    /// 1. Config (no dependencies, root domain)
    /// 2. Keystore (depends on config for keyring vs filestore preference)
    /// 3. Enclave (depends on keystore for UEK)
    /// 4. Client (depends on config for API key)
    /// 5. Trades (depends on enclave and client)
    /// 6. Alerts (depends on client for subscriptions)
    /// 7. MCP (depends on client, enclave, trades, alerts)
    /// 8. IPC (depends on all domains via domain_gateways)
    pub async fn new(config_path: Option<PathBuf>) -> Result<Self, AppError> {
        // Create EventBus
        let event_bus = EventBus::new(128);

        // Create mpsc channels for domains (64 capacity each)
        // Config creates its own channel internally
        let (keystore_tx, keystore_rx) = mpsc::channel::<KeystoreRequest>(64);
        let (enclave_tx, enclave_rx) = mpsc::channel::<EnclaveRequest>(64);
        let (client_tx, _client_rx) = mpsc::channel::<ClientRequest>(64);
        let (trades_tx, trades_rx) = mpsc::channel::<TradesRequest>(64);
        // MCP will create its own channel - no mcp_tx/mcp_rx here
        let (alerts_tx, alerts_rx) = mpsc::channel::<AlertsRequest>(64);

        // Create domain gateway registry with senders for all domains
        // Config, MCP and IPC will get their senders from handles after creation
        let domain_gateways = {
            // Create temporary placeholders for Config, MCP and IPC senders
            let (config_placeholder_tx, _) = mpsc::channel::<ConfigRequest>(1);
            let (mcp_placeholder_tx, _) = mpsc::channel::<McpRequest>(1);
            let (ipc_placeholder_tx, _) = mpsc::channel::<IpcDomainRequest>(1);
            DomainGatewayRegistry {
                config_tx: config_placeholder_tx,
                keystore_tx: keystore_tx.clone(),
                enclave_tx: enclave_tx.clone(),
                client_tx: client_tx.clone(),
                trades_tx: trades_tx.clone(),
                mcp_tx: mcp_placeholder_tx,
                alerts_tx: alerts_tx.clone(),
                ipc_tx: ipc_placeholder_tx,
            }
        };

        // Initialize domains IN ORDER with their receivers
        // Dependencies flow from left to right

        // 1. Config - no dependencies, root domain
        // Clone config_path before moving into ConfigHandle
        let config_path_for_config = config_path.clone();
        let config = ConfigHandle::new(config_path_for_config, event_bus.clone())
            .await
            .map_err(|e| AppError::ConfigInit(e.to_string()))?;

        // Get host capabilities and keyring preference from config
        let host_caps = config
            .get_host_capabilities()
            .await
            .map_err(|e| AppError::ConfigInit(e.to_string()))?;
        let keyring_available = host_caps.keyring_available;

        // 2. Keystore - depends on config for keyring vs filestore preference
        let backend = if keyring_available {
            BackendType::Keyring
        } else {
            BackendType::Filestore
        };

        // Load config to get keystore settings and iris_url
        let config_data = if let Some(ref path) = config_path.clone() {
            crate::domains::config::Config::load(Some(path.clone())).unwrap_or_default()
        } else {
            crate::domains::config::Config::default()
        };

        // Determine iris_url with priority: CLI flag > env var > default
        let iris_url = std::env::var("EDGE_MCP_URL")
            .ok()
            .unwrap_or_else(|| super::DEFAULT_IRIS_URL.to_string());

        let keystore = KeystoreHandle::new(config_data, backend, keystore_rx, event_bus.clone())
            .await
            .map_err(|e| AppError::KeystoreInit(e.to_string()))?;

        // 3. Enclave - depends on keystore for UEK
        let enclave = EnclaveHandle::new(enclave_rx, event_bus.clone())
            .await
            .map_err(|e| AppError::EnclaveInit(e.to_string()))?;

        // 4. Client - depends on config for API key
        let client = ClientHandle::new(event_bus.clone());

        // Get API key from config and connect
        let api_key = config
            .get_value("api_key")
            .await
            .map_err(|e| AppError::ClientInit(format!("Failed to get API key: {}", e)))?
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| AppError::ClientInit("API key not found in config".to_string()))?;

        client
            .connect(iris_url, api_key, false)
            .await
            .map_err(|e| AppError::ClientInit(e.to_string()))?;

        // 5. Trades - depends on enclave, client
        let trades = TradesHandle::new(trades_rx, event_bus.clone())
            .await
            .map_err(|e| AppError::TradesInit(e.to_string()))?;

        // 6. Alerts - depends on client for subscriptions
        let alerts = AlertsHandle::new(&client, alerts_rx, event_bus.clone())
            .await
            .map_err(|e| AppError::AlertsInit(e.to_string()))?;

        // 7. MCP - depends on client, enclave, trades, alerts (via handles)
        // Note: Handle now creates its own channel internally, following Demeter pattern
        let (mcp, _mcp_actor) = McpHandle::new(
            client.clone(),
            enclave.clone(),
            trades.clone(),
            alerts.clone(),
            event_bus.clone(),
        );

        // 8. IPC - depends on domain_gateways (has all senders for routing)
        // IPC creates its own channel internally (Demeter pattern)
        let (ipc, _ipc_actor_handle) = IpcHandle::new(domain_gateways.clone(), event_bus.clone());

        // Update domain_gateways with the actual Config and IPC senders
        let mut domain_gateways = domain_gateways;
        domain_gateways.config_tx = config.sender().clone();
        domain_gateways.ipc_tx = ipc.sender().clone();

        Ok(Self {
            config,
            keystore,
            enclave,
            client,
            trades,
            mcp,
            alerts,
            ipc,
            event_bus,
            domain_gateways,
        })
    }

    /// Create a new App with minimal configuration (for testing)
    ///
    /// This creates an App without connecting to external services.
    /// Useful for unit tests and when full initialization isn't needed.
    #[cfg(test)]
    pub async fn new_minimal() -> Result<Self, AppError> {
        let event_bus = EventBus::new(128);

        // Create mpsc channels for direct domain communication
        // Config creates its own channel internally
        let (keystore_tx, keystore_rx) = mpsc::channel::<KeystoreRequest>(64);
        let (enclave_tx, enclave_rx) = mpsc::channel::<EnclaveRequest>(64);
        let (client_tx, _client_rx) = mpsc::channel::<ClientRequest>(64);
        let (trades_tx, trades_rx) = mpsc::channel::<TradesRequest>(64);
        // MCP creates its own channel internally
        let (alerts_tx, alerts_rx) = mpsc::channel::<AlertsRequest>(64);

        // Create domain gateway registry - Config, MCP and IPC will be updated after handle creation
        let domain_gateways = {
            let (config_placeholder_tx, _) = mpsc::channel::<ConfigRequest>(1);
            let (mcp_placeholder_tx, _) = mpsc::channel::<McpRequest>(1);
            let (ipc_placeholder_tx, _) = mpsc::channel::<IpcDomainRequest>(1);
            DomainGatewayRegistry {
                config_tx: config_placeholder_tx,
                keystore_tx: keystore_tx.clone(),
                enclave_tx: enclave_tx.clone(),
                client_tx: client_tx.clone(),
                trades_tx: trades_tx.clone(),
                mcp_tx: mcp_placeholder_tx,
                alerts_tx: alerts_tx.clone(),
                ipc_tx: ipc_placeholder_tx,
            }
        };

        // Create handles with dummy receivers (they'll be dropped but that's fine for minimal mode)
        let config = ConfigHandle::new(None, event_bus.clone())
            .await
            .map_err(|e| AppError::ConfigInit(e.to_string()))?;
        let keystore = KeystoreHandle::new(
            crate::domains::config::Config::default(),
            BackendType::Keyring,
            keystore_rx,
            event_bus.clone(),
        )
        .await
        .map_err(|e| AppError::KeystoreInit(e.to_string()))?;
        let enclave = EnclaveHandle::new(enclave_rx, event_bus.clone())
            .await
            .map_err(|e| AppError::EnclaveInit(e.to_string()))?;
        let client = ClientHandle::new(event_bus.clone());
        let trades = TradesHandle::new(trades_rx, event_bus.clone())
            .await
            .map_err(|e| AppError::TradesInit(e.to_string()))?;
        let alerts = AlertsHandle::new(&client, alerts_rx, event_bus.clone())
            .await
            .map_err(|e| AppError::AlertsInit(e.to_string()))?;
        let (mcp, _) = McpHandle::new(
            client.clone(),
            enclave.clone(),
            trades.clone(),
            alerts.clone(),
            event_bus.clone(),
        );
        // IPC creates its own channel internally (Demeter pattern)
        let (ipc, _ipc_actor_handle) = IpcHandle::new(domain_gateways.clone(), event_bus.clone());

        // Update domain_gateways with the actual Config and IPC senders
        let mut domain_gateways = domain_gateways;
        domain_gateways.config_tx = config.sender().clone();
        domain_gateways.ipc_tx = ipc.sender().clone();

        Ok(Self {
            config,
            keystore,
            enclave,
            client,
            trades,
            mcp,
            alerts,
            ipc,
            event_bus,
            domain_gateways,
        })
    }

    /// Run CLI command
    ///
    /// Routes CLI commands to appropriate domains and returns the result.
    ///
    /// # Arguments
    /// * `cmd` - The command to execute
    ///
    /// # Returns
    /// * `CommandOutput` - The result of the command execution
    pub async fn run_command(&self, cmd: Command) -> Result<CommandOutput, AppError> {
        match cmd {
            Command::Key(subcmd) => self.run_key_command(subcmd).await,
            Command::Wallet(subcmd) => self.run_wallet_command(subcmd).await,
            Command::Serve(transport) => self.run_serve_command(transport).await,
            Command::Subscribe(subcmd) => self.run_subscribe_command(subcmd).await,
            Command::Trade(subcmd) => self.run_trade_command(subcmd).await,
            Command::Config(subcmd) => self.run_config_command(subcmd).await,
        }
    }

    /// Run as daemon (persistent process)
    ///
    /// Keeps running until shutdown signal, accepting IPC connections.
    /// Handles both SIGINT (Ctrl+C) and SIGTERM signals for graceful shutdown.
    pub async fn run_daemon(&self) -> Result<(), AppError> {
        // Start IPC server to accept external connections
        let listener = IpcListener::WebSocket {
            host: "127.0.0.1".to_string(),
            port: 9090,
        };

        self.ipc
            .start(listener)
            .await
            .map_err(|e| AppError::Ipc(format!("Failed to start IPC: {}", e)))?;

        // Set up signal handlers for graceful shutdown
        // Handle both SIGINT (Ctrl+C) and SIGTERM
        let mut sigterm = signal(SignalKind::terminate())
            .map_err(|e| AppError::Shutdown(format!("Failed to set up SIGTERM handler: {}", e)))?;

        tokio::select! {
            _ = tokio::signal::ctrl_c() => {
                eprintln!("Received SIGINT (Ctrl+C), initiating graceful shutdown");
            }
            _ = sigterm.recv() => {
                eprintln!("Received SIGTERM, initiating graceful shutdown");
            }
        }

        // Graceful shutdown
        self.shutdown().await;

        Ok(())
    }

    /// Graceful shutdown
    ///
    /// Shuts down all domains in reverse dependency order:
    /// IPC → MCP → Alerts → Trades → Client → Enclave → Keystore → Config
    pub async fn shutdown(&self) {
        // Shutdown in reverse dependency order
        let _ = self.ipc.stop().await;
        let _ = self.mcp.stop().await;
        let _ = self.alerts.shutdown().await;
        let _ = self.client.disconnect().await;
        let _ = self.enclave.zeroize_keys().await;
        let _ = self.keystore.lock().await;
        // Config has no shutdown - it's just file-based
    }

    // === Command Handlers ===

    /// Run key management command
    async fn run_key_command(&self, cmd: KeyCommand) -> Result<CommandOutput, AppError> {
        match cmd {
            KeyCommand::Unlock { password } => self
                .keystore
                .unlock(password)
                .await
                .map(|_| CommandOutput::Success)
                .map_err(|e| AppError::Command(format!("Key unlock failed: {}", e))),
            KeyCommand::Lock => self
                .keystore
                .lock()
                .await
                .map(|_| CommandOutput::Success)
                .map_err(|e| AppError::Command(format!("Key lock failed: {}", e))),
            KeyCommand::ChangePassword {
                old_password,
                new_password,
            } => self
                .keystore
                .change_password(old_password, new_password)
                .await
                .map(|_| CommandOutput::Success)
                .map_err(|e| AppError::Command(format!("Password change failed: {}", e))),
        }
    }

    /// Run wallet management command
    async fn run_wallet_command(&self, cmd: WalletCommand) -> Result<CommandOutput, AppError> {
        match cmd {
            WalletCommand::List => {
                let wallets = self
                    .enclave
                    .list_wallets()
                    .await
                    .map_err(|e| AppError::Command(format!("List wallets failed: {}", e)))?;

                // Convert wallets to JSON for output
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

                Ok(CommandOutput::Data(serde_json::json!({ "wallets": wallets_json })))
            }
            WalletCommand::Create { chain, name } => {
                let chain_type = match chain.as_str() {
                    "evm" | "ethereum" => ChainType::EVM,
                    "svm" | "solana" => ChainType::SVM,
                    _ => return Err(AppError::Command(format!("Unknown chain: {}", chain))),
                };

                let wallet = self
                    .enclave
                    .create_wallet(chain_type, name)
                    .await
                    .map_err(|e| AppError::Command(format!("Create wallet failed: {}", e)))?;

                Ok(CommandOutput::Data(serde_json::json!({
                    "name": wallet.name,
                    "address": wallet.address,
                    "chain": format!("{:?}", wallet.chain),
                })))
            }
            WalletCommand::Import {
                chain,
                name,
                private_key,
            } => {
                let chain_type = match chain.as_str() {
                    "evm" | "ethereum" => ChainType::EVM,
                    "svm" | "solana" => ChainType::SVM,
                    _ => return Err(AppError::Command(format!("Unknown chain: {}", chain))),
                };

                let wallet = self
                    .enclave
                    .import_wallet(chain_type, name, private_key)
                    .await
                    .map_err(|e| AppError::Command(format!("Import wallet failed: {}", e)))?;

                Ok(CommandOutput::Data(serde_json::json!({
                    "name": wallet.name,
                    "address": wallet.address,
                    "chain": format!("{:?}", wallet.chain),
                })))
            }
            WalletCommand::Delete { name } => self
                .enclave
                .delete_wallet(name)
                .await
                .map(|_| CommandOutput::Success)
                .map_err(|e| AppError::Command(format!("Delete wallet failed: {}", e))),
        }
    }

    /// Run serve command (start MCP server)
    async fn run_serve_command(&self, transport: TransportType) -> Result<CommandOutput, AppError> {
        self.mcp
            .start(transport)
            .await
            .map(|_| CommandOutput::Success)
            .map_err(|e| AppError::Mcp(format!("MCP server failed to start: {}", e)))
    }

    /// Run subscribe command
    async fn run_subscribe_command(&self, cmd: SubscribeCommand) -> Result<CommandOutput, AppError> {
        use crate::domains::alerts::DeliveryConfig;

        match cmd {
            SubscribeCommand::Register {
                procedure: _procedure,
                delivery,
            } => {
                let delivery_config = match delivery {
                    DeliveryType::Webhook { url, secret } => DeliveryConfig::Webhook { url, secret },
                    DeliveryType::Redis { url, channel } => DeliveryConfig::Redis { url, channel },
                    DeliveryType::Telegram { bot_token, chat_id } => DeliveryConfig::Telegram { bot_token, chat_id },
                };

                let result = self
                    .alerts
                    .register_delivery(delivery_config)
                    .await
                    .map_err(|e| AppError::Command(format!("Register delivery failed: {:?}", e)))?;

                Ok(CommandOutput::Data(serde_json::json!({
                    "status": "registered",
                    "result": format!("{:?}", result),
                })))
            }
            SubscribeCommand::List => {
                let result = self
                    .alerts
                    .list_alerts()
                    .await
                    .map_err(|e| AppError::Command(format!("List alerts failed: {:?}", e)))?;

                Ok(CommandOutput::Data(serde_json::json!({
                    "alerts": format!("{:?}", result),
                })))
            }
        }
    }

    /// Run trade command
    async fn run_trade_command(&self, cmd: TradeCommand) -> Result<CommandOutput, AppError> {
        use crate::domains::trades::actor::{ChainType, TradeAction};

        match cmd {
            TradeCommand::Create { wallet, chain, action } => {
                let chain_type = match chain.as_str() {
                    "evm" | "ethereum" => ChainType::Ethereum,
                    "svm" | "solana" => ChainType::Solana,
                    _ => return Err(AppError::Command(format!("Unknown chain: {}", chain))),
                };

                let action = match action {
                    TradeActionInput::Swap {
                        from_token,
                        to_token,
                        amount,
                    } => TradeAction::Swap {
                        from_token,
                        to_token,
                        amount,
                    },
                    TradeActionInput::Transfer { to, token, amount } => TradeAction::Transfer { to, token, amount },
                };

                let intent_id = self
                    .trades
                    .create_intent(wallet, chain_type, action)
                    .await
                    .map_err(|e| AppError::Command(format!("Create trade intent failed: {}", e)))?;

                Ok(CommandOutput::Data(serde_json::json!({ "intent_id": intent_id })))
            }
            TradeCommand::Confirm { intent_id } => {
                let tx_hash = self
                    .trades
                    .confirm_intent(intent_id)
                    .await
                    .map_err(|e| AppError::Command(format!("Confirm trade failed: {}", e)))?;

                Ok(CommandOutput::Data(serde_json::json!({
                    "intent_id": intent_id,
                    "tx_hash": tx_hash,
                })))
            }
            TradeCommand::Status { intent_id } => {
                let status = self
                    .trades
                    .get_status(intent_id)
                    .await
                    .map_err(|e| AppError::Command(format!("Get trade status failed: {}", e)))?;

                Ok(CommandOutput::Data(serde_json::json!({
                    "intent_id": intent_id,
                    "status": format!("{:?}", status),
                })))
            }
            TradeCommand::Cancel { intent_id } => self
                .trades
                .cancel_intent(intent_id)
                .await
                .map(|_| CommandOutput::Success)
                .map_err(|e| AppError::Command(format!("Cancel trade failed: {}", e))),
            TradeCommand::List => {
                let (pending, active, history) = self
                    .trades
                    .list_trades(10)
                    .await
                    .map_err(|e| AppError::Command(format!("List trades failed: {}", e)))?;

                Ok(CommandOutput::Data(serde_json::json!({
                    "pending": pending.len(),
                    "active": active.len(),
                    "history": history.len(),
                })))
            }
        }
    }

    /// Run config command
    async fn run_config_command(&self, cmd: ConfigCommand) -> Result<CommandOutput, AppError> {
        match cmd {
            ConfigCommand::Get { key } => {
                let value = self
                    .config
                    .get_value(&key)
                    .await
                    .map_err(|e| AppError::Command(format!("Get config failed: {}", e)))?;

                Ok(CommandOutput::Data(value))
            }
            ConfigCommand::Set { key, value } => self
                .config
                .set_value(&key, value)
                .await
                .map(|_| CommandOutput::Success)
                .map_err(|e| AppError::Command(format!("Set config failed: {}", e))),
            ConfigCommand::Reload => self
                .config
                .reload()
                .await
                .map(|_| CommandOutput::Success)
                .map_err(|e| AppError::Command(format!("Reload config failed: {}", e))),
            ConfigCommand::Path => {
                let path = self
                    .config
                    .get_config_path()
                    .await
                    .map_err(|e| AppError::Command(format!("Get config path failed: {}", e)))?;

                Ok(CommandOutput::Data(serde_json::json!({
                    "path": path.to_string_lossy().to_string(),
                })))
            }
        }
    }
}

/// App-level errors
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Config initialization failed: {0}")]
    ConfigInit(String),
    #[error("Keystore initialization failed: {0}")]
    KeystoreInit(String),
    #[error("Enclave initialization failed: {0}")]
    EnclaveInit(String),
    #[error("Client initialization failed: {0}")]
    ClientInit(String),
    #[error("Trades initialization failed: {0}")]
    TradesInit(String),
    #[error("MCP initialization failed: {0}")]
    McpInit(String),
    #[error("Alerts initialization failed: {0}")]
    AlertsInit(String),
    #[error("IPC initialization failed: {0}")]
    IpcInit(String),
    #[error("Command execution failed: {0}")]
    Command(String),
    #[error("MCP error: {0}")]
    Mcp(String),
    #[error("IPC error: {0}")]
    Ipc(String),
    #[error("Shutdown failed: {0}")]
    Shutdown(String),
}

/// CLI commands
#[derive(Debug, Clone)]
pub enum Command {
    /// Key management commands
    Key(KeyCommand),
    /// Wallet management commands
    Wallet(WalletCommand),
    /// Start MCP server
    Serve(TransportType),
    /// Subscribe to alerts
    Subscribe(SubscribeCommand),
    /// Trade commands
    Trade(TradeCommand),
    /// Configuration commands
    Config(ConfigCommand),
}

/// Key management subcommands
#[derive(Debug, Clone)]
pub enum KeyCommand {
    /// Unlock the keystore
    Unlock { password: String },
    /// Lock the keystore
    Lock,
    /// Change the keystore password
    ChangePassword { old_password: String, new_password: String },
}

/// Wallet management subcommands
#[derive(Debug, Clone)]
pub enum WalletCommand {
    /// List all wallets
    List,
    /// Create a new wallet
    Create { chain: String, name: String },
    /// Import a wallet from private key
    Import {
        chain: String,
        name: String,
        private_key: String,
    },
    /// Delete a wallet
    Delete { name: String },
}

/// Subscribe subcommands
#[derive(Debug, Clone)]
pub enum SubscribeCommand {
    /// Register a new subscription
    Register { procedure: String, delivery: DeliveryType },
    /// List all subscriptions
    List,
}

/// Delivery type for subscriptions
#[derive(Debug, Clone)]
pub enum DeliveryType {
    /// Webhook delivery
    Webhook { url: String, secret: Option<String> },
    /// Redis delivery
    Redis { url: String, channel: String },
    /// Telegram delivery
    Telegram { bot_token: String, chat_id: String },
}

/// Trade subcommands
#[derive(Debug, Clone)]
pub enum TradeCommand {
    /// Create a trade intent
    Create {
        wallet: String,
        chain: String,
        action: TradeActionInput,
    },
    /// Confirm a trade intent
    Confirm { intent_id: u64 },
    /// Get trade status
    Status { intent_id: u64 },
    /// Cancel a trade intent
    Cancel { intent_id: u64 },
    /// List all trades
    List,
}

/// Trade action input
#[derive(Debug, Clone)]
pub enum TradeActionInput {
    /// Swap tokens
    Swap {
        from_token: String,
        to_token: String,
        amount: String,
    },
    /// Transfer tokens
    Transfer { to: String, token: String, amount: String },
}

/// Config subcommands
#[derive(Debug, Clone)]
pub enum ConfigCommand {
    /// Get a config value
    Get { key: String },
    /// Set a config value
    Set { key: String, value: serde_json::Value },
    /// Reload config from disk
    Reload,
    /// Get config file path
    Path,
}

/// Command output
#[derive(Debug, Clone)]
pub enum CommandOutput {
    /// Command succeeded
    Success,
    /// Command failed with error message
    Error(String),
    /// Command returned data
    Data(serde_json::Value),
}

impl std::fmt::Display for CommandOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommandOutput::Success => write!(f, "Success"),
            CommandOutput::Error(e) => write!(f, "Error: {}", e),
            CommandOutput::Data(data) => write!(f, "{}", serde_json::to_string_pretty(data).unwrap_or_default()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_app_creation() {
        // This test would require mocking all the domain dependencies
        // For now we just verify the types compile correctly
        let _: Result<App, AppError> = Err(AppError::ConfigInit("test".to_string()));
    }

    #[test]
    fn test_app_error_display() {
        let err = AppError::ConfigInit("test error".to_string());
        assert!(err.to_string().contains("Config initialization failed"));
        assert!(err.to_string().contains("test error"));
    }

    #[test]
    fn test_command_variants() {
        let cmd = Command::Config(ConfigCommand::Path);
        assert!(matches!(cmd, Command::Config(ConfigCommand::Path)));

        let cmd = Command::Key(KeyCommand::Lock);
        assert!(matches!(cmd, Command::Key(KeyCommand::Lock)));
    }

    #[test]
    fn test_command_output_display() {
        let output = CommandOutput::Success;
        assert_eq!(output.to_string(), "Success");

        let output = CommandOutput::Error("test".to_string());
        assert!(output.to_string().contains("test"));
    }
}
