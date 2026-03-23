//! Shared application logic for Edge Trade binaries.
//!
//! This module provides the main application loop that is shared between
//! the desktop and server binaries. The key command implementations differ
//! based on compile-time feature flags.

use std::process;
use std::sync::Arc;

use clap::Parser;
use colored_json::to_colored_json_auto;
use tokio::sync::RwLock;

use crate::app::client::parse_api_credentials;
use crate::app::handler::{handle_key, handle_ping, handle_server, handle_skill, handle_version, handle_wallet};
use crate::app::{KeyCreateFn, KeyDeleteFn, KeyLockFn, KeyUnlockFn, KeyUpdateFn};
use crate::client::new_client;
use crate::commands::serve::mcp::EdgeServer;
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
    pub fn new() -> Self {
        let session = if keyring_available() {
            Session::Keyring(crate::session::KeyringSession::new())
        } else {
            messages::warning::keyring_unavailable();
            Session::File(crate::session::FileStoreSession::new())
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
    pub fn new_with_keyring() -> Self {
        Self {
            session: Session::Keyring(crate::session::KeyringSession::new()),
            manifest_manager: None,
        }
    }

    /// Create a new App explicitly using the file storage backend.
    pub fn new_with_file() -> Self {
        Self {
            session: Session::File(crate::session::FileStoreSession::new()),
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
        Self::new()
    }
}

/// Main application entry point.
///
/// Accepts key command implementations based on the compile-time feature flag.
/// Desktop and server binaries call this with their respective key commands.
pub async fn run(
    key_create: KeyCreateFn,
    key_unlock: KeyUnlockFn,
    key_lock: KeyLockFn,
    key_update: KeyUpdateFn,
    key_delete: KeyDeleteFn,
) {
    // Create the App instance with session management
    let mut app = App::new();
    let cli = Cli::parse();

    // ------------------------------------------------------------------------
    //
    // Commands that do not require the API client
    //
    // ------------------------------------------------------------------------
    // Note: Key commands still need the client for update operations
    // (e.g., rotating keys on the server), so we initialize it early

    if matches!(cli.command, Some(Commands::Ping)) {
        handle_ping(cli.verbose).await;
        return;
    }

    if matches!(cli.command, Some(Commands::Version)) {
        handle_version();
        return;
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
    .unwrap_or_else(|e| {
        messages::error::connection_failed(&e.to_string());
        process::exit(1);
    });

    if let Some(Commands::Key { command }) = &cli.command {
        if let Err(code) = handle_key(
            command,
            key_create,
            key_unlock,
            key_lock,
            key_update,
            key_delete,
            &api_client,
        )
        .await
        {
            process::exit(code);
        }
        return;
    }

    if let Some(Commands::Wallet { command }) = &cli.command {
        if let Err(code) = handle_wallet(command, &api_client).await {
            process::exit(code);
        }
        return;
    }

    // ------------------------------------------------------------------------
    //
    // Commands that require the API client + updated manifest
    //
    // ------------------------------------------------------------------------
    // Initialize manifest through App - this couples session and manifest lifecycle
    if let Err(e) = app
        .init_manifest(
            client_credentials.iris_url,
            client_credentials.api_key,
            true, // enable background refresh
        )
        .await
    {
        messages::error::manifest_load_error(&e.to_string());
        process::exit(1);
    }

    let shared_manifest = app.manifest().expect("Manifest should be initialized");

    if matches!(cli.command, Some(Commands::ListTools)) {
        let manifest = shared_manifest.read().await;
        messages::success::json_output(&to_colored_json_auto(&manifest.tools).unwrap());
        return;
    }

    if let Some(Commands::Skill { command: cmd }) = &cli.command {
        let manifest = shared_manifest.read().await;
        if let Err(code) = handle_skill(cmd, &manifest) {
            process::exit(code);
        }
        return;
    }

    // ------------------------------------------------------------------------
    //
    // Commands that require the Server
    //
    // ------------------------------------------------------------------------
    let server = EdgeServer::new(api_client, shared_manifest.clone())
        .await
        .map_err(|e| {
            messages::error::iris_connection_failed(&e.to_string());
            1
        });

    if let Err(code) = handle_server(&cli, server.unwrap()).await {
        process::exit(code);
    }
}
