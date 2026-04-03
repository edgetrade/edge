//! Application runner
//!
//! Entry point for CLI vs daemon mode execution

use std::path::PathBuf;

use clap::Parser;

use crate::app::cli::{Cli, Commands, Transport};
use crate::app::handler::{handle_order, handle_ping, handle_version};
use crate::app::orchestrator::{App, Command, CommandOutput, KeyCommand, WalletCommand};
use crate::domains::mcp::TransportType;

/// Parse CLI transport into orchestrator TransportType
fn parse_transport(transport: Transport) -> Result<TransportType, Box<dyn std::error::Error>> {
    match transport {
        Transport::Stdio => Ok(TransportType::Stdio),
        Transport::Http => Ok(TransportType::Http {
            host: "127.0.0.1".to_string(),
            port: 3000,
        }),
    }
}

/// Run the application
///
/// Entry point that parses CLI and either:
/// - Runs a single CLI command and exits
/// - Starts the daemon and runs until shutdown
pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // Parse config path
    let config_path = if cli.config.is_empty() {
        None
    } else {
        Some(PathBuf::from(&cli.config))
    };

    // Clone config_path for potential later use
    let config_path_clone = config_path.clone();

    // Initialize App orchestrator with iris_url from CLI
    let app = App::new(config_path).await?;

    if cli.daemon {
        // Run as persistent daemon
        app.run_daemon().await?;
    } else {
        // Run CLI command
        let output = match cli.command {
            Some(Commands::Order { command }) => {
                // Get config for session creation
                let config = crate::domains::config::Config::load(config_path_clone.clone())
                    .map_err(|e| format!("Failed to load config: {}", e))?;

                // Create session
                let session = crate::domains::keystore::Session::new(config);

                // Get IrisClient from client handle
                let client = app
                    .client
                    .get_client()
                    .await
                    .map_err(|e| format!("Failed to get client: {}", e))?
                    .ok_or("Client not connected")?;

                // Call handle_order with ? operator for proper error handling
                return handle_order(&command, &session, &client)
                    .await
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error>);
            }
            Some(Commands::Key { command }) => {
                let cmd = command.ok_or("Key command required")?;
                let key_cmd = parse_key_command(cmd)?;
                app.run_command(Command::Key(key_cmd)).await?
            }
            Some(Commands::Wallet { command }) => {
                let cmd = command.ok_or("Wallet command required")?;
                let wallet_cmd = parse_wallet_command(cmd)?;
                app.run_command(Command::Wallet(wallet_cmd)).await?
            }
            Some(Commands::Serve { args, command: _ }) => {
                let transport = parse_transport(args.transport)?;
                app.run_command(Command::Serve(transport)).await?
            }
            Some(Commands::ListTools) => {
                // ListTools not yet implemented in orchestrator
                return Err("ListTools command not yet implemented in daemon mode".into());
            }
            Some(Commands::Skill { command: _ }) => {
                // handle_skill(&command, &app.manifest)?;
                // TODO: Implement skill command
                CommandOutput::Success
            }
            Some(Commands::Ping) => {
                handle_ping(cli.verbose).await?;
                CommandOutput::Success
            }
            Some(Commands::Version) => {
                handle_version()?;
                CommandOutput::Success
            }
            None => {
                // No command - print help
                return Err("No command specified. Use --help for usage information.".into());
            }
        };

        // Output result
        match output {
            CommandOutput::Success => {}
            CommandOutput::Error(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
            CommandOutput::Data(data) => {
                println!("{}", serde_json::to_string_pretty(&data)?);
            }
        }
    }

    Ok(())
}

/// Parse CLI key command into orchestrator KeyCommand
fn parse_key_command(cmd: crate::app::cli::KeyCommand) -> Result<KeyCommand, Box<dyn std::error::Error>> {
    use crate::app::cli::KeyCommand as CliKeyCmd;

    match cmd {
        CliKeyCmd::Create => Err("Key create not yet implemented".into()),
        CliKeyCmd::Unlock => {
            // TODO: Get password from user input
            Err("Key unlock requires interactive password input".into())
        }
        CliKeyCmd::Lock => Ok(KeyCommand::Lock),
        CliKeyCmd::Update => Err("Key update not yet implemented".into()),
        CliKeyCmd::Delete => Err("Key delete not yet implemented".into()),
    }
}

/// Parse CLI wallet command into orchestrator WalletCommand
fn parse_wallet_command(cmd: crate::app::cli::WalletCommand) -> Result<WalletCommand, Box<dyn std::error::Error>> {
    use crate::app::cli::WalletCommand as CliWalletCmd;

    match cmd {
        CliWalletCmd::List => Ok(WalletCommand::List),
        CliWalletCmd::Create { chain_type, name } => Ok(WalletCommand::Create {
            chain: chain_type,
            name: name.unwrap_or_else(|| "default".to_string()),
        }),
        CliWalletCmd::Import {
            chain_type,
            name,
            key_file,
        } => {
            let private_key = if let Some(path) = key_file {
                std::fs::read_to_string(path)?
            } else {
                // TODO: Get from interactive input
                return Err("Private key input required".into());
            };
            Ok(WalletCommand::Import {
                chain: chain_type,
                name: name.unwrap_or_else(|| "imported".to_string()),
                private_key: private_key.trim().to_string(),
            })
        }
        CliWalletCmd::Delete { address } => Ok(WalletCommand::Delete { name: address }),
        CliWalletCmd::Prove { .. } => Err("Prove game not yet implemented".into()),
    }
}
