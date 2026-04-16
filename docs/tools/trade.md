# trade

MCP tool name: `orders` (this page is named `trade.md` historically; the actual tool you invoke is `orders`)

Place and manage orders. Trading is live with a 1% trading fee (10% discount with a referral code).

The orders namespace exposes 16 actions. The two that place new orders are `place_limit_order` (waits for a trigger) and `place_spot_order` (executes immediately).

## Actions

| Action | Purpose |
|--------|---------|
| `place_limit_order` | Place a limit order (triggers on price or market cap) |
| `place_spot_order` | Place a spot order (executes immediately) |
| `list_orders` | List orders for a wallet (filter by status, type, chain, dates) |
| `order` | Fetch a single order by ID |
| `cancel_order` | Cancel a single open order by task ID |
| `cancel_all_orders` | Cancel every active limit order for the caller |
| `apply_entry_strategy` | Attach a saved entry strategy to a token |
| `apply_exit_strategy` | Attach a saved exit strategy to a token |
| `create_entry_strategy` | Create a reusable buy/DCA strategy |
| `create_exit_strategy` | Create a reusable TP/SL strategy |
| `list_entry_strategies` | List your saved entry strategies |
| `list_exit_strategies` | List your saved exit strategies |
| `update_entry_strategy` | Update a saved entry strategy |
| `update_exit_strategy` | Update a saved exit strategy |
| `remove_entry_strategy` | Delete an entry strategy |
| `remove_exit_strategy` | Delete an exit strategy |

## Placing a limit order

Triggers on price or market cap. Both `order` and `envelope` may be required by the server; check the live `tools/list` response on your client for the current schema. The shape of the `order` object:

```json
{"action": "place_limit_order", "schema": 1, "data": {
  "order": {
    "tokenId": {
      "tokenChainId": "solana",
      "tokenContractAddress": "EPjFWdd5Au..."
    },
    "side": "buy",
    "amount": {"type": "native", "value": 100000000},
    "pairAddress": "HWy1jotHpo6U...",
    "counterTokenAddress": null,
    "expiration": 3600,
    "exitStrategyId": null,
    "entryStrategyId": null,
    "wallets": [{"address": "7xKXtg2CW8..."}],
    "txPreset": {
      "method": "normal",
      "slippage": 15,
      "maxBaseGas": 0,
      "priorityGas": 0,
      "bribe": 0,
      "key": "a"
    },
    "triggerTokenPriceUsd": "0.0015",
    "triggerMarketcapUsd": null
  }
}}
```

Set either `triggerTokenPriceUsd` or `triggerMarketcapUsd` (or both for a dual trigger). For immediate execution, use `place_spot_order` instead. Its `order.triggerTokenPriceUsd` is omitted and the order fills against current market price.

## Placing a spot order

```json
{"action": "place_spot_order", "schema": 1, "data": {
  "order": { "...same shape as above..." },
  "envelope": "<encrypted-intent-envelope>"
}}
```

`place_spot_order` requires both `order` and `envelope`. The envelope is an encrypted intent your client constructs to commit to the trade parameters before the server executes. See [Security: How Edge Keeps Your Keys Safe](../../security/how-edge-keeps-your-keys-safe.md) for the model.

## Required `order` fields

- `tokenId.tokenChainId` + `tokenId.tokenContractAddress`: the token you are trading
- `side`: `"buy"` or `"sell"`
- `amount`: an object with `type` and `value`
- `pairAddress`: the pair to route through; resolve via `token_info_with_pricing`
- `wallets`: array of `{address}` (each must be registered to your API key)
- `txPreset`: all six fields required (even if zero). `method`, `slippage`, `maxBaseGas`, `priorityGas`, `bribe`, `key`
- `expiration`: TTL in seconds (e.g., 3600 for one hour)

### Amount types

| `amount.type` | `amount.value` unit |
|---------------|---------------------|
| `native` | Lamports (Solana) or wei (EVM) as a number |
| `token` | Token's smallest denomination |
| `percentage` | 1-100, percent of current holdings |

## Listing orders

```json
{"action": "list_orders", "schema": 1, "data": {
  "status": "Working",
  "type": "limit",
  "limit": 20,
  "chainId": "solana",
  "tokenAddresses": [],
  "wallets": [],
  "taskIds": [],
  "includeHidden": false,
  "withToken": false
}}
```

The response wraps results in `{items: [], next}`; unwrap with `data.items`.

Valid `status`: `Working`, `Placing`, `Filled`, `Cancelled`, `Expired`, `Rejected`.

## Strategies

Entry strategies fire buys when price drops by set percentages (DCA). Exit strategies fire sells at take-profit or stop-loss levels. Strategies are reusable: create one, then `apply_entry_strategy` or `apply_exit_strategy` to attach it to a specific token.

```json
// Entry strategy: DCA on dips. Note nested under `entry_strategy`.
{"action": "create_entry_strategy", "schema": 1, "data": {
  "entry_strategy": {
    "name": "DCA Buy",
    "chainId": "solana",
    "steps": [
      {"buyAmountNativeToken": 50000000,  "percentToTrigger": 10},
      {"buyAmountNativeToken": 100000000, "percentToTrigger": 25}
    ]
  }
}}

// Exit strategy: TP + SL. Nested under `exit_strategy`. No chainId.
{"action": "create_exit_strategy", "schema": 1, "data": {
  "exit_strategy": {
    "name": "TP+SL",
    "steps": [{
      "tpPercentToTrigger": 100,
      "tpPercentOfBagToSell": 50,
      "tpPercentOfCostToSell": null,
      "slPercentToTrigger": 30,
      "slPercentToSell": 100
    }]
  }
}}

// Then attach to a token
{"action": "apply_entry_strategy", "schema": 1, "data": {
  "entryStrategyId": "<from create response>",
  "tokenChainId": "solana",
  "tokenContractAddress": "EPjFWdd5Au..."
}}
```

`percentToTrigger` is a positive number representing a percentage move.

See [Concepts: Strategy](../concepts.md#strategy) for the step shape.

## Common errors

| Error | Cause | Fix |
|-------|-------|-----|
| `INSUFFICIENT_BALANCE` | Wallet lacks enough tokens | Check `wallet_holdings` first |
| `SLIPPAGE_EXCEEDED` | Price moved beyond tolerance | Increase `txPreset.slippage` or retry |
| `PAIR_NOT_FOUND` | Invalid pair address | Resolve via `token_info_with_pricing` |
| `ORDER_EXPIRED` | Order timed out | Increase `expiration` or verify liquidity |
| `INVALID_CHAIN` | Bad chain ID | EVM uses numbers, Solana uses `"solana"` |

## See also

- [Agent Patterns: Price before order](../agent-patterns.md#pattern-1-price-before-order)
- [Concepts: Order / Strategy](../concepts.md#order)
- [portfolio](./portfolio.md): check holdings before selling
