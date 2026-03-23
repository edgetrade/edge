//! Session management module for Edge CLI.
//!
//! This module provides session storage implementations:
//! - **keyring**: OS keyring storage (preferred)
//! - **filestore**: File-based fallback when keyring unavailable
//!
//! The unified `Session` enum automatically selects keyring if available,
//! otherwise falls back to file-based storage with a warning.

pub mod crypto;
pub mod filestore;
pub mod keyring;
pub mod transport;

// Re-export both backends
pub use filestore::{Session as FileStoreSession, SessionError as FileStoreError};
pub use keyring::{Session as KeyringSession, SessionError as KeyringError};

use crate::messages;
use crypto::UsersEncryptionKeys;

/// Unified session error type.
#[derive(Debug, Clone, thiserror::Error)]
pub enum SessionError {
    #[error("Keyring error: {0}")]
    Keyring(String),
    #[error("File storage error: {0}")]
    FileStore(String),
    #[error("Session not found")]
    NotFound,
    #[error("Session corrupted")]
    Corrupted,
}

impl From<KeyringError> for SessionError {
    fn from(e: KeyringError) -> Self {
        SessionError::Keyring(e.to_string())
    }
}

impl From<FileStoreError> for SessionError {
    fn from(e: FileStoreError) -> Self {
        match e {
            FileStoreError::Storage(msg) => SessionError::FileStore(msg),
            FileStoreError::Encoding(msg) => SessionError::FileStore(msg),
            FileStoreError::Corrupted => SessionError::Corrupted,
        }
    }
}

/// Unified session backend that automatically selects keyring or file storage.
///
/// This enum provides a single interface for session management that:
/// 1. Attempts to use the OS keyring first (preferred for security)
/// 2. Falls back to file-based storage if keyring is unavailable
///
/// Both backends provide the same operations: unlock, lock, get_user_encryption_key, is_unlocked.
#[derive(Debug)]
pub enum Session {
    /// OS keyring backend (preferred)
    Keyring(KeyringSession),
    /// File-based fallback backend
    File(FileStoreSession),
}

/// Check if the OS keyring is available by attempting a probe operation.
///
/// This function tries to access the keyring to determine if it's functional.
/// It performs a lightweight check that won't affect existing stored data.
///
/// # Returns
/// `true` if the keyring is accessible, `false` otherwise.
pub fn keyring_available() -> bool {
    use keyring::Entry;

    // Try to create an entry - this tests if the keyring service is available
    match Entry::new("edge-probe", "probe") {
        Ok(entry) => {
            // Try to get password to verify keyring is functional
            // We don't care about the result, just that it doesn't panic or hang
            let _: Result<String, _> = entry.get_password();
            true
        }
        Err(_) => false,
    }
}

impl Session {
    /// Create a new session with automatic backend selection.
    ///
    /// This constructor probes the OS keyring and:
    /// - Returns `Session::Keyring` if the keyring is available
    /// - Returns `Session::File` if the keyring is unavailable (prints warning)
    ///
    /// # Example
    /// ```rust
    /// use poseidon::session::Session;
    ///
    /// let session = Session::new();
    /// if session.is_unlocked() {
    ///     println!("Session is active");
    /// }
    /// ```
    pub fn new() -> Self {
        if keyring_available() {
            Session::Keyring(KeyringSession::new())
        } else {
            messages::warning::keyring_unavailable();
            Session::File(FileStoreSession::new())
        }
    }

    /// Create a new session explicitly using the keyring backend.
    ///
    /// # Panics
    /// Panics if the keyring is not available. Use `Session::new()` for automatic fallback.
    pub fn new_keyring() -> Self {
        Session::Keyring(KeyringSession::new())
    }

    /// Create a new session explicitly using the file storage backend.
    pub fn new_file() -> Self {
        Session::File(FileStoreSession::new())
    }

    /// Store the user encryption key (unlock the session).
    ///
    /// # Arguments
    /// * `uek` - The user encryption key to store
    pub fn unlock(&self, uek: &UsersEncryptionKeys) -> Result<(), SessionError> {
        match self {
            Session::Keyring(s) => s.unlock(uek).map_err(|e| e.into()),
            Session::File(s) => s.unlock(uek).map_err(|e| e.into()),
        }
    }

    /// Lock the session by deleting the stored key.
    ///
    /// This operation is idempotent - it succeeds even if no session exists.
    pub fn lock(&self) -> Result<(), SessionError> {
        match self {
            Session::Keyring(s) => s.lock().map_err(|e| e.into()),
            Session::File(s) => s.lock().map_err(|e| e.into()),
        }
    }

    /// Get the user encryption key from the session.
    ///
    /// # Returns
    /// `Ok(Some(UsersEncryptionKeys))` if a key exists,
    /// `Ok(None)` if no session is active.
    pub fn get_user_encryption_key(&self) -> Result<Option<UsersEncryptionKeys>, SessionError> {
        match self {
            Session::Keyring(s) => s.get_user_encryption_key().map_err(|e| e.into()),
            Session::File(s) => s.get_user_encryption_key().map_err(|e| e.into()),
        }
    }

    /// Check if the session is unlocked (key exists).
    pub fn is_unlocked(&self) -> bool {
        match self {
            Session::Keyring(s) => s.is_unlocked(),
            Session::File(s) => s.is_unlocked(),
        }
    }

    /// Save the user encryption key (alias for unlock).
    ///
    /// # Arguments
    /// * `uek` - The user encryption key to store
    pub fn save(&self, uek: &UsersEncryptionKeys) -> Result<(), SessionError> {
        self.unlock(uek)
    }

    /// Change the stored user encryption key.
    ///
    /// # Arguments
    /// * `uek` - The new user encryption key to store
    pub fn change(&self, uek: &UsersEncryptionKeys) -> Result<(), SessionError> {
        match self {
            Session::Keyring(s) => s.change(uek).map_err(|e| e.into()),
            Session::File(s) => s.unlock(uek).map_err(|e| e.into()),
        }
    }

    /// Get the user encryption key (alias for get_user_encryption_key).
    pub fn get(&self) -> Result<Option<UsersEncryptionKeys>, SessionError> {
        self.get_user_encryption_key()
    }
}

impl Default for Session {
    fn default() -> Self {
        Self::new()
    }
}
