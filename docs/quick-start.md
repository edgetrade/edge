# Quick Start

## Installation

### Option 1: npm (easiest)

```bash
npx @edgedottrade/mcp --help
```

No installation required! The npm wrapper automatically downloads the correct binary for your platform.

### Option 2: Claude Desktop Plugin

```bash
/plugin marketplace add edgedottrade/mcp
/plugin install edge@edgedottrade/mcp
```

Configure your API key when prompted, and the plugin will be ready to use.

### Option 3: OpenClaw Skill

```bash
claw skill install edge
```

Or add to your `clawhub.yaml`:

```yaml
skills:
  - edge
```

### Option 4: cargo

```bash
cargo install edge-trade
```

### Option 5: From source

```bash
git clone https://github.com/edgetrade/mcp.git
cd mcp
cargo build --release -p edge-trade
```

## Get API Key

Visit [https://app.trade.edge/settings/api-keys](https://app.trade.edge/settings/api-keys) to create an API key.

## Configuration

### Claude Desktop

If you installed via npm or cargo (not using the plugin):

```json
{
  "mcpServers": {
    "edge": {
      "command": "npx",
      "args": ["-y", "@edgedottrade/mcp", "--api-key", "sk-your-key-here"]
    }
  }
}
```

Or with cargo:

```json
{
  "mcpServers": {
    "edge": {
      "command": "edge",
      "args": ["--api-key", "sk-your-key-here"]
    }
  }
}
```

### Cursor

Add to your MCP settings:

```json
{
  "mcpServers": {
    "edge": {
      "command": "npx",
      "args": ["-y", "@edgedottrade/mcp", "--api-key", "sk-your-key-here"]
    }
  }
}
```

### Continue

Add to your `config.json`:

```json
{
  "mcpServers": {
    "edge": {
      "command": "npx",
      "args": ["-y", "@edgedottrade/mcp", "--api-key", "sk-your-key-here"]
    }
  }
}
```

## First Tool Call

Test the installation:

```bash
npx @edgedottrade/mcp --api-key sk-your-key-here help search
```

Or if installed via cargo:

```bash
edge --api-key sk-your-key-here help search
```

With the plugin installed, just ask your agent to:

```md
Search for tokens on Base
```
