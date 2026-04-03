//! Manifest cache for disk I/O operations.
//!
//! Provides filesystem-backed caching of MCP manifest data with TTL-based
//! expiration. Stores cache in XDG config directory with proper permissions.

use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use super::types::McpManifest;

/// Default cache TTL in minutes
pub const DEFAULT_TTL_MINUTES: u64 = 10;

/// Manifest cache filename
pub const MANIFEST_FILENAME: &str = "manifest.json";

/// Error type for cache operations.
#[derive(Debug, thiserror::Error)]
pub enum CacheError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON parse error: {0}")]
    Parse(#[from] serde_json::Error),
    #[error("Config directory not found")]
    NoConfigDir,
}

/// Cached manifest with timestamp for TTL tracking.
#[derive(Serialize, Deserialize)]
struct CachedManifest {
    #[serde(flatten)]
    manifest: McpManifest,
    fetched_at: String,
}

/// Returns the full path to the manifest cache file.
pub fn manifest_path(config_dir: &Path) -> PathBuf {
    config_dir.join(MANIFEST_FILENAME)
}

/// Load manifest from disk cache if it exists.
///
/// Returns `Some((manifest, fetched_at))` if found and parsed successfully,
/// `None` if the file doesn't exist or cannot be parsed.
pub fn load_cached_manifest(config_dir: &Path) -> Option<(McpManifest, chrono::DateTime<chrono::Utc>)> {
    let path = manifest_path(config_dir);

    if !path.exists() {
        return None;
    }

    let mut file = fs::File::open(&path).ok()?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).ok()?;

    let cached: CachedManifest = serde_json::from_str(&contents).ok()?;
    let fetched_at = chrono::DateTime::parse_from_rfc3339(&cached.fetched_at)
        .ok()?
        .with_timezone(&chrono::Utc);

    Some((cached.manifest, fetched_at))
}

/// Save manifest to disk with current timestamp.
///
/// Creates the config directory if it doesn't exist and sets proper
/// permissions (0o700 for directory, 0o600 for file on Unix).
pub fn save_manifest(config_dir: &Path, manifest: &McpManifest) -> Result<(), CacheError> {
    if !config_dir.exists() {
        fs::create_dir_all(config_dir)?;

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(config_dir, fs::Permissions::from_mode(0o700))?;
        }
    }

    let cached = CachedManifest {
        manifest: manifest.clone(),
        fetched_at: chrono::Utc::now().to_rfc3339(),
    };

    let path = manifest_path(config_dir);
    let json = serde_json::to_vec_pretty(&cached)?;

    let mut file = fs::File::create(&path)?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&path, fs::Permissions::from_mode(0o600))?;
    }

    file.write_all(&json)?;
    file.sync_all()?;

    Ok(())
}

/// Check if the given timestamp is within the TTL window.
///
/// Returns `true` if the elapsed time since `last_fetched` is less
/// than `ttl_minutes`, `false` otherwise.
pub fn is_fresh(last_fetched: chrono::DateTime<chrono::Utc>, ttl_minutes: u64) -> bool {
    let now = chrono::Utc::now();
    let elapsed = now.signed_duration_since(last_fetched);
    elapsed.num_minutes() < ttl_minutes as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manifest_path_construction() {
        let config_dir = Path::new("/tmp/test-config");
        let path = manifest_path(config_dir);
        assert_eq!(path, PathBuf::from("/tmp/test-config/manifest.json"));
    }

    #[test]
    fn test_is_fresh_within_ttl() {
        let recent = chrono::Utc::now() - chrono::Duration::minutes(5);
        assert!(is_fresh(recent, 10));
    }

    #[test]
    fn test_is_fresh_expired() {
        let old = chrono::Utc::now() - chrono::Duration::minutes(15);
        assert!(!is_fresh(old, 10));
    }

    #[test]
    fn test_is_fresh_at_exact_boundary() {
        let exactly_ten_minutes_ago = chrono::Utc::now() - chrono::Duration::minutes(10);
        // At exactly the boundary, it should NOT be fresh (<, not <=)
        assert!(!is_fresh(exactly_ten_minutes_ago, 10));
    }

    #[test]
    fn test_load_nonexistent_cache() {
        let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
        let result = load_cached_manifest(temp_dir.path());
        assert!(result.is_none());
    }

    #[test]
    fn test_save_and_load_manifest() {
        let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");

        let manifest = McpManifest {
            tools: vec![],
            resources: vec![],
            prompts: vec![],
            skills: vec![],
        };

        save_manifest(temp_dir.path(), &manifest).expect("Failed to save manifest");

        let loaded = load_cached_manifest(temp_dir.path());
        assert!(loaded.is_some());

        let (loaded_manifest, fetched_at) = loaded.unwrap();
        assert!(loaded_manifest.tools.is_empty());
        assert!(loaded_manifest.resources.is_empty());
        assert!(loaded_manifest.prompts.is_empty());
        assert!(loaded_manifest.skills.is_empty());

        // Verify timestamp is recent
        let now = chrono::Utc::now();
        let elapsed = now.signed_duration_since(fetched_at);
        assert!(elapsed.num_seconds() < 5);
    }
}
