# Continue

## Install

Copy to `.continue/config.json`:

```json
{
  "mcpServers": {
    "edge": {
      "command": "edge",
      "args": ["--api-key", "sk-..."]
    }
  }
}
```

## Verify

```bash
edge --api-key sk-YOUR_KEY search tokens --chainId 8453 --query DOGE
```

## Patterns

See [agent-patterns.md](../agent-patterns.md) for correct usage patterns.
