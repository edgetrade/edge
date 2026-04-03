//! Desktop key unlock command - keyring only.
//!
//! Checks if a key exists in the OS keyring.
//! No password prompts - the keyring itself provides the security.

use crate::messages;

/// Unlock keyring - informational message about the session backend.
pub fn keyring_unlock() -> crate::error::Result<()> {
    messages::success::keyring_unlock_help();
    Ok(())
}
