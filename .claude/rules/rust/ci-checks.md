# CI Checks

## Standard Sequence

```bash
# 1. Format check (fast, catches style issues)
cargo fmt --check

# 2. Lint (catches common mistakes, enforces idioms)
cargo clippy -- -D warnings

# 3. Tests
cargo test

# 4. Doc build (catches broken doc links)
cargo doc --no-deps --document-private-items
```

**この順番に意味がある**: 速いチェックを先に置いて失敗を早期検出する。
fmt → clippy はコンパイルより速い。テスト前に弾く。

## MSRV Verification

```bash
# Cargo.toml の rust-version が実際に通るか確認
rustup install 1.70
cargo +1.70 test
cargo +1.70 clippy -- -D warnings
```

MSRV を上げる PR には必ずこの検証を含める。

## GitHub Actions Example

```yaml
jobs:
  check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: rustfmt, clippy
      - uses: Swatinem/rust-cache@v2

      - run: cargo fmt --check
      - run: cargo clippy -- -D warnings
      - run: cargo test
      - run: cargo doc --no-deps

  msrv:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@1.70   # Cargo.toml の rust-version に合わせる
      - run: cargo test
```

`Swatinem/rust-cache` でビルドキャッシュを有効化するとCIが大幅に速くなる。

## Security Checks (Optional but Recommended)

```yaml
  audit:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: rustsec/audit-check@v2
        with:
          token: ${{ secrets.GITHUB_TOKEN }}
```

または `cargo audit` を CI に追加:

```bash
cargo install cargo-audit --locked
cargo audit
```

## Pre-Commit Hooks (Local)

```bash
# .git/hooks/pre-commit
#!/bin/sh
set -e
cargo fmt --check
cargo clippy -- -D warnings
```

自動修正したい場合:

```bash
cargo fmt          # format in-place
cargo clippy --fix -- -D warnings  # safe auto-fixes
```

`cargo clippy --fix` は安全な修正のみ適用する。破壊的な変更は手動。

## Clippy Lints Configuration

```toml
# Cargo.toml または .clippy.toml
[lints.clippy]
pedantic = "warn"          # 厳しめのリント（警告として）
unwrap_used = "warn"       # unwrap() を警告
expect_used = "allow"      # expect() は許可（メッセージ付きなら OK）
```

または `#![deny(clippy::pedantic)]` をクレートルートに。
CI では `-D warnings` で全警告をエラー化する。
