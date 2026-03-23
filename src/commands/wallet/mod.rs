//! Wallet management commands for Edge CLI.
//!
//! Provides commands for creating, importing, listing, and deleting wallets
//! for EVM (Ethereum) and Solana blockchains.

pub mod create;
pub mod delete;
pub mod game;
pub mod import;
pub mod list;
pub mod prove;
pub mod verification;

mod name;

pub use create::wallet_create;
pub use delete::wallet_delete;
pub use import::wallet_import;
pub use list::wallet_list;
pub use prove::wallet_prove;
