# Static Binary Build

For sandbox environments and embedded deployment, build fully static binaries.

## Target

```bash
# Add MUSL target (once)
rustup target add x86_64-unknown-linux-musl

# Build static binary
cargo build --release --target=x86_64-unknown-linux-musl
```

Output: `target/x86_64-unknown-linux-musl/release/<binary>`

## Cargo.toml Release Profile

```toml
[profile.release]
opt-level = 3
lto = true           # Link-time optimization (smaller + faster)
codegen-units = 1    # Better optimization (slower build)
strip = true         # Remove debug symbols (~50% size reduction)
panic = "abort"      # Smaller binary, no unwinding
```

## OpenSSL Pitfall

OpenSSL links dynamically by default. Avoid it:

```toml
# Bad: links to system OpenSSL
reqwest = "0.12"

# Good: use rustls (pure Rust TLS, truly static)
reqwest = { version = "0.12", default-features = false, features = ["rustls-tls"] }
```

## Cross-Compilation

```bash
# Install cross
cargo install cross

# Build for ARM
cross build --release --target=aarch64-unknown-linux-musl
```

## Verification

```bash
# Confirm static linkage
file target/x86_64-unknown-linux-musl/release/<binary>
# Should show: "statically linked"

ldd target/x86_64-unknown-linux-musl/release/<binary>
# Should show: "not a dynamic executable"
```

## Expected Binary Sizes

- Typical CLI tool: 2–10 MB (stripped, MUSL)
- With async (Tokio): 5–15 MB
