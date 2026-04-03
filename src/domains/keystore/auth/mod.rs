//! Authentication module for the keystore domain.
//!
//! Provides password-based and passkey-based authentication methods
//! for deriving the master key used in the Edge key hierarchy.
//!
//! The authentication flow:
//! 1. User authenticates (password or passkey)
//! 2. Master key is derived from authentication material
//! 3. User encryption key and KWK are derived from master key
//! 4. KWK decrypts the blind_user_key to recover UEK
//! 5. UEK is stored in the session for wallet operations

pub mod passkey;
pub mod password;
pub mod types;
