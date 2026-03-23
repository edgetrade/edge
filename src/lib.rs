//! Edge Trade library

pub mod app;
pub mod client;
pub mod commands;
pub mod config;
pub mod manifest;
pub mod messages;
pub mod session;
pub mod utils;
pub mod wallet;

#[cfg(test)]
pub mod test_utils {
    //! Shared test utilities for ensuring test isolation.
    //!
    //! These mutexes are used to serialize tests that share global resources
    //! like the OS keyring or file system paths.

    use std::sync::Mutex;

    /// Global mutex for all keyring tests across all modules.
    /// All keyring tests use the same keyring entry (service="edge", username="user-encryption-key"),
    /// so they must be serialized to prevent conflicts when running tests in parallel.
    #[cfg(feature = "keyring-tests")]
    pub static KEYRING_TEST_MUTEX: Mutex<()> = Mutex::new(());

    /// Global mutex for all filestore session tests across all modules.
    /// Filestore session tests use the same session file path (~/.config/edge/session),
    /// so they must be serialized to prevent conflicts when running tests in parallel.
    pub static FILESTORE_TEST_MUTEX: Mutex<()> = Mutex::new(());

    /// Global mutex for transport key cache tests across all modules.
    /// Transport key cache tests use the same config directory (~/.config/edge/transport_keys.json),
    /// so they must be serialized to prevent conflicts when running tests in parallel.
    pub static TRANSPORT_CACHE_TEST_MUTEX: Mutex<()> = Mutex::new(());
}
