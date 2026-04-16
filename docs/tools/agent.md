# agent

Namespace: `agent` · 7 actions

Wallet-encryption and intent-envelope helpers used by the non-custodial trading flow. These actions exist so a client can construct the encrypted material that `place_spot_order` (and related wallet operations) require. They are not for day-to-day market intelligence work.

{% hint style="info" %}
Most agent workflows never call this namespace directly. If you only read market data and place limit orders bound to wallets already registered to your API key, the `intelligence`, `tokens`, `pairs`, `wallet`, and `orders` namespaces cover everything you need. The `agent` namespace becomes relevant when your client is building the encrypted intent for a spot order, registering a new client-side wallet, or unwrapping a key inside the vault flow.
{% endhint %}

## Why this namespace exists

Edge is non-custodial: your private key is encrypted twice (once with a secret only you hold, once with a key that lives only inside a hardware-isolated vault). Signatures only happen inside the vault, and only when you provide your portion of the decryption mechanism. Without it, the vault mathematically cannot sign. See [Security: How Edge keeps your keys safe](../../security/how-edge-keeps-your-keys-safe.md) for the full architecture.

The actions in the `agent` namespace expose the building blocks of that flow: producing and verifying the encrypted material the vault expects, and managing the wallets associated with your API key.

## Discovering the exact actions

The `agent` namespace has **7 actions**. Action names and exact parameter shapes are subject to schema changes as the wallet-encryption flow evolves; treat the live MCP `tools/list` manifest as the source of truth.

To enumerate them from any MCP client:

```json
{"jsonrpc": "2.0", "id": 1, "method": "tools/list"}
```

Then look at the `agent` tool's `inputSchema` for the supported `action` enum and the per-action `data` shape. From a typical client:

```text
tool: agent
arguments: {"action": "<one_of_the_7>", "schema": 1, "data": {...}}
```

## When you'd call this from an agent

Most agents do not need to call the `agent` namespace directly. The two times you do:

1. **Constructing the `envelope` field for `place_spot_order`.** `place_spot_order` requires an encrypted intent envelope alongside the `order` block. Your client builds the envelope from the user's approval and the trade parameters; the vault verifies it before signing.
2. **Provisioning a new wallet from your client.** When your application creates a wallet, the wrapping happens client-side; the resulting blobs are registered against your API key via the `agent` namespace.

For limit orders against already-registered wallets, you do not need to touch this namespace. `place_limit_order` is sufficient.

## Related

- [Security: How Edge keeps your keys safe](../../security/how-edge-keeps-your-keys-safe.md): the three-layer architecture that this namespace plumbs.
- [trade tool reference](trade.md): `place_limit_order` and `place_spot_order`, including the `envelope` requirement on spot orders.
- [Authentication](../authentication.md): API key setup; wallet registration is per-key.
