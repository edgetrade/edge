//! Domains module - Actor-based domain modules
//!
//! Each domain follows the actor/handler pattern with 4 files:
//! - mod.rs: Public exports, domain constants
//! - handle.rs: Thin gateway - public API, sends messages
//! - actor.rs: State owner - business logic, receives messages
//! - messages.rs: Command/Query enums
//! - errors.rs: Domain-specific errors

pub mod alerts;
pub mod client;
pub mod config;
pub mod enclave;
pub mod ipc;
pub mod keystore;
pub mod mcp;
pub mod trades;
