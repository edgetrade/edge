# Cookbook

End-to-end recipes for common agent workflows. Each recipe combines several actions and shows the full sequence. For atomic how-tos, see [Agent Patterns](agent-patterns.md). For per-action parameters, see [Tools](tools/README.md).

All calls use the action envelope `{"action": "...", "schema": 1, "data": {...}}`.

## Recipe index

| Recipe | What it builds |
|--------|----------------|
| [1. Token safety audit](#recipe-1-token-safety-audit) | Full risk report before you buy |
| [2. New-token screener](#recipe-2-new-token-screener) | Find Solana tokens that pass your filters |
| [3. DCA entry strategy](#recipe-3-dca-entry-strategy) | Accumulate a position in steps as price drops |
| [4. Stop-loss exit strategy](#recipe-4-stop-loss-exit-strategy) | Automatic exit on drawdown with take-profit tiers |
| [5. Whale-watching via polling](#recipe-5-whale-watching-via-polling) | Watch wallets by polling `wallet_swaps` |
| [6. Pair monitoring with webhook delivery](#recipe-6-pair-monitoring-with-webhook-delivery) | Poll metrics or receive webapp-configured webhooks |
| [7. Multi-wallet portfolio roll-up](#recipe-7-multi-wallet-portfolio-roll-up) | Consolidated PnL and holdings across every wallet |
| [8. Copy-trade a wallet](#recipe-8-copy-trade-a-wallet) | Mirror another trader's recent swaps into your own wallet |

---

## Recipe 1: Token safety audit

**Goal**: Produce a full risk summary for a token before placing an order.

**Steps**:

1. Resolve the token to its canonical pair.
2. Read top holders for sniper and insider flags.
3. Read the deployer's track record.
4. Read the latest pair metrics and swap activity.

**Calls**:

```json
// 1. Resolve to pair + live price
{"action": "token_info_with_pricing", "schema": 1, "data": {
  "chainId": "solana",
  "tokenContractAddress": "EPjFWdd5Au...",
  "useBestPair": true,
  "pairContractAddress": null
}}
// Save response.pair.pairContractAddress and response.pair.tokenDeployerAddress

// 2. Top holders with sniper/insider flags
{"action": "token_top_holders", "schema": 1, "data": {
  "chainId": "solana",
  "tokenContractAddress": "EPjFWdd5Au...",
  "walletAddresses": null
}}

// 3. Deployer's other tokens (rug history)
{"action": "token_dev_tokens", "schema": 1, "data": {
  "chainId": "solana",
  "deployerAddress": "<from step 1>"
}}

// 4. Recent volume and buy/sell ratio
{"action": "pair_metrics", "schema": 1, "data": {
  "pairChainId": "solana",
  "pairContractAddress": "<from step 1>",
  "interval": "24h"
}}

// 5. Last 50 swaps (note pair_swaps uses chainId + pairAddress, unlike other pair_* calls)
{"action": "pair_swaps", "schema": 1, "data": {
  "chainId": "solana",
  "pairAddress": "<from step 1>",
  "tokenAddress": null,
  "makerAddress": null,
  "fromBlock": null,
  "toBlock": null,
  "limit": 50
}}
```

**What to look for**:

- `isInsider` or `isSniper` flagged on many top holders: high rug risk
- Deployer has many prior tokens with zero survivors: treat as churn
- Buy/sell ratio from `pair_metrics["24h"]` skewed heavily to sells: distribution phase
- Top 10 holders owning most of the supply: concentrated supply

---

## Recipe 2: New-token screener

**Goal**: Find Solana tokens that pass a set of quality filters.

`screen_tokens` is **Solana-only**. On EVM chains the `chainId` filter is silently ignored.

**Steps**:

1. Screen by market cap, liquidity, and sniper cap.
2. Enrich each result with live pricing.

**Calls**:

```json
// 1. Screen (data is an ARRAY, not an object)
{"action": "screen_tokens", "schema": 1, "data": [{
  "sortColumn": "creation_timestamp",
  "limit": 20,
  "filters": {
    "isGraduated": true,
    "minLiquidityUsd": 50000,
    "minHolderCount": 100,
    "maxSnipersHoldingPercentage": 5
  }
}]}

// 2. For each result, enrich with pricing and pair
{"action": "token_info_with_pricing", "schema": 1, "data": {
  "chainId": "solana",
  "tokenContractAddress": "<result.address>",
  "useBestPair": true
}}
```

See [Tools: screen](tools/screen.md) for every available filter.

**Ranking heuristic**: once candidates pass the filters, sort by `volumeUsd24h / marketCapUsd` (velocity) as a tiebreaker.

---

## Recipe 3: DCA entry strategy

**Goal**: Buy a position in steps as price drops. No manual limit-order management.

**Steps**:

1. Get the current pair price.
2. Create an entry strategy with percentage-drop triggers.

**Calls**:

```json
// 1. Current price reference
{"action": "pair_metrics", "schema": 1, "data": {
  "pairChainId": "solana",
  "pairContractAddress": "HWy1jotHpo6U...",
  "interval": "24h"
}}

// 2. Create the entry strategy. Nested under data.entry_strategy (snake_case).
{"action": "create_entry_strategy", "schema": 1, "data": {
  "entry_strategy": {
    "name": "DCA on dips",
    "chainId": "solana",
    "steps": [
      {"buyAmountNativeToken": 50000000,  "percentToTrigger": 5},
      {"buyAmountNativeToken": 100000000, "percentToTrigger": 10},
      {"buyAmountNativeToken": 200000000, "percentToTrigger": 20}
    ]
  }
}}
```

Field notes:

- `buyAmountNativeToken` is an integer in lamports (Solana) or wei (EVM). `50000000` lamports = `0.05 SOL`.
- `percentToTrigger` is positive and represents a price-drop percentage. `10` means "buy when price is 10% lower".
- Entry strategies require `chainId` inside the strategy. Exit strategies do not.

Attach the strategy to a token by calling `apply_entry_strategy` with `strategyId`, `tokenChainId`, and `tokenContractAddress`.

Other actions: `apply_entry_strategy`, `list_entry_strategies`, `update_entry_strategy`, `remove_entry_strategy`.

---

## Recipe 4: Stop-loss exit strategy

**Goal**: Take profit in tiers and cut losses at a floor, automatically.

**Steps**:

1. Confirm you hold the token.
2. Create an exit strategy with TP tiers and a stop-loss step.

**Calls**:

```json
// 1. Confirm position (response wraps the array in data.items)
{"action": "wallet_holdings", "schema": 1, "data": {
  "wallets": ["7xKXtg2CW8..."],
  "filters": {"isInTrade": true, "chainId": "solana"},
  "limit": 100,
  "sortByColumn": "pnlUsd",
  "sortDirection": "desc",
  "cursor": null
}}

// 2. Create the exit strategy. Nested under data.exit_strategy (snake_case).
// Exit strategies do NOT take a chainId.
{"action": "create_exit_strategy", "schema": 1, "data": {
  "exit_strategy": {
    "name": "Tiered take-profit with stop-loss",
    "steps": [
      {"tpPercentToTrigger": 50,  "tpPercentOfBagToSell": 25,  "tpPercentOfCostToSell": null, "slPercentToTrigger": null, "slPercentToSell": null},
      {"tpPercentToTrigger": 100, "tpPercentOfBagToSell": 50,  "tpPercentOfCostToSell": null, "slPercentToTrigger": null, "slPercentToSell": null},
      {"tpPercentToTrigger": 300, "tpPercentOfBagToSell": 100, "tpPercentOfCostToSell": null, "slPercentToTrigger": null, "slPercentToSell": null},
      {"tpPercentToTrigger": null, "tpPercentOfBagToSell": null, "tpPercentOfCostToSell": null, "slPercentToTrigger": 30, "slPercentToSell": 100}
    ]
  }
}}
```

Field notes:

- Each step has all 5 fields; unused fields are `null`.
- `tpPercentToTrigger: 50` means "take profit when up 50%".
- `slPercentToTrigger: 30` means "stop loss when down 30%" (positive value).
- `tpPercentOfBagToSell` vs `tpPercentOfCostToSell`: choose one. Bag = % of current holdings; Cost = % of original cost basis.

Attach with `apply_exit_strategy` (`strategyId`, `tokenChainId`, `tokenContractAddress`).

Other actions: `list_exit_strategies`, `update_exit_strategy`, `remove_exit_strategy`.

---

## Recipe 5: Whale-watching via polling

**Goal**: Track a set of wallets and surface every new swap.

The `orders` namespace does not expose alert-registration actions. The supported way to watch a wallet from an agent is to **poll `wallet_swaps`** on a cadence that fits your latency budget. For push delivery, configure alerts in the Edge webapp (Settings > Alerts) and point them at your webhook. See [Webhooks](webhooks.md) for the receiver-side payload shape and signature verification.

**Steps**:

1. On each poll, request the most recent N swaps for the wallet.
2. Diff against the highest `transactionHash` you have already processed.
3. Process new swaps; persist the new high-water mark.

**Calls**:

```json
// Pull the last 50 swaps for a wallet (wallet_swaps takes makerAddresses, plural)
{"action": "wallet_swaps", "schema": 1, "data": {
  "limit": 50,
  "makerAddresses": ["7xKXtg2CW8..."],
  "chainId": "solana",
  "tokenContractAddress": null,
  "pairContractAddress": null,
  "previousPairContractAddress": null
}}
```

Each swap carries `tokensBought`, `tokensSold`, `priceUsd`, `fromAddress`, `transactionHash`. There is no `side` field: use `tokensBought > "0"` for buys, `tokensSold > "0"` for sells. Amounts are returned as strings; compare as strings or parse to `BigInt` before numeric comparison.

**Cadence guidance**: poll every 10-30 seconds for active wallets. Bursty wallets benefit from a shorter interval; long-tail wallets from a longer one. If you exceed rate limits, back off. See [Errors: Rate limits](errors.md#rate-limiting).

For push delivery instead of polling, see [Subscriptions](subscriptions.md) for the supported alert channels (webhook, Redis stream, Telegram) and how they are configured.

---

## Recipe 6: Pair monitoring with webhook delivery

**Goal**: React when a pair's price, liquidity, or volume crosses a threshold.

Pair-update push delivery is configured in the Edge webapp (Settings > Alerts), not via MCP actions. From an agent you have two options:

- **Poll `pair_metrics` on a cadence** and compare to the last reading you stored.
- **Configure a webhook in the Edge webapp** and have your endpoint receive deliveries.

This recipe shows the polling option; for the webhook receiver side, see [Webhooks](webhooks.md).

**Calls**:

```json
// Pull the latest 1h metrics for a pair on Base (note the field naming)
{"action": "pair_metrics", "schema": 1, "data": {
  "pairChainId": 8453,
  "pairContractAddress": "0xPAIR...",
  "interval": "1h"
}}
```

`pair_metrics` returns every interval at the top level. Read `response["1h"].priceUsd`, `response["1h"].liquidityUsd`, `response["1h"].volumeUsd`. Persist the previous reading and fire your downstream action when the delta crosses your threshold.

**Webhook receiver shape** (when configured via the Edge webapp):

```json
{
  "event": "on_pair_updates",
  "data": {"priceUsd": "...", "liquidityUsd": "...", "volumeUsd": "..."},
  "timestamp": 1702000720,
  "signature": "sha256=abc123..."
}
```

Verify `signature` using your shared secret with constant-time HMAC-SHA256 comparison. See [Webhooks](webhooks.md) for Node.js and Python verification code.

Delivery failures appear in the webapp alerts dashboard. See [Errors: Subscription errors](errors.md#subscription-errors).

---

## Recipe 7: Multi-wallet portfolio roll-up

**Goal**: Show consolidated PnL across every wallet a user owns, not per-wallet.

`wallet_summary` is per-wallet and returns an array per chain; roll up in the agent.

**Steps**:

1. Batch up to 5 wallets per `wallet_summary` call.
2. Sum `totalSoldUsd + remainingUsd - totalCostUsd` to get PnL.
3. Call `wallet_holdings` per batch to merge positions by token address.

**Calls**:

```json
{"action": "wallet_summary", "schema": 1, "data": {
  "wallets": ["7xKXtg2CW8...", "9pQmK4pVn...", "2aB6HxW1x..."],
  "filters": {
    "isInTrade": true,
    "chainId": null,
    "tokenContractAddresses": [],
    "excludeTokenContractAddresses": [],
    "minMostRecentTx": null
  }
}}

{"action": "wallet_holdings", "schema": 1, "data": {
  "wallets": ["7xKXtg2CW8...", "9pQmK4pVn...", "2aB6HxW1x..."],
  "filters": {"isInTrade": true, "chainId": null},
  "limit": 100,
  "sortByColumn": "pnlUsd",
  "sortDirection": "desc",
  "cursor": null
}}
```

Gotchas:

- `wallet_summary` does not return a single `totalPnlUsd` or `winRate`. Compute manually: `totalSoldUsd + remainingUsd - totalCostUsd`.
- `wallet_holdings` wraps the array in `data.items`.
- Both cap at **5 wallets per call**. For larger sets, batch across calls.

---

## Recipe 8: Copy-trade a wallet

**Goal**: Mirror a wallet's recent swaps into your own wallet. Basic pattern; no slippage or size scaling.

**Steps**:

1. Read the target wallet's recent swaps.
2. For each new swap, resolve the traded token to a pair.
3. Submit a matching market order.

**Calls**:

```json
// 1. Pull last N swaps
{"action": "wallet_swaps", "schema": 1, "data": {
  "limit": 20,
  "makerAddresses": ["TargetWhaleAddr..."],
  "chainId": "solana",
  "tokenContractAddress": null,
  "pairContractAddress": null,
  "previousPairContractAddress": null
}}

// 2. For each new swap, resolve the traded token to a pair
{"action": "token_info_with_pricing", "schema": 1, "data": {
  "chainId": "solana",
  "tokenContractAddress": "<swap.tokenContractAddress>",
  "useBestPair": true
}}

// 3. Place a spot order on your wallet for immediate execution.
// place_spot_order requires both `order` and `envelope` (an encrypted intent
// constructed by your client. See Security: How Edge Keeps Your Keys Safe).
{"action": "place_spot_order", "schema": 1, "data": {
  "order": {
    "tokenId": {"tokenChainId": "solana", "tokenContractAddress": "<from step 2>"},
    "side": "buy",
    "amount": {"type": "native", "value": 100000000},
    "pairAddress": "<from step 2 pair.pairContractAddress>",
    "wallets": [{"address": "YourWalletAddr..."}],
    "txPreset": {"method": "normal", "slippage": 15, "maxBaseGas": 0, "priorityGas": 0, "bribe": 0, "key": "a"},
    "expiration": 3600,
    "entryStrategyId": null,
    "exitStrategyId": null,
    "counterTokenAddress": null
  },
  "envelope": "<encrypted-intent-envelope>"
}}
```

Gotchas:

- `wallet_swaps` has no `side` field. Check `tokensBought > "0"` for a buy, `tokensSold > "0"` for a sell.
- Field is `tokensBought` / `tokensSold` (strings) and `fromAddress` (not `makerAddress`) and `transactionHash` (not `txHash`).
- Keep a set of transaction hashes you have already mirrored so you do not double-fire.
- Your wallet must be registered to the API key.

**Size scaling**: copy by fixed amount (shown) is naive. Real copy-trade usually scales by percentage of the target's wallet balance, capped at your own position-sizing rules.

---

## Building your own recipes

Each recipe follows a shape:

1. One or two `read` actions to get current state
2. A transformation step in your agent or backend
3. One or more `write` actions to take an effect
4. Optional subscription to follow up asynchronously

If you hit rate limits, cache `read` results for a minute or two. See [Errors: Rate limits](errors.md#rate-limiting) for per-action caching guidance.

## See also

- [Agent Patterns](agent-patterns.md): atomic patterns that back each recipe step
- [Tools](tools/README.md): every action and its parameters
- [REST API](rest-api.md): same actions over HTTP for non-MCP tools
- [Subscriptions](subscriptions.md): alert types, buffer behavior, webhooks
- [Errors](errors.md): error codes and retry guidance
