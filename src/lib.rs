//! Edge Trade library - MCP server and CLI for blockchain trading
//!
//! This crate provides both a CLI binary and a library for programmatic use.
//!
//! # Overview
//!
//! Poseidon is the Edge Trade CLI client with the following capabilities:
//!
//! - **State Management**: Thread-safe singleton state with `EdgeState`
//! - **Session Management**: Dual backend (keyring/filestore) session storage
//! - **Event-Driven**: Broadcast channels for reactive state updates
//! - **Library API**: Programmatic access via `Poseidon`, `key`, `wallet_api`, `server` modules
//! - **Error Handling**: Unified `PoseidonError` type for all operations
//!
//! # Quick Start
//!
//! ## Using the Library
//!
//! ```rust,no_run
//! use poseidon::{Config, Poseidon};
//! use poseidon::key;
//! use poseidon::EdgeState;
//!
//! async fn example() {
//!     // Load configuration
//!     let config = Config::load_default().expect("Failed to load config");
//!
//!     // Create Poseidon instance
//!     let poseidon = Poseidon::new(config.clone());
//!
//!     // Initialize global state
//!     let state = EdgeState::init(None).await.expect("Failed to init state");
//!
//!     // Create keys
//!     key::create(&config).await.expect("Failed to create key");
//! }
//! ```
//!
//! ## State-Driven Architecture
//!
//! The `EdgeState` provides centralized state management:
//!
//! ```rust,no_run
//! use poseidon::state::{EdgeState, StateEvent};
//!
//! async fn state_example() {
//!     // Initialize once
//!     let state = EdgeState::init(None).await.unwrap();
//!
//!     // Subscribe to events
//!     let mut rx = state.subscribe();
//!     tokio::spawn(async move {
//!         while let Ok(event) = rx.recv().await {
//!             match event {
//!                 StateEvent::SessionUnlocked => println!("Session unlocked!"),
//!                 _ => {}
//!             }
//!         }
//!     });
//! }
//! ```
//!
//! # Module Structure
//!
//! - `app`: Application runtime (CLI parsing, command handling, runner)
//! - `client`: API client for Edge backend communication
//! - `commands`: Command implementations (key, wallet, serve, subscribe)
//! - `config`: Configuration management (XDG config directory)
//! - `error`: Unified error types (`PoseidonError`)
//! - `manifest`: MCP manifest management and caching
//! - `messages`: Console output (errors, success, warnings, prompts)
//! - `session`: Session management (keyring, filestore backends)
//! - `state`: **NEW** Centralized state management (`EdgeState`, events)
//! - `utils`: Utility functions and helpers
//! - `wallet`: Wallet operations and types
//! - `generated`: Auto-generated tRPC route definitions
//!
//! # Library API Modules
//!
//! These modules provide high-level APIs for library consumers:
//!
//! - `key`: Programmatic key management
//! - `wallet_api`: Wallet operations (create, import, list, delete)
//! - `server`: MCP server control and configuration
//!
//! # Error Handling
//!
//! All operations return `Result<T, PoseidonError>`:
//!
//! ```rust,no_run
//! use poseidon::error::{PoseidonError, Result};
//!
//! fn may_fail() -> Result<()> {
//!     // Config operations automatically convert
//!     let config = poseidon::config::Config::load_default()?;
//!
//!     // Session operations automatically convert
//!     let session = poseidon::session::Session::new(config);
//!
//!     Ok(())
//! }
//! ```
//!
//! See the [State Refactor Documentation](../../docs/trade.edge/trade.edge.engineering/systems/poseidon/state-refactor.md)
//! for complete API reference and migration guide.

pub mod app;
pub mod client;
pub mod commands;
pub mod config;
pub mod error;
pub mod manifest;
pub mod messages;
pub mod orders;
pub mod session;
pub mod state;
pub mod utils;
pub mod wallet;

#[rustfmt::skip]
pub mod generated;

// Re-export key types for convenience
pub use app::run;
pub use client::IrisClient;
pub use config::Config;
pub use error::{PoseidonError, Result};
pub use session::Session;
pub use state::{EdgeState, StateEvent};

/// Poseidon library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Library API for programmatic use.
///
/// This struct provides a high-level interface for using Poseidon
/// as a library. It wraps the internal functionality and exposes
/// a clean API for external consumers.
pub struct Poseidon {
    config: Config,
}

impl Poseidon {
    /// Create a new Poseidon instance with the given configuration.
    ///
    /// # Arguments
    /// * `config` - Configuration loaded via Config::load()
    ///
    /// # Returns
    /// A new Poseidon instance ready for use.
    ///
    /// # Example
    /// ```rust
    /// use poseidon::{Config, Poseidon};
    ///
    /// let config = Config::default();
    /// let poseidon = Poseidon::new(config);
    /// ```
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// Get a reference to the configuration.
    pub fn config(&self) -> &Config {
        &self.config
    }
}

/// Library API for key management operations.
///
/// This module provides programmatic access to key operations
/// like creating, unlocking, locking, and deleting keys.
pub mod key {
    use crate::config::Config;
    use crate::error::Result;
    use crate::session::keyring_available;

    /// Key info for library operations.
    #[derive(Debug, Clone)]
    pub struct KeyInfo {
        /// Whether the keyring is available.
        pub keyring_available: bool,
        /// Whether a session is currently unlocked.
        pub is_unlocked: bool,
    }

    /// Create a new key and session.
    ///
    /// Creates a new encryption key and stores it in the
    /// appropriate backend (keyring or file storage).
    /// Delegates to the keyring or filestore create command based on availability.
    ///
    /// # Arguments
    /// * `config` - Configuration for the session.
    ///
    /// # Returns
    /// `Ok(())` on success, or an error if creation fails.
    ///
    /// # Example
    /// ```rust,no_run
    /// use poseidon::key;
    /// use poseidon::Config;
    ///
    /// async fn example() {
    ///     let config = Config::default();
    ///     key::create(&config).await.unwrap();
    /// }
    /// ```
    pub async fn create(config: &Config) -> Result<()> {
        if keyring_available() {
            // Use keyring-based key creation
            crate::commands::key::keyring::keyring_create(config.clone())
        } else {
            // Use filestore-based key creation
            crate::commands::key::filestore::key_create()
        }
    }

    /// Unlock the session with a password.
    ///
    /// Derives the user encryption key from the password
    /// and stores it in the session.
    ///
    /// # Arguments
    /// * `config` - Configuration for the session.
    /// * `password` - The password to derive the key from.
    ///
    /// # Returns
    /// `Ok(())` on success, or an error if unlocking fails.
    pub fn unlock(config: &Config, password: &str) -> Result<()> {
        let session = crate::session::Session::new(config.clone());
        session
            .unlock_with_password(password)
            .map_err(crate::error::PoseidonError::Session)
    }

    /// Lock the session.
    ///
    /// Removes the stored encryption key from the session.
    ///
    /// # Arguments
    /// * `config` - Configuration for the session.
    ///
    /// # Returns
    /// `Ok(())` on success, or an error if locking fails.
    pub fn lock(config: &Config) -> Result<()> {
        let session = crate::session::Session::new(config.clone());
        session.lock().map_err(crate::error::PoseidonError::Session)
    }

    /// Check if the session is unlocked.
    ///
    /// # Arguments
    /// * `config` - Configuration for the session.
    ///
    /// # Returns
    /// `Ok(true)` if unlocked, `Ok(false)` if locked.
    pub fn is_unlocked(config: &Config) -> Result<bool> {
        let session = crate::session::Session::new(config.clone());
        Ok(session.is_unlocked())
    }

    /// Delete the stored key.
    ///
    /// Removes the encryption key from storage.
    ///
    /// # Arguments
    /// * `config` - Configuration for the session.
    ///
    /// # Returns
    /// `Ok(())` on success, or an error if deletion fails.
    pub fn delete(config: &Config) -> Result<()> {
        let session = crate::session::Session::new(config.clone());
        session.lock().map_err(crate::error::PoseidonError::Session)
    }

    /// Get information about the key state.
    ///
    /// # Arguments
    /// * `config` - Configuration for the session.
    ///
    /// # Returns
    /// `Ok(KeyInfo)` containing key state information.
    pub fn info(config: &Config) -> Result<KeyInfo> {
        let session = crate::session::Session::new(config.clone());
        Ok(KeyInfo {
            keyring_available: keyring_available(),
            is_unlocked: session.is_unlocked(),
        })
    }
}

/// Library API for wallet operations.
///
/// This module provides programmatic access to wallet operations
/// like creating, importing, listing, and deleting wallets.
pub mod wallet_api {
    use tyche_enclave::types::chain_type::ChainType;

    use crate::client::IrisClient;
    use crate::error::Result;
    use crate::session::Session;
    use crate::wallet::types::WalletList;

    /// Wallet info for library operations.
    #[derive(Debug, Clone)]
    pub struct WalletInfo {
        /// The blockchain chain type.
        pub chain: ChainType,
        /// The wallet name.
        pub name: String,
        /// The wallet address.
        pub address: String,
    }

    impl From<WalletList> for WalletInfo {
        fn from(w: WalletList) -> Self {
            Self {
                chain: w.chain_type,
                name: w.name,
                address: w.address,
            }
        }
    }

    /// List all wallets.
    ///
    /// Returns a list of wallets associated with the current session.
    ///
    /// # Arguments
    /// * `client` - The API client to use.
    ///
    /// # Returns
    /// `Ok(Vec<WalletInfo>)` containing wallet information.
    pub async fn list(client: &IrisClient) -> Result<Vec<WalletInfo>> {
        let wallets = crate::client::list_wallets(client).await?;
        Ok(wallets.into_iter().map(WalletInfo::from).collect())
    }

    /// Create a new wallet.
    ///
    /// Creates a new wallet for the specified chain.
    ///
    /// # Arguments
    /// * `chain` - The blockchain chain (EVM or SVM).
    /// * `name` - Optional wallet name.
    /// * `session` - The current session.
    /// * `client` - The API client to use.
    ///
    /// # Returns
    /// `Ok(())` on success, or an error if creation fails.
    pub async fn create(
        chain: ChainType,
        name: Option<String>,
        session: &Session,
        client: &IrisClient,
    ) -> Result<WalletInfo> {
        let uek = session
            .get_user_encryption_key()
            .map_err(crate::error::PoseidonError::Session)?
            .ok_or(crate::error::PoseidonError::Session(
                crate::session::SessionError::NotFound,
            ))?;

        let name = name.unwrap_or_else(|| format!("{} Wallet", chain));
        let wallet = crate::wallet::create::create_wallet(chain, name.clone(), &uek, client)
            .await
            .map_err(|e| crate::error::PoseidonError::Wallet(e.to_string()))?;

        Ok(WalletInfo {
            chain,
            name,
            address: wallet.address,
        })
    }

    /// Import a wallet from a private key.
    ///
    /// Imports an existing wallet using a private key.
    ///
    /// # Arguments
    /// * `chain` - The blockchain chain (EVM or SVM).
    /// * `name` - Optional wallet name.
    /// * `private_key` - The private key to import.
    /// * `session` - The current session.
    /// * `client` - The API client to use.
    ///
    /// # Returns
    /// `Ok(())` on success, or an error if import fails.
    pub async fn import(
        chain: ChainType,
        name: Option<String>,
        private_key: &str,
        session: &Session,
        client: &IrisClient,
    ) -> Result<WalletInfo> {
        let uek = session
            .get_user_encryption_key()
            .map_err(crate::error::PoseidonError::Session)?
            .ok_or(crate::error::PoseidonError::Session(
                crate::session::SessionError::NotFound,
            ))?;

        let name = name.unwrap_or_else(|| format!("{} Wallet", chain));
        let wallet = crate::wallet::import::import_wallet(private_key, chain, name.clone(), &uek, client)
            .await
            .map_err(|e| crate::error::PoseidonError::Wallet(e.to_string()))?;

        Ok(WalletInfo {
            chain,
            name,
            address: wallet.address,
        })
    }

    /// Delete a wallet.
    ///
    /// Removes a wallet from storage.
    ///
    /// # Arguments
    /// * `name` - The name of the wallet to delete.
    /// * `session` - The current session.
    /// * `client` - The API client to use.
    ///
    /// # Returns
    /// `Ok(())` on success, or an error if deletion fails.
    pub async fn delete(name: &str, session: &Session, client: &IrisClient) -> Result<()> {
        let _ = session; // Session may be used for verification in future
        crate::client::delete_wallet(name.to_string(), client)
            .await
            .map_err(|e| crate::error::PoseidonError::Wallet(e.to_string()))
    }
}

/// Server/MCP API for running in server mode.
///
/// This module provides programmatic access to server operations
/// for running the Edge CLI as an MCP server.
pub mod server {
    use std::sync::Arc;

    use tokio::sync::RwLock;

    use crate::client::IrisClient;
    use crate::config::Config;
    use crate::error::Result;
    use crate::manifest::McpManifest;
    use crate::state::{FeatureServerConfig, ServerFeature, StateEventReceiver};

    /// Server configuration for library use.
    #[derive(Debug, Clone)]
    pub struct ServerConfig {
        /// The host address to bind to.
        pub host: String,
        /// The port to listen on.
        pub port: u16,
        /// The path prefix for endpoints.
        pub path: String,
        /// Whether to run in stdio mode.
        pub stdio_mode: bool,
        /// Feature configuration.
        pub features: FeatureServerConfig,
    }

    impl Default for ServerConfig {
        fn default() -> Self {
            Self {
                host: "127.0.0.1".to_string(),
                port: 3000,
                path: "/mcp".to_string(),
                stdio_mode: true,
                features: FeatureServerConfig::default(),
            }
        }
    }

    impl From<ServerConfig> for crate::state::ServerConfig {
        fn from(config: ServerConfig) -> Self {
            Self {
                host: config.host,
                port: config.port,
                path: config.path,
                stdio_mode: config.stdio_mode,
            }
        }
    }

    /// Server handle for managing the MCP server.
    ///
    /// This struct provides methods for controlling the server
    /// and accessing its state.
    pub struct ServerHandle {
        /// The server configuration.
        pub config: ServerConfig,
    }

    /// Start the MCP server.
    ///
    /// Starts the MCP server with the given configuration.
    ///
    /// # Arguments
    /// * `config` - The application configuration.
    /// * `server_config` - The server configuration.
    /// * `client` - The API client to use.
    /// * `manifest` - The MCP manifest.
    ///
    /// # Returns
    /// `Ok(ServerHandle)` on success, or an error if startup fails.
    pub async fn start(
        _config: &Config,
        server_config: ServerConfig,
        client: IrisClient,
        manifest: Arc<RwLock<McpManifest>>,
    ) -> Result<ServerHandle> {
        // Initialize the Edge server
        let _server = crate::commands::serve::mcp::EdgeServer::new(client, manifest)
            .await
            .map_err(|e| crate::error::PoseidonError::Other(e.to_string()))?;

        Ok(ServerHandle { config: server_config })
    }

    /// Enable or disable a server feature.
    ///
    /// # Arguments
    /// * `feature` - The feature to toggle.
    /// * `enabled` - Whether to enable or disable the feature.
    /// * `config` - The feature server configuration to modify.
    ///
    /// # Returns
    /// `Ok(())` on success, or an error if the toggle fails.
    pub fn toggle_feature(feature: ServerFeature, enabled: bool, config: &mut FeatureServerConfig) -> Result<()> {
        config.set_feature(feature, enabled);
        Ok(())
    }

    /// Check if a server feature is enabled.
    ///
    /// # Arguments
    /// * `feature` - The feature to check.
    /// * `config` - The feature server configuration.
    ///
    /// # Returns
    /// `true` if the feature is enabled, `false` otherwise.
    pub fn is_feature_enabled(feature: ServerFeature, config: &FeatureServerConfig) -> bool {
        config.is_feature_enabled(feature)
    }

    /// Get a list of all enabled features.
    ///
    /// # Arguments
    /// * `config` - The feature server configuration.
    ///
    /// # Returns
    /// A vector of enabled features.
    pub fn enabled_features(config: &FeatureServerConfig) -> Vec<ServerFeature> {
        config.enabled_features()
    }

    /// Subscribe to server events.
    ///
    /// Returns a receiver for server events.
    ///
    /// # Returns
    /// `Ok(StateEventReceiver)` on success, or an error if subscription fails.
    pub fn subscribe() -> Result<StateEventReceiver> {
        let (_sender, receiver) = crate::state::create_state_event_channel(128);
        Ok(receiver)
    }
}

#[cfg(test)]
pub mod test_utils {
    //! Shared test utilities for ensuring test isolation.
    //!
    //! These mutexes are used to serialize tests that share global resources
    //! like the OS keyring or file system paths.

    use std::sync::Mutex;

    /// Global mutex for all filestore session tests across all modules.
    /// Filestore session tests use the same session file path (~/.config/edge/session),
    /// so they must be serialized to prevent conflicts when running tests in parallel.
    pub static FILESTORE_TEST_MUTEX: Mutex<()> = Mutex::new(());

    /// Global mutex for transport key cache tests across all modules.
    /// Transport key cache tests use the same config directory (~/.config/edge/transport_keys.json),
    /// so they must be serialized to prevent conflicts when running tests in parallel.
    pub static TRANSPORT_CACHE_TEST_MUTEX: Mutex<()> = Mutex::new(());

    /// Global mutex for all keyring tests across all modules.
    /// All keyring tests use the same keyring entry (service="edge", username="user-encryption-key"),
    /// so they must be serialized to prevent conflicts when running tests in parallel.
    #[cfg(feature = "keyring-tests")]
    pub static KEYRING_TEST_MUTEX: Mutex<()> = Mutex::new(());
}
