# Subscriptions

Edge supports real-time push delivery of market, wallet, and order events. You configure subscriptions in the Edge webapp; events stream to your registered target (webhook, Redis stream, or Telegram chat).

{% hint style="warning" %}
Alert subscriptions are **configured in the Edge webapp** (Settings > Alerts), not via MCP actions. From an agent, use the `edge://alerts` MCP resource to read the catalog of available alert types; manage actual subscriptions in the webapp.
{% endhint %}

## How it works

1. Open the Edge webapp at [edge.trade](https://edge.trade) and go to **Settings > Alerts**.
2. Pick an alert type (e.g. `on_wallet_swaps`) and supply the filters it requires.
3. Pick a delivery target (webhook URL, Redis stream, or Telegram chat).
4. Save. Events flow to your delivery target automatically.
5. Edit or remove subscriptions from the same screen.

To programmatically discover what alert types and input filters exist, read the `edge://alerts` MCP resource from your agent.

## Alert types

The live catalog ships in the `edge://alerts` MCP resource. Each entry includes its `inputSchema` so you can shape the filters correctly. At time of writing:

| `alert_name` | What it streams |
|---|---|
| `on_pair_updates` | Pair metrics (price/volume/liquidity) or pair state |
| `on_pair_swaps` | Every swap on a pair |
| `on_wallet_swaps` | Every swap on one or more wallets |
| `on_token_updates` | Token events (e.g., holder distribution changes) |
| `on_portfolio_updates` | Wallet holdings changes |
| `on_order_updates` | Order fills, cancellations, rejections |
| `on_memescope` | Live Memescope token discoveries |

## Delivery targets

### Webhook

Configure a public HTTPS endpoint and a shared secret. Edge POSTs each event to the URL with an `X-Edge-Signature` header containing an HMAC-SHA256 signature you can verify. See [Webhooks](webhooks.md) for verification code, payload shape, and local-testing notes.

### Redis stream

Provide a Redis URL and channel name. Edge pushes events onto the stream; your backend consumes them at its own pace. Useful when the receiver is already running Redis. See [Redis stream delivery](redis-streams.md).

### Telegram

Create a Telegram bot via `@BotFather` and provide its token plus the chat ID. Edge sends formatted messages to the chat or group.

## Polling alternatives from MCP

If you don't want to run a webhook receiver (or you're prototyping), you can poll from an agent instead:

| Need | Poll this action |
|---|---|
| Wallet swaps | `wallet.wallet_swaps` |
| Pair swaps | `pairs.pair_swaps` |
| Pair metrics | `pairs.pair_metrics` |
| Order status | `orders.list_orders` (filter by `status` and `taskIds`) |
| Holdings change | `wallet.wallet_holdings` |

Diff each result against the previous poll and act on what changed. Cadence depends on latency budget and rate limits. 10–30 s is reasonable for active wallets and pairs. See [Errors: Rate limits](errors.md#rate-limiting).

## Errors

| Code | Meaning | Where it shows up |
|------|---------|---------------------|
| `WEBHOOK_DELIVERY_FAILED` | Your endpoint returned 4xx/5xx or was unreachable | Webapp alerts dashboard |
| Webhook signature mismatch | Receiver-side | Your endpoint's logs (`X-Edge-Signature` did not match recomputed HMAC) |

See [Errors: Subscription errors](errors.md#subscription-errors).

## See also

- [alerts tool reference](tools/alerts.md): the `edge://alerts` MCP resource and per-alert input shapes
- [Webhooks](webhooks.md): HMAC verification, local testing with ngrok
- [Redis stream delivery](redis-streams.md): consume events from a Redis stream
- [Errors](errors.md): full error code list
