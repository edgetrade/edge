# Trade

Limit orders, entry/exit strategies, price impact.

## Purpose

Place and manage limit orders. Includes entry/exit strategy framework for automated trading on price targets. Requires API key authentication.

## Price-Resolution Workflow

Before placing orders, the system must resolve execution prices from multiple sources to protect against MEV and slippage.

**Workflow**:

1. **Agent submits order** with target price (`price` field)
2. **System fetches current pair metrics** from DEX (Uniswap V3, etc.)
3. **Compare prices**: if market price is better, upgrade to market price
4. **Validate slippage**: ensure target price is within slippage tolerance
5. **Place order** on DEX via SDK integration

**Example**:
```
Agent: "Buy 10 USDC worth of GEM at $0.50"
→ System: Pair metrics show current price is $0.48
→ Resolved: Buy at $0.48 (better than target)
→ Order placed with slippage limit 1%
```

## Inputs (varies by operation)

### Place order

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| chainId | string | Yes | — | Numeric chain ID |
| pair | string | Yes | — | Pair address (not token; use inspect to find) |
| side | enum | Yes | — | `"buy"` or `"sell"` |
| quantity | string | Yes | — | Amount to trade (wei format) |
| price | string | Yes | — | Target price (wei format) |
| slippage | number | No | 0.01 | Slippage tolerance (0.01 = 1%) |
| expiresAt | number | No | +1h | Order expiry (Unix timestamp) |

### List orders

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| wallet | string | Yes | — | Wallet address |
| chainId | string | No | — | Filter to chain (omit for all) |
| status | enum | No | — | `"open"`, `"filled"`, `"cancelled"` (omit for all) |
| limit | number | No | 50 | Results per page |

### Cancel order

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| orderId | string | Yes | — | Order ID to cancel |

### Entry strategy

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| name | string | Yes | — | Strategy name |
| pair | string | Yes | — | Pair address |
| chainId | string | Yes | — | Numeric chain ID |
| entryPrice | string | Yes | — | Trigger price (wei) |
| quantity | string | Yes | — | Amount to buy |
| slippage | number | No | 0.01 | Slippage tolerance |

### Exit strategy

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| name | string | Yes | — | Strategy name |
| pair | string | Yes | — | Pair address |
| exitPrice | string | Yes | — | Trigger price (wei) |
| quantity | string | Yes | — | Amount to sell |
| profitTarget | number | No | — | Optional % gain target |

## Output

### Place order returns

```typescript
{
  orderId: string;
  status: "pending" | "submitted" | "filled" | "cancelled";
  filledAmount: string; // wei
  filledPrice: string; // wei
  resolvedPrice: string; // price after resolution workflow
  timestamp: number;
}
```

### List orders returns

```typescript
[
  {
    orderId: string;
    chainId: string;
    pair: string;
    side: "buy" | "sell";
    quantity: string;
    price: string;
    filledAmount: string;
    status: "open" | "filled" | "cancelled";
    createdAt: number;
    expiresAt: number;
  }
]
```

## Examples

### Example 1: Place simple limit order

**Natural language**: Buy 100 tokens at $0.50 on Base

```json
{
  "method": "orders.place",
  "params": {
    "chainId": "8453",
    "pair": "0x7f5c764cbc14f9669b88837ca1490cca17c31607",
    "side": "buy",
    "quantity": "100000000000000000000",
    "price": "500000000000000000",
    "slippage": 0.01
  }
}
```

**Response excerpt**:
```json
{
  "result": {
    "orderId": "order_abc123",
    "status": "submitted",
    "filledAmount": "100000000000000000000",
    "filledPrice": "510000000000000000",
    "resolvedPrice": "510000000000000000",
    "timestamp": 1702000650
  }
}
```

### Example 2: Sell existing holdings

**Natural language**: Sell 5 WETH at market on Ethereum

```json
{
  "method": "orders.place",
  "params": {
    "chainId": "1",
    "pair": "0x88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640",
    "side": "sell",
    "quantity": "5000000000000000000",
    "price": "2000000000000000000000",
    "slippage": 0.02
  }
}
```

**Response excerpt**:
```json
{
  "result": {
    "orderId": "order_def456",
    "status": "filled",
    "filledAmount": "5000000000000000000",
    "filledPrice": "2010000000000000000000",
    "resolvedPrice": "2010000000000000000000",
    "timestamp": 1702000700
  }
}
```

### Example 3: List open orders

**Natural language**: Show all pending orders on Base

```json
{
  "method": "orders.list",
  "params": {
    "wallet": "0x1234567890123456789012345678901234567890",
    "chainId": "8453",
    "status": "open"
  }
}
```

**Response excerpt**:
```json
{
  "result": [
    {
      "orderId": "order_abc123",
      "chainId": "8453",
      "pair": "0x7f5c764cbc14f9669b88837ca1490cca17c31607",
      "side": "buy",
      "quantity": "100000000000000000000",
      "price": "500000000000000000",
      "filledAmount": "0",
      "status": "open",
      "createdAt": 1702000650,
      "expiresAt": 1702004250
    }
  ]
}
```

### Example 4: Create entry strategy (DCA-style)

**Natural language**: Set up auto-buy: buy 50 tokens if price drops below $0.45

```json
{
  "method": "orders.createEntryStrategy",
  "params": {
    "name": "DCA Buy",
    "pair": "0x7f5c764cbc14f9669b88837ca1490cca17c31607",
    "chainId": "8453",
    "entryPrice": "450000000000000000",
    "quantity": "50000000000000000000",
    "slippage": 0.01
  }
}
```

**Response excerpt**:
```json
{
  "result": {
    "strategyId": "strat_xyz789",
    "status": "active",
    "createdAt": 1702000650
  }
}
```

### Example 5: Create exit strategy (take-profit)

**Natural language**: Auto-sell if price reaches $1.50 or profit hits 50%

```json
{
  "method": "orders.createExitStrategy",
  "params": {
    "name": "Take Profit 50%",
    "pair": "0x7f5c764cbc14f9669b88837ca1490cca17c31607",
    "exitPrice": "1500000000000000000",
    "quantity": "100000000000000000000",
    "profitTarget": 50
  }
}
```

**Response excerpt**:
```json
{
  "result": {
    "strategyId": "strat_tp123",
    "status": "active",
    "createdAt": 1702000750
  }
}
```

## Mistakes

| Error | Cause | Fix |
|-------|-------|-----|
| `NOT_IMPLEMENTED` | Execution not yet live | Trading coming soon; see status updates |
| `INSUFFICIENT_BALANCE` | Wallet lacks enough tokens | Check portfolio holdings first |
| `SLIPPAGE_EXCEEDED` | Price moved beyond tolerance | Increase slippage or retry with better price |
| `PAIR_NOT_FOUND` | Pair address invalid | Use inspect to find canonical pair |
| `ORDER_EXPIRED` | Order timed out waiting for fill | Reduce `expiresAt` duration; check liquidity |

## See also

- [Price before order](../agent-patterns.md#pattern-1-price-before-order) — always check metrics first
- [inspect](./inspect.md) — pair_metrics for current prices
- [portfolio](./portfolio.md) — check holdings before selling
- [alerts](./alerts.md) — subscribe to order fill notifications
