use clap::CommandFactory;
use tyche_enclave::types::chain_type::ChainType;

use crate::app::cli::{OrderCommand, Transport};
use crate::commands;
use crate::commands::key::filestore::{
    key_create as filestore_create, key_delete as filestore_delete, key_lock as filestore_lock,
    key_unlock as filestore_unlock, key_update as filestore_update,
};
use crate::commands::key::keyring::{keyring_create, keyring_delete, keyring_lock, keyring_unlock, keyring_update};
use crate::commands::serve::mcp::EdgeServer;
use crate::config::Config;
use crate::error::PoseidonError;
use crate::manifest::McpManifest;
use crate::messages;
use crate::orders;
use crate::session::Session;
use crate::utils::urls::EDGE_MCP_URL;

use super::cli::{Cli, KeyCommand, ServeArgs, SkillCommand, WalletCommand};

pub async fn serve(args: &ServeArgs, server: EdgeServer) -> Result<(), PoseidonError> {
    match args.transport {
        Transport::Http => server
            .serve_http(&args.host, &args.port, &args.path)
            .await
            .map_err(|e| PoseidonError::Command(format!("MCP server error: {}", e))),
        Transport::Stdio => server
            .serve_stdio()
            .await
            .map_err(|e| PoseidonError::Command(format!("MCP server error: {}", e))),
    }
}

pub async fn handle_order(
    command: &Option<OrderCommand>,
    session: &Session,
    client: &crate::client::IrisClient,
) -> Result<(), PoseidonError> {
    match command {
        Some(OrderCommand::PlaceSpot {
            side,
            size,
            chain,
            token,
        }) => orders::place_spot(side, size, chain, token, session, client)
            .await
            .map_err(PoseidonError::from),
        None => {
            // Print help when no subcommand is provided
            let cmd = Cli::command();
            let sub = cmd
                .find_subcommand("order")
                .expect("order subcommand exists");
            println!("{}", sub.clone().render_long_help());
            Ok(())
        }
    }
}

pub struct KeyCommandArgs {
    pub command: Option<KeyCommand>,
    pub config: Config,
    pub client: crate::client::IrisClient,
    pub session: Session,
}

pub async fn handle_key(args: KeyCommandArgs) -> Result<(), PoseidonError> {
    match args.command {
        Some(KeyCommand::Create) => match args.session {
            Session::Keyring(_) => keyring_create(args.config),
            Session::File(_) => filestore_create(),
        },
        Some(KeyCommand::Unlock) => match args.session {
            Session::Keyring(_) => keyring_unlock(),
            Session::File(_) => filestore_unlock(args.config),
        },
        Some(KeyCommand::Lock) => match args.session {
            Session::Keyring(_) => keyring_lock(),
            Session::File(_) => filestore_lock(args.config),
        },
        Some(KeyCommand::Update) => match args.session {
            Session::Keyring(_) => keyring_update(args.config, &args.client).await,
            Session::File(_) => filestore_update(args.config, &args.client).await,
        },
        Some(KeyCommand::Delete) => match args.session {
            Session::Keyring(_) => keyring_delete(),
            Session::File(_) => filestore_delete(),
        },
        None => {
            // Print help when no subcommand is provided
            let cmd = Cli::command();
            let sub = cmd.find_subcommand("key").expect("key subcommand exists");
            println!("{}", sub.clone().render_long_help());
            Ok(())
        }
    }
}

pub async fn handle_wallet(
    command: &Option<WalletCommand>,
    session: &Session,
    client: &crate::client::IrisClient,
) -> Result<(), PoseidonError> {
    match command {
        Some(WalletCommand::Create { chain_type, name }) => {
            let chain_type = ChainType::parse(chain_type)
                .map_err(|_| PoseidonError::InvalidInput("Invalid chain type".to_string()))?;
            commands::wallet::wallet_create(chain_type, name.clone(), session, client).await
        }
        Some(WalletCommand::Import {
            chain_type,
            name,
            key_file,
        }) => {
            let chain_type = ChainType::parse(chain_type)
                .map_err(|_| PoseidonError::InvalidInput("Invalid chain type".to_string()))?;
            commands::wallet::wallet_import(chain_type, name.clone(), key_file.clone(), session, client).await
        }
        Some(WalletCommand::List) => commands::wallet::wallet_list(client).await,
        Some(WalletCommand::Delete { address }) => commands::wallet::wallet_delete(address.clone(), client).await,
        Some(WalletCommand::Prove { game }) => commands::wallet::wallet_prove(*game, session, client).await,
        None => {
            // Print help when no subcommand is provided
            let cmd = Cli::command();
            let sub = cmd
                .find_subcommand("wallet")
                .expect("wallet subcommand exists");
            println!("{}", sub.clone().render_long_help());
            Ok(())
        }
    }
}

pub fn handle_skill(command: &Option<SkillCommand>, manifest: &McpManifest) -> Result<(), PoseidonError> {
    match command {
        Some(SkillCommand::List) => {
            for skill in &manifest.skills {
                messages::success::skill_output(&skill.name, &skill.description);
            }
            Ok(())
        }
        Some(SkillCommand::Install { name, path }) => match manifest.skills.iter().find(|s| &s.name == name) {
            Some(skill) => {
                let dir = std::path::Path::new(path).join(name);
                std::fs::create_dir_all(&dir).map_err(PoseidonError::Io)?;
                std::fs::write(dir.join("SKILL.md"), &skill.content).map_err(PoseidonError::Io)?;
                messages::error::skill_installed(name, &dir.display().to_string());
                Ok(())
            }
            None => {
                messages::error::skill_not_found(name);
                Err(PoseidonError::NotFound(format!("Skill: {}", name)))
            }
        },
        None => {
            // Print help when no subcommand is provided
            let cmd = Cli::command();
            let sub = cmd
                .find_subcommand("skill")
                .expect("skill subcommand exists");
            println!("{}", sub.clone().render_long_help());
            Ok(())
        }
    }
}

pub async fn handle_ping(verbose: bool) -> Result<(), PoseidonError> {
    let iris_url = std::env::var("EDGE_MCP_URL").unwrap_or_else(|_| EDGE_MCP_URL.to_string());
    let ping_url = format!("{}/ping", iris_url);

    if verbose {
        messages::error::pinging(&ping_url);
    }

    match reqwest::Client::new().get(&ping_url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                if verbose {
                    messages::success::ping_success(&response.status().to_string());
                }
                Ok(())
            } else {
                Err(PoseidonError::Command(format!(
                    "Ping failed with status: {}",
                    response.status()
                )))
            }
        }
        Err(e) => Err(PoseidonError::Client(crate::messages::IrisClientError::Connection(
            e.to_string(),
        ))),
    }
}

pub fn handle_version() -> Result<(), PoseidonError> {
    let pkg_version = env!("CARGO_PKG_VERSION");
    let sha = option_env!("VERGEN_GIT_SHA").unwrap_or("unknown");
    let short_sha = &sha[..sha.len().min(7)];
    let describe = option_env!("VERGEN_GIT_DESCRIBE").unwrap_or("");
    if describe.is_empty() || describe.starts_with(short_sha) {
        println!("edge {pkg_version} (commit {short_sha})");
    } else {
        println!("edge {pkg_version} ({describe}, commit {short_sha})");
    }
    Ok(())
}
