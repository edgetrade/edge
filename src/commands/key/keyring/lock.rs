//! Desktop key lock command - keyring only.
//!
//! Removes the user encryption key from the OS keyring.
//! No password prompts - the keyring itself provides the security.

use crate::messages;

/// Lock keyring - informational message about the session backend.
pub fn keyring_lock() -> messages::success::CommandResult<()> {
    messages::success::keyring_lock_help();
    Ok(())
}
