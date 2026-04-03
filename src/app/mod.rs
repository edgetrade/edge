//! App module - main application entry point
//!
//! Migrated from app/mod.rs, app/handler.rs, app/client.rs, app/runner.rs
//! Now uses the orchestrator pattern with domain handles.
//!
//! # Module Structure
//!
//! - `orchestrator`: App handle that orchestrates all 8 domains
//! - `cli`: Command-line interface definitions using clap
//! - `runner`: Main application runner with session and manifest management (legacy)
//!
//! # Usage
//!
//! For the new orchestrator-based API:
//! ```rust,no_run
//! use poseidon::app::orchestrator::{App, Command, KeyCommand};
//!
//! async fn example() {
//!     let app = App::new(None).await.unwrap();
//!     let result = app.run_command(Command::Key(KeyCommand::Lock)).await;
//! }
//! ```

pub mod cli;
pub mod commands;
pub mod orchestrator;

pub use orchestrator::{
    App, AppError, Command, CommandOutput, ConfigCommand, DeliveryType, KeyCommand, SubscribeCommand, TradeActionInput,
    TradeCommand, WalletCommand,
};

/// Default Iris WebSocket URL for MCP client connections
pub const DEFAULT_IRIS_URL: &str = "wss://iris.edge.trade";

// Re-export legacy modules for backward compatibility
pub mod client;
pub mod handler;
pub mod runner;

// Legacy exports
pub use runner::run;
