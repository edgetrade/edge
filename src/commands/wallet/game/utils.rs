use crate::messages;

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
