use clap::{Parser, Subcommand};
use std::process;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;

mod alerts;
mod client;
mod manifest;
mod server;
mod subscriptions;
mod urls;

use manifest::McpManifest;
use server::{EdgeServer, inject_local_agent_actions, inject_local_resources};
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
        global = true,
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

    #[arg(long, global = true, help = "Print verbose connection and request logs to stderr")]
    verbose: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Print version information and exit")]
    Version,
    #[command(about = "Print available MCP tools as JSON and exit")]
    ListTools,
    #[command(about = "Ping the Edge API and exit with 0 on success")]
    Ping,
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

    if matches!(cli.command, Some(Commands::Version)) {
        let pkg_version = env!("CARGO_PKG_VERSION");
        let sha = option_env!("VERGEN_GIT_SHA").unwrap_or("unknown");
        let short_sha = &sha[..sha.len().min(7)];
        let describe = option_env!("VERGEN_GIT_DESCRIBE").unwrap_or("");
        // describe is empty when there's no tag reachable; when it matches a
        // tag exactly it's just the tag name, otherwise "tag-N-gSHA".
        if describe.is_empty() || describe.starts_with(short_sha) {
            println!("edge {pkg_version} (commit {short_sha})");
        } else {
            println!("edge {pkg_version} ({describe}, commit {short_sha})");
        }
        return;
    }

    if matches!(cli.command, Some(Commands::Ping)) {
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

    if matches!(cli.command, Some(Commands::ListTools)) {
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
                                Ok(mut new_manifest) => {
                                    inject_local_agent_actions(&mut new_manifest);
                                    inject_local_resources(&mut new_manifest);
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

    let result = match cli.command {
        Some(Commands::Server { host, port, path }) => server.serve_http(&host, port, &path).await,
        None if cli.transport == "sse" => {
            eprintln!("[edge] --transport sse is deprecated, use the server subcommand");
            server.serve_http("127.0.0.1", 3000, "mcp").await
        }
        None if cli.transport == "http" => server.serve_http("127.0.0.1", 3000, "mcp").await,
        None => server.serve_stdio().await,
        _ => unreachable!(),
    };

    if let Err(e) = result {
        eprintln!("MCP server error: {}", e);
        process::exit(1);
    }
}
