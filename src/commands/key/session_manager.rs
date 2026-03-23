//! Session manager module for ensuring session readiness.
//!
//! Provides a unified interface for ensuring a session is ready (unlocked)
//! for wallet operations. Automatically handles keyring vs filestore backends.

use crate::commands::key::filestore::storage::{default_blind_user_key_path, default_salt_path};
use crate::messages;
use crate::session::{Session, keyring_available};

/// Ensure the session is ready (unlocked) for wallet operations.
///
/// This function provides a unified interface that:
/// 1. Checks if the session is already unlocked
/// 2. If not unlocked, determines the appropriate backend (keyring or filestore)
/// 3. For keyring: creates a new key if none exists
/// 4. For filestore: unlocks if keys exist, creates if not
///
/// # Arguments
/// * `context` - The context for messaging ("wallet" for wallet-specific messages)
///
/// # Returns
/// `Ok(())` when the session is ready (unlocked)
///
/// # Errors
/// Returns an error if:
/// - Key creation fails
/// - Key unlock fails
/// - Storage operations fail
pub fn ensure_session_ready(context: &str) -> messages::success::CommandResult<()> {
    let session = Session::new();

    // If already unlocked, nothing to do
    if session.is_unlocked() {
        return Ok(());
    }

    // Check if keyring is available
    if keyring_available() {
        // Keyring available - create key (keyring_create_with_context handles "already exists")
        crate::commands::key::keyring::create::keyring_create_with_context(context)?;
    } else {
        // Filestore fallback
        if check_filestore_keys_exist()? {
            // Keys exist - unlock them
            crate::commands::key::filestore::unlock::key_unlock_with_context(context)?;
        } else {
            // No keys - create them
            crate::commands::key::filestore::create::key_create_with_context(context)?;
        }
    }

    Ok(())
}

/// Check if filestore keys exist on the filesystem.
///
/// Returns `true` if both blind_user_key and salt files exist.
///
/// # Errors
/// Returns an error if the storage paths cannot be determined.
fn check_filestore_keys_exist() -> messages::success::CommandResult<bool> {
    let blind_key_path = default_blind_user_key_path()
        .ok_or_else(|| messages::error::CommandError::Storage("Could not determine key path".to_string()))?;
    let salt_path = default_salt_path()
        .ok_or_else(|| messages::error::CommandError::Storage("Could not determine salt path".to_string()))?;

    Ok(blind_key_path.exists() && salt_path.exists())
}
