# Mastra

## Install

```bash
npm install mastra
```

## Config

```typescript
const edge = new MastraMCPClient({
  name: 'edge',
  server: { command: 'edge', args: ['--api-key', 'sk-...'] },
});
const tools = await edge.tools();
```

## Verify

```bash
edge --api-key sk-YOUR_KEY search tokens --chainId 8453 --query DOGE
```

## Patterns

See [agent-patterns.md](../agent-patterns.md) for correct usage patterns.
