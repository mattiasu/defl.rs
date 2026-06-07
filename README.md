# defl

A small HTTP proxy service running on a Raspberry Pi, that works behind a security layer, e.g. Cloudflare Tunnel. It receives requests and forwards them to a target service (allowed list in service config), translating the header namespace in the process.

## What it does

1. Receives a REQUEST 
2. Reads `X-Target-Url` to know where to forward the request
3. Forwards only headers prefixed with `defl-` — stripping the prefix before sending
4. Synthesises `Authorization: Bearer <token>` from `defl-forward-auth-token` if present
5. Drops everything else (CF Access headers, host, content-length, etc.)

This keeps the request auth fully separate from the target service's auth — neither side knows the other's credentials.

## Header protocol

| Incoming header | Forwarded as |
|---|---|
| `X-Target-Url` | Consumed — not forwarded |
| `defl-forward-auth-token: <val>` | `Authorization: Bearer <val>` |
| `defl-x-title: <val>` | `x-title: <val>` |
| `defl-<anything>: <val>` | `<anything>: <val>` |
| Everything else | Dropped |

## Running

The service listens on `0.0.0.0:8080`. It is managed as a systemd service on the Pi — port 8080 should not be open to the internet.

```
sudo systemctl status defl
sudo journalctl -u defl -n 50
```

## Deploying

From the project root on your client:

```bash
./deploy.sh
```

This will: build a release binary for ARM64, stop the service, copy the binary, and restart the service. It exits on any failure with a description of the state left behind.

## Building manually

```bash
cargo build --release --target aarch64-unknown-linux-gnu
```

Requires the `aarch64-unknown-linux-gnu` rustup target and a matching GCC linker (`aarch64-unknown-linux-gnu-gcc`), configured in `.cargo/config.toml`.
