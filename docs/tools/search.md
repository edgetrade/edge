# Search

Search tokens by name or address.

## Purpose

Find tokens across supported chains by fuzzy-matching token names, symbols, or addresses. Returns paginated results with metadata for further inspection.

## Inputs

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| chainId | string | Yes | — | Numeric chain ID: `"1"` (Ethereum), `"8453"` (Base), `"42161"` (Arbitrum) |
| query | string | Yes | — | Search term: token name, symbol, or address (0x...) |
| limit | number | No | 50 | Max results (1–100) |
| offset | number | No | 0 | Pagination offset |

## Output

Returns array of tokens with:
- `address` — token address (checksummed)
- `symbol` — token symbol
- `name` — token name
- `decimals` — decimal places
- `chainId` — numeric chain ID
- `liquidity` — total USD liquidity (null if unknown)
- `volume24h` — 24h trading volume in USD (null if <1h old)

## Examples

### Example 1: Find USDC

**Natural language**: Search for USDC on Base chain

```json
{
  "method": "market.searchTokens",
  "params": {
    "chainId": "8453",
    "query": "USDC"
  }
}
```

**Response excerpt**:
```json
{
  "result": [
    {
      "address": "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913",
      "symbol": "USDC",
      "name": "USDC Coin",
      "decimals": 6,
      "chainId": "8453",
      "liquidity": 150000000,
      "volume24h": 45000000
    }
  ]
}
```

### Example 2: Search by partial name

**Natural language**: Find all tokens with "dog" in the name on Ethereum

```json
{
  "method": "market.searchTokens",
  "params": {
    "chainId": "1",
    "query": "dog",
    "limit": 10
  }
}
```

**Response excerpt**:
```json
{
  "result": [
    {
      "address": "0x6b175474e89094c44da98b954eedeac495271d0f",
      "symbol": "DAI",
      "name": "Dai Stablecoin",
      "decimals": 18,
      "chainId": "1",
      "liquidity": 2500000000,
      "volume24h": 800000000
    }
  ]
}
```

### Example 3: Search by address

**Natural language**: Look up Arbitrum USDC by its contract address

```json
{
  "method": "market.searchTokens",
  "params": {
    "chainId": "42161",
    "query": "0xFF970A61A04b1cA14834A43f5dE4533eBDDB5F86"
  }
}
```

**Response excerpt**:
```json
{
  "result": [
    {
      "address": "0xFF970A61A04b1cA14834A43f5dE4533eBDDB5F86",
      "symbol": "USDC.e",
      "name": "Bridged USDC",
      "decimals": 6,
      "chainId": "42161",
      "liquidity": 80000000,
      "volume24h": 25000000
    }
  ]
}
```

## Mistakes

| Error | Cause | Fix |
|-------|-------|-----|
| `NOT_FOUND` | Chain ID is invalid or not supported | Use numeric strings: `"1"`, `"8453"`, `"42161"` |
| Empty array | Query is too specific or no matches exist | Broaden search term; try symbol or name fragments |
| `RATE_LIMIT` | Too many requests from this IP/key | Reduce request frequency; batch searches when possible |

## See also

- [Token → pair](../agent-patterns.md#pattern-2-token--pair) — use token address from search result
- [inspect](./inspect.md) — full token analytics after search
- [concepts](../concepts.md) — address formats, chain ID reference
