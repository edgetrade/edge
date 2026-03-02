use clap::{Parser, Subcommand};
use std::process;

mod client;
mod server;
mod subscriptions;
mod types;

use server::EdgeServer;

#[derive(Parser)]
#[command(name = "edge")]
#[command(about = "Edge Trade MCP client", long_about = None)]
struct Cli {
    #[arg(long)]
    api_key: Option<String>,

    #[arg(long, default_value = "wss://api.iris.trade.edge")]
    iris_url: String,

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
}

#[derive(Subcommand)]
enum Commands {
    Help { tool: Option<String> },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    if cli.docs {
        eprintln!("Edge Trade Documentation: https://docs.edge.trade/agents");
        if let Ok(browser) = std::env::var("BROWSER") {
            let _ = process::Command::new(browser)
                .arg("https://docs.edge.trade/agents")
                .spawn();
        }
        return;
    }

    if cli.list_tools {
        print_tools_list();
        return;
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
            eprintln!("See: https://docs.edge.trade/agents/authentication");
            process::exit(1);
        });

    let server = EdgeServer::new(&cli.iris_url, &api_key)
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
    println!("Docs: https://docs.edge.trade/agents");
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
            println!("\nSee: https://docs.edge.trade/agents/tools/search");
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
            println!("\nSee: https://docs.edge.trade/agents/tools/inspect");
        }
        "screen" => {
            println!("screen - Screen tokens by market cap, liquidity, and holder metrics\n");
            println!("See: https://docs.edge.trade/agents/tools/screen");
        }
        "portfolio" => {
            println!("portfolio - View wallet holdings, history, and transactions\n");
            println!("See: https://docs.edge.trade/agents/tools/portfolio");
        }
        "trade" => {
            println!("trade - Place limit orders, manage strategies, estimate impact\n");
            println!("See: https://docs.edge.trade/agents/tools/trade");
        }
        "alerts" => {
            println!("alerts - Subscribe to price alerts and order updates\n");
            println!("See: https://docs.edge.trade/agents/tools/alerts");
        }
        _ => {
            println!("Unknown tool: {}", tool);
            println!("Available tools: search, inspect, screen, portfolio, trade, alerts");
            println!("\nUse 'edge help' to see all tools");
        }
    }
}
