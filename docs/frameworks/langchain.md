# LangChain

## Install

```bash
pip install langchain-mcp-adapters
```

## Config

```python
from langchain_mcp_adapters.client import MultiServerMCPClient

async with MultiServerMCPClient({
    "edge": { "command": "edge", "args": ["--api-key", "sk-..."], "transport": "stdio" }
}) as client:
    tools = client.get_tools()
    agent = create_react_agent(model, tools)
```

## Verify

```bash
edge --api-key sk-YOUR_KEY search tokens --chainId 8453 --query DOGE
```

## Patterns

See [agent-patterns.md](../agent-patterns.md) for correct usage patterns.
