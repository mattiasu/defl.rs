# CLAUDE.md — defl.rs - pi web deflector in Rust

## Project goal

Build a Rust binary that runs on a Raspberry Pi (mupi), exposes an api that fowards the request to a specified new API.
The forwar API should be passed on in the incoming reqest.
learning project — favour explicitness and clarity over cleverness.

---

## What we are building

- Small webserver
- exposed API that can receive a request and pass it forward
---

## Target hardware

- **Development machine**: iMac (Apple Silicon or Intel — confirm arch with
  `uname -m`)
- **Target device**: Raspberry Pi (`mupi`), user `muhlegard`, ARM64 Linux
- **SSH access**: already configured with passwordless SSH from Mac

---

## Rust concepts this project teaches (in order of encounter)

1. `cargo` project structure, `Cargo.toml`, dependencies
2. Structs and `impl` blocks
3. `Result<T, E>` and the `?` operator for error propagation
4. `serde` + `serde_json` for serialization
5. Ownership and borrowing — hits naturally when passing data between functions
6. Cross-compilation and deployment

---

## Project structure to aim for

```
pi-metrics/
├── Cargo.toml
├── CLAUDE.md           ← this file
├── src/
│   ├── main.rs         ← entry point
```

---

## Key dependencies (Cargo.toml)

```toml
```

Add `tokio` only when you are ready to make it async — do not start there.

---

## Development workflow

### 1. Develop and test on Mac

```bash
cargo build                  # compiles for your Mac — catches most errors
cargo run                    # runs locally (most collectors will fail gracefully
                             #   since /proc doesn't exist on macOS — that's fine)
cargo clippy                 # Rust's linter — treat warnings as errors
cargo fmt                    # format code
```

### 2. Cross-compile for Raspberry Pi (ARM64)

Install `cross` once:

```bash
cargo install cross
```

Build for Pi:

```bash
cross build --release --target aarch64-unknown-linux-gnu
```

The binary lands at:
```
target/aarch64-unknown-linux-gnu/release/def.rs
```

### 3. Deploy to mupi

```bash
scp target/aarch64-unknown-linux-gnu/release/defl.rs muhlegard@mupi:~/defl.rs
```

### 4. Run on mupi

```bash
ssh muhlegard@mupi
./defl.rs
```

Or keep it running detached:

```bash
nohup ./defl.rs > defl.rs.log 2>&1 &
```

### 5. Iterate

Edit on Mac → `cross build --release` → `scp` → test. Full loop is ~2 min.

---

## Suggested build order (learning milestones)


---

## Error handling philosophy for this project

- Use `Result<T, Box<dyn std::error::Error>>` early on — simple and flexible
- Graduate to a custom error enum (`thiserror` crate) once you feel the pain
  of matching on `Box<dyn Error>`
- Never use `.unwrap()` in the polling loop — a bad read should log and
  continue, not crash the collector

---

## Things to avoid while learning

- Do not start with `async`/`await` — the borrow checker is harder to reason
  about in async context; get comfortable with sync Rust first
- Do not reach for a web framework yet — stdout and files are enough
- Do not use `.clone()` to silence borrow checker errors without understanding
  why — ask Claude to explain the ownership issue instead

---


## Reminder: this is a learning project

When something doesn't compile, read the error message fully — Rust's compiler
errors are genuinely helpful. Paste them to Claude with the surrounding code
and ask what ownership or type concept is behind the error. That's the fastest
way to actually learn Rust.
