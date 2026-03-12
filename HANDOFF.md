# HANDOFF — Session Transition Notes

**Last Updated**: 2026-03-13
**Previous Work**: フェーズ0完了（HTTP CONNECT proxy 実装・テスト済み）

## Current State

### Completed

**フェーズ0 — sandbox 用 HTTP CONNECT proxy**:
- `proxy/src/main.rs` 69行で実装完了
- evil.com → 403、example.com → 通過 ✅
- `load_allowlist` ユニットテスト 5件（t_wada TDD）
- Rust 1.94.0 / build-essential がホストに必要

**ビジョン回収（2026-03-13）**:
- 本来の目的: パーソナルウェブアーカイブプロキシ → `.spec/PLAN.md` に記録済み
- フェーズ0は sandbox 用途として先に実装した中間ステップ

### Not Started (priority order)

1. **フェーズ1 SPEC.md 作成** — HTTPS インターセプト設計（SDD ワークフローで進める）
2. **自前 CA 方式の調査** — rustls vs openssl、ブラウザへのインストール
3. **ログ形式の設計** — WARC / SQLite / plain files の比較

詳細は `.spec/TODO.md` 参照。

## Next Session: Read First

- `.spec/PLAN.md` — ビジョン全体（フェーズ0完了後の次フェーズが書いてある）
- `.spec/TODO.md` — フェーズ1タスクリスト
- `proxy/src/main.rs` — 現在の実装（69行）

## Key Decisions Made

- **フェーズ0 = sandbox 用途**: tutus の AI エージェント通信制御
- **フェーズ1 = パーソナルアーカイブ**: 全ブラウザ通信記録、HTTPS インターセプト必須
- **allowlist → blocklist**: パーソナル用途では全通過がデフォルトになる可能性

## Changed Files

- `.spec/PLAN.md`: 本来のビジョン追記（2026-03-13）
- `.spec/TODO.md`: フェーズ1タスク追加（2026-03-13）
- `proxy/src/main.rs`: ユニットテスト追加
- `proxy/Cargo.toml`: tempfile dev-dependency 追加
