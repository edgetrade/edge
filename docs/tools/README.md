# Tools

The Edge MCP server exposes 6 tools, each mapping to a namespace with one or more actions. Tool name = namespace. All calls use `{"action": "ACTION_NAME", "schema": 1, "data": {...}}`.

| Tool / Namespace | Actions | Purpose |
|------------------|---------|---------|
| [search](./search.md) (`intelligence`) | `search_tokens`, `screen_tokens`, `search_swaps` | Token and swap discovery |
| [inspect](./inspect.md) (`tokens`) | `token_info_with_pricing`, `token_top_holders`, `token_top_traders`, `token_dev_tokens` | Token-scoped analytics |
| pairs (`pairs`) | `pair_metrics`, `pair_ohlcv`, `pair_swaps`, `pair_info` | Pair-scoped analytics |
| [screen](./screen.md) | `screen_tokens` (part of `intelligence`) | Onchain-metric token screening (Solana only) |
| [portfolio](./portfolio.md) (`wallet`) | `wallet_holdings`, `wallet_summary`, `wallet_swaps`, `wallet_history`, `native_balances` | Wallet holdings and history |
| [trade](./trade.md) (`orders`) | `place_limit_order`, `place_spot_order`, `list_orders`, `order`, `cancel_order`, `cancel_all_orders`, + 10 strategy actions | Order placement and management |
| [agent](./agent.md) (`agent`) | 7 wallet-encryption actions for the non-custodial trading flow | Encrypted wallet creation, registration, signing |
| [alerts](./alerts.md) (catalog only) | `edge://alerts` MCP resource | Catalog of subscribable alert types; subscriptions are configured in the Edge webapp |

## Total

- **6 tools** across 6 namespaces (`intelligence`, `tokens`, `pairs`, `wallet`, `orders`, `agent`)
- **39 actions** total (3 + 4 + 4 + 5 + 16 + 7), confirmed against the live MCP `tools/list` manifest
- **Alerts** are configured in the Edge webapp (Settings > Alerts) and delivered via webhook, Redis stream, or Telegram. Available alert types are listed in the `edge://alerts` MCP resource.

See [Edge for Agents](../README.md) for the top-level index and [Agent Patterns](../agent-patterns.md) for common workflows.
