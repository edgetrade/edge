# Agent Patterns

## Pattern 1: Price before order

**Problem**: Placing a limit order requires computing the exact target price. You need current market data first.

**Correct sequence**:

1. `inspect pair_metrics` — fetch current price, spread, 24h range
2. Compute target price (e.g., buy 5% below current, or sell 5% above)
3. `trade place` — submit order at computed price

**Code example**:

```typescript
const metrics = await iris.inspect.pair_metrics({ pairAddress: "0xabc...", chainId: "8453" });
const buyTarget = metrics.price * 0.95; // 5% below
const order = await iris.trade.place({
  chainId: "8453",
  pair: "0xabc...",
  side: "buy",
  quantity: "10",
  price: buyTarget.toString(),
});
```

---

## Pattern 2: Token → pair

**Problem**: You have a token address but need pair-scoped data (candles, swaps, metrics).

**Correct sequence**:

1. `inspect token_overview` — returns `pairAddress` for the canonical pair
2. Use that `pairAddress` in subsequent calls: `pair_metrics`, `pair_candles`, `pair_swaps`

**Code example**:

```typescript
const tokenData = await iris.inspect.token_overview({ address: "0x1234...", chainId: "8453" });
const pairAddress = tokenData.pairAddress; // Extract from response

const candles = await iris.inspect.pair_candles({ pairAddress, chainId: "8453", period: "1h" });
const swaps = await iris.inspect.pair_swaps({ pairAddress, chainId: "8453", limit: 50 });
```

---

## Pattern 3: Chain ID format

**Problem**: Chain identifiers must be numeric strings, not names.

**Correct format**:

- Ethereum: `"1"`
- Base: `"8453"`
- Arbitrum: `"42161"`

**Common mistake**: Using `"base"` or `"ethereum"` will return NOT_FOUND or INVALID_CHAIN_ID.

**Code example**:

```typescript
// ✅ Correct
await iris.search.tokens({ chainId: "8453", query: "DOGE" });

// ❌ Wrong
await iris.search.tokens({ chainId: "base", query: "DOGE" }); // NOT_FOUND
```

---

## Pattern 4: Subscription lifecycle

**Problem**: Subscriptions must be explicitly managed to avoid resource leaks and missed events.

**Correct sequence**:

1. `alerts subscribe` — create subscription, save `subscription_id`
2. On each agent turn: `alerts poll` — fetch buffered events
3. On cleanup (session end): `alerts unsubscribe` — release subscription

**Code example**:

```typescript
// At session start
const sub = await iris.alerts.subscribe({ alert_type: "price_change", chainId: "8453" });
const subscriptionId = sub.id;

// Each turn
const events = await iris.alerts.poll({ subscription_id: subscriptionId });
for (const event of events) {
  // Process event
}

// At session end
await iris.alerts.unsubscribe({ subscription_id: subscriptionId });
```

---

## Pattern 5: Long-term swaps

**Problem**: Portfolio swap history differs by time horizon. You must choose the right backend.

**Correct usage**:

- `portfolio swaps` with `long_term: false` (default) — Silver TPC backend (~90 days)
- `portfolio swaps` with `long_term: true` — Iceberg backend (all history, higher latency)

**Code example**:

```typescript
// Recent 90 days
const recent = await iris.portfolio.swaps({
  wallet: "0xuser...",
  chainId: "8453",
  long_term: false,
});

// All history (slower)
const allHistory = await iris.portfolio.swaps({
  wallet: "0xuser...",
  chainId: "8453",
  long_term: true,
});
```

---

## Pattern 6: Memescope

**Problem**: Detecting emerging tokens requires both point-in-time snapshots and live feeds.

**Correct sequence**:

1. `screen` — get a snapshot of current meme tokens (one-time fetch)
2. `alerts subscribe` with `alert_type: "memescope"` — get live feed of new listings/activity
3. Use alerts for detection; use `screen` for periodic re-ranking

**Code example**:

```typescript
// Initial snapshot
const snapshot = await iris.screen.memes({ chainId: "8453", limit: 20 });

// Subscribe to live feed
const sub = await iris.alerts.subscribe({ alert_type: "memescope", chainId: "8453" });

// Each turn
const newTokens = await iris.alerts.poll({ subscription_id: sub.id });
for (const token of newTokens) {
  const overview = await iris.inspect.token_overview({ address: token.address, chainId: "8453" });
  // Analyze
}
```
