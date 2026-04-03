//! Proof game command for Edge CLI.
//!
//! The prove game is an interactive demonstration of Edge's security model.
//! It allows users to create sealed intents and test wallet access constraints
//! without performing actual blockchain transactions.
//!
//! Games:
//! - Game 1: The Blind Oracle (test constraint-based access)
//! - Game 2: The Vault (test password-based encryption)

use crate::domains::client::IrisClient;
use crate::domains::enclave::errors::EnclaveError;
use crate::domains::enclave::errors::EnclaveResult;
use crate::domains::enclave::wallet::game::messages as game_messages;
use crate::domains::enclave::wallet::game::verification::maybe_write_verification_instructions;
use crate::domains::enclave::wallet::game::{envelope_game, intents_game};
use crate::domains::keystore::Session;
use crate::messages;

/// Play the prove game.
///
/// This is the main entry point for the prove game command. It delegates
/// to specific game implementations based on the game parameter.
///
/// # Arguments
/// * `game` - Which game to play (1, 2, or None for menu)
/// * `client` - The Iris API client
///
/// # Returns
/// Ok(()) on success, or an error if the game fails
pub async fn wallet_prove(game: Option<u8>, session: &Session, client: &IrisClient) -> EnclaveResult<()> {
    game_messages::welcome_message();
    let game_choice: u64 = match game {
        Some(1) => 1,
        Some(2) => 2,
        Some(3) => 3,
        _ => {
            game_messages::game_menu_invalid_choice();
            show_game_menu()?
        }
    };
    match game_choice {
        1 => {
            game_messages::game_1_title();
            intents_game::play_game(session, client)
                .await
                .map_err(|e| EnclaveError::Wallet(format!("Game 1 failed: {}", e)))?;
        }
        2 => {
            game_messages::game_2_title();
            envelope_game::play_game(session, client)
                .await
                .map_err(|e| EnclaveError::Wallet(format!("Game 2 failed: {}", e)))?;
        }
        3 => {
            game_messages::game_1_title();
            intents_game::play_game(session, client)
                .await
                .map_err(|e| EnclaveError::Wallet(format!("Game 1 failed: {}", e)))?;

            game_messages::game_2_title();
            envelope_game::play_game(session, client)
                .await
                .map_err(|e| EnclaveError::Wallet(format!("Game 2 failed: {}", e)))?;
        }
        _ => {
            return Err(EnclaveError::InvalidInput("Invalid game selection".to_string()));
        }
    }

    maybe_write_verification_instructions()
        .map_err(|e| EnclaveError::Wallet(format!("Failed to write verification instructions: {}", e)))?;
    game_messages::display_verification_summary();
    game_messages::game_complete();
    Ok(())
}

fn show_game_menu() -> EnclaveResult<u64> {
    game_messages::game_menu();
    messages::prompt::prompt_number("Enter your choice (1, 2, or 3): ")
        .map_err(|e| EnclaveError::InvalidInput(format!("Failed to read choice: {}", e)))
}
