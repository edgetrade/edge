# OpenAI Agents SDK

## Install

```bash
pip install openai
```

## Config

```python
# Option A: local binary
async with MCPServerStdio(command="edge", args=["--api-key", "sk-..."]) as mcp:
    agent = Agent(name="Trader", mcp_servers=[mcp])

# Option B: hosted (OpenAI calls the server)
agent = Agent(name="Trader", tools=[HostedMCPTool(tool_config={
    "type": "mcp", "server_label": "edge",
    "server_url": "https://decerver.edge.trade/mcp",
    "headers": { "Authorization": "Bearer sk-..." }
})])
```

## Verify

```bash
edge --api-key sk-YOUR_KEY search tokens --chainId 8453 --query DOGE
```

## Patterns

See [agent-patterns.md](../agent-patterns.md) for correct usage patterns.
