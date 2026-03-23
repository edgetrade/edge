//! Command implementations for Edge CLI.
//!
//! Provides interactive commands for key management, wallet operations,
//! and session lifecycle management.
//!
//! # Feature Boundaries
//!
//! This module defines common error types that are agnostic to the
//! desktop/server feature split. Feature-specific code should convert
//! their errors to these generic types using `.map_err()` rather than
//! implementing `From` trait for feature-specific types.

pub mod key;
pub mod serve;
pub mod subscribe;
pub mod wallet;

pub use crate::messages;
