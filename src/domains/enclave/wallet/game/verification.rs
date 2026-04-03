//! Verification module for the prove game feature.
//!
//! Handles writing verification instructions to game.toml and provides
//! manual verification of game outcomes.

use std::io::Write;

use crate::domains::enclave::wallet::game::game_state::GameResultEntry;
use crate::domains::enclave::wallet::game::game_state::{GameState, game_state_path, load_game_state};

/// Verification instructions for game.toml
pub const VERIFICATION_INSTRUCTIONS: &str = r#"
# =============================================================================
# VERIFICATION INSTRUCTIONS
# =============================================================================
# This file contains the state and results from the Edge Prove Game.
# The Prove Game demonstrates Edge's security model through two interactive games:
#
# Game 1: The Blind Oracle - Tests constraint-based wallet access
# Game 2: The Vault - Tests password-based wallet encryption
#
# This section explains how to independently verify all cryptographic operations.

# =============================================================================
# GAME 1: THE BLIND ORACLE - VERIFICATION
# =============================================================================
# In Game 1, constraint values are sealed into intents using age encryption.
# The enclave uses the transport key's ed25519 public key for sealing.
#
# To verify a sealed intent:
#
# 1. EXTRACT THE SEALED ENVELOPE
#    Copy the 'envelope' field from any [[sealed_intents]] entry above.
#    The envelope is base64-encoded age-encrypted data.
#
# 2. VERIFY INTENT SEALING WITH AGE
#    Install age: https://github.com/FiloSottile/age
#
#    # The enclave's transport public key is needed for verification
#    # This key is returned by the GetTransportKey operation
#    # Convert the ed25519 public key to age format and decrypt:
#
#    # Save the envelope to a file (base64 decode first):
#    echo "<envelope-base64>" | base64 -d > sealed_intent.age
#
#    # Decrypt with the appropriate private key (requires enclave cooperation)
#    age -d -i <transport-private-key> sealed_intent.age
#
#    # The decrypted content is a JSON ExecutionPayload containing:
#    # - game_key: The 32-byte game key (hex encoded)
#    # - intent: The SealedIntent with constraint values
#
# 3. SIGNATURE VERIFICATION WITH CAST/FOUNDRY
#    When wallet access is granted, the enclave produces a signature.
#    This signature can be verified using cast:
#
#    # Install Foundry: https://book.getfoundry.sh/
#    # The signature format is "r|s|v" as returned by the enclave
#
#    # Recover the signer address from the signature:
#    cast verify-message <message-hash> <signature> --address <expected-address>
#
#    # Or verify a signed transaction:
#    cast verify-signature <message> <signature> <signer-address>
#
# 4. CONSTRAINT MATCHING VERIFICATION
#    The enclave grants wallet access only when the test value matches
#    one of the constraint values stored in the sealed intent.
#    This can be verified by checking:
#    - The constraint_value in [[sealed_intents]]
#    - The test value used during the game
#    - The wallet_accessed field in [[game_results]]

[[verification.steps]]
step = 1
description = "Extract sealed envelope from game.toml"
command = "cat ~/.config/edge/game.toml | grep -A5 'sealed_intents'"

[[verification.steps]]
step = 2
description = "Base64 decode the envelope"
command = "echo '<envelope-data>' | base64 -d > intent.age"

[[verification.steps]]
step = 3
description = "Verify with age (requires transport private key)"
command = "age -d -i transport.key intent.age"

[[verification.steps]]
step = 4
description = "Verify signature with cast"
command = "cast verify-signature <message> <signature> <address>"

# =============================================================================
# GAME 2: THE VAULT - VERIFICATION
# =============================================================================
# In Game 2, passwords are converted to encryption keys via HKDF-SHA256.
# The wallet is encrypted with AES-256-GCM using these derived keys.
#
# To verify key derivation and encryption:
#
# 1. HKDF KEY DERIVATION WITH OPENSSL
#    The derived keys stored in [derived_keys] were created using:
#
#    # Using OpenSSL to replicate HKDF-SHA256 derivation:
#    openssl dgst -sha256 -hmac "edge-vault-game" \
#      -binary <<< "<password>" | xxd -p
#
#    # Or more accurately with the proper HKDF extract-and-expand:
#    # HKDF-Extract(salt=none, IKM=password)
#    # HKDF-Expand(PRK, info="edge-vault-game", L=32)
#
#    # Verify against stored derived key:
#    # The base64 values in [derived_keys] should match your derivation
#
# 2. AES-GCM DECRYPTION VERIFICATION
#    The encrypted blobs in [encrypted_blobs] use AES-256-GCM:
#
#    # The format is: nonce (12 bytes) || ciphertext || auth_tag (16 bytes)
#    # Using OpenSSL for decryption verification:
#
#    # Extract nonce and ciphertext from the blob:
#    echo "<blob-base64>" | base64 -d > encrypted.blob
#    head -c 12 encrypted.blob > nonce.bin
#    tail -c +13 encrypted.blob > ciphertext.bin
#
#    # Decrypt with the derived key:
#    openssl aes-256-gcm -d \
#      -K <derived-key-hex> \
#      -iv $(xxd -p nonce.bin) \
#      -in ciphertext.bin \
#      -out decrypted.key \
#      -tag $(tail -c 16 encrypted.blob | xxd -p)
#
# 3. WALLET PRIVATE KEY RECOVERY
#    If decryption succeeds, the output should be the wallet's private key.
#    Verify by deriving the address:
#
#    # The decrypted private key is 32 bytes (secp256k1)
#    # Derive public key and Ethereum address:
#
#    # Using cast to derive address from private key:
#    cast wallet address --private-key <decrypted-private-key-hex>
#
#    # This should match the wallet address stored in [wallet].address
#
# 4. HKDF VERIFICATION DETAILS
#    The exact parameters used for key derivation:
#    - Algorithm: HKDF-SHA256 (RFC 5869)
#    - Salt: None (empty salt)
#    - IKM: The UTF-8 password bytes
#    - Info: The ASCII string "edge-vault-game"
#    - Output length: 32 bytes
#
#    Python verification:
#    ```python
#    from cryptography.hazmat.primitives.kdf.hkdf import HKDF
#    from cryptography.hazmat.primitives import hashes
#
#    hkdf = HKDF(
#        algorithm=hashes.SHA256(),
#        length=32,
#        salt=None,
#        info=b"edge-vault-game",
#    )
#    key = hkdf.derive(password.encode())
#    # key should match the base64-decoded derived key
#    ```

[[verification.hkdf_steps]]
step = 1
description = "Derive key from password using HKDF-SHA256"
info = "salt=None, info='edge-vault-game', length=32"

[[verification.hkdf_steps]]
step = 2
description = "Compare with stored derived key"
field = "derived_keys.<password_id>"

[[verification.hkdf_steps]]
step = 3
description = "Decrypt wallet blob with derived key"
algorithm = "AES-256-GCM"
nonce_size = 12

[[verification.hkdf_steps]]
step = 4
description = "Verify recovered private key matches wallet"
command = "cast wallet address --private-key <key>"

# =============================================================================
# MANUAL VERIFICATION WORKFLOW
# =============================================================================
# To perform complete manual verification:
#
# 1. Complete both games at least once:
#    edge wallet prove --game 1
#    edge wallet prove --game 2
#
# 2. This file will be populated with all game data
#
# 3. Use the verification steps above to independently verify:
#    - Intent sealing (Game 1)
#    - Signature validity (Game 1)
#    - Key derivation (Game 2)
#    - Wallet decryption (Game 2)

[[verification.checklist]]
item = "Intent sealing verified with age"
description = "Decrypt at least one sealed intent envelope"

[[verification.checklist]]
item = "Signature verified with cast"
description = "Verify at least one enclave signature"

[[verification.checklist]]
item = "HKDF derivation verified"
description = "Reproduce key derivation from password"

[[verification.checklist]]
item = "AES-GCM decryption verified"
description = "Decrypt wallet blob and recover private key"

[[verification.checklist]]
item = "Wallet address matches"
description = "Derived address from recovered key matches stored address"
"#;

/// Write verification instructions to game.toml
///
/// This appends the verification instructions as comments to the game.toml file.
/// The instructions explain how to manually verify:
/// - Intent sealing with age encryption
/// - Signatures with cast/foundry
/// - HKDF key derivation
/// - AES-GCM decryption
///
/// # Returns
/// Ok(()) on success, or an error if writing fails
pub fn write_verification_instructions() -> crate::error::Result<()> {
    let path = game_state_path()?;

    // Append verification instructions to the file
    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(&path)?;

    writeln!(file, "{}", VERIFICATION_INSTRUCTIONS)?;

    Ok(())
}

/// Check if verification instructions should be written.
///
/// Instructions are written once when both games have been completed
/// at least once.
///
/// # Returns
/// Ok(true) if instructions were written, Ok(false) otherwise
pub fn maybe_write_verification_instructions() -> crate::error::Result<bool> {
    let state = load_game_state()?;

    // Check if we have results from both games
    let has_game1 = state.game_results.iter().any(|r| r.game_type == 1);
    let has_game2 = state.game_results.iter().any(|r| r.game_type == 2);

    if has_game1 && has_game2 {
        // Check if instructions already exist by looking for a marker
        let path = game_state_path()?;
        let contents = std::fs::read_to_string(&path)?;

        if !contents.contains("VERIFICATION INSTRUCTIONS") {
            write_verification_instructions()?;
            return Ok(true);
        }
    }

    Ok(false)
}

/// Format game results for display.
///
/// Shows clear formatting of all game outcomes from the game state.
///
/// # Arguments
/// * `state` - The game state containing results
///
/// # Returns
/// Formatted string of game results
pub fn format_game_results(state: &GameState) -> String {
    let mut output = String::new();

    output.push_str("\n========================================\n");
    output.push_str("         Stored Game Results            \n");
    output.push_str("========================================\n\n");

    if state.game_results.is_empty() {
        output.push_str("No game results found.\n");
        return output;
    }

    // Group results by game type
    let game1_results: Vec<_> = state
        .game_results
        .iter()
        .filter(|r| r.game_type == 1)
        .collect();
    let game2_results: Vec<_> = state
        .game_results
        .iter()
        .filter(|r| r.game_type == 2)
        .collect();

    // Display Game 1 results
    if !game1_results.is_empty() {
        output.push_str("Game 1 (Blind Oracle) Results:\n");
        output.push_str("------------------------------\n");
        for (i, result) in game1_results.iter().enumerate() {
            output.push_str(&format_result(i + 1, result));
        }
        output.push('\n');
    }

    // Display Game 2 results
    if !game2_results.is_empty() {
        output.push_str("Game 2 (Vault) Results:\n");
        output.push_str("-----------------------\n");
        for (i, result) in game2_results.iter().enumerate() {
            output.push_str(&format_result(i + 1, result));
        }
        output.push('\n');
    }

    // Display sealed intents summary
    if !state.sealed_intents.is_empty() {
        output.push_str("Stored Sealed Intents:\n");
        output.push_str("----------------------\n");
        output.push_str(&format!("  Count: {}\n", state.sealed_intents.len()));
        for intent in &state.sealed_intents {
            output.push_str(&format!(
                "  - {} (constraint: {})\n",
                intent.id,
                intent.constraint_value.as_deref().unwrap_or("none")
            ));
        }
        output.push('\n');
    }

    // Display derived keys summary
    if !state.derived_keys.is_empty() {
        output.push_str("Stored Derived Keys:\n");
        output.push_str("-------------------\n");
        for id in state.derived_keys.keys() {
            output.push_str(&format!("  - {}\n", id));
        }
        output.push('\n');
    }

    output
}

/// Format a single game result entry.
fn format_result(index: usize, result: &GameResultEntry) -> String {
    let status = if result.success { "✓ SUCCESS" } else { "✗ FAILED" };

    let mut output = format!("  {}. {} ({})\n", index, result.session_id, status);
    output.push_str(&format!("     Timestamp: {}\n", result.timestamp));

    if let Some(ref sig) = result.signature {
        let preview = if sig.len() > 30 {
            format!("{}...", &sig[..30])
        } else {
            sig.clone()
        };
        output.push_str(&format!("     Signature: {}\n", preview));
    }

    if let Some(ref err) = result.enclave_error {
        output.push_str(&format!("     Error: {}\n", err));
    }

    output.push('\n');
    output
}
