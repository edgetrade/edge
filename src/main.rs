use clap::{Parser, Subcommand};
use std::process;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;

mod client;
mod manifest;
mod server;
mod subscriptions;
mod urls;

use manifest::McpManifest;
use server::EdgeServer;
use urls::{DOCS_BASE_URL, IRIS_API_URL};

#[derive(Parser)]
#[command(name = "edge")]
#[command(
    about = "Edge Trade MCP server — connects AI agents to real-time market data, portfolio tracking, and trading."
)]
#[command(long_about = None)]
struct Cli {
    #[arg(
        long,
        env = "EDGE_API_KEY",
        help = "Edge API key (or set EDGE_API_KEY env var). Get one at https://edge.trade"
    )]
    api_key: Option<String>,

    #[arg(
        long,
        default_value = "stdio",
        help = "Transport: stdio (default) or http. Use stdio for Cursor/Claude Desktop; use http to serve over a local port."
    )]
    transport: String,

    #[arg(
        long,
        default_value = "127.0.0.1",
        help = "Host address to bind when using --transport http"
    )]
    host: Option<String>,

    #[arg(long, default_value = "3000", help = "Port to listen on when using --transport http")]
    port: u16,

    #[arg(long, help = "Open the Edge Trade documentation in your browser")]
    docs: bool,

    #[arg(long, help = "Print available MCP tools as JSON and exit")]
    list_tools: bool,

    #[arg(long, help = "Ping the Edge API and exit with 0 on success")]
    ping: bool,

    #[arg(long, help = "Print verbose connection and request logs to stderr")]
    verbose: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Manage Edge Trade skills")]
    Skill {
        #[command(subcommand)]
        command: SkillCommand,
    },
}

#[derive(Subcommand)]
enum SkillCommand {
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

async fn fetch_manifest(url: &str, api_key: &str) -> McpManifest {
    let client = reqwest::Client::new();
    let deadline = tokio::time::Instant::now() + Duration::from_secs(180);
    let mut delay = Duration::from_secs(1);
    loop {
        match client.get(url).bearer_auth(api_key).send().await {
            Ok(r) if r.status().is_success() => {
                return r.json::<McpManifest>().await.unwrap_or_else(|e| {
                    eprintln!("[edge] manifest parse error: {e}");
                    process::exit(1);
                });
            }
            Ok(r) => eprintln!("[edge] manifest fetch failed: HTTP {}", r.status()),
            Err(e) => eprintln!("[edge] manifest fetch error: {e}"),
        }
        if tokio::time::Instant::now() + delay > deadline {
            eprintln!("[edge] could not reach iris after 3 minutes. Exiting.");
            process::exit(1);
        }
        tokio::time::sleep(delay).await;
        delay = (delay * 2).min(Duration::from_secs(30));
    }
}

async fn fetch_manifest_raw(url: &str, api_key: &str) -> Result<Vec<u8>, reqwest::Error> {
    reqwest::Client::new()
        .get(url)
        .bearer_auth(api_key)
        .send()
        .await?
        .bytes()
        .await
        .map(|b| b.to_vec())
}

fn sha256(data: &[u8]) -> Vec<u8> {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    if cli.docs {
        eprintln!("Edge Trade Documentation: {}", DOCS_BASE_URL);
        if let Ok(browser) = std::env::var("BROWSER") {
            let _ = process::Command::new(browser).arg(DOCS_BASE_URL).spawn();
        }
        return;
    }

    if cli.ping {
        let iris_url = std::env::var("EDGE_IRIS_URL").unwrap_or_else(|_| IRIS_API_URL.to_string());
        let ping_url = format!("{}/ping", iris_url);

        if cli.verbose {
            eprintln!("[edge] pinging {}", ping_url);
        }

        match reqwest::Client::new().get(&ping_url).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    if cli.verbose {
                        eprintln!("[edge] ping successful: {}", response.status());
                    }
                    process::exit(0);
                } else {
                    eprintln!("Ping failed with status: {}", response.status());
                    process::exit(1);
                }
            }
            Err(e) => {
                eprintln!("Ping failed: {}", e);
                process::exit(1);
            }
        }
    }

    let api_key = cli.api_key.unwrap_or_else(|| {
        eprintln!("Error: API key required. Set EDGE_API_KEY or use --api-key");
        eprintln!("See: {}/authentication", DOCS_BASE_URL);
        process::exit(1);
    });

    let iris_url = std::env::var("EDGE_IRIS_URL").unwrap_or_else(|_| IRIS_API_URL.to_string());
    let manifest_url = format!("{}/mcp/manifest", iris_url);

    let manifest = fetch_manifest(&manifest_url, &api_key).await;

    if cli.list_tools {
        println!("{}", serde_json::to_string_pretty(&manifest.tools).unwrap());
        return;
    }

    if let Some(Commands::Skill { command }) = &cli.command {
        match command {
            SkillCommand::List => {
                for skill in &manifest.skills {
                    println!("{}: {}", skill.name, skill.description);
                }
            }
            SkillCommand::Install { name, path } => match manifest.skills.iter().find(|s| &s.name == name) {
                Some(skill) => {
                    let dir = std::path::Path::new(path).join(name);
                    if let Err(e) = std::fs::create_dir_all(&dir) {
                        eprintln!("[edge] failed to create directory: {}", e);
                        process::exit(1);
                    }
                    if let Err(e) = std::fs::write(dir.join("SKILL.md"), &skill.content) {
                        eprintln!("[edge] failed to write skill: {}", e);
                        process::exit(1);
                    }
                    eprintln!("[edge] installed skill '{}' to {}", name, dir.display());
                }
                None => {
                    eprintln!("[edge] skill '{}' not found in manifest", name);
                    process::exit(1);
                }
            },
        }
        return;
    }

    let shared_manifest = Arc::new(RwLock::new(manifest));

    let server = EdgeServer::new(&iris_url, &api_key, shared_manifest.clone(), cli.verbose)
        .await
        .unwrap_or_else(|e| {
            eprintln!("Failed to connect to Iris: {}", e);
            process::exit(1);
        });

    {
        let manifest_url = manifest_url.clone();
        let api_key = api_key.clone();
        let manifest_ref = shared_manifest.clone();
        let initial_body = {
            let guard = manifest_ref.read().await;
            serde_json::to_vec(&*guard).unwrap_or_default()
        };
        let mut current_hash = sha256(&initial_body);
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(Duration::from_secs(60)).await;
                match fetch_manifest_raw(&manifest_url, &api_key).await {
                    Ok(body) => {
                        let new_hash = sha256(&body);
                        if new_hash != current_hash {
                            match serde_json::from_slice::<McpManifest>(&body) {
                                Ok(new_manifest) => {
                                    *manifest_ref.write().await = new_manifest;
                                    current_hash = new_hash;
                                    eprintln!("[edge] manifest reloaded");
                                }
                                Err(e) => eprintln!("[edge] manifest parse error after update: {e}"),
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("[edge] heartbeat: could not reach iris: {e} — serving cached manifest")
                    }
                }
            }
        });
    }

    let result = match cli.transport.as_str() {
        "stdio" => server.serve_stdio().await,
        "sse" | "http" => {
            let host = cli.host.unwrap_or_else(|| "127.0.0.1".to_string());
            if cli.transport == "sse" {
                eprintln!("[edge] --transport sse is deprecated, use --transport http");
            }
            server.serve_http(&host, cli.port).await
        }
        _ => {
            eprintln!("Unknown transport: {}. Use stdio or http", cli.transport);
            process::exit(1);
        }
    };

    if let Err(e) = result {
        eprintln!("MCP server error: {}", e);
        process::exit(1);
    }
}
