use alloy::hex::encode_prefixed;

use crate::domains::client::generated::routes::requests::agent_proof_game::{
    ProofGameRequestOrdersItem, ProofGameResponse, ProofGameResponseResultsItem,
};
use crate::domains::enclave::wallet::game::game_state::GameWallet;

pub fn welcome_message() {
    println!("\n========================================");
    println!("         Welcome to the Proof Game     ");
    println!("========================================\n");
    println!("The prove game demonstrates Edge's security model.");
    println!("You'll create sealed intents and test wallet access constraints.\n");
}

pub fn game_menu() {
    println!("Select a game:");
    println!("  1. The Blind Oracle - Test constraint-based access");
    println!("  2. The Vault - Test password-based encryption");
    println!("  3. Play both games\n");
}

pub fn game_1_title() {
    println!("\n--- Game 1: The Blind Oracle ---\n");
}

pub fn game_2_title() {
    println!("\n--- Game 2: The Vault ---\n");
}

pub fn game_wallet(wallet: &GameWallet) {
    println!("For this game, we will NOT be using your 'real' wallet");
    println!("Instead, we will use a temporary wallet for this game.");
    println!("You may delete it at any time after the game is complete.");
    println!("The wallet address is: {}\n", wallet.address);
}

pub fn game_1_select() {
    println!("To play the game, first we will ask you to select three numbers.");
    println!("These numbers will then be sealed in an \"intent\" envelope that.");
    println!("only our vault can decrypt. Our vault uses these intents to ensure");
    println!("that the wallet can only be accessed to do your bidding.\n");
}

pub fn game_2_select() {
    println!("To play the game, first we will ask you to create two passwords.");
    println!("These passwords will then be used to derive encryption keys we will.");
    println!("send to the vault with your order. If they are the right password.");
    println!("the vault will decrypt the wallet and grant access to the wallet.");
    println!("Only the correct password will be able to provide your share of");
    println!("the decryption mechanism.\n");
}

pub fn game_sealing() {
    println!("Sealing your intentions so only our vault can decrypt...");
}

pub fn game_sealed(orders: &[ProofGameRequestOrdersItem]) {
    println!("\nAll {} orders created and sealed!\n", orders.len());
}

pub fn status_sending(orders: &[ProofGameRequestOrdersItem]) {
    println!("Sending {} orders to the enclave...\n", orders.len());
}

pub fn respond_value(value: &str) {
    println!("\nValue you gave us => {}\n", value);
}

pub fn game_menu_invalid_choice() {
    println!("Invalid choice, please try again.");
}

pub fn game_order_result(i: usize, result: &ProofGameResponseResultsItem) {
    let status = if result.enclave_error.is_none() && result.signature.is_some() {
        game_wallet_accessed()
    } else if let Some(ref err) = result.enclave_error {
        game_wallet_not_accessed(Some(err))
    } else {
        game_wallet_not_accessed(None)
    };

    println!("Order {}: {}\n", i + 1, status);
    if let Some(ref sig) = result.signature {
        println!("  Signature: {}...", encode_prefixed(&sig[..sig.len().min(20)]));
    }
}

pub fn game_result_title() {
    println!("\n--- Game Results ---\n");
}

pub fn game_wallet_accessed() -> String {
    "✓ WALLET ACCESSED".to_string()
}

pub fn game_wallet_not_accessed(error: Option<&str>) -> String {
    let leading_char = "✗ Access denied: ";
    match error {
        Some(err) => format!("{}{}", leading_char, err),
        None => leading_char.to_string(),
    }
}

pub fn game_wallet_succeeded(wallet: &GameWallet, response: &ProofGameResponse) {
    println!();
    println!("✓✓✓ SUCCESS! Wallet was accessed! ✓✓✓");
    println!();
    println!("The enclave granted access to the wallet.");
    println!();
    println!("Wallet: {}", wallet.address);

    if let Some(result) = response
        .results
        .iter()
        .find(|r| r.enclave_error.is_none() && r.signature.is_some())
        && let Some(ref sig) = result.signature
    {
        println!("\nSignature: {}", encode_prefixed(sig));
        println!("\nTo verify this signature:");
        println!("  1. The signature proves the enclave accessed the wallet");
        println!("  2. The constraint-based access control worked correctly\n");
    };
}

pub fn game_wallet_failed(wallet: &GameWallet) {
    println!("✗ Wallet access denied.\n");
    println!("The enclave did not grant access to the wallet.\n");
    println!("  Wallet: {}\n", wallet.address);
    println!("This behavior is expected if the final result did not match your sealed");
    println!("information. The vaule correctly enforced the access control.\n");
}

pub fn game_complete() {
    println!("\n========================================");
    println!("         Proof Game Complete!          ");
    println!("========================================\n");
}

/// Display verification summary after games complete.
///
/// Shows the user how to manually verify the game results.
pub fn display_verification_summary() {
    println!("\n========================================");
    println!("      Verification Instructions        ");
    println!("========================================\n");

    println!("All game data has been saved to ~/.config/edge/game.toml\n");

    println!("To verify the cryptographic operations:\n");

    println!("Game 1 (Blind Oracle):");
    println!("  - View sealed intents in game.toml");
    println!("  - Use 'age' tool to verify intent encryption");
    println!("  - Use 'cast' (Foundry) to verify signatures\n");

    println!("Game 2 (Vault):");
    println!("  - View derived keys and encrypted blobs in game.toml");
    println!("  - Reproduce HKDF-SHA256 key derivation with OpenSSL");
    println!("  - Decrypt wallet blobs with AES-256-GCM\n");

    println!("Detailed instructions have been added to game.toml\n");
}
