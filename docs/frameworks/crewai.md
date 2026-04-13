# CrewAI

## Install

```bash
pip install crewai
```

## Config

```python
trader = Agent(
    role="Crypto Trader",
    goal="Execute profitable trades on-chain",
    llm=LLM(model="gpt-4o"),
    mcps=[{ "command": "edge", "args": ["--api-key", "sk-..."] }]
)
```

## Verify

```bash
edge --api-key sk-YOUR_KEY search tokens --chainId 8453 --query DOGE
```

## Patterns

See [agent-patterns.md](../agent-patterns.md) for correct usage patterns.
