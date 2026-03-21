# Rust Testing

## TDD: Follow t_wada's Approach

When instructing Claude to do TDD, say **"t_wada の推奨する進め方に従って"**.
Person names dramatically improve Claude's instruction-following accuracy.
Ref: https://memory-lovers.blog/entry/2025/06/27/102550

### RED → GREEN → REFACTOR in Rust

Rust has **two kinds of RED**:

1. **Compile RED** — the code doesn't compile yet (`cargo test --no-run`)
2. **Test RED** — compiles but test fails (`cargo test`)

Both are valid RED. Start with whichever is closer to the intent.

```bash
cargo test --no-run   # confirm compile RED (fastest feedback)
cargo test            # confirm test RED
```

**Rules**:
- Write the failing test first — never write implementation before a failing test exists
- One failing test at a time
- Write the minimum code to pass — resist over-implementing
- Refactor only on GREEN — never refactor on RED
- Prefer `Result`-returning tests over `#[should_panic]` — more composable

```rust
// Prefer this (t_wada style)
#[test]
fn test_parse_host_with_port() -> Result<(), Box<dyn std::error::Error>> {
    let host = parse_host("example.com:443")?;
    assert_eq!(host.as_str(), "example.com");
    Ok(())
}

// Over this
#[test]
#[should_panic]
fn test_parse_host_panics() { ... }
```

## File Structure

```
src/
├── lib.rs
├── module.rs        ← unit tests here, in #[cfg(test)] block
tests/
├── common/
│   └── mod.rs       ← shared test helpers
└── integration.rs   ← integration tests (public API only)
```

## Unit Tests

Place in the same file as the code being tested:

```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_positive_numbers() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_add_negative_numbers() {
        assert_eq!(add(-1, -1), -2);
    }

    #[test]
    #[should_panic(expected = "overflow")]
    fn test_add_overflow_panics() {
        add(i32::MAX, 1);
    }
}
```

## Test Naming

Pattern: `test_<function>_<scenario>`

```rust
fn test_parse_host_with_port()       // happy path with port
fn test_parse_host_without_port()    // default port case
fn test_parse_host_empty_string()    // edge case
fn test_parse_host_invalid_chars()   // error case
```

## Integration Tests

Test public API end-to-end from `tests/`:

```rust
// tests/proxy.rs
use ductus::proxy;

#[test]
fn test_proxy_allows_listed_domain() {
    let result = proxy::check_allowed("example.com", &allowlist());
    assert!(result.is_ok());
}
```

## Async Tests

```rust
#[tokio::test]
async fn test_async_operation() {
    let result = async_fn().await;
    assert!(result.is_ok());
}
```

## Doc Tests

Write examples in doc comments — they run as tests:

```rust
/// ```
/// use mylib::parse_host;
/// assert_eq!(parse_host("example.com:443").unwrap().as_str(), "example.com");
/// ```
pub fn parse_host(header: &str) -> Result<String, Error> { }
```

Run with: `cargo test --doc`

## Running Tests

```bash
cargo test           # all tests
cargo test --doc     # doc tests only
cargo test <name>    # filter by test name
cargo test -- --nocapture  # show println! output
```
