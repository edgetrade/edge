//! Client module - Re-exports from domains::client for generated route compatibility
//!
//! This module exists to provide the `crate::client` path that build.rs-generated
//! routes expect. The actual implementation is in `domains::client`.

pub use crate::domains::client::{
    IrisClient,
    Route,
    RouteExecutor,
    RouteType,
    // Legacy route functions
    delete_wallet,
    list_wallets,
    new_client,
    place_spot_order,
    proof_game,
    rotate_user_encryption_key,
    upsert_wallet,
};
