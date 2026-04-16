# Authentication

All Edge Agent API calls require an API key. This page covers how to obtain a key and use it across different connection methods.

## Getting your API key

1. Log in to [edge.trade](https://edge.trade)
2. Go to **Settings** → **API Keys**
3. Click **Create API Key**
4. Copy the key. It is shown only once.

Your key starts with `sk-` and looks like: `sk-xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx`

{% hint style="warning" %}
Store your API key securely. Do not commit it to source code, share it publicly, or include it in client-side applications. If compromised, revoke it immediately and create a new one.
{% endhint %}

## Authentication methods

### MCP server (recommended)

Pass the API key when starting the MCP server:

```bash
npx @edgedottrade/edge --api-key YOUR_API_KEY
```

Or via environment variable:

```bash
export EDGE_API_KEY=sk-your-key-here
npx @edgedottrade/edge
```

### Configuration file

For Claude Desktop, Cursor, and similar tools, add to your MCP config:

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

Using an environment variable (recommended, avoids hardcoding the key):

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

### HTTP Bearer token (MCP Streamable HTTP)

If you need HTTP transport instead of stdio (e.g., connecting from a service mesh), use MCP Streamable HTTP with the Bearer token. The Edge backend is Cloudflare-protected and rejects arbitrary HTTP clients, so use a compliant MCP client library. Hand-rolled `curl` will fail. See [REST / HTTP access](rest-api.md) for the full story.

```
Authorization: Bearer sk-your-key-here
MCP-Protocol-Version: 2025-06-18
Content-Type: application/json
Accept: application/json, text/event-stream
```

## Key management

### OS keyring (automatic)

The Edge CLI automatically uses your OS keyring when available:

```bash
# Store key in keyring
edge key create

# Verify key is stored
edge key unlock

# Remove key from keyring
edge key lock
```

If the keyring is not available, the CLI falls back to file-based storage.

### Config file

Manual override via `~/.config/edge/config.toml`:

```toml
[auth]
api_key = "sk-your-key-here"
use_keyring = false  # Force file storage instead of keyring
```

### Environment variable

Set `EDGE_API_KEY` in your shell profile:

```bash
# ~/.bashrc or ~/.zshrc
export EDGE_API_KEY=sk-your-key-here
```

### Custom config path

Override the config file location:

```bash
edge --config /path/to/custom/config.toml --api-key sk-your-key-here
```

Or via environment variable:

```bash
export EDGE_CONFIG=/path/to/custom/config.toml
```

## Revoking a key

1. Go to **Settings** → **API Keys** in the Edge webapp
2. Find the key you want to revoke
3. Click **Revoke**
4. The key is immediately invalidated. All active sessions using it will fail.

After revoking, create a new key and update your configuration.

## Permissions

All API keys have full read/write access. Wallet-scoped operations require the wallet to be registered to your account in Edge.

## Error responses

| Error | HTTP Status | Meaning |
|-------|------------|---------|
| `UNAUTHORIZED` | 401 | API key is missing or invalid |
| `FORBIDDEN` | 403 | API key is valid but lacks permission for this action (e.g., unregistered wallet) |
| `RATE_LIMIT` | 429 | Too many requests. Reduce frequency. |

## See also

- [Quick Start](quick-start.md): Full setup walkthrough
- [CLI Reference](cli-reference.md): Command-line key management
- [Errors](errors.md): Complete error code reference
