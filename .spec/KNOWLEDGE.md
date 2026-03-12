# KNOWLEDGE — Accumulated Discoveries

> Written by the agent. Accumulates across sessions.
> Record decisions, rationale, and non-obvious findings here.
> Do not write session-specific state — use HANDOFF.md for that.

## Decisions

| Decision | Rationale | Date |
|---|---|---|
| tokio + clap + serde + toml | SPEC.md 確定済み。軽量で十分 | 2026-03-10 |
| 完全一致のみ（ワイルドカードなし v1） | YAGNI。育てる設計なので後から追加 | 2026-03-10 |

## Technical Findings

- `TcpStream::split()` で ReadHalf/WriteHalf に分割後、NLL により最終使用後にborrowが解放される。`copy_bidirectional(&mut stream)` は両 half の最終使用後に呼べる
- `build-essential`（cc リンカー）がホストにないと `cargo build` 失敗。WSL2 初期状態では未インストールの場合あり

## TDD を実施しなかった（2026-03-12 反省）

**何が起きたか**: SPEC.md 確定 → TODO.md 作成 → いきなり実装。tdd-guide エージェントを使わず、テストを一切書かなかった。

**本来のフロー**:
1. tdd-guide エージェントで RED テストを先に書く
2. `cargo test` で失敗を確認
3. 最小実装でグリーンにする
4. リファクタリング

**やるべきこと**: 統合テストを追加する（`tests/proxy_test.rs`）

## 参照すべきリソース（2026-03-12 記録）

- **cloudnative-co/claude-code-starter-kit**: Claude Code 開発環境の事実上の標準。TDD・エージェント構成・safety hook の参考に。Rust 固有の内容はないが workflow は参考になる
- **contextus-dev-rust/rules/**: Rust ベストプラクティス（linting, testing, error-handling 等）。実装前に必ず参照する

## Rejected Approaches

- `--permission-mode auto` on sandbox: サーバー依存・root 制限あり → `bypassPermissions` + non-root が正解（tutus 側で解決済み）
