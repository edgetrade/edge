pub mod cache;
pub mod fetch;
pub mod inject;
pub mod manager;
pub mod types;

pub use cache::{CacheError, DEFAULT_TTL_MINUTES, MANIFEST_FILENAME};
pub use fetch::{FetchError, fetch_manifest, fetch_manifest_raw};
pub use inject::{inject_local_agent_actions, inject_local_resources};
pub use manager::ManifestManager;
pub use types::*;

/// Error type for manifest operations.
#[derive(Debug, thiserror::Error)]
pub enum ManifestError {
    #[error("Cache error: {0}")]
    Cache(#[from] cache::CacheError),
    #[error("Fetch error: {0}")]
    Fetch(#[from] fetch::FetchError),
    #[error("Config error: {0}")]
    Config(String),
}

/// SHA256 helper for hash comparison.
fn sha256(data: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    let hash = Sha256::digest(data);
    hex::encode(hash)
}
