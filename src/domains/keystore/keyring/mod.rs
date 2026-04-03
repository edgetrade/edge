//! Keyring-based key operations for the keystore domain.
//!
//! Provides desktop key management commands using OS keyring storage.

pub mod create;
pub mod delete;
pub mod lock;
pub mod unlock;
pub mod update;

pub use create::{keyring_create, keyring_create_with_context};
pub use delete::keyring_delete;
pub use lock::keyring_lock;
pub use unlock::keyring_unlock;
pub use update::keyring_update;
