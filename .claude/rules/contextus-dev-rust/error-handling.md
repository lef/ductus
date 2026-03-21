# Error Handling

Supplement to `rust-style.md`. See that file for basics.

When instructing Claude to refactor error handling, say **"Martin Fowler のRefactoring 手法で"**.
This anchors the concept and improves accuracy over just "refactor".
Ref: https://memory-lovers.blog/entry/2025/06/27/102550

## Choosing Between thiserror and anyhow

```
Is this a library (others call your code)?
  YES → thiserror: define typed errors, callers can match on them
  NO  → anyhow: ergonomic propagation, errors go to logs/user
```

```toml
# Library
[dependencies]
thiserror = "1"

# Binary / application
[dependencies]
anyhow = "1"

# Both (library with binary)
[dependencies]
thiserror = "1"  # library errors
anyhow = "1"     # application layer
```

## thiserror Pattern (Library)

```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProxyError {
    #[error("domain not in allowlist: {0}")]
    DomainBlocked(String),

    #[error("invalid host header: {0}")]
    InvalidHost(String),

    #[error("connection failed")]
    ConnectionFailed(#[from] std::io::Error),
}
```

## anyhow Pattern (Application)

```rust
use anyhow::{Context, Result};

fn run() -> Result<()> {
    let config = load_config()
        .context("failed to load config")?;

    let listener = TcpListener::bind(&config.addr)
        .with_context(|| format!("failed to bind to {}", config.addr))?;

    Ok(())
}
```

## Context on Errors

Always add context when propagating across boundaries:

```rust
// Bad: context lost
let data = read_file(path)?;

// Good: context preserved
let data = read_file(path)
    .with_context(|| format!("failed to read config from {}", path.display()))?;
```

## Doc Comment Format

```rust
/// Checks if a domain is in the allowlist.
///
/// # Errors
///
/// Returns [`ProxyError::DomainBlocked`] if the domain is not permitted.
/// Returns [`ProxyError::InvalidHost`] if the domain format is invalid.
pub fn check_allowed(domain: &str) -> Result<(), ProxyError> { }
```

## Panic Policy

| Location | Rule |
|---|---|
| Library code | Never panic. Return `Result`. |
| Application startup | `expect("reason")` is OK for unrecoverable init failures |
| Tests | `unwrap()` is OK |
| Production logic | Never `unwrap()` or `panic!()` |
