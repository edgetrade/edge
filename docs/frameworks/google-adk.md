# Google ADK

## Install

```bash
pip install google-adk
```

## Config

```python
from google.adk.tools.mcp_tool.mcp_toolset import MCPToolset, StdioServerParameters

toolset = MCPToolset(connection_params=StdioServerParameters(
    command="edge", args=["--api-key", "sk-..."]
))
```

## Verify

```bash
edge --api-key sk-YOUR_KEY search tokens --chainId 8453 --query DOGE
```

## Patterns

See [agent-patterns.md](../agent-patterns.md) for correct usage patterns.
