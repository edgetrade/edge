use std::process;

use std::path::PathBuf;

use crate::domains::config::Config;
use crate::messages;
use crate::utils::urls::EDGE_MCP_URL;

use super::cli::Cli;

#[derive(Clone)]
pub struct AppClientCredentials {
    pub api_key: String,
    pub iris_url: String,
    pub verbose: bool,
}

pub async fn parse_api_credentials(cli: &Cli) -> AppClientCredentials {
    // Priority: CLI arg > EDGE_API_KEY env var > config file (loaded from default or explicit path)
    let api_key = cli
        .api_key
        .clone()
        .or_else(|| std::env::var("EDGE_API_KEY").ok())
        .or_else(|| {
            let config_path = Some(PathBuf::from(&cli.config));
            Config::load(config_path).ok().and_then(|c| c.api_key)
        })
        .unwrap_or_else(|| {
            messages::error::api_key_required();
            messages::error::api_key_docs_url();
            process::exit(1);
        });

    let iris_url = std::env::var("EDGE_MCP_URL").unwrap_or_else(|_| EDGE_MCP_URL.to_string());

    AppClientCredentials {
        api_key,
        iris_url,
        verbose: cli.verbose,
    }
}
