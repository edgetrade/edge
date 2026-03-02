# Screen

Filter tokens by mcap, liquidity, sniper %, insider %, social presence.

## Purpose

Batch-filter tokens across a chain by multiple criteria (market cap, liquidity, holder concentration). Returns paginated results ranked by relevance. Use to identify gems or filter out risky tokens.

## Inputs

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| chainId | string | Yes | — | Numeric chain ID: `"1"`, `"8453"`, `"42161"` |
| minLiquidity | number | No | 0 | Minimum USD liquidity |
| maxLiquidity | number | No | Infinity | Maximum USD liquidity |
| minMcap | number | No | 0 | Minimum market cap USD |
| maxMcap | number | No | Infinity | Maximum market cap USD |
| maxSniperPercent | number | No | 100 | Filter: top holder sniper % (0–100) |
| maxInsiderPercent | number | No | 100 | Filter: insider holder % (0–100) |
| minVolume24h | number | No | 0 | Minimum 24h volume USD |
| includeMemescope | boolean | No | false | Include meme/community tokens |
| limit | number | No | 50 | Results per page (1–1000) |
| offset | number | No | 0 | Pagination offset |

## Output

Returns array of tokens with:
- `address` — token address
- `symbol` — token symbol
- `name` — token name
- `liquidity` — total USD liquidity
- `mcap` — market cap USD
- `volume24h` — 24h trading volume
- `topHolderPercent` — top holder % of supply
- `sniperPercent` — sniper addresses %
- `insiderPercent` — insider addresses %
- `socialScore` — community strength (0–100, null if unknown)

## Examples

### Example 1: Screen for low-cap gems (Base chain)

**Natural language**: Find Base tokens under $10M market cap with good liquidity

```json
{
  "method": "market.screenTokens",
  "params": {
    "chainId": "8453",
    "minLiquidity": 100000,
    "maxMcap": 10000000,
    "minVolume24h": 50000,
    "maxSniperPercent": 30,
    "limit": 20
  }
}
```

**Response excerpt**:
```json
{
  "result": [
    {
      "address": "0x1234567890123456789012345678901234567890",
      "symbol": "GEM",
      "name": "Gem Token",
      "liquidity": 250000,
      "mcap": 5000000,
      "volume24h": 180000,
      "topHolderPercent": 15,
      "sniperPercent": 5,
      "insiderPercent": 10,
      "socialScore": 72
    }
  ]
}
```

### Example 2: Exclude risky holder patterns

**Natural language**: Arbitrum tokens with healthy distribution (no sniper concentration)

```json
{
  "method": "market.screenTokens",
  "params": {
    "chainId": "42161",
    "minLiquidity": 500000,
    "maxSniperPercent": 10,
    "maxInsiderPercent": 15,
    "limit": 15
  }
}
```

**Response excerpt**:
```json
{
  "result": [
    {
      "address": "0x...",
      "symbol": "SAFE",
      "name": "Safe Token",
      "liquidity": 2100000,
      "mcap": 35000000,
      "volume24h": 5600000,
      "topHolderPercent": 8,
      "sniperPercent": 2,
      "insiderPercent": 5,
      "socialScore": 88
    }
  ]
}
```

### Example 3: Memescope snapshot

**Natural language**: Find trending meme tokens on Base

```json
{
  "method": "market.screenTokens",
  "params": {
    "chainId": "8453",
    "includeMemescope": true,
    "minVolume24h": 100000,
    "limit": 50
  }
}
```

**Response excerpt**:
```json
{
  "result": [
    {
      "address": "0xmeme1234567890123456789012345678901234",
      "symbol": "DOGE2",
      "name": "Doge 2.0",
      "liquidity": 450000,
      "mcap": 2500000,
      "volume24h": 820000,
      "topHolderPercent": 22,
      "sniperPercent": 12,
      "insiderPercent": 8,
      "socialScore": 45
    }
  ]
}
```

### Example 4: Premium tokens with social proof

**Natural language**: Ethereum tokens with strong community and established liquidity

```json
{
  "method": "market.screenTokens",
  "params": {
    "chainId": "1",
    "minLiquidity": 5000000,
    "minMcap": 50000000,
    "maxSniperPercent": 5,
    "limit": 10,
    "offset": 0
  }
}
```

**Response excerpt**:
```json
{
  "result": [
    {
      "address": "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48",
      "symbol": "USDC",
      "name": "USD Coin",
      "liquidity": 50000000,
      "mcap": 33000000000,
      "volume24h": 45000000,
      "topHolderPercent": 12,
      "sniperPercent": 1,
      "insiderPercent": 2,
      "socialScore": 98
    }
  ]
}
```

## Mistakes

| Error | Cause | Fix |
|-------|-------|-----|
| Empty array | Criteria too restrictive | Loosen min/max ranges; check if chain has activity |
| `INVALID_CHAIN` | Chain ID not supported | Use numeric strings: `"1"`, `"8453"`, `"42161"` |
| `RATE_LIMIT` | Too many requests | Cache results for 5–10 minutes between calls |
| `socialScore: null` | Data not aggregated yet | Try again later; score updates every 24h |

## See also

- [Memescope](../agent-patterns.md#pattern-6-memescope) — snapshot + live feed via alerts
- [inspect](./inspect.md) — deep-dive on individual tokens
- [search](./search.md) — find specific tokens by name
- [alerts](./alerts.md) — subscribe to new token listings
