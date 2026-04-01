//! Shared application logic for Edge Trade binaries.
//!
//! This module provides the main application loop that is shared between
//! the desktop and server binaries. The key command implementations differ
//! based on compile-time feature flags.

use std::path::PathBuf;
use std::sync::Arc;

use clap::{CommandFactory, Parser};
use colored_json::to_colored_json_auto;
use tokio::sync::RwLock;

use crate::app::client::parse_api_credentials;
use crate::app::handler::{
    KeyCommandArgs, handle_key, handle_order, handle_ping, handle_skill, handle_version, handle_wallet, serve,
};
use crate::client::new_client;
use crate::commands::serve::mcp::EdgeServer;
use crate::config::Config;
use crate::error::PoseidonError;
use crate::manifest::{ManifestManager, McpManifest};
use crate::messages;
use crate::session::crypto::UsersEncryptionKeys;
use crate::session::{Session, SessionError, keyring_available};

use super::cli::{Cli, Commands};

/// Main application struct with unified session and manifest management.
///
/// The `App` struct provides a high-level interface for the Edge CLI
/// that automatically handles session backend selection (keyring vs file storage)
/// and manifest lifecycle management.
#[derive(Debug)]
pub struct App {
    /// The session backend (keyring or file storage)
    session: Session,
    /// The manifest manager (lazy-initialized on first access)
    manifest_manager: Option<ManifestManager>,
}

impl App {
    /// Create a new App instance with automatic session backend selection.
    ///
    /// This constructor:
    /// 1. Probes the OS keyring to check availability
    /// 2. Uses `KeyringSession` if the keyring is available
    /// 3. Falls back to `FileStoreSession` with a warning if keyring fails
    pub fn new(config: Config) -> Self {
        let session = if keyring_available() {
            Session::Keyring(crate::session::KeyringSession::new(config))
        } else {
            messages::warning::keyring_unavailable();
            Session::File(crate::session::FileStoreSession::new(config))
        };
        Self {
            session,
            manifest_manager: None,
        }
    }

    /// Create a new App explicitly using the keyring backend.
    ///
    /// # Panics
    /// Panics if the keyring is not available.
    pub fn new_with_keyring(config: Config) -> Self {
        Self {
            session: Session::Keyring(crate::session::KeyringSession::new(config)),
            manifest_manager: None,
        }
    }

    /// Create a new App explicitly using the file storage backend.
    pub fn new_with_file(config: Config) -> Self {
        Self {
            session: Session::File(crate::session::FileStoreSession::new(config)),
            manifest_manager: None,
        }
    }

    /// Check if the session is unlocked.
    pub fn is_unlocked(&self) -> bool {
        self.session.is_unlocked()
    }

    /// Unlock the session with the provided password.
    pub fn unlock(&mut self, uek: &UsersEncryptionKeys) -> Result<(), SessionError> {
        self.session.unlock(uek)
    }

    /// Lock the session.
    pub fn lock(&mut self) -> Result<(), SessionError> {
        self.session.lock()
    }

    /// Initialize the manifest manager.
    ///
    /// This fetches or loads the manifest and optionally starts background refresh.
    pub async fn init_manifest(
        &mut self,
        url_input: String,
        api_key: String,
        refresh: bool,
    ) -> Result<(), crate::manifest::ManifestError> {
        let url = format!("{}/mcp/manifest", url_input);
        let manager = ManifestManager::new(url, api_key, refresh).await?;
        self.manifest_manager = Some(manager);
        Ok(())
    }

    /// Get the manifest if initialized.
    pub fn manifest(&self) -> Option<Arc<RwLock<McpManifest>>> {
        self.manifest_manager.as_ref().map(|m| m.manifest())
    }

    /// Get the manifest manager if initialized.
    pub fn manifest_manager(&self) -> Option<&ManifestManager> {
        self.manifest_manager.as_ref()
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new(Config::default())
    }
}

/// Main application entry point.
pub async fn run() -> Result<(), PoseidonError> {
    // Create the App instance with session management
    let cli = Cli::parse();
    let config_path = Some(PathBuf::from(&cli.config));
    let config = Config::load(config_path).unwrap_or_default();
    let mut app = App::new(config.clone());

    // ------------------------------------------------------------------------
    //
    // Commands that do not require the API client
    //
    // ------------------------------------------------------------------------
    // Note: Key commands still need the client for update operations
    // (e.g., rotating keys on the server), so we initialize it early

    if matches!(cli.command, Some(Commands::Ping)) {
        return handle_ping(cli.verbose).await;
    }

    if matches!(cli.command, Some(Commands::Version)) {
        return handle_version();
    }

    // ------------------------------------------------------------------------
    //
    // Commands that require the API client
    //
    // ------------------------------------------------------------------------
    let client_credentials = parse_api_credentials(&cli).await;
    let api_client = new_client(
        client_credentials.clone().iris_url,
        client_credentials.clone().api_key,
        client_credentials.verbose,
    )
    .await
    .map_err(PoseidonError::Client)?;

    if let Some(Commands::Order { command }) = &cli.command {
        return handle_order(command, &app.session, &api_client).await;
    }

    if let Some(Commands::Key { command }) = &cli.command {
        return handle_key(KeyCommandArgs {
            command: Some(command.clone().unwrap()),
            config: config.clone(),
            client: api_client.clone(),
            session: app.session.clone(),
        })
        .await;
    }

    if let Some(Commands::Wallet { command }) = &cli.command {
        return handle_wallet(command, &app.session, &api_client).await;
    }

    // ------------------------------------------------------------------------
    //
    // Commands that require the API client + updated manifest
    //
    // ------------------------------------------------------------------------
    // Initialize manifest through App - this couples session and manifest lifecycle
    app.init_manifest(
        client_credentials.iris_url,
        client_credentials.api_key,
        true, // enable background refresh
    )
    .await
    .map_err(|e| PoseidonError::Manifest(e.to_string()))?;

    let shared_manifest = app
        .manifest()
        .ok_or_else(|| PoseidonError::Manifest("Manifest should be initialized".to_string()))?;

    if matches!(cli.command, Some(Commands::ListTools)) {
        let manifest = shared_manifest.read().await;
        let json = to_colored_json_auto(&manifest.tools).map_err(|e| PoseidonError::Serialization(e.to_string()))?;
        messages::success::json_output(&json);
        return Ok(());
    }

    if let Some(Commands::Skill { command: cmd }) = &cli.command {
        let manifest = shared_manifest.read().await;
        return handle_skill(cmd, &manifest);
    }

    // ------------------------------------------------------------------------
    //
    // Commands that require the Server
    //
    // ------------------------------------------------------------------------
    if let Some(Commands::Serve { command: _, args }) = &cli.command {
        let server = EdgeServer::new(api_client, shared_manifest.clone())
            .await
            .map_err(|e: Box<dyn std::error::Error>| PoseidonError::Other(e.to_string()))?;

        return serve(args, server).await;
    }

    // ------------------------------------------------------------------------
    //
    // No command matched - display help
    //
    // ------------------------------------------------------------------------
    Cli::command().print_long_help()?;
    Ok(())
}
