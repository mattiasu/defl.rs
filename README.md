# defl

HTTP proxy that forwards requests to an allowlisted target, translating headers in the process. Designed to sit behind a security tunnel on a Pi — keeping inbound auth separate from target service auth.

## Header protocol

Only `defl-` prefixed headers are forwarded, with the prefix stripped. Everything else is dropped.

| Incoming | Forwarded as |
|---|---|
| `X-Target-Url` | Consumed, not forwarded |
| `defl-forward-auth-token: <val>` | `Authorization: Bearer <val>` |
| `defl-<name>: <val>` | `<name>: <val>` |

## Prerequisites

- Rust via [rustup](https://rustup.rs)
- ARM64 target: `rustup target add aarch64-unknown-linux-gnu`
- ARM64 linker: `aarch64-unknown-linux-gnu-gcc` (wired in `.cargo/config.toml`)

## Run locally

```bash
ALLOWED_DOMAINS=ntfy.sh cargo run
```

## Deploy

Targets a Pi over SSH. Edit `REMOTE` in `deploy.sh` if your host differs.

```bash
./deploy.sh
```

Builds for ARM64, stops the service, copies the binary, restarts. Exits on any failure with an explanation.

## Configuration

The service reads `ALLOWED_DOMAINS` (comma-separated) at startup — it will not start without it. Set it via the systemd `EnvironmentFile`:

```ini
# /etc/systemd/system/defl.env
ALLOWED_DOMAINS=ntfy.sh,httpbin.org
```

Reload after changes: `sudo systemctl daemon-reload && sudo systemctl restart defl`

## Manage on the Pi

```bash
sudo systemctl status defl
sudo journalctl -u defl -n 50
```
