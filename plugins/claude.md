# Claude Desktop

Install the Edge MCP server as a Claude Desktop plugin for direct access to trading and market intelligence tools in Claude conversations.

## Installation

Add to your Claude Desktop config (`claude_desktop_config.json`):

```json
{
  "mcpServers": {
    "edge": {
      "command": "npx",
      "args": ["-y", "@edgedottrade/edge", "--api-key", "${EDGE_API_KEY}"]
    }
  }
}
```

Set the environment variable:

```bash
export EDGE_API_KEY=sk-your-key-here
```

Restart Claude Desktop. The Edge tools are now available in your conversations.

## Claude Code

For the Claude Code CLI, see the dedicated [Claude Code](../frameworks/claude-code.md) guide. It uses `claude mcp add` instead of editing `claude_desktop_config.json`.

## Available tools

Once connected, Claude has access to 6 tools with 39 actions:

| Tool | What it does |
|------|-------------|
| **search** | Find tokens by name, address, or onchain metrics |
| **inspect** | Token-scoped analytics: pricing, top holders, top traders, dev history |
| **pairs** | Pair-scoped analytics: metrics, candles, swaps |
| **portfolio** | Wallet holdings, PnL, trade history, native balances |
| **trade** | Limit and spot orders, entry/exit strategies |
| **agent** | Encrypted wallet management for the non-custodial flow |

## Example prompts

- "Search for PEPE on Solana"
- "Show me the top holders of this token"
- "Screen for Solana tokens with over $50K liquidity"
- "Check my wallet holdings"
- "Place a limit buy for 0.1 SOL"

## Claude Skills

Edge is also available as a Claude Skill at [clawhub.ai/lexomis/edge](https://clawhub.ai/lexomis/edge).

See [Quick Start](../quick-start.md) for full setup instructions.
