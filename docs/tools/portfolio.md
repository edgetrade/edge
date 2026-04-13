# Portfolio

Holdings, history, top traders, wallet scan, native balances.

## Purpose

Query wallet holdings, historical trades, top traders on specific tokens, and multi-chain native balances. Requires API key authentication.

## Inputs (varies by view)

### Holdings

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| wallet | string | Yes | — | Wallet address (checksummed EVM or base58 SVM) |
| chainId | string | No | — | Filter to specific chain; omit for all chains |

### Historical swaps

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| wallet | string | Yes | — | Wallet address |
| long_term | boolean | No | false | `false` = last 90 days (Silver TPC); `true` = all history (Iceberg, slower) |
| limit | number | No | 100 | Max results (1–1000) |

### Top traders on token

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| chainId | string | Yes | — | Numeric chain ID |
| tokenAddress | string | Yes | — | Token address |
| limit | number | No | 50 | Top N traders to return |

### Native balances

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| wallet | string | Yes | — | Wallet address |

### Wallet scan

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| wallet | string | Yes | — | Wallet address to scan |
| chainId | string | No | — | Filter to specific chain |

## Output

### Holdings returns

```typescript
[
  {
    chainId: string;
    tokenAddress: string;
    symbol: string;
    balance: string; // wei
    valueUsd: number;
    unrealizedPnl: number | null; // USD
    pnlPercent: number | null; // 0–100
  }
]
```

### Historical swaps returns

```typescript
[
  {
    txHash: string;
    timestamp: number; // Unix timestamp
    tokenIn: string;
    amountIn: string; // wei
    tokenOut: string;
    amountOut: string; // wei
    valueUsd: number;
    pnlOnTrade: number | null; // USD
  }
]
```

### Top traders returns

```typescript
[
  {
    address: string;
    txCount: number;
    totalVolumeUsd: number;
    unrealizedGains: number; // USD
    winRate: number; // 0–1
  }
]
```

### Native balances returns

```typescript
{
  [chainId: string]: {
    balance: string; // wei
    valueUsd: number;
  }
}
```

## Examples

### Example 1: Check wallet holdings

**Natural language**: Get all holdings for a wallet on Base

```json
{
  "method": "portfolio.getHoldings",
  "params": {
    "wallet": "0x1234567890123456789012345678901234567890",
    "chainId": "8453"
  }
}
```

**Response excerpt**:
```json
{
  "result": [
    {
      "chainId": "8453",
      "tokenAddress": "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913",
      "symbol": "USDC",
      "balance": "5000000000",
      "valueUsd": 5000,
      "unrealizedPnl": null,
      "pnlPercent": null
    },
    {
      "chainId": "8453",
      "tokenAddress": "0x4200000000000000000000000000000000000006",
      "symbol": "WETH",
      "balance": "1500000000000000000",
      "valueUsd": 3000,
      "unrealizedPnl": 500,
      "pnlPercent": 20
    }
  ]
}
```

### Example 2: Historical swaps (recent 90 days)

**Natural language**: See wallet's last 10 trades

```json
{
  "method": "portfolio.getWalletSwaps",
  "params": {
    "wallet": "0x1234567890123456789012345678901234567890",
    "long_term": false,
    "limit": 10
  }
}
```

**Response excerpt**:
```json
{
  "result": [
    {
      "txHash": "0xabc123...",
      "timestamp": 1702000650,
      "tokenIn": "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913",
      "amountIn": "1000000000",
      "tokenOut": "0x4200000000000000000000000000000000000006",
      "amountOut": "600000000000000000",
      "valueUsd": 1000,
      "pnlOnTrade": 50
    }
  ]
}
```

### Example 3: Top traders on a token

**Natural language**: Who are the best traders in Ethereum USDC?

```json
{
  "method": "portfolio.getTopTraders",
  "params": {
    "chainId": "1",
    "tokenAddress": "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48",
    "limit": 10
  }
}
```

**Response excerpt**:
```json
{
  "result": [
    {
      "address": "0x1234567890123456789012345678901234567890",
      "txCount": 450,
      "totalVolumeUsd": 2500000,
      "unrealizedGains": 125000,
      "winRate": 0.68
    }
  ]
}
```

### Example 4: Multi-chain native balances

**Natural language**: Check wallet's ETH, WBASE, and ARB balances

```json
{
  "method": "portfolio.getNativeBalances",
  "params": {
    "wallet": "0x1234567890123456789012345678901234567890"
  }
}
```

**Response excerpt**:
```json
{
  "result": {
    "1": {
      "balance": "5000000000000000000",
      "valueUsd": 10000
    },
    "8453": {
      "balance": "2000000000000000000",
      "valueUsd": 4000
    },
    "42161": {
      "balance": "1500000000000000000",
      "valueUsd": 3000
    }
  }
}
```

### Example 5: Long-term trade history (all history)

**Natural language**: Get complete swap history for Iceberg analysis

```json
{
  "method": "portfolio.getWalletSwaps",
  "params": {
    "wallet": "0x1234567890123456789012345678901234567890",
    "long_term": true,
    "limit": 500
  }
}
```

**Response excerpt**:
```json
{
  "result": [
    {
      "txHash": "0x...",
      "timestamp": 1680000000,
      "tokenIn": "0x...",
      "amountIn": "5000000000000000000",
      "tokenOut": "0x...",
      "amountOut": "2500000000000000000",
      "valueUsd": 5000,
      "pnlOnTrade": -500
    }
  ]
}
```

## Mistakes

| Error | Cause | Fix |
|-------|-------|-----|
| `UNAUTHORIZED` | API key invalid or missing | Provide valid API key in Authorization header |
| `FORBIDDEN` | Wallet not associated with API key | Register wallet via `agent.registerWallet` first |
| `NOT_FOUND` | Wallet has no activity on chain | Try a different wallet; check chain ID |
| `RATE_LIMIT` | Too many requests | Cache results; batch portfolio calls |
| `long_term: true` returns slowly | Iceberg lookup is expensive | Reduce limit or use `long_term: false` for recent trades |

## See also

- [Long-term swaps](../agent-patterns.md#pattern-5-long-term-swaps) — choose backend (`long_term` flag)
- [trade](./trade.md) — place orders from insights
- [inspect](./inspect.md) — analyze tokens held in portfolio
- [alerts](./alerts.md) — subscribe to portfolio updates
