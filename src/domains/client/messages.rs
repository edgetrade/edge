//! Client domain messages
//!
//! Defines the command/query enums for client actor communication.
//! Uses PoseidonRequest pattern for req/resp communication.

use serde_json::Value;
use tokio::sync::mpsc::UnboundedReceiver;

use crate::domains::client::errors::ClientError;
use crate::domains::client::manifest::types::McpManifest;
use crate::domains::enclave::wallet::types::Wallet;
use crate::domains::keystore::session::crypto::UsersEncryptionKeys;
use crate::event_bus::PoseidonRequest;

/// Messages sent to the ClientActor
#[derive(Debug)]
#[allow(clippy::large_enum_variant)]
pub enum ClientMessage {
    /// Connect to the Iris API
    Connect {
        url: String,
        api_key: String,
        verbose: bool,
    },

    /// Disconnect from the Iris API
    Disconnect,

    /// Execute a query
    Query { path: String, input: Value },

    /// Execute a mutation
    Mutation { path: String, input: Value },

    /// Subscribe to a real-time event stream
    Subscribe { path: String, input: Value },

    /// Subscribe for dispatch (alert delivery)
    SubscribeForDispatch {
        procedure: String,
        input: Value,
        alert_id: u64,
        alert_name: String,
    },

    /// Unsubscribe from an event stream
    Unsubscribe { subscription_id: u32 },

    /// Get the current manifest
    GetManifest,

    /// Refresh the manifest from server
    RefreshManifest,

    /// Ping the server
    Ping,

    /// Get the Iris client
    GetClient,

    // === Route-based messages (moved from routes.rs) ===
    /// Create or update an encrypted wallet
    UpsertWallet {
        wallet: Wallet,
        user_key: UsersEncryptionKeys,
    },

    /// List all encrypted wallets
    ListWallets,

    /// Delete an encrypted wallet by address
    DeleteWallet { address: String },

    /// Rotate the user encryption key
    RotateUserEncryptionKey {
        new_key: UsersEncryptionKeys,
        old_key: UsersEncryptionKeys,
    },

    /// Conduct the proof game
    ProofGame {
        request: crate::domains::client::generated::routes::requests::agent_proof_game::ProofGameRequest,
    },

    /// Execute a generic route (low-level access)
    ExecuteRoute { path: String, input: Value },
}

/// Response types for client operations
#[derive(Debug)]
pub enum ClientResponse {
    /// Connection successful
    Connected,

    /// Disconnected
    Disconnected,

    /// Query result
    QueryResult(Value),

    /// Mutation result
    MutationResult(Value),

    /// Subscription created with ID and receiver
    Subscribed {
        id: u32,
        receiver: UnboundedReceiver<Value>,
    },

    /// Dispatch subscription created
    DispatchSubscribed { id: u32 },

    /// Unsubscribed
    Unsubscribed,

    /// Manifest retrieved
    Manifest(McpManifest),

    /// Manifest refreshed
    ManifestRefreshed,

    /// Ping successful
    Pong,

    /// Client retrieved
    Client(Option<crate::domains::client::trpc::IrisClient>),

    // === Route-based responses ===
    /// Wallet updated successfully
    WalletUpdated,

    /// List of wallets
    WalletList(Vec<crate::domains::enclave::wallet::types::WalletList>),

    /// Wallet deleted successfully
    WalletDeleted,

    /// User encryption key rotated successfully
    KeyRotated,

    /// Proof game result
    ProofGameResult(crate::domains::client::generated::routes::requests::agent_proof_game::ProofGameResponse),

    /// Route execution result
    RouteResult(Value),
}

/// Request type using PoseidonRequest pattern
///
/// ClientRequest wraps ClientMessage with trace context and reply channel.
/// This enables request/response communication with telemetry support.
pub type ClientRequest = PoseidonRequest<ClientMessage, ClientResponse, ClientError>;
