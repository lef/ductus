# Cargo Workspaces

## Single Crate vs Workspace

| Situation | Use |
|---|---|
| One binary or one library | Single crate |
| Binary + library (separate compilation) | Workspace |
| Multiple related binaries | Workspace |
| Shared types across crates | Workspace |
| Plugin or extension crate | Workspace |

YAGNI: 単一クレートで始める。分割が必要になってから workspace に移行する。

## Basic Workspace Layout

```
my-project/
├── Cargo.toml          ← workspace root
├── crates/
│   ├── core/           ← shared types, no I/O
│   │   └── Cargo.toml
│   ├── server/         ← binary
│   │   └── Cargo.toml
│   └── client/         ← binary
│       └── Cargo.toml
└── tests/              ← integration tests using workspace crates
```

```toml
# Root Cargo.toml
[workspace]
members = ["crates/*"]
resolver = "2"          # always use resolver 2 in workspaces
```

## Shared Dependency Versions

```toml
# Root Cargo.toml — define versions once
[workspace.dependencies]
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
anyhow = "1"

# Crate Cargo.toml — inherit, don't repeat versions
[dependencies]
tokio.workspace = true
serde.workspace = true
anyhow.workspace = true

# Override features per-crate if needed
tokio = { workspace = true, features = ["net"] }
```

## Cross-Crate Testing

```toml
# crates/server/Cargo.toml
[dev-dependencies]
core = { path = "../core" }
```

Integration tests in `tests/` at the workspace root can use all crates:

```rust
// tests/end_to_end.rs
use server::Server;
use core::Config;
```

## Common Pitfalls

**Dependency version conflicts**
- Two crates requiring incompatible semver ranges compile separately (duplication)
- Use `[workspace.dependencies]` to enforce a single version

**Cyclic dependencies**
- Rust forbids cycles between crates in the same workspace
- If you need A→B and B→A, extract shared types into a third crate C

**Feature unification**
- In a workspace, features of the same crate are unified across all dependents
- Adding a feature in one crate can unexpectedly affect another — check with `cargo tree -f "{p} {f}"`

**Build times**
- Splitting into many crates can increase incremental build times due to more codegen units
- Profile with `cargo build --timings` before splitting
