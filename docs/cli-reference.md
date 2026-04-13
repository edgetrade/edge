# CLI Reference

Unified command-line interface for Edge Trade.

## Installation

```bash
cargo install edge-trade
```

## Global Flags

| Flag | Description |
|------|-------------|
| `--config <PATH>` | Override config file location |
| `--api-key <KEY>` | Set API key |
| `--verbose` | Enable verbose logging |

Override via environment variable:

```bash
export EDGE_CONFIG=/path/to/config.toml
```

## Configuration

Config file location (XDG-compliant):

- **Linux**: `~/.config/edge/config.toml`
- **macOS**: `~/Library/Application Support/edge/config.toml`
- **Windows**: `%APPDATA%\edge\config.toml`

### Format

```toml
[session]
use_keyring = true  # Use OS keyring (default: auto-detect)
```

Force file storage:

```toml
[session]
use_keyring = false
```

Remove the line to re-trigger auto-detection on next run.

## Session Storage

The CLI automatically detects OS keyring availability:

1. **First run**: Probes keyring, caches result in config
2. **Keyring available**: Stores session keys securely
3. **Keyring unavailable**: Falls back to file storage with warning
4. **Override**: Edit config to force keyring or file mode

## Key Management

Create session key:

```bash
edge key create
```

Unlock session:

```bash
edge key unlock
```

Lock session:

```bash
edge key lock
```

Generate new key:

```bash
edge key update
```

## Wallet Management

Create wallet:

```bash
# EVM wallet
edge wallet create --chain evm

# SVM wallet
edge wallet create --chain svm
```

Import wallet:

```bash
# EVM wallet
edge wallet import --chain evm /path/to/private.key

# SVM wallet
edge wallet import --chain svm /path/to/private.key
```

## Examples

### Force file storage (server environment)

```toml
# ~/.config/edge/config.toml
[session]
use_keyring = false
```

```bash
edge key create
edge wallet create --chain evm
```

### Custom config location

```bash
edge --config /etc/edge/config.toml key create
```

### CI/CD pipeline

```bash
export EDGE_CONFIG=/tmp/edge-ci/config.toml
edge --api-key "$API_KEY" wallet create --chain evm
```
