//! Shared CLI definitions for Edge Trade binaries.
//!
//! This module provides the common CLI structure used by both

use clap::{Parser, Subcommand};

/// Common CLI structure for all Edge Trade binaries.
#[derive(Parser)]
#[command(name = "edge")]
#[command(
    about = "Edge Trade MCP server — connects AI agents to real-time market data, portfolio tracking, and trading."
)]
#[command(long_about = None)]
pub struct Cli {
    #[arg(
        long,
        global = true,
        env = "EDGE_API_KEY",
        help = "Edge API key (or set EDGE_API_KEY env var). Get one at https://edge.trade"
    )]
    pub api_key: Option<String>,

    #[arg(
        long,
        default_value = "stdio",
        help = "Transport: stdio (default) or http. Use stdio for Cursor/Claude Desktop; use http to serve over a local port."
    )]
    pub transport: String,

    #[arg(
        long,
        global = true,
        env = "EDGE_CONFIG",
        help = "Path to configuration file (default: platform-specific XDG config directory)"
    )]
    pub config: Option<String>,

    #[arg(long, global = true, help = "Print verbose connection and request logs to stderr")]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// Common commands available in all Edge Trade binaries.
#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Serve the MCP server over HTTP")]
    Server {
        #[arg(long, default_value = "127.0.0.1", help = "Host address to bind")]
        host: String,
        #[arg(long, default_value = "3000", help = "Port to listen on")]
        port: u16,
        #[arg(
            long,
            default_value = "mcp",
            help = "Path prefix for the HTTP endpoint (e.g. mcp → /mcp)"
        )]
        path: String,
    },
    #[command(
        about = "Manage Edge keys which are used to encrypt the messages and information sent to our servers.\nThese keys are important but they are not your wallet keys."
    )]
    Key {
        #[command(subcommand)]
        command: Option<KeyCommand>,
    },
    #[command(
        about = "Manage Edge wallets that your agent will have access to.\nYour wallet will never be in a position where it will be able to do anything without your approval."
    )]
    Wallet {
        #[command(subcommand)]
        command: Option<WalletCommand>,
    },
    #[command(about = "Manage Edge's skills")]
    Skill {
        #[command(subcommand)]
        command: Option<SkillCommand>,
    },
    #[command(about = "Print available MCP tools as JSON and exit")]
    ListTools,
    #[command(about = "Ping the Edge API and exit on success")]
    Ping,
    #[command(about = "Print version information and exit")]
    Version,
}

/// Key management commands.
#[derive(Subcommand)]
pub enum KeyCommand {
    #[command(about = "Create new key configuration")]
    Create,
    #[command(about = "Unlock the session (only available for keystore sessions)")]
    Unlock,
    #[command(about = "Lock the session (only available for keystore sessions)")]
    Lock,
    #[command(about = "Update authentication (change password)")]
    Update,
    #[command(about = "Delete key configuration")]
    Delete,
}

/// Wallet management commands.
#[derive(Subcommand)]
pub enum WalletCommand {
    #[command(about = "Create a new wallet")]
    Create {
        #[arg(long, help = "Blockchain type (evm or svm)")]
        chain_type: String,
        #[arg(long, help = "Optional wallet name")]
        name: Option<String>,
    },
    #[command(about = "Import a wallet from private key file or manual input")]
    Import {
        #[arg(long, help = "Blockchain type (evm or svm)")]
        chain_type: String,
        #[arg(long, help = "Optional wallet name")]
        name: Option<String>,
        #[arg(long)]
        key_file: Option<String>,
    },
    #[command(about = "List wallets for the agent")]
    List,
    #[command(about = "Delete a wallet")]
    Delete {
        #[arg(long, help = "Wallet address")]
        address: String,
    },
    #[command(about = "Play the prove game to test wallet security constraints")]
    Prove {
        #[arg(long, help = "Game to play: 1 (Blind Oracle), 2 (The Vault), or both")]
        game: Option<u8>,
        #[arg(long, help = "Replay the last game without creating new intents")]
        replay: bool,
    },
}

/// Skill management commands.
#[derive(Subcommand)]
pub enum SkillCommand {
    #[command(about = "List available skills from the manifest")]
    List,
    #[command(about = "Install a skill to a local directory")]
    Install {
        /// Name of the skill to install
        name: String,
        /// Directory to install into (writes <dir>/<name>/SKILL.md)
        #[arg(long)]
        path: String,
    },
}
