# Inspect

Nine views for in-depth token and pair analysis.

## Purpose

Access detailed analytics on tokens and trading pairs. Nine specialized views provide different lenses: token fundamentals, pair liquidity/volume, holder concentration, on-chain activity, and pair lifecycle.

## Views

### View 1: Token overview

Returns core token metadata and canonical pair address.

**Inputs**: `chainId`, `address`

**Output**:
- `address` — token address
- `symbol` — token symbol
- `name` — token name
- `decimals` — decimal places
- `pairAddress` — canonical trading pair (use for pair-scoped calls)
- `totalSupply` — total token supply
- `owner` — contract owner (null if renounced)
- `deployed` — deployment timestamp

### View 2: Pair metrics

Current price, volume, and liquidity snapshot.

**Inputs**: `chainId`, `pairAddress`

**Output**:
- `price` — current price in native token (number)
- `priceUsd` — current price in USD (number)
- `liquidity` — total USD liquidity (number)
- `volume1h` — 1-hour trading volume USD
- `volume24h` — 24-hour trading volume USD
- `mcap` — market cap USD (null if unknown)
- `spread` — bid-ask spread as decimal (0.01 = 1%)

### View 3: Token holders

Holder concentration analysis, sniper/insider detection.

**Inputs**: `chainId`, `tokenAddress`, `limit` (optional, default 100)

**Output**:
```typescript
[
  {
    address: string;
    balance: string; // wei
    percentage: number; // 0–100
    isSniper: boolean;
    isInsider: boolean;
    txCount: number;
  }
]
```

### View 4: Token analytics

Activity metrics and trend indicators.

**Inputs**: `chainId`, `tokenAddress`

**Output**:
- `uniqueBuyers24h` — unique buyer addresses in 24h
- `uniqueSellers24h` — unique seller addresses in 24h
- `buyVolume24h` — buy-side volume USD
- `sellVolume24h` — sell-side volume USD
- `txCount24h` — total transactions in 24h
- `priceChange1h` — price % change in 1h
- `priceChange24h` — price % change in 24h

### View 5: Graduation

Pair graduation history (DEX → CEX listing).

**Inputs**: `chainId`, `pairAddress`

**Output**:
```typescript
{
  status: "pending" | "graduated" | "failed";
  graduatedAt: string | null; // ISO 8601 timestamp
  exchange: string | null; // e.g., "Binance"
  symbol: string | null;
}
```

### View 6: Pair overview

Advanced pair metadata and pool composition.

**Inputs**: `chainId`, `pairAddress`

**Output**:
- `pairAddress` — pair contract address
- `tokenA` — first token in pair
- `tokenB` — second token in pair
- `reserve0` — reserve of token A
- `reserve1` — reserve of token B
- `totalLiquidity` — USD value
- `exchangeName` — DEX name (e.g., "Uniswap V3")

### View 7: Pair candles

OHLC candlestick data for charting.

**Inputs**: `chainId`, `pairAddress`, `period` (1m|5m|1h|4h|1d), `limit` (optional, default 100)

**Output**:
```typescript
[
  {
    time: number; // Unix timestamp
    open: number;
    high: number;
    low: number;
    close: number;
    volume: number; // USD
  }
]
```

### View 8: Pair swaps

Recent swap history for the pair.

**Inputs**: `chainId`, `pairAddress`, `limit` (optional, default 50)

**Output**:
```typescript
[
  {
    txHash: string;
    timestamp: number; // Unix timestamp
    maker: string; // trader address
    tokenIn: string;
    amountIn: string; // wei
    tokenOut: string;
    amountOut: string; // wei
    priceImpact: number; // 0–1 (0.05 = 5%)
  }
]
```

### View 9: Token holders (analytics)

Historical holder concentration trend.

**Inputs**: `chainId`, `tokenAddress`, `period` (1d|7d|30d), `limit` (optional, default 10)

**Output**:
```typescript
[
  {
    snapshotDate: string; // ISO 8601
    topHolderPercent: number; // top holder % of supply
    top10Percent: number; // top 10 holders
    concentration: number; // Herfindahl index (0–1)
  }
]
```

---

## Examples

### Example 1: Get token basics

**Natural language**: Look up Ethereum USDC token metadata

```json
{
  "method": "market.getToken",
  "params": {
    "chainId": "1",
    "address": "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48"
  }
}
```

**Response excerpt**:
```json
{
  "result": {
    "address": "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48",
    "symbol": "USDC",
    "name": "USD Coin",
    "decimals": 6,
    "pairAddress": "0x88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640",
    "totalSupply": "46000000000000",
    "owner": null,
    "deployed": 1533424661
  }
}
```

### Example 2: Check pair metrics before trading

**Natural language**: Get current price and liquidity for Base USDC/ETH

```json
{
  "method": "market.getPairMetrics",
  "params": {
    "chainId": "8453",
    "pairAddress": "0x7f5c764cbc14f9669b88837ca1490cca17c31607"
  }
}
```

**Response excerpt**:
```json
{
  "result": {
    "price": 1.002,
    "priceUsd": 1.002,
    "liquidity": 12500000,
    "volume1h": 450000,
    "volume24h": 8900000,
    "mcap": null,
    "spread": 0.005
  }
}
```

### Example 3: Analyze holder distribution

**Natural language**: Check top holders and sniper flags for a token

```json
{
  "method": "market.getToken",
  "params": {
    "chainId": "8453",
    "address": "0x...",
    "view": "token_holders",
    "limit": 20
  }
}
```

**Response excerpt**:
```json
{
  "result": [
    {
      "address": "0xdead000000000000000000000000000000000000",
      "balance": "1000000000000000000000000",
      "percentage": 35.5,
      "isSniper": false,
      "isInsider": false,
      "txCount": 1
    },
    {
      "address": "0x123456...",
      "balance": "500000000000000000000",
      "percentage": 12.2,
      "isSniper": true,
      "isInsider": false,
      "txCount": 2
    }
  ]
}
```

### Example 4: Get candlesticks for charting

**Natural language**: Fetch 1-hour candles for the last 24 hours

```json
{
  "method": "market.getPairCandles",
  "params": {
    "chainId": "42161",
    "pairAddress": "0xc6f780497a95e61da4313261c692a3c2fb0d5f78",
    "period": "1h",
    "limit": 24
  }
}
```

**Response excerpt**:
```json
{
  "result": [
    {
      "time": 1702000800,
      "open": 1.0045,
      "high": 1.0062,
      "low": 1.0032,
      "close": 1.0055,
      "volume": 2300000
    }
  ]
}
```

### Example 5: Check recent swaps and price impact

**Natural language**: See latest swaps on a pair to estimate slippage

```json
{
  "method": "market.getSwaps",
  "params": {
    "chainId": "8453",
    "pairAddress": "0x7f5c764cbc14f9669b88837ca1490cca17c31607",
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
      "maker": "0x1234567890123456789012345678901234567890",
      "tokenIn": "0x...",
      "amountIn": "1000000000000000000",
      "tokenOut": "0x...",
      "amountOut": "995000000000000000",
      "priceImpact": 0.005
    }
  ]
}
```

## Mistakes

| Error | Cause | Fix |
|-------|-------|-----|
| `NOT_FOUND` | Token/pair does not exist on chain | Verify chain ID and address are correct; use search first |
| `INVALID_ADDRESS` | Address not checksummed or malformed | Use search to get canonical address |
| View returns `null` | Data not available or too new | Try again after ~10 minutes; check volume first |
| `RATE_LIMIT` | Too many requests | Batch queries; cache results for 1–5 minutes |

## See also

- [Price before order](../agent-patterns.md#pattern-1-price-before-order) — use pair_metrics to compute target price
- [Token → pair](../agent-patterns.md#pattern-2-token--pair) — token_overview returns pairAddress
- [search](./search.md) — find tokens first
- [screen](./screen.md) — filter tokens by multiple metrics
- [trade](./trade.md) — place order after analysis
