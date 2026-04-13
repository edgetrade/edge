# @edgedottrade/edge

Decentralized server for Edge - blockchain trading and market intelligence for AI agents.

## Installation

```bash
npm install -g @edgedottrade/edge
```

Or use with npx (no installation required):

```bash
npx @edgedottrade/edge --help
```

## Usage

### With Claude Desktop

Add to your Claude Desktop config:

```json
{
  "mcpServers": {
    "edge": {
      "command": "npx",
      "args": ["-y", "@edgedottrade/edge", "--api-key", "sk-your-key-here"]
    }
  }
}
```

### With Cursor

Add to your Cursor MCP config:

```json
{
  "mcpServers": {
    "edge": {
      "command": "npx",
      "args": ["-y", "@edgedottrade/edge", "--api-key", "sk-your-key-here"]
    }
  }
}
```

### Direct Usage

```bash
edge --api-key sk-your-key-here
```

## What's Included

This package automatically downloads the appropriate `edge` binary for your platform:

- Linux (x86_64, aarch64)
- macOS (Intel, Apple Silicon)
- Windows (x86_64)

The binary is downloaded from [GitHub Releases](https://github.com/edgetrade/mcp/releases) during installation.

## Tools

- **search** - Find tokens by name or address
- **inspect** - Token/pair analytics with 9 views
- **screen** - Filter tokens by market metrics
- **portfolio** - Holdings, PnL, and trade history
- **trade** - Limit orders and strategies
- **alerts** - Real-time price and order alerts

## Documentation

Full docs: [https://docs.edge.trade/agents](https://docs.edge.trade/agents)

## License

MIT