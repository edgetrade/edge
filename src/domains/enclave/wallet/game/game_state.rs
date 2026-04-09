//! Game state management for the prove game feature.
//!
//! Handles persistence of wallet data, sealed intents, and game results
//! in the game.toml file located at ~/.config/edge/game.toml.

use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

use base64::Engine;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use erato::messages::envelopes::transport::TransportEnvelopeKey;

use crate::domains::client::IrisClient;
use crate::domains::client::get_transport_key;
use crate::domains::config::Config;
use crate::domains::enclave::errors::EnclaveError;
use crate::domains::enclave::wallet::types::WalletError;
use crate::domains::keystore::Session;
use crate::domains::keystore::session::crypto::UsersEncryptionKeys;

/// Default game state file name.
pub const GAME_STATE_FILE: &str = "game.toml";

/// Game state containing all prove game wallet and intent data.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GameState {
    /// Game wallet (secp256k1 keypair for EVM compatibility).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet: Option<GameWallet>,
    /// Map of password index to derived key.
    #[serde(default)]
    pub derived_keys: HashMap<String, String>,
    /// Map of password index to encrypted wallet blob.
    #[serde(default)]
    pub encrypted_blobs: HashMap<String, String>,
    /// Sealed intents for Game 1 (The Blind Oracle).
    #[serde(default)]
    pub sealed_intents: Vec<SealedIntentEntry>,
    /// Game results from prove game execution.
    #[serde(default)]
    pub game_results: Vec<GameResultEntry>,
    /// Timestamp of last update.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<String>,
}

/// A game wallet for prove game use.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameWallet {
    /// Wallet address (derived from public key).
    pub address: String,
    /// Base64-encoded private key.
    pub private_key: String,
    /// Chain type (always EVM for prove game).
    pub chain_type: String,
    /// Creation timestamp.
    pub created_at: String,
}

/// A sealed intent entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SealedIntentEntry {
    /// Intent ID.
    pub id: String,
    /// The sealed envelope data (base64 encoded).
    pub envelope: String,
    /// The constraint value (for oracle game).
    pub constraint_value: Option<String>,
    /// Creation timestamp.
    pub created_at: String,
}

/// A game result entry (aligned with ProofGameResponseResultsItem).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameResultEntry {
    /// Game session ID.
    pub session_id: String,
    /// Game type (1 or 2).
    pub game_type: u8,
    /// Whether the game succeeded.
    pub success: bool,
    /// Signature if wallet was accessed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    /// Error message if failed (enclave_error pattern).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enclave_error: Option<String>,
    /// Timestamp.
    pub timestamp: String,
}

/// Error type for game state operations.
#[derive(Debug, Clone, thiserror::Error)]
pub enum GameStateError {
    #[error("IO error: {0}")]
    Io(String),
    #[error("Parse error: {0}")]
    Parse(String),
    #[error("Serialization error: {0}")]
    Serialization(String),
    #[error("Wallet not found")]
    WalletNotFound,
    #[error("Invalid key")]
    InvalidKey,
    #[error("Cannot find agent ID")]
    AgentIdNotFound,
}

impl From<WalletError> for GameStateError {
    fn from(e: WalletError) -> Self {
        GameStateError::Io(e.to_string())
    }
}

impl From<std::io::Error> for GameStateError {
    fn from(e: std::io::Error) -> Self {
        GameStateError::Io(e.to_string())
    }
}

impl From<toml::de::Error> for GameStateError {
    fn from(e: toml::de::Error) -> Self {
        GameStateError::Parse(e.to_string())
    }
}

impl From<toml::ser::Error> for GameStateError {
    fn from(e: toml::ser::Error) -> Self {
        GameStateError::Serialization(e.to_string())
    }
}

impl From<GameStateError> for EnclaveError {
    fn from(e: GameStateError) -> Self {
        EnclaveError::GameState(e.to_string())
    }
}

/// Generate a unique session ID for the prove game.
pub fn generate_session_id() -> String {
    use uuid::Uuid;
    format!(
        "prove-game-{}",
        Uuid::new_v4()
            .to_string()
            .split('-')
            .next()
            .unwrap_or("session")
    )
}

/// Get the game info from the session.
///
/// # Arguments
/// * `session` - The session to get the game info from.
/// * `client` - The Iris client to get the transport key from.
///
/// # Returns
/// The agent ID and user encryption key.
pub async fn get_game_info(
    session: &Session,
    client: &IrisClient,
) -> Result<(Uuid, UsersEncryptionKeys, TransportEnvelopeKey), GameStateError> {
    // Get transport keys for sealing
    let enclave_keys = get_transport_key(client)
        .await
        .map_err(|e| GameStateError::Io(e.to_string()))?;
    let transport_key = TransportEnvelopeKey::Unsealing(enclave_keys.deterministic);

    let mut aid = session
        .get_config()
        .map_err(|e| GameStateError::Io(e.to_string()))?
        .clone()
        .agent_id;

    if aid.is_none() {
        get_transport_key(client).await?;
        let config = Config::load(None).map_err(|e| GameStateError::Io(e.to_string()))?;
        let agent_id = config.agent_id;
        if agent_id.is_none() {
            return Err(GameStateError::AgentIdNotFound);
        };
        aid = Some(agent_id.unwrap());
    }

    let agent_id = aid.unwrap();

    let user_key = session
        .get_user_encryption_key()
        .map_err(|e| GameStateError::Io(e.to_string()))?
        .ok_or_else(|| GameStateError::Io("Session unavailable".to_string()))?;
    Ok((agent_id, user_key, transport_key))
}

/// Get the path to the game state file.
///
/// Returns ~/.config/edge/game.toml (or platform equivalent).
pub fn game_state_path() -> Result<PathBuf, GameStateError> {
    let config_dir = Config::config_path()
        .ok_or_else(|| GameStateError::Io("No config path".to_string()))?
        .parent()
        .ok_or_else(|| GameStateError::Io("No config dir".to_string()))?
        .to_path_buf();
    Ok(config_dir.join(GAME_STATE_FILE))
}

/// Load the game state from disk.
///
/// If the file doesn't exist, returns a default empty GameState.
pub fn load_game_state() -> Result<GameState, GameStateError> {
    let path = effective_game_state_path()?;

    if !path.exists() {
        return Ok(GameState::default());
    }

    let contents = fs::read_to_string(&path)?;
    let state: GameState = toml::from_str(&contents)?;
    Ok(state)
}

/// Save the game state to disk.
///
/// Creates the config directory if it doesn't exist.
pub fn save_game_state(state: &GameState) -> Result<(), GameStateError> {
    let path = effective_game_state_path()?;
    let dir = path
        .parent()
        .ok_or_else(|| GameStateError::Io("No parent dir".to_string()))?;

    if !dir.exists() {
        fs::create_dir_all(dir)?;
    }

    let contents = toml::to_string_pretty(state)?;
    let mut file = fs::File::create(&path)?;
    file.write_all(contents.as_bytes())?;

    Ok(())
}

/// Get or create a game wallet.
///
/// If a wallet already exists in game.toml, returns it.
/// Otherwise, creates a new secp256k1 keypair and saves it.
///
/// # Arguments
/// * `create_if_missing` - If true, creates a new wallet if none exists
///
/// # Returns
/// The GameWallet (either existing or newly created)
pub fn get_or_create_wallet(create_if_missing: bool) -> Result<GameWallet, GameStateError> {
    let mut state = load_game_state()?;

    if let Some(wallet) = state.wallet.clone() {
        return Ok(wallet);
    }

    if !create_if_missing {
        return Err(GameStateError::WalletNotFound);
    }

    // Create new wallet
    let wallet = create_game_wallet()?;
    state.wallet = Some(wallet.clone());
    state.last_updated = Some(chrono::Utc::now().to_rfc3339());
    save_game_state(&state)?;

    Ok(wallet)
}

/// Create a new game wallet with secp256k1 keypair.
///
/// Generates a random secp256k1 keypair for EVM compatibility.
pub fn create_game_wallet() -> Result<GameWallet, GameStateError> {
    use aes_gcm::aead::rand_core;
    use k256::ecdsa::{SigningKey, VerifyingKey};
    use sha3::{Digest, Keccak256};

    // Generate random secp256k1 keypair
    let signing_key = SigningKey::random(&mut rand_core::OsRng);
    let private_key_bytes = signing_key.to_bytes();

    // Derive Ethereum address
    let verifying_key = VerifyingKey::from(&signing_key);
    let public_key_bytes = verifying_key.to_encoded_point(false).as_bytes().to_vec();

    if public_key_bytes.len() != 65 || public_key_bytes[0] != 0x04 {
        return Err(GameStateError::InvalidKey);
    }

    let hash = Keccak256::digest(&public_key_bytes[1..]);
    let address = format!("0x{}", hex::encode(&hash[hash.len() - 20..]));

    let wallet = GameWallet {
        address,
        private_key: base64::engine::general_purpose::STANDARD.encode(private_key_bytes),
        chain_type: "EVM".to_string(),
        created_at: chrono::Utc::now().to_rfc3339(),
    };

    Ok(wallet)
}

/// Store a sealed intent.
///
/// Adds a new sealed intent entry to the game state.
pub fn store_sealed_intent(
    id: String,
    envelope: Vec<u8>,
    constraint_value: Option<String>,
) -> Result<(), GameStateError> {
    let mut state = load_game_state()?;

    let entry = SealedIntentEntry {
        id: id.clone(),
        envelope: base64::engine::general_purpose::STANDARD.encode(envelope),
        constraint_value,
        created_at: chrono::Utc::now().to_rfc3339(),
    };

    // Remove existing intent with same ID if present
    state.sealed_intents.retain(|i| i.id != id);
    state.sealed_intents.push(entry);
    state.last_updated = Some(chrono::Utc::now().to_rfc3339());

    save_game_state(&state)?;
    Ok(())
}

/// Store a derived key for password-based games.
///
/// Stores an HKDF-derived key indexed by password identifier.
pub fn store_derived_key(password_id: String, key: &[u8; 32]) -> Result<(), GameStateError> {
    let mut state = load_game_state()?;

    state
        .derived_keys
        .insert(password_id, base64::engine::general_purpose::STANDARD.encode(key));
    state.last_updated = Some(chrono::Utc::now().to_rfc3339());

    save_game_state(&state)?;
    Ok(())
}

/// Store an encrypted wallet blob.
///
/// Stores a wallet blob encrypted with a specific password.
pub fn store_encrypted_blob(password_id: String, blob: Vec<u8>) -> Result<(), GameStateError> {
    let mut state = load_game_state()?;

    state
        .encrypted_blobs
        .insert(password_id, base64::engine::general_purpose::STANDARD.encode(blob));
    state.last_updated = Some(chrono::Utc::now().to_rfc3339());

    save_game_state(&state)?;
    Ok(())
}

/// Store a game result.
pub fn store_game_result(result: GameResultEntry) -> Result<(), GameStateError> {
    let mut state = load_game_state()?;

    state.game_results.push(result);
    state.last_updated = Some(chrono::Utc::now().to_rfc3339());

    save_game_state(&state)?;
    Ok(())
}

/// Get the effective game state path with thread-local test override support.
fn effective_game_state_path() -> Result<PathBuf, GameStateError> {
    #[cfg(test)]
    {
        let test_path = TEST_GAME_STATE_PATH.with(|p| p.borrow().clone());
        if let Some(path) = test_path {
            return Ok(path);
        }
    }
    game_state_path()
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_game_state_default() {
        let state = GameState::default();
        assert!(state.wallet.is_none());
        assert!(state.sealed_intents.is_empty());
        assert!(state.game_results.is_empty());
    }

    #[test]
    fn test_game_wallet_creation() {
        let wallet = GameWallet {
            address: "0x123".to_string(),
            private_key: "key".to_string(),
            chain_type: "EVM".to_string(),
            created_at: "2024-01-01T00:00:00Z".to_string(),
        };

        assert_eq!(wallet.address, "0x123");
        assert_eq!(wallet.chain_type, "EVM");
    }

    #[test]
    fn test_sealed_intent_entry() {
        let entry = SealedIntentEntry {
            id: "intent-1".to_string(),
            envelope: "base64data".to_string(),
            constraint_value: Some("42".to_string()),
            created_at: "2024-01-01T00:00:00Z".to_string(),
        };

        assert_eq!(entry.id, "intent-1");
        assert_eq!(entry.constraint_value, Some("42".to_string()));
    }

    #[test]
    fn test_game_result_entry() {
        let entry = GameResultEntry {
            session_id: "session-1".to_string(),
            game_type: 1,
            success: true,
            signature: Some("sig".to_string()),
            enclave_error: None,
            timestamp: "2024-01-01T00:00:00Z".to_string(),
        };

        assert_eq!(entry.session_id, "session-1");
        assert_eq!(entry.game_type, 1);
        assert!(entry.success);
    }

    #[test]
    fn test_game_state_serialization() {
        let state = GameState {
            wallet: Some(GameWallet {
                address: "0xabc".to_string(),
                private_key: "key".to_string(),
                chain_type: "EVM".to_string(),
                created_at: "2024-01-01T00:00:00Z".to_string(),
            }),
            derived_keys: HashMap::new(),
            encrypted_blobs: HashMap::new(),
            sealed_intents: vec![SealedIntentEntry {
                id: "intent-1".to_string(),
                envelope: "data".to_string(),
                constraint_value: None,
                created_at: "2024-01-01T00:00:00Z".to_string(),
            }],
            game_results: vec![],
            last_updated: Some("2024-01-01T00:00:00Z".to_string()),
        };

        let toml_str = toml::to_string(&state).unwrap();
        assert!(toml_str.contains("address = \"0xabc\""));
        assert!(toml_str.contains("id = \"intent-1\""));
    }

    #[test]
    fn test_game_state_deserialization() {
        let toml_str = r#"
[wallet]
address = "0xabc"
private_key = "key"
chain_type = "EVM"
created_at = "2024-01-01T00:00:00Z"

[[sealed_intents]]
id = "intent-1"
envelope = "data"
created_at = "2024-01-01T00:00:00Z"
"#;

        let state: GameState = toml::from_str(toml_str).unwrap();
        assert!(state.wallet.is_some());
        assert_eq!(state.wallet.unwrap().address, "0xabc");
        assert_eq!(state.sealed_intents.len(), 1);
    }
}

// Test-only helpers for test isolation
#[cfg(test)]
thread_local! {
    static TEST_GAME_STATE_PATH: std::cell::RefCell<Option<PathBuf>> = const { std::cell::RefCell::new(None) };
}

/// Set a test-only game state path override for the current thread.
#[cfg(test)]
pub fn set_test_game_state_path(path: PathBuf) {
    TEST_GAME_STATE_PATH.with(|p| {
        *p.borrow_mut() = Some(path);
    });
}

/// Clear the test game state path override for the current thread.
#[cfg(test)]
pub fn clear_test_game_state_path() {
    TEST_GAME_STATE_PATH.with(|p| {
        *p.borrow_mut() = None;
    });
}
