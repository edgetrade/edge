//! Unified main entry point for Edge CLI.
//!
//! This binary combines both keyring-based and file-based session storage
//! into a single executable. The session backend is selected based on
//! configuration, or auto-detected on first run.

use std::future::Future;
use std::path::PathBuf;
use std::pin::Pin;

use clap::Parser;
use poseidon::app::cli::Cli;
use poseidon::app::runner::run;
use poseidon::app::{KeyCreateFn, KeyDeleteFn, KeyLockFn, KeyUnlockFn, KeyUpdateFn};
use poseidon::client::IrisClient;
use poseidon::config::{set_config_path_override, should_use_keyring};
use poseidon::messages;
use poseidon::messages::success::CommandResult;

/// Keyring-based key command implementations (preferred when available).
mod keyring {
    pub use poseidon::commands::key::keyring::{
        keyring_create as key_create, keyring_delete as key_delete, keyring_lock as key_lock,
        keyring_unlock as key_unlock, keyring_update as key_update,
    };
}

/// File-based key command implementations (fallback when keyring unavailable).
mod filestore {
    pub use poseidon::commands::key::filestore::{key_create, key_delete, key_lock, key_unlock, key_update};
}

/// Key update function wrapper for keyring variant.
fn keyring_update_wrapper(client: &IrisClient) -> Pin<Box<dyn Future<Output = CommandResult<()>> + '_>> {
    Box::pin(keyring::key_update(client))
}

/// Key update function wrapper for filestore variant.
fn filestore_update_wrapper(client: &IrisClient) -> Pin<Box<dyn Future<Output = CommandResult<()>> + '_>> {
    Box::pin(filestore::key_update(client))
}

/// Select key command implementations based on configuration.
///
/// Checks the config file for the session.use_keyring setting.
/// If not set, probes the OS keyring once and saves the result to config.
/// Users can override by editing the config file directly.
fn select_key_commands() -> (KeyCreateFn, KeyUnlockFn, KeyLockFn, KeyUpdateFn, KeyDeleteFn) {
    if should_use_keyring() {
        (
            keyring::key_create,
            keyring::key_unlock,
            keyring::key_lock,
            Box::new(keyring_update_wrapper),
            keyring::key_delete,
        )
    } else {
        messages::warning::file_storage_fallback();
        (
            filestore::key_create,
            filestore::key_unlock,
            filestore::key_lock,
            Box::new(filestore_update_wrapper),
            filestore::key_delete,
        )
    }
}

#[tokio::main]
async fn main() {
    // Parse CLI arguments early to get config path override
    let cli = Cli::parse();

    // Set config path override if provided via --config or EDGE_CONFIG
    if let Some(config_path) = cli.config {
        set_config_path_override(Some(PathBuf::from(config_path)));
    }

    // Select the appropriate key command implementations based on config
    let (key_create, key_unlock, key_lock, key_update, key_delete): (
        KeyCreateFn,
        KeyUnlockFn,
        KeyLockFn,
        KeyUpdateFn,
        KeyDeleteFn,
    ) = select_key_commands();

    // Run the application with the selected key commands
    run(key_create, key_unlock, key_lock, key_update, key_delete).await;
}
