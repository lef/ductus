# Dependency Management

## Preferred Stable-API Crates

これらのクレートは API が安定しており、AI の幻覚リスクが低い:

| Purpose | Crate | Notes |
|---|---|---|
| Async runtime | `tokio = "1"` | de facto standard |
| Serialization | `serde = "1"` | always specify features |
| CLI | `clap = "4"` | derive feature |
| Error (library) | `thiserror = "1"` | |
| Error (binary) | `anyhow = "1"` | |
| HTTP client | `reqwest = "0.12"` | with `rustls-tls` |
| Logging | `tracing = "0.1"` | prefer over `log` |

## MSRV Declaration

```toml
# Cargo.toml
[package]
rust-version = "1.70"   # Minimum Supported Rust Version
```

MSRV を下げるのはコストがかかる。最初から設定して CI で検証する。

```bash
# CI での MSRV 検証
cargo +1.70 test
```

## Version Pinning Strategy

```toml
# Binary (application): semver range, Cargo.lock をコミットする
tokio = "1"        # 1.x.y の最新を許容
serde = "1.0.100"  # 特定バージョンに固定したい場合

# Library: 範囲を広めに取る（利用者の依存解決を助ける）
tokio = "1"        # "^1.0" と同じ
serde = "1"
```

**Cargo.lock**:
- Binary: コミットする（再現可能ビルド）
- Library: コミットしない（`.gitignore` に追加）

## Hallucinated Crate 防止

AI が存在しないクレートや API を生成することがある:

```bash
# 依存追加後すぐに確認
cargo add some-crate
cargo check          # ← 必ずここで止まらず確認する
```

疑わしいクレートは先に確認:

```bash
cargo search some-crate   # crates.io に存在するか
cargo info some-crate     # バージョン・ダウンロード数確認
```

ダウンロード数が少ない / 最終更新が古い / ドキュメントがないクレートは避ける。

## Security Scanning

```bash
# 脆弱性スキャン（RustSec Advisory DB 参照）
cargo install cargo-audit
cargo audit

# ライセンス・重複依存の強制
cargo install cargo-deny
cargo deny check
```

```toml
# deny.toml — プロジェクトルートに配置
[licenses]
allow = ["MIT", "Apache-2.0", "BSD-2-Clause", "BSD-3-Clause"]

[bans]
multiple-versions = "warn"   # 同一クレートの複数バージョンを警告
```

## Dependency Hygiene

```bash
cargo tree              # 依存ツリー全体を確認
cargo tree -d           # 重複クレートのみ表示
cargo machete           # 未使用依存の検出（要インストール）
```
