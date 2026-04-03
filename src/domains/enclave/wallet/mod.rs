//! Wallet operations for the enclave domain.
//!
//! Provides wallet management including create, import, delete,
//! list operations and proof games.

pub mod create;
pub mod delete;
pub mod game;
pub mod import;
pub mod list;
pub mod name;
pub mod operations;
pub mod proof;
pub mod types;

pub use create::wallet_create;
pub use delete::wallet_delete;
pub use import::wallet_import;
pub use list::wallet_list;
pub use operations::{create_wallet, import_wallet};
pub use proof::wallet_prove;
pub use types::{Wallet, WalletError, WalletList, WalletResult};
