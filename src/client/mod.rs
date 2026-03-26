mod trpc;
pub use trpc::{IrisClient, Route, RouteType, new_client};

mod executor;
pub use executor::{RouteExecutor, mutation, query};

mod routes;
pub use routes::{
    delete_wallet, list_wallets, proof_game, rotate_user_encryption_key, upsert_encrypted_wallet, upsert_wallet,
};

mod subscribe;
pub use subscribe::{DispatchParams, IrisClientInner, subscribe, subscribe_for_dispatch, unsubscribe};

pub use crate::messages::IrisClientError;
