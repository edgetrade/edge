use clap::{Parser, Subcommand};
use std::process;

mod client;
mod server;
mod subscriptions;
mod types;

use server::EdgeServer;
use types::urls::{DOCS_BASE_URL, IRIS_API_URL};

#[derive(Parser)]
#[command(name = "edge")]
#[command(about = "Edge's MCP client", long_about = None)]
#[command(disable_help_subcommand = true)]
struct Cli {
    #[arg(long)]
    api_key: Option<String>,

    #[arg(long, default_value = "stdio")]
    transport: String,

    #[arg(long)]
    host: Option<String>,

    #[arg(long, default_value = "3000")]
    port: u16,

    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(long)]
    docs: bool,

    #[arg(long)]
    list_tools: bool,

    #[arg(long)]
    ping: bool,

    #[arg(long)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    Help { tool: Option<String> },
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

    if cli.list_tools {
        print_tools_list();
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

    if let Some(Commands::Help { tool }) = cli.command {
        if let Some(tool_name) = tool {
            print_tool_help(&tool_name);
        } else {
            print_general_help();
        }
        return;
    }

    let api_key = cli
        .api_key
        .or_else(|| std::env::var("EDGE_API_KEY").ok())
        .unwrap_or_else(|| {
            eprintln!("Error: API key required. Set EDGE_API_KEY or use --api-key");
            eprintln!("See: {}/authentication", DOCS_BASE_URL);
            process::exit(1);
        });

    let iris_url = IRIS_API_URL.to_string();

    let server = EdgeServer::new(&iris_url, &api_key, cli.verbose)
        .await
        .unwrap_or_else(|e| {
            eprintln!("Failed to connect to Iris: {}", e);
            process::exit(1);
        });

    let result = match cli.transport.as_str() {
        "stdio" => server.serve_stdio().await,
        "sse" => {
            let host = cli.host.unwrap_or_else(|| "127.0.0.1".to_string());
            server.serve_sse(&host, cli.port).await
        }
        "http" => {
            let host = cli.host.unwrap_or_else(|| "127.0.0.1".to_string());
            server.serve_http(&host, cli.port).await
        }
        _ => {
            eprintln!("Unknown transport: {}. Use stdio, sse, or http", cli.transport);
            process::exit(1);
        }
    };

    if let Err(e) = result {
        eprintln!("MCP server error: {}", e);
        process::exit(1);
    }
}

fn print_tools_list() {
    let tools = serde_json::json!({
        "tools": [
            {"name": "search", "description": "Search tokens by name or address"},
            {"name": "inspect", "description": "Inspect tokens and pairs with multiple views"},
            {"name": "screen", "description": "Screen tokens by market cap, liquidity, and holder metrics"},
            {"name": "portfolio", "description": "View wallet holdings, history, and transactions"},
            {"name": "trade", "description": "Place limit orders, manage strategies, estimate impact"},
            {"name": "alerts", "description": "Subscribe to price alerts and order updates"}
        ]
    });
    println!("{}", serde_json::to_string_pretty(&tools).unwrap());
}

fn print_general_help() {
    println!("Edge Trade MCP Client\n");
    println!("Transport modes:");
    println!("  --transport stdio  - Standard I/O (default, for Cursor/Claude Desktop)");
    println!("  --transport sse    - Server-Sent Events HTTP server");
    println!("  --transport http   - Streamable HTTP server\n");
    println!("Available tools:");
    println!("  search     - Search tokens by name or address");
    println!("  inspect    - Inspect tokens and pairs (9 views)");
    println!("  screen     - Screen tokens by filters");
    println!("  portfolio  - View wallet holdings and history");
    println!("  trade      - Place orders and manage strategies");
    println!("  alerts     - Subscribe to real-time alerts\n");
    println!("Usage: edge help <tool> for detailed information");
    println!("Docs: {}", DOCS_BASE_URL);
}

fn print_tool_help(tool: &str) {
    match tool {
        "search" => {
            println!("search - Search tokens by name or address\n");
            println!("Parameters:");
            println!("  query: String       - Token name or address to search");
            println!("  chain_id: String?   - Optional chain ID to filter results\n");
            println!("Example:");
            println!(r#"  {{"query": "USDC", "chain_id": "8453"}}"#);
            println!("\nSee: {}/tools/search", DOCS_BASE_URL);
        }
        "inspect" => {
            println!("inspect - Inspect tokens and pairs with multiple views\n");
            println!("Parameters:");
            println!("  chain_id: String    - Chain ID");
            println!("  address: String     - Token or pair address");
            println!("  view: String        - View type (see below)\n");
            println!("Views:");
            println!("  token_overview   - Basic token information");
            println!("  token_holders    - Top holders with sniper/insider flags");
            println!("  token_analytics  - Top traders by PnL");
            println!("  graduation       - Bonding curve graduation status");
            println!("  pair_overview    - Pair details and liquidity");
            println!("  pair_metrics     - Price, volume, and market cap");
            println!("  pair_candles     - OHLC candlestick data");
            println!("  pair_swaps       - Recent swap transactions\n");
            println!("Example:");
            println!(r#"  {{"chain_id": "8453", "address": "0x...", "view": "pair_metrics"}}"#);
            println!("\nSee: {}/tools/inspect", DOCS_BASE_URL);
        }
        "screen" => {
            println!("screen - Screen tokens by market cap, liquidity, and holder metrics\n");
            println!("See: {}/tools/screen", DOCS_BASE_URL);
        }
        "portfolio" => {
            println!("portfolio - View wallet holdings, history, and transactions\n");
            println!("See: {}/tools/portfolio", DOCS_BASE_URL);
        }
        "trade" => {
            println!("trade - Place limit orders, manage strategies, estimate impact\n");
            println!("See: {}/tools/trade", DOCS_BASE_URL);
        }
        "alerts" => {
            println!("alerts - Subscribe to price alerts and order updates\n");
            println!("See: {}/tools/alerts", DOCS_BASE_URL);
        }
        _ => {
            println!("Unknown tool: {}", tool);
            println!("Available tools: search, inspect, screen, portfolio, trade, alerts");
            println!("\nUse 'edge help' to see all tools");
        }
    }
}
