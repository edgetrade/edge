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

## Key Management

The `edge` CLI automatically detects if your OS keyring is available and uses it by default. If unavailable, it falls back to file-based storage.

```bash
# Create key (auto-detects keyring or file storage)
edge key create

# Verify key exists
edge key unlock

# Remove key
edge key lock

# Generate new key
edge key update
```

### Override Config Location

```bash
# Use custom config file
edge --config /path/to/config.toml key create

# Or via environment variable
export EDGE_CONFIG=/path/to/config.toml
edge key create
```

### Force Storage Mode

Edit `~/.config/edge/config.toml`:

```toml
[session]
use_keyring = true   # Force OS keyring
# use_keyring = false  # Force file storage
```

Remove the line to re-trigger auto-detection.

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
