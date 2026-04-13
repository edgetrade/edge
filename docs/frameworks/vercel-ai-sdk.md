# Vercel AI SDK

## Installation

```bash
npm install ai @ai-sdk/openai
```

## Configuration

```typescript
import { experimental_createMCPClient as createMCPClient } from 'ai';
import { Experimental_StdioMCPTransport as StdioTransport } from 'ai/mcp-stdio';

const client = await createMCPClient({
  transport: new StdioTransport({ 
    command: 'npx',
    args: ['-y', '@edgedottrade/mcp', '--api-key', 'sk-your-key-here']
  }),
});

const tools = await client.tools();
```

## Verify

Test the connection:

```typescript
const result = await tools.search({ query: 'BONK' });
console.log(result);
```

## Next Steps

See [Agent Patterns](../agent-patterns.md) for common usage patterns.
