//! Prove game command for Edge CLI.
//!
//! The prove game is an interactive demonstration of Edge's security model.
//! It allows users to create sealed intents and test wallet access constraints
//! without performing actual blockchain transactions.
//!
//! Games:
//! - Game 1: The Blind Oracle (test constraint-based access)
//! - Game 2: The Vault (test password-based encryption)

use crate::client::IrisClient;
use crate::commands::wallet::game::game_state::load_game_state;
use crate::commands::wallet::verification::{
    display_verification_summary, format_game_results, maybe_write_verification_instructions, prompt_replay,
};
use crate::messages;

use super::game;

/// Play the prove game.
///
/// This is the main entry point for the prove game command. It delegates
/// to specific game implementations based on the game parameter.
///
/// # Arguments
/// * `game` - Which game to play (1, 2, or None for menu)
/// * `replay` - Whether to replay using existing intents
/// * `client` - The Iris API client
///
/// # Returns
/// Ok(()) on success, or an error if the game fails
pub async fn wallet_prove(game: Option<u8>, replay: bool, client: &IrisClient) -> messages::success::CommandResult<()> {
    // Show welcome message
    println!("\n========================================");
    println!("         Welcome to the Prove Game     ");
    println!("========================================\n");
    println!("The prove game demonstrates Edge's security model.");
    println!("You'll create sealed intents and test wallet access constraints.\n");

    // If replay mode, show existing results first
    if replay {
        let state = load_game_state().map_err(|e| messages::error::CommandError::Storage(e.to_string()))?;
        if !state.game_results.is_empty() {
            println!("{}", format_game_results(&state));
        } else {
            println!("No previous game data found. Replay mode will use newly created data.\n");
        }
    }

    // Determine which game to play
    let game_choice = match game {
        Some(1) => 1u8,
        Some(2) => 2u8,
        Some(3) => 3u8,
        _ => {
            // Show menu and get selection
            show_game_menu()?
        }
    };

    // Play the selected game(s)
    match game_choice {
        1 => {
            println!("\n--- Game 1: The Blind Oracle ---\n");
            game::intents_game::play_game(replay, client).await?;

            // After Game 1, prompt for replay
            if !replay && prompt_replay(1)? {
                println!("\n--- Replaying Game 1 ---\n");
                game::intents_game::play_game(true, client).await?;
            }
        }
        2 => {
            println!("\n--- Game 2: The Vault ---\n");
            game::envelope_game::play_game(replay, client).await?;

            // After Game 2, prompt for replay
            if !replay && prompt_replay(2)? {
                println!("\n--- Replaying Game 2 ---\n");
                game::envelope_game::play_game(true, client).await?;
            }
        }
        3 => {
            // Play both games in sequence
            println!("\n--- Game 1: The Blind Oracle ---\n");
            game::intents_game::play_game(replay, client).await?;

            // Prompt for replay after Game 1
            let replay_game1 = if !replay { prompt_replay(1)? } else { false };
            if replay_game1 {
                println!("\n--- Replaying Game 1 ---\n");
                game::intents_game::play_game(true, client).await?;
            }

            println!("\n--- Game 2: The Vault ---\n");
            game::envelope_game::play_game(replay, client).await?;

            // Prompt for replay after Game 2
            let replay_game2 = if !replay { prompt_replay(2)? } else { false };
            if replay_game2 {
                println!("\n--- Replaying Game 2 ---\n");
                game::envelope_game::play_game(true, client).await?;
            }
        }
        _ => {
            return Err(messages::error::CommandError::InvalidInput(
                "Invalid game selection".to_string(),
            ));
        }
    }

    // Write verification instructions if both games have been completed
    maybe_write_verification_instructions()?;

    // Display verification summary
    display_verification_summary();

    println!("\n========================================");
    println!("         Prove Game Complete!          ");
    println!("========================================\n");

    Ok(())
}

/// Show the game selection menu and return the user's choice.
fn show_game_menu() -> messages::success::CommandResult<u8> {
    println!("Select a game:");
    println!("  1. The Blind Oracle - Test constraint-based access");
    println!("  2. The Vault - Test password-based encryption");
    println!("  3. Play both games\n");

    let choice = rpassword::prompt_password("Enter your choice (1, 2, or 3): ")
        .map_err(|e| messages::error::CommandError::Io(e.to_string()))?;

    match choice.trim() {
        "1" => Ok(1),
        "2" => Ok(2),
        "3" => Ok(3),
        _ => {
            println!("Invalid choice, defaulting to Game 1.");
            Ok(1)
        }
    }
}

/// Generate a unique session ID for the prove game.
pub fn generate_session_id() -> String {
    use uuid::Uuid;
    format!(
        "prove-game-{}",
        Uuid::new_v4()
            .to_string()
            .split('-')
            .next()
            .unwrap_or("session")
    )
}

/// Prompt the user for input.
pub fn prompt_user(message: &str) -> messages::success::CommandResult<String> {
    println!("{}", message);
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .map_err(|e| messages::error::CommandError::Io(e.to_string()))?;
    Ok(input.trim().to_string())
}

/// Prompt the user for a number.
pub fn prompt_number(message: &str) -> messages::success::CommandResult<u64> {
    let input = prompt_user(message)?;
    input
        .parse::<u64>()
        .map_err(|_| messages::error::CommandError::InvalidInput("Please enter a valid number".to_string()))
}
