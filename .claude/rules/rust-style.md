# Rust Style Guide

## Basics

- Always use `cargo fmt` formatting (enforced via `rustfmt`)
- Always use `cargo clippy` and resolve warnings before committing
- Prefer `thiserror` for library errors, `anyhow` for binary errors
- Use `#[must_use]` on functions whose return value must not be ignored

## Error Handling

- Never use `.unwrap()` or `.expect()` in production code paths
- Use `?` for propagation; reserve `unwrap()` for tests only
- Error messages in lowercase without trailing period (Rust convention)

## Naming

- Types and traits: `UpperCamelCase`
- Functions, variables, modules: `snake_case`
- Constants: `SCREAMING_SNAKE_CASE`
- Avoid abbreviations unless universally known (`url`, `ip`, `http`)

## Structure

- One module per file; keep files under 300 lines
- `main.rs` / `lib.rs` should be thin — delegate to modules
- Put integration tests in `tests/`, unit tests in `#[cfg(test)]` blocks

## Dependencies

- Minimize external crates (YAGNI)
- Prefer `tokio` for async runtime
- Prefer `clap` for CLI argument parsing
- Audit new dependencies with `cargo audit`

## Performance

- Avoid premature optimization
- Profile before optimizing (`cargo flamegraph`)
- Prefer `&str` over `String` in function parameters where possible

## Comments

- Comment non-obvious logic only
- Use `///` for public API docs; `//` for internal comments
- Reference design docs by section if applicable
