//! Desktop key update command - keyring only.
//!
//! Generates a new user encryption key and replaces the existing one
//! in the OS keyring. No password prompts, no file storage.

use crate::domains::client::{IrisClient, rotate_user_encryption_key};
use crate::domains::config::Config;
use crate::domains::keystore::KeyringSession as Session;
use crate::domains::keystore::keyring::keyring_create;
use crate::error::PoseidonError;
use crate::messages;

// TODO: trigger the rotate key operation in tyche

/// Update the key by generating a new one.
///
/// This function:
/// 1. Checks if a key exists in the keyring
/// 2. Generates a new random 32-byte UserEncryptionKey
/// 3. Replaces the existing key in the OS keyring
/// 4. Prints success message
///
/// # Errors
/// Returns an error if:
/// - No existing key exists (must create first)
/// - Key generation fails
/// - Keyring is inaccessible
pub async fn keyring_update(config: Config, client: &IrisClient) -> crate::error::Result<()> {
    let session = Session::new(config.clone());

    // Check if key exists first
    if !session.is_unlocked() {
        return Err(PoseidonError::Session(crate::domains::keystore::SessionError::Keyring(
            "No key found. Run 'edge key create' first.".to_string(),
        )));
    }

    let old = session.get_user_encryption_key().unwrap();
    if old.is_none() {
        return Err(PoseidonError::Session(crate::domains::keystore::SessionError::Keyring(
            "No key found. Run 'edge key create' first.".to_string(),
        )));
    }

    let old_uek = old.unwrap();

    keyring_create(config)?;

    let new = session.get_user_encryption_key().unwrap();
    if new.is_none() {
        return Err(PoseidonError::Session(crate::domains::keystore::SessionError::Keyring(
            "No key found. Run 'edge key create' first.".to_string(),
        )));
    }

    let new_uek = new.unwrap();
    rotate_user_encryption_key(&new_uek, &old_uek, client)
        .await
        .map_err(|e| PoseidonError::Session(crate::domains::keystore::SessionError::Keyring(e.to_string())))?;

    messages::success::key_updated();
    Ok(())
}
