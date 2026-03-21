# Rust Linting (Clippy)

## Default Setup

Always run `cargo clippy` before committing. Treat warnings as errors in CI:

```bash
cargo clippy -- -D warnings
```

## Clippy Lint Categories

| Category | Default | Action |
|---|---|---|
| **correctness** | deny | Always fix. These are bugs. |
| **suspicious** | warn | Fix unless intentional; add comment if allowing |
| **complexity** | warn | Simplify the code |
| **perf** | warn | Easy wins; fix |
| **style** | warn | Fix for idiomatic code |
| **pedantic** | off | Opt-in for libraries |
| **restriction** | off | Cherry-pick specific lints only |
| **nursery** | off | Experimental; avoid |

## Recommended clippy.toml

```toml
msrv = "1.75.0"  # Set to your actual MSRV
```

## In-code Configuration

Enable pedantic for library crates at the top of `lib.rs`:
```rust
#![warn(clippy::pedantic)]
// Allow specific pedantic lints that disagree with project style:
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::must_use_candidate)]
```

## Never Allow

These lint categories must never be silenced without documented justification:
- `clippy::correctness` — these are bugs
- `clippy::suspicious` — document why it's intentional if allowing

## Rustfmt

Run `cargo fmt` before committing. Enforce in CI:

```bash
cargo fmt --check
```

Recommended `rustfmt.toml`:
```toml
edition = "2021"
max_width = 100
reorder_imports = true
reorder_modules = true
```
