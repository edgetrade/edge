use thiserror::Error;

use crate::utils::urls::DOCS_BASE_URL;

/// Error type for command operations.
///
/// This is a generic error type that does not depend on feature-specific
/// modules. All feature-specific errors should be converted to this
/// type at the feature boundary using `.map_err()`.
#[derive(Debug, Error)]
pub enum CommandError {
    #[error("Authentication error: {0}")]
    Authentication(String),
    #[error("Crypto error: {0}")]
    Crypto(String),
    #[error("Storage error: {0}")]
    Storage(String),
    #[error("Session error: {0}")]
    Session(String),
    #[error("IO error: {0}")]
    Io(String),
    #[error("Configuration already exists")]
    AlreadyExists,
    #[error("Configuration not found")]
    NotFound,
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Wallet error: {0}")]
    Wallet(String),
}

/// Error codes for consistent error message formatting.
#[derive(Debug, Clone, Copy)]
pub struct ErrorCode(&'static str);

impl ErrorCode {
    // Key management errors
    pub const KEY_CREATE_FAILED: Self = ErrorCode("ERR_KEY_CREATE_FAILED");
    pub const KEY_UPDATE_FAILED: Self = ErrorCode("ERR_KEY_UPDATE_FAILED");
    pub const KEY_DELETE_FAILED: Self = ErrorCode("ERR_KEY_DELETE_FAILED");
    pub const KEY_LOCK_FAILED: Self = ErrorCode("ERR_KEY_LOCK_FAILED");
    pub const KEY_UNLOCK_FAILED: Self = ErrorCode("ERR_KEY_UNLOCK_FAILED");
    pub const KEY_SHOW_FAILED: Self = ErrorCode("ERR_KEY_SHOW_FAILED");
    pub const KEY_EXPORT_FAILED: Self = ErrorCode("ERR_KEY_EXPORT_FAILED");

    // Filestore errors
    pub const FILESTORE_CREATE_FAILED: Self = ErrorCode("ERR_FILESTORE_CREATE_FAILED");
    pub const FILESTORE_UPDATE_FAILED: Self = ErrorCode("ERR_FILESTORE_UPDATE_FAILED");
    pub const FILESTORE_DELETE_FAILED: Self = ErrorCode("ERR_FILESTORE_DELETE_FAILED");
    pub const FILESTORE_LOCK_FAILED: Self = ErrorCode("ERR_FILESTORE_LOCK_FAILED");
    pub const FILESTORE_UNLOCK_FAILED: Self = ErrorCode("ERR_FILESTORE_UNLOCK_FAILED");
    pub const FILESTORE_SHOW_FAILED: Self = ErrorCode("ERR_FILESTORE_SHOW_FAILED");

    // Keyring errors
    pub const KEYRING_CREATE_FAILED: Self = ErrorCode("ERR_KEYRING_CREATE_FAILED");
    pub const KEYRING_UPDATE_FAILED: Self = ErrorCode("ERR_KEYRING_UPDATE_FAILED");
    pub const KEYRING_DELETE_FAILED: Self = ErrorCode("ERR_KEYRING_DELETE_FAILED");
    pub const KEYRING_LOCK_FAILED: Self = ErrorCode("ERR_KEYRING_LOCK_FAILED");
    pub const KEYRING_UNLOCK_FAILED: Self = ErrorCode("ERR_KEYRING_UNLOCK_FAILED");
    pub const KEYRING_SHOW_FAILED: Self = ErrorCode("ERR_KEYRING_SHOW_FAILED");

    // Wallet errors
    pub const WALLET_CREATE_FAILED: Self = ErrorCode("ERR_WALLET_CREATE_FAILED");
    pub const WALLET_IMPORT_FAILED: Self = ErrorCode("ERR_WALLET_IMPORT_FAILED");
    pub const WALLET_DELETE_FAILED: Self = ErrorCode("ERR_WALLET_DELETE_FAILED");
    pub const WALLET_SHOW_FAILED: Self = ErrorCode("ERR_WALLET_SHOW_FAILED");
    pub const WALLET_LIST_FAILED: Self = ErrorCode("ERR_WALLET_LIST_FAILED");

    // Session errors
    pub const SESSION_UNLOCK_FAILED: Self = ErrorCode("ERR_SESSION_UNLOCK_FAILED");
    pub const SESSION_LOCK_FAILED: Self = ErrorCode("ERR_SESSION_LOCK_FAILED");

    // Manifest errors
    pub const MANIFEST_FETCH_FAILED: Self = ErrorCode("ERR_MANIFEST_FETCH_FAILED");
    pub const MANIFEST_REFRESH_FAILED: Self = ErrorCode("ERR_MANIFEST_REFRESH_FAILED");
    pub const MANIFEST_SAVE_FAILED: Self = ErrorCode("ERR_MANIFEST_SAVE_FAILED");
    pub const MANIFEST_PARSE_FAILED: Self = ErrorCode("ERR_MANIFEST_PARSE_FAILED");

    // Client API key errors
    pub const API_KEY_REQUIRED: Self = ErrorCode("ERR_API_KEY_REQUIRED");
    pub const API_KEY_INVALID: Self = ErrorCode("ERR_API_KEY_INVALID");
}

impl ErrorCode {
    /// Returns the error code string
    pub fn as_str(&self) -> &'static str {
        self.0
    }
}

impl std::fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// Only implement From for types that are common to both features.
// Feature-specific types should use .map_err() at the boundary.

impl From<std::io::Error> for CommandError {
    fn from(e: std::io::Error) -> Self {
        CommandError::Io(e.to_string())
    }
}

impl From<crate::commands::key::filestore::crypto::types::CryptoError> for CommandError {
    fn from(e: crate::commands::key::filestore::crypto::types::CryptoError) -> Self {
        CommandError::Crypto(e.to_string())
    }
}

impl From<crate::wallet::types::WalletError> for CommandError {
    fn from(e: crate::wallet::types::WalletError) -> Self {
        CommandError::Wallet(e.to_string())
    }
}

impl From<crate::commands::key::filestore::storage::StorageError> for CommandError {
    fn from(e: crate::commands::key::filestore::storage::StorageError) -> Self {
        CommandError::Storage(e.to_string())
    }
}

impl From<crate::commands::wallet::game::game_state::GameStateError> for CommandError {
    fn from(e: crate::commands::wallet::game::game_state::GameStateError) -> Self {
        CommandError::Storage(e.to_string())
    }
}

/// Prints connection failed message (for single-argument error)
pub fn connection_failed(err: &str) {
    eprintln!("Failed to connect to Edge servers: {}", err);
}

/// Prints connection failed message (for two-argument verbose logging)
pub fn connection_failed_url_key(url: &str, _key: &str) {
    eprintln!("[edge] connecting to {}", url);
}

/// Prints query error message
pub fn query_error(path: &str, err: &str) {
    eprintln!("[edge] ✗ {} (query/mutation): {}", path, err);
}

/// Prints mutation error message
pub fn mutation_error(path: &str, input: &str, err: &str) {
    eprintln!("Mutation error at path '{}' with input '{}': {}", path, input, err);
}

/// Prints subscription error message
pub fn subscription_error(path: &str, id: &str, err: &str) {
    eprintln!("Subscription error at path '{}' with id '{}': {}", path, id, err);
}

/// Prints reconnect failed message
pub fn reconnect_failed(path: &str, id: &str) {
    eprintln!("Reconnect failed at path '{}' with id '{}'", path, id);
}

/// Prints auth error message
pub fn auth_error(path: &str, id: &str, err: &str) {
    eprintln!("Auth error at path '{}' with id '{}': {}", path, id, err);
}

/// Prints SSE parse error message
pub fn sse_parse_error(data: &str) {
    eprintln!("SSE parse error: '{}'", data);
}

/// Prints manifest save error message
pub fn manifest_save_error(err: &str) {
    eprintln!("Failed to save manifest: {}", err);
}

/// Prints manifest parse error message
pub fn manifest_parse_error(err: &str) {
    eprintln!("Manifest parse error: {}", err);
}

/// Prints MCP server error message
pub fn mcp_server_error(err: &str) {
    eprintln!("MCP server error: {}", err);
}

/// Prints deprecated SSE transport warning
pub fn deprecated_transport_sse() {
    eprintln!("[edge] --transport sse is deprecated...");
}

/// Prints key command error message
pub fn key_command_error(command: &str, chain: &str) {
    eprintln!("[edge] key {} failed: {}", command, chain);
}

/// Prints invalid chain type error message
pub fn invalid_chain_type() {
    eprintln!("[edge] error: chain_type must be 'evm' or 'svm'");
}

/// Prints wallet command error message
pub fn wallet_command_error(command: &str, err: &str) {
    eprintln!("[edge] wallet {} failed: {}", command, err);
}

/// Prints create directory error message
pub fn create_dir_error(err: &str) {
    eprintln!("[edge] failed to create directory: {}", err);
}

/// Prints write skill error message
pub fn write_skill_error(err: &str) {
    eprintln!("[edge] failed to write skill: {}", err);
}

/// Prints skill installed message
pub fn skill_installed(name: &str, dir: &str) {
    eprintln!("[edge] installed skill '{}' to {}", name, dir);
}

/// Prints skill not found error message
pub fn skill_not_found(name: &str) {
    eprintln!("[edge] skill '{}' not found in manifest", name);
}

/// Prints pinging URL message
pub fn pinging(ping_url: &str) {
    eprintln!("[edge] pinging {}", ping_url);
}

/// Prints ping failed status message
pub fn ping_failed_status(status: &str) {
    eprintln!("Ping failed with status: {}", status);
}

/// Prints ping failed error message
pub fn ping_failed_error(err: &str) {
    eprintln!("Ping failed: {}", err);
}

/// Prints API key required error message
pub fn api_key_required() {
    eprintln!("Error: API key required. Set EDGE_API_KEY or use --api-key");
}

/// Prints API key documentation URL
pub fn api_key_docs_url() {
    println!("See: {}/authentication", DOCS_BASE_URL);
}

/// Prints HTTP server starting message
pub fn http_server_starting(addr: &str, path: &str) {
    println!("Starting HTTP server on http://{}{}", addr, path);
}

/// Prints alert registered message
pub fn alert_registered(name: &str, id: u64, summary: &str) {
    eprintln!("[edge] ✓ alert '{}' (id={}) → {}", name, id, summary);
}

/// Prints fetch HTTP error message
pub fn fetch_http_error(status: u16) {
    eprintln!("[edge] manifest fetch failed: HTTP {}", status);
}

/// Prints session error message
pub fn session_error(err: &str) {
    eprintln!("Session error: {}", err);
}

/// Prints fetch error message
pub fn fetch_error(url: &str) {
    eprintln!("[edge] manifest fetch error: {}", url);
}

/// Prints API key invalid message
pub fn api_key_invalid() {
    eprintln!("Error: Invalid API key. See: {}/authentication", DOCS_BASE_URL);
}

/// Prints Iris connection failed message
pub fn iris_connection_failed(err: &str) {
    eprintln!("Failed to connect to Iris: {}", err);
}

/// Prints manifest load error message
pub fn manifest_load_error(err: &str) {
    eprintln!("Failed to load manifest: {}", err);
}

// ============== Context-Aware Constructor Functions ==============
// These functions delegate console output to the messages module

/// Key creation failed
pub fn key_create_failed(chain: &str) {
    key_command_error("create", chain);
}

/// Key update failed
pub fn key_update_failed(chain: &str) {
    key_command_error("update", chain);
}

/// Key deletion failed
pub fn key_delete_failed(chain: &str) {
    key_command_error("delete", chain);
}

/// Key lock failed
pub fn key_lock_failed(chain: &str) {
    key_command_error("lock", chain);
}

/// Key unlock failed
pub fn key_unlock_failed(chain: &str) {
    key_command_error("unlock", chain);
}

/// Key show failed
pub fn key_show_failed(chain: &str) {
    key_command_error("show", chain);
}

/// Key export failed
pub fn key_export_failed(chain: &str) {
    key_command_error("export", chain);
}

// --- Filestore Command Errors ---

/// Filestore creation failed
pub fn filestore_create_failed(name: &str) {
    key_command_error("create_filestore", name);
}

/// Filestore update failed
pub fn filestore_update_failed(name: &str) {
    key_command_error("update_filestore", name);
}

/// Filestore deletion failed
pub fn filestore_delete_failed(name: &str) {
    key_command_error("delete_filestore", name);
}

/// Filestore lock failed
pub fn filestore_lock_failed(name: &str) {
    key_command_error("lock_filestore", name);
}

/// Filestore unlock failed
pub fn filestore_unlock_failed(name: &str) {
    key_command_error("unlock_filestore", name);
}

/// Filestore show failed
pub fn filestore_show_failed(name: &str) {
    key_command_error("show_filestore", name);
}

// --- Keyring Command Errors ---

/// Keyring creation failed
pub fn keyring_create_failed(chain: &str) {
    key_command_error("create_keyring", chain);
}

/// Keyring update failed
pub fn keyring_update_failed(chain: &str) {
    key_command_error("update_keyring", chain);
}

/// Keyring deletion failed
pub fn keyring_delete_failed(chain: &str) {
    key_command_error("delete_keyring", chain);
}

/// Keyring lock failed
pub fn keyring_lock_failed(chain: &str) {
    key_command_error("lock_keyring", chain);
}

/// Keyring unlock failed
pub fn keyring_unlock_failed(chain: &str) {
    key_command_error("unlock_keyring", chain);
}

/// Keyring show failed
pub fn keyring_show_failed(chain: &str) {
    key_command_error("show_keyring", chain);
}

// --- Wallet Command Errors ---

/// Wallet creation failed
pub fn wallet_create_failed(chain: &str) {
    wallet_command_error("create", chain);
}

/// Wallet import failed
pub fn wallet_import_failed(chain: &str) {
    wallet_command_error("import", chain);
}

/// Wallet deletion failed
pub fn wallet_delete_failed(chain: &str) {
    wallet_command_error("delete", chain);
}

/// Wallet show failed
pub fn wallet_show_failed(chain: &str) {
    wallet_command_error("show", chain);
}

/// Wallet list failed
pub fn wallet_list_failed(err: &str) {
    wallet_command_error("list", err);
}

// --- Session Command Errors ---

/// Session unlock failed
pub fn session_unlock_failed(err: &str) {
    session_error(err);
}

/// Session lock failed
pub fn session_lock_failed(err: &str) {
    session_error(err);
}

// --- Manifest Errors ---

/// Manifest fetch failed
pub fn manifest_fetch_failed(url: &str) {
    fetch_error(url);
}

/// Manifest refresh failed
pub fn manifest_refresh_failed(err: &str) {
    manifest_parse_error(err);
}

// --- Utility Functions ---

/// Display an error code and message
pub fn display_error(error_code: ErrorCode, message: &str) {
    eprintln!("[{}] {}", error_code.as_str(), message);
}

/// Display an error with context
pub fn display_error_with_context(error_code: ErrorCode, context: &str, message: &str) {
    eprintln!("[{}] {}: {}", error_code.as_str(), context, message);
}
