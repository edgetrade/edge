//! Passkey/WebAuthn authentication for Edge CLI.
//!
//! This module provides structure for passkey-based authentication
//! using the WebAuthn PRF (Pseudo-Random Function) extension.
//!
//! # PRF Extension Flow
//!
//! The PRF extension allows extracting deterministic random bytes from
//! a hardware authenticator during authentication. This output can be
//! used as the master key or fed through HKDF for domain separation.
//!
//! ## Registration Flow (stubbed)
//! 1. Start passkey registration with PRF extension
//! 2. User interacts with authenticator (fingerprint, PIN, etc.)
//! 3. Store credential ID locally
//!
//! ## Authentication Flow (stubbed)
//! 1. Start passkey authentication with PRF extension
//! 2. User interacts with authenticator
//! 3. Extract PRF output from authenticator response
//! 4. Map PRF output to master key using HKDF with domain separation
//! 5. Derive UEK and KWK from master key (same as password flow)
//! 6. Unwrap blind_user_key
//!
//! # Implementation Status
//!
//! This module is currently a **placeholder/structure only**.
//! Full passkey implementation is deferred to a later phase.
//! The structure is in place to ensure the auth module API is
//! forward-compatible with passkey support.
//!
//! When implementing, consider these crates:
//! - `webauthn-rs`: Full WebAuthn server-side implementation
//! - `fido2`: Lower-level FIDO2/CTAP library
//!
//! Note: The Edge CLI operates as a command-line tool, so full
//! passkey support may require integration with platform authenticators
//! or browser-based flows.

use crate::commands::key::filestore::crypto::types::MasterKey;

use super::types::{AuthError, AuthResult, AuthenticationResult, Authenticator};

/// Passkey authenticator implementing the `Authenticator` trait.
///
/// This struct holds the credential ID and challenge state for
/// passkey authentication using the PRF extension.
///
/// # Implementation Note
///
/// This is a placeholder structure. Full implementation requires:
/// 1. Integration with platform WebAuthn APIs or browser flow
/// 2. PRF extension support in the authenticator
/// 3. Secure storage of credential ID
#[derive(Debug, Clone)]
pub struct PasskeyAuth {
    /// The credential ID from registration (placeholder).
    credential_id: Option<Vec<u8>>,
    /// The RP ID (relying party ID) for this passkey.
    rp_id: String,
}

impl PasskeyAuth {
    /// Create a new passkey authenticator.
    ///
    /// # Arguments
    /// * `rp_id` - The relying party ID (e.g., "edge.trade").
    pub fn new(rp_id: impl Into<String>) -> Self {
        Self {
            credential_id: None,
            rp_id: rp_id.into(),
        }
    }

    /// Create a passkey authenticator with an existing credential.
    ///
    /// # Arguments
    /// * `rp_id` - The relying party ID.
    /// * `credential_id` - The stored credential ID from registration.
    pub fn with_credential(rp_id: impl Into<String>, credential_id: Vec<u8>) -> Self {
        Self {
            credential_id: Some(credential_id),
            rp_id: rp_id.into(),
        }
    }

    /// Get the credential ID if set.
    pub fn credential_id(&self) -> Option<&[u8]> {
        self.credential_id.as_deref()
    }

    /// Get the RP ID.
    pub fn rp_id(&self) -> &str {
        &self.rp_id
    }
}

impl Authenticator for PasskeyAuth {
    /// Authenticate using a passkey.
    ///
    /// # Implementation Note
    ///
    /// This method is currently stubbed and returns `AuthError::NotImplemented`.
    /// Full implementation would:
    /// 1. Start a WebAuthn authentication ceremony with PRF extension
    /// 2. Send challenge to platform authenticator (via OS API or browser)
    /// 3. Receive authenticator response with PRF output
    /// 4. Derive master key from PRF output using HKDF
    fn authenticate(&self) -> AuthResult<AuthenticationResult> {
        // Stub implementation - full passkey support deferred
        Err(AuthError::NotImplemented)
    }
}

/// Passkey registration state and configuration.
///
/// Represents an in-progress passkey registration. After successful
/// registration, the credential ID should be stored for future
/// authentication.
///
/// # Implementation Note
///
/// This is a placeholder structure. Full implementation would
/// use `webauthn-rs` or similar for the registration ceremony.
#[derive(Debug, Clone)]
pub struct PasskeyRegistration {
    /// The challenge bytes for this registration.
    challenge: Vec<u8>,
    /// The RP ID for this registration.
    rp_id: String,
    /// The user ID for this registration.
    user_id: String,
    /// The user display name.
    user_name: String,
}

impl PasskeyRegistration {
    /// Create a new passkey registration.
    ///
    /// # Arguments
    /// * `rp_id` - The relying party ID.
    /// * `user_id` - The unique user ID.
    /// * `user_name` - The user's display name.
    pub fn new(rp_id: impl Into<String>, user_id: impl Into<String>, user_name: impl Into<String>) -> Self {
        // Generate random challenge
        let mut challenge = vec![0u8; 32];
        getrandom::fill(&mut challenge).expect("System RNG available");

        Self {
            challenge,
            rp_id: rp_id.into(),
            user_id: user_id.into(),
            user_name: user_name.into(),
        }
    }

    /// Start the registration process.
    ///
    /// # Implementation Note
    ///
    /// This method is currently stubbed and returns `AuthError::NotImplemented`.
    /// Full implementation would start a WebAuthn registration ceremony
    /// with the PRF extension enabled.
    ///
    /// # Returns
    /// The credential ID on success.
    pub fn register(&self) -> AuthResult<Vec<u8>> {
        // Stub implementation - full passkey support deferred
        // PRF extension should be requested during registration
        Err(AuthError::NotImplemented)
    }

    /// Get the challenge bytes.
    pub fn challenge(&self) -> &[u8] {
        &self.challenge
    }

    /// Get the RP ID.
    pub fn rp_id(&self) -> &str {
        &self.rp_id
    }

    /// Get the user ID.
    pub fn user_id(&self) -> &str {
        &self.user_id
    }

    /// Get the user display name.
    pub fn user_name(&self) -> &str {
        &self.user_name
    }
}

/// Derive a master key from PRF extension output.
///
/// The PRF extension provides deterministic random bytes from the
/// hardware authenticator. This function applies HKDF with domain
/// separation to derive a properly formatted master key.
///
/// # Arguments
/// * `prf_output` - The raw PRF output from the authenticator.
///
/// # Returns
/// The derived `MasterKey`.
///
/// # Domain Separation
///
/// Uses HKDF info string "edge-v1:prf-to-master-key" to ensure
/// the derived key is domain-separated from other uses of the
/// same PRF output.
pub fn derive_master_key_from_prf(prf_output: &[u8]) -> MasterKey {
    use hkdf::Hkdf;
    use sha2::Sha256;

    // Use HKDF to derive a 32-byte master key from PRF output
    // This provides domain separation and consistent output length
    let hkdf = Hkdf::<Sha256>::new(None, prf_output);
    let mut master_key = [0u8; 32];
    hkdf.expand(b"edge-v1:prf-to-master-key", &mut master_key)
        .expect("HKDF expand for PRF");

    MasterKey(master_key)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_passkey_auth_struct() {
        let auth = PasskeyAuth::new("edge.trade");
        assert_eq!(auth.rp_id(), "edge.trade");
        assert!(auth.credential_id().is_none());
    }

    #[test]
    fn test_passkey_auth_with_credential() {
        let credential_id = vec![0xAB, 0xCD, 0xEF];
        let auth = PasskeyAuth::with_credential("edge.trade", credential_id.clone());
        assert_eq!(auth.credential_id(), Some(credential_id.as_slice()));
    }

    #[test]
    fn test_passkey_auth_authenticate_not_implemented() {
        let auth = PasskeyAuth::new("edge.trade");
        let result = auth.authenticate();
        assert!(matches!(result, Err(AuthError::NotImplemented)));
    }

    #[test]
    fn test_passkey_registration_struct() {
        let reg = PasskeyRegistration::new("edge.trade", "user-123", "Alice");
        assert_eq!(reg.rp_id(), "edge.trade");
        assert_eq!(reg.user_id(), "user-123");
        assert_eq!(reg.user_name(), "Alice");
        assert_eq!(reg.challenge().len(), 32);
    }

    #[test]
    fn test_passkey_registration_register_not_implemented() {
        let reg = PasskeyRegistration::new("edge.trade", "user-123", "Alice");
        let result = reg.register();
        assert!(matches!(result, Err(AuthError::NotImplemented)));
    }

    #[test]
    fn test_derive_master_key_from_prf_deterministic() {
        let prf_output = vec![0x42u8; 32];

        let key1 = derive_master_key_from_prf(&prf_output);
        let key2 = derive_master_key_from_prf(&prf_output);

        assert_eq!(key1.0, key2.0, "Same PRF output must produce same key");
    }

    #[test]
    fn test_derive_master_key_from_prf_different_inputs() {
        let prf_output1 = vec![0x42u8; 32];
        let prf_output2 = vec![0x43u8; 32];

        let key1 = derive_master_key_from_prf(&prf_output1);
        let key2 = derive_master_key_from_prf(&prf_output2);

        assert_ne!(key1.0, key2.0, "Different PRF outputs must produce different keys");
    }

    #[test]
    fn test_derive_master_key_from_prf_key_size() {
        let prf_output = vec![0xABu8; 64];
        let key = derive_master_key_from_prf(&prf_output);
        assert_eq!(key.0.len(), 32);
    }
}
