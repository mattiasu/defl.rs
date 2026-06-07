# CLAUDE.md — defl

Small synchronous HTTP proxy. Receives a request, validates the target host against an allowlist, translates headers via the `defl-` protocol, forwards.

---

## Guarded decisions

**Sync over async** — low-concurrency, I/O-bound workload. `tiny_http` + `ureq` is sufficient. Do not introduce `tokio` without a concrete concurrency requirement.

**No framework** — one route, explicit dispatch. A framework adds abstraction without benefit.

**Flat structure** — everything in `src/main.rs`. Do not split until there is a clear reason.

**Allowlist is required** — `ALLOWED_DOMAINS` must be set at startup. No fallback, no default. An unconfigured proxy is a misconfigured proxy.

---

## Header protocol

| Incoming | Forwarded as |
|---|---|
| `X-Target-Url` | Consumed, not forwarded |
| `defl-forward-auth-token: <val>` | `Authorization: Bearer <val>` |
| `defl-<name>: <val>` | `<name>: <val>` |
| Everything else | Dropped |

---

## Error contracts

- Missing `X-Target-Url` → 400
- Host not in allowlist → 403
- Upstream failure → 502
- Startup misconfiguration → panic (intentional)
- No `.unwrap()` in request handling
