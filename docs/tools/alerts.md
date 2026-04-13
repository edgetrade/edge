# Alerts

Subscribe, poll, unsubscribe; webhook delivery supported.

## Purpose

Real-time and polling-based subscriptions to market events, portfolio updates, and order fills. Supports both WebSocket (push) and HTTP webhook (push) delivery, with polling fallback for stateless agents.

## Alert Types

| Type | Inputs | Description |
|------|--------|-------------|
| `pair_state` | chainId, pairAddress | Price, liquidity, and volume updates for a pair |
| `pair_metrics` | chainId, pairAddress, interval (1m/5m/1h) | OHLC candle closes |
| `pair_swaps` | chainId, pairAddress | Individual swap events on a pair |
| `token_updates` | chainId, tokenAddress | Token metadata changes and activity spikes |
| `token_holders` | chainId, tokenAddress | Holder concentration changes, whale movements |
| `wallet_swaps` | walletAddresses (array) | Any trade from monitored wallets |
| `portfolio_updates` | walletAddress, chainId (optional) | Holdings changes, new positions |
| `memescope` | filters (optional) | New meme token listings and activity |
| `order_updates` | walletAddress | Order state changes (submitted, filled, cancelled) |

## Subscription Lifecycle

### Step 1: Subscribe (once per session)

```json
{
  "method": "alerts.subscribe",
  "params": {
    "alert_type": "price",
    "chainId": "8453",
    "address": "0x..."
  }
}
```

Returns: `{ subscription_id: "sub_abc123" }`

### Step 2: Poll (each agent turn)

```json
{
  "method": "alerts.poll",
  "params": {
    "subscription_id": "sub_abc123"
  }
}
```

Returns: Array of buffered events (max 1000 per poll)

### Step 3: Unsubscribe (on cleanup)

```json
{
  "method": "alerts.unsubscribe",
  "params": {
    "subscription_id": "sub_abc123"
  }
}
```

## Inputs (by method)

### Subscribe

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| alert_type | enum | Yes | — | Type from table above |
| chainId | string | Conditional | — | Required for most types (check table) |
| address | string | Conditional | — | Token/pair address (check type) |
| walletAddress | string | Conditional | — | For portfolio/order alerts |
| walletAddresses | array | Conditional | — | For wallet_swaps (multiple) |
| interval | string | Conditional | — | For pair_metrics: `1m`, `5m`, `1h` |
| filters | object | Conditional | — | For memescope: `{ mcap_range: [min, max], ... }` |

### Poll

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| subscription_id | string | Yes | — | From subscribe response |

### Register webhook

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| alert_type | enum | Yes | — | Type to monitor |
| webhookUrl | string | Yes | — | HTTP(S) endpoint to POST to |
| webhookSecret | string | No | — | HMAC secret for validation |
| chainId | string | Conditional | — | Chain filter |
| address | string | Conditional | — | Token/pair filter |
| threshold | number | No | — | % change threshold for triggers |

## Output

### Subscribe returns

```typescript
{
  subscription_id: string;
  status: "active" | "pending";
  createdAt: number;
  expiresAt: number; // session timeout
}
```

### Poll returns

```typescript
[
  {
    event: string; // e.g., "pair_state_update"
    data: any; // event-specific payload
    timestamp: number;
  }
]
```

### Webhook POST body

```typescript
{
  event: string;
  data: any;
  timestamp: number;
  signature: string; // HMAC-SHA256(body, secret)
}
```

## Examples

### Example 1: Subscribe to pair price updates

**Natural language**: Watch Base USDC/ETH pair for price changes

```json
{
  "method": "alerts.subscribe",
  "params": {
    "alert_type": "pair_state",
    "chainId": "8453",
    "pairAddress": "0x7f5c764cbc14f9669b88837ca1490cca17c31607"
  }
}
```

**Response**:

```json
{
  "result": {
    "subscription_id": "sub_pair123",
    "status": "active",
    "createdAt": 1702000650,
    "expiresAt": 1702084650
  }
}
```

### Example 2: Poll for buffered events

**Natural language**: Check for new price updates

```json
{
  "method": "alerts.poll",
  "params": {
    "subscription_id": "sub_pair123"
  }
}
```

**Response excerpt**:

```json
{
  "result": [
    {
      "event": "pair_state_update",
      "data": {
        "price": 1.0055,
        "liquidity": 12600000,
        "volume24h": 8950000
      },
      "timestamp": 1702000720
    }
  ]
}
```

### Example 3: Subscribe to token activity spikes

**Natural language**: Alert on memescope: new trending meme tokens

```json
{
  "method": "alerts.subscribe",
  "params": {
    "alert_type": "memescope",
    "filters": {
      "mcap_range": [100000, 10000000],
      "min_volume_24h": 50000
    }
  }
}
```

**Response**:

```json
{
  "result": {
    "subscription_id": "sub_meme456",
    "status": "active",
    "createdAt": 1702000650,
    "expiresAt": 1702084650
  }
}
```

### Example 4: Portfolio webhook (push delivery)

**Natural language**: Send portfolio changes to my server

```json
{
  "method": "alerts.registerWebhook",
  "params": {
    "alert_type": "portfolio_updates",
    "webhookUrl": "https://my-agent-server.com/webhooks/portfolio",
    "webhookSecret": "whsec_abc123xyz789",
    "walletAddress": "0x1234567890123456789012345678901234567890",
    "chainId": "8453"
  }
}
```

**Response**:

```json
{
  "result": {
    "subscription_id": "webhook_pf789",
    "status": "active",
    "createdAt": 1702000650,
    "expiresAt": 1702084650
  }
}
```

### Example 5: Order fill notifications

**Natural language**: Get alerts when orders fill

```json
{
  "method": "alerts.subscribe",
  "params": {
    "alert_type": "order_updates",
    "walletAddress": "0x1234567890123456789012345678901234567890"
  }
}
```

**Response**:

```json
{
  "result": {
    "subscription_id": "sub_orders789",
    "status": "active",
    "createdAt": 1702000650,
    "expiresAt": 1702084650
  }
}
```

Then poll:

```json
{
  "method": "alerts.poll",
  "params": {
    "subscription_id": "sub_orders789"
  }
}
```

**Response excerpt**:

```json
{
  "result": [
    {
      "event": "order_updated",
      "data": {
        "orderId": "order_abc123",
        "status": "filled",
        "filledAmount": "100000000000000000000",
        "filledPrice": "510000000000000000"
      },
      "timestamp": 1702000720
    }
  ]
}
```

## Mistakes

| Error | Cause | Fix |
|-------|-------|-----|
| `SUBSCRIPTION_EXPIRED` | Session timed out (default 24h) | Re-subscribe; configure longer timeout if needed |
| `BUFFER_FULL` | 1000+ events queued (ring buffer capped) | Poll more frequently |
| `WEBHOOK_DELIVERY_FAILED` | Endpoint returned 4xx/5xx | Check webhook URL and ensure endpoint is reachable |
| `INVALID_FILTER` | Filter schema mismatch | Check filter format for alert type |
| `RATE_LIMIT` | Too many concurrent subscriptions | Consolidate subscriptions; unsubscribe unused |

## See also

- [Subscription lifecycle](../agent-patterns.md#pattern-4-subscription-lifecycle) — subscribe once, poll each turn, cleanup
- [Memescope](../agent-patterns.md#pattern-6-memescope) — snapshot + live feed
- [trade](./trade.md) — place orders on alert triggers
- [portfolio](./portfolio.md) — monitor holdings changes
