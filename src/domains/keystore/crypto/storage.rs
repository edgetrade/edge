//! Local filesystem storage for Edge CLI secrets.
//!
//! Handles secure storage of the blind_user_key with appropriate
//! file permissions (0o600 on Unix systems).

use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

/// The default filename for the blind user key.
pub const BLIND_USER_KEY_FILENAME: &str = "blind_user_key";

/// The default filename for the salt.
pub const SALT_FILENAME: &str = "salt";

/// The config directory name for XDG config.
pub const CONFIG_DIR_NAME: &str = "edge";

/// The default filename for the session file.
pub const SESSION_FILENAME: &str = "session";

/// Error type for storage operations.
#[derive(Debug, Clone, thiserror::Error)]
pub enum StorageError {
    #[error("IO error: {0}")]
    Io(String),
    #[error("Invalid permissions on file: {0}")]
    InvalidPermissions(String),
    #[error("File not found: {0}")]
    NotFound(String),
}

impl From<std::io::Error> for StorageError {
    fn from(e: std::io::Error) -> Self {
        StorageError::Io(e.to_string())
    }
}

/// Get the default storage directory path using XDG config directory.
///
/// Uses the `dirs` crate to find the config directory and appends `edge`.
/// Returns `None` if the config directory cannot be determined.
///
/// - Linux: `~/.config/edge/`
/// - macOS: `~/Library/Application Support/edge/`
/// - Windows: `%APPDATA%\edge\`
pub fn default_storage_dir() -> Option<PathBuf> {
    dirs::config_dir().map(|config| config.join(CONFIG_DIR_NAME))
}

/// Ensure the storage directory exists.
///
/// Creates the directory if it doesn't exist, including parent directories.
/// Sets permissions to 0o700 (owner read/write/execute only) on Unix systems.
///
/// # Arguments
/// * `path` - The directory path to ensure exists
///
/// # Returns
/// The path to the storage directory.
pub fn ensure_storage_dir(path: &Path) -> Result<PathBuf, StorageError> {
    if !path.exists() {
        fs::create_dir_all(path)?;

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(path, fs::Permissions::from_mode(0o700))?;
        }
    }

    Ok(path.to_path_buf())
}

/// Store the blind user key to the filesystem.
///
/// Writes the encrypted key data to the specified path with
/// permissions 0o600 (owner read/write only) on Unix systems.
///
/// # Arguments
/// * `path` - The file path to write to
/// * `data` - The encrypted key data to store
///
/// # Returns
/// `Ok(())` on success, or a `StorageError` on failure.
pub fn store_blind_user_key(path: &Path, data: &[u8]) -> Result<(), StorageError> {
    let mut file = fs::File::create(path)?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o600))?;
    }

    file.write_all(data)?;
    file.sync_all()?;

    Ok(())
}

/// Load the blind user key from the filesystem.
///
/// Reads the encrypted key data from the specified path.
/// On Unix systems, verifies that the file has owner-only permissions.
///
/// # Arguments
/// * `path` - The file path to read from
///
/// # Returns
/// The file contents as a byte vector.
pub fn load_blind_user_key(path: &Path) -> Result<Vec<u8>, StorageError> {
    if !path.exists() {
        return Err(StorageError::NotFound(path.display().to_string()));
    }

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let metadata = fs::metadata(path)?;
        let permissions = metadata.permissions().mode();
        // Check that others have no permissions (mode & 0o77 == 0)
        // and that at least owner has read/write (mode & 0o600 == 0o600)
        if permissions & 0o077 != 0 {
            return Err(StorageError::InvalidPermissions(format!(
                "File {} has permissions {:o}, expected 0o600",
                path.display(),
                permissions & 0o777
            )));
        }
    }

    let mut file = fs::File::open(path)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    Ok(data)
}

/// Store the salt to the filesystem.
///
/// Writes the salt to a file alongside the blind_user_key.
/// Uses the same permissions (0o600 on Unix).
///
/// # Arguments
/// * `path` - The file path to write to
/// * `salt` - The salt bytes to store
///
/// # Returns
/// `Ok(())` on success, or a `StorageError` on failure.
pub fn store_salt(path: &Path, salt: &[u8]) -> Result<(), StorageError> {
    let mut file = fs::File::create(path)?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o600))?;
    }

    file.write_all(salt)?;
    file.sync_all()?;

    Ok(())
}

/// Load the salt from the filesystem.
///
/// Reads the salt from the specified path.
/// On Unix systems, verifies that the file has owner-only permissions.
///
/// # Arguments
/// * `path` - The file path to read from
///
/// # Returns
/// The salt bytes.
pub fn load_salt(path: &Path) -> Result<Vec<u8>, StorageError> {
    if !path.exists() {
        return Err(StorageError::NotFound(path.display().to_string()));
    }

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let metadata = fs::metadata(path)?;
        let permissions = metadata.permissions().mode();
        if permissions & 0o077 != 0 {
            return Err(StorageError::InvalidPermissions(format!(
                "File {} has permissions {:o}, expected 0o600",
                path.display(),
                permissions & 0o777
            )));
        }
    }

    let mut file = fs::File::open(path)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    Ok(data)
}

/// Get the default path for the blind user key.
///
/// Returns the path in the XDG config directory: `<config_dir>/edge/blind_user_key`.
pub fn default_blind_user_key_path() -> Option<PathBuf> {
    default_storage_dir().map(|dir| dir.join(BLIND_USER_KEY_FILENAME))
}

/// Get the default path for the salt.
///
/// Returns the path in the XDG config directory: `<config_dir>/edge/salt`.
pub fn default_salt_path() -> Option<PathBuf> {
    default_storage_dir().map(|dir| dir.join(SALT_FILENAME))
}

/// Get the XDG config directory path for edge.
///
/// Returns `None` if the config directory cannot be determined.
pub fn config_dir() -> Option<PathBuf> {
    dirs::config_dir().map(|config| config.join(CONFIG_DIR_NAME))
}

/// Ensure the XDG config directory exists.
///
/// Creates the directory if it doesn't exist, including parent directories.
/// Sets permissions to 0o700 (owner read/write/execute only) on Unix systems.
///
/// # Returns
/// The path to the config directory, or `StorageError` if creation fails.
pub fn ensure_config_dir() -> Result<PathBuf, StorageError> {
    let path = config_dir().ok_or_else(|| StorageError::Io("Could not determine config directory".to_string()))?;

    if !path.exists() {
        fs::create_dir_all(&path)?;

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&path, fs::Permissions::from_mode(0o700))?;
        }
    }

    Ok(path)
}

/// Store session data to the XDG config file.
///
/// Writes the session data to `<config_dir>/edge/session` with
/// permissions 0o600 (owner read/write only) on Unix systems.
///
/// # Arguments
/// * `data` - The session data to store
///
/// # Returns
/// `Ok(())` on success, or a `StorageError` on failure.
pub fn store_session_file(data: &[u8]) -> Result<(), StorageError> {
    let config_dir = ensure_config_dir()?;
    let path = config_dir.join(SESSION_FILENAME);

    let mut file = fs::File::create(&path)?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&path, fs::Permissions::from_mode(0o600))?;
    }

    file.write_all(data)?;
    file.sync_all()?;

    Ok(())
}

/// Load session data from the XDG config file.
///
/// Reads the session data from `<config_dir>/edge/session`.
/// On Unix systems, verifies that the file has owner-only permissions.
///
/// # Returns
/// `Some(data)` if the file exists and is valid, `None` if the file doesn't exist,
/// or `StorageError` if there's a problem reading or the file has bad permissions.
pub fn load_session_file() -> Result<Option<Vec<u8>>, StorageError> {
    let config_dir =
        config_dir().ok_or_else(|| StorageError::Io("Could not determine config directory".to_string()))?;
    let path = config_dir.join(SESSION_FILENAME);

    if !path.exists() {
        return Ok(None);
    }

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let metadata = fs::metadata(&path)?;
        let permissions = metadata.permissions().mode();
        // Check that others have no permissions (mode & 0o77 == 0)
        if permissions & 0o077 != 0 {
            return Err(StorageError::InvalidPermissions(format!(
                "Session file {} has permissions {:o}, expected 0o600",
                path.display(),
                permissions & 0o777
            )));
        }
    }

    let mut file = fs::File::open(&path)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    Ok(Some(data))
}

/// Delete the session file from the XDG config directory.
///
/// Removes `<config_dir>/edge/session` if it exists.
///
/// # Returns
/// `Ok(())` on success or if file doesn't exist, or `StorageError` on failure.
pub fn delete_session_file() -> Result<(), StorageError> {
    let config_dir =
        config_dir().ok_or_else(|| StorageError::Io("Could not determine config directory".to_string()))?;
    let path = config_dir.join(SESSION_FILENAME);

    if path.exists() {
        fs::remove_file(&path)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_ensure_storage_dir_creates_directory() {
        let temp = tempfile::tempdir().expect("Failed to create temp dir");

        let result = ensure_storage_dir(temp.path());
        assert!(result.is_ok());
        assert!(temp.path().exists());
    }

    #[test]
    fn test_store_and_load_blind_user_key() {
        let temp = tempfile::tempdir().expect("Failed to create temp dir");
        let key_path = temp.path().join("test_key");
        let test_data = vec![0xde, 0xad, 0xbe, 0xef];

        store_blind_user_key(&key_path, &test_data).unwrap();
        let loaded = load_blind_user_key(&key_path).unwrap();

        assert_eq!(loaded, test_data);

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let metadata = fs::metadata(&key_path).unwrap();
            let permissions = metadata.permissions().mode() & 0o777;
            assert_eq!(permissions, 0o600, "File should have 0o600 permissions");
        }
    }

    #[test]
    fn test_load_nonexistent_file() {
        let temp = tempfile::tempdir().expect("Failed to create temp dir");

        let result = load_blind_user_key(&temp.path().join("nonexistent"));
        assert!(matches!(result, Err(StorageError::NotFound(_))));
    }

    #[test]
    fn test_store_and_load_salt() {
        let temp = tempfile::tempdir().expect("Failed to create temp dir");

        let salt_path = temp.path().join("test_salt");
        let test_salt = vec![0xca, 0xfe, 0xba, 0xbe];

        store_salt(&salt_path, &test_salt).unwrap();
        let loaded = load_salt(&salt_path).unwrap();

        assert_eq!(loaded, test_salt);
    }

    #[cfg(unix)]
    #[test]
    fn test_load_rejects_world_readable_file() {
        let temp = tempfile::tempdir().expect("Failed to create temp dir");

        let key_path = temp.path().join("insecure_key");
        let mut file = fs::File::create(&key_path).unwrap();
        file.write_all(&[0xde, 0xad, 0xbe, 0xef]).unwrap();

        // Make file world-readable
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&key_path, fs::Permissions::from_mode(0o644)).unwrap();

        let result = load_blind_user_key(&key_path);
        assert!(
            matches!(result, Err(StorageError::InvalidPermissions(_))),
            "Should reject world-readable file"
        );
    }

    #[test]
    fn test_default_paths() {
        let key_path = default_blind_user_key_path();
        let salt_path = default_salt_path();

        assert!(key_path.is_some());
        assert!(salt_path.is_some());

        let key_path = key_path.unwrap();
        let salt_path = salt_path.unwrap();

        assert!(key_path.to_string_lossy().contains("edge"));
        assert!(key_path.to_string_lossy().contains("blind_user_key"));
        assert!(salt_path.to_string_lossy().contains("edge"));
        assert!(salt_path.to_string_lossy().contains("salt"));
    }
}
