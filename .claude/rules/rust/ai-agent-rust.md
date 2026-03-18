# Rust for AI Agents

Guidelines specific to AI agents writing Rust code.
Based on: Microsoft Pragmatic Rust Guidelines, RustAssistant research, rust-skills community rules.

## Model Switching for Rust Code

> **2026-03-18 更新**: Opus 4.6 (1M context) の登場により、Rust 開発時のモデル切り替えは
> 基本的に不要になった。1M context で Rust コードの読み書きに十分な能力がある。
> コスト最適化が必要な場合のみ、以下のルールを適用する。

コスト最適化モード（オプション）:
- Rust コードを書く必要が生じたら：**ユーザーにモデル切り替えを提案してから**実装を開始する
- Rust コードが不要になったら：**その旨をユーザーに伝える**

## Person-Name Technique for Better Instruction Accuracy

Naming experts when giving Claude instructions dramatically improves accuracy:

| Task | Say this |
|---|---|
| TDD | **"t_wada の推奨する進め方に従って"** |
| Refactoring | **"Martin Fowler のRefactoring 手法で"** |
| Small cleanups | **"Kent Beck のTidyings で"** |
| Clean code | **"Robert C. Martin のClean Code 原則で"** |
| Error design | **"thiserror / anyhow の使い分けで（dtolnay のベストプラクティス）"** |

Ref: https://memory-lovers.blog/entry/2025/06/27/102550

## Core Principle

> The more your code looks like majority Rust code, the better AI can help with it.
> — Microsoft Pragmatic Rust Guidelines

Use idiomatic patterns. Avoid clever abstractions. Standard = AI-friendly.

## What AI Agents Do Well

- Boilerplate with clear specifications
- Standard `Result<T, E>` error propagation with `?`
- CLI tools using `clap` 4.x
- Simple async I/O with Tokio
- Builder patterns and newtype wrappers
- Unit tests for pure functions

## Where Human Review Is Required

- `unsafe` blocks — always review manually
- Complex generic code with multiple trait bounds
- Lifetime-heavy code (especially `async` + lifetimes)
- Performance-critical paths — AI may produce correct but slow code
- Lock-free concurrency patterns
- FFI (C interop)

## API Hallucination Prevention

AI training data may be months stale. Always:

1. **Specify crate versions explicitly** in prompts and Cargo.toml
2. **Run `cargo check` immediately** after generated code — catch hallucinated APIs fast
3. **Use `cargo doc --open`** to verify API exists before trusting generated code
4. Prefer crates with stable APIs (tokio, serde, clap, anyhow) over rapidly-evolving ones

```toml
# Always pin major versions
tokio = "1"      # Not "tokio = "*"
serde = "1"
clap = "4"
```

## Known LLM Failure Patterns

| Pattern | Why It Fails | Alternative |
|---|---|---|
| Complex generics with multiple bounds | LLM adds incorrect bounds | Use concrete types first, generalize later |
| `async fn` + lifetime parameters | Hard for LLMs to reason about | Restructure to avoid borrowed futures |
| Custom allocators | Rare in training data | Use standard allocator |
| Macro-heavy code | Hard to reason about | Prefer functions |

## TDD as Safety Net

Use the compiler and tests as verification:

1. Write a failing test first
2. Let the agent implement until `cargo test` passes
3. `cargo clippy` and `cargo fmt` after
4. Human reviews correctness of logic, not syntax

The Rust compiler catches entire classes of bugs — lean on it.

## Idiomatic Patterns to Enforce

```rust
// Always: propagate errors with ?
fn process() -> Result<Output, Error> {
    let data = read_input()?;
    let result = transform(data)?;
    Ok(result)
}

// Never: unwrap in non-test code
let data = read_input().unwrap();  // panics in production

// Always: newtype for domain clarity (helps AI understand intent)
struct Port(u16);
struct DomainName(String);

// Never: bare primitives for domain concepts
fn connect(host: String, port: u16) { }  // ambiguous
```

## Strong Types Help AI

Custom types prevent AI from confusing domain concepts:

```rust
// Good: AI can't mix up Port and timeout
struct Port(u16);
struct TimeoutSecs(u64);

fn connect(host: &str, port: Port, timeout: TimeoutSecs) { }
```

## Documentation for Agent Context

Write doc comments as if explaining to an agent with no context:

```rust
/// Parses a domain name from a CONNECT request host header.
///
/// The host header format is `hostname:port` or `hostname` (port defaults to 80).
/// Returns only the hostname portion.
///
/// # Errors
///
/// Returns `ParseError::InvalidHost` if the hostname contains invalid characters.
pub fn parse_host(header: &str) -> Result<DomainName, ParseError> { }
```
