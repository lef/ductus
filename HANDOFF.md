# HANDOFF — Session Transition Notes

**Last Updated**: 2026-03-10 22:55
**Previous Work**: CLAUDE.md作成、HTTP CONNECTプロキシのPLAN.md・SPEC.md作成

## Current State

### Completed (this session)
- `CLAUDE.md` 作成（codebase初期化 `/init`）
- `.spec/PLAN.md` 作成（HTTP CONNECT proxy, ~50行Rust, domain allowlist）
- `.spec/SPEC.md` 作成（設計を詳細化、ユーザーとの質疑応答を経て確定）

### In Progress
- なし

### Not Started (priority order)
1. `.spec/TODO.md` 作成（SPEC.md から実装タスクを分解）
2. `proxy/` ディレクトリ作成、Cargo.toml 作成
3. `proxy/src/main.rs` 実装（~50–60行）
4. `proxy/config.toml` と `proxy/allowlist.txt` のサンプル作成
5. 動作確認

## Next Session: Read First

- `.spec/SPEC.md` — 設計の全詳細はここ
- `.spec/PLAN.md` — ユーザーの元の意図
- まず TODO.md を作ってユーザーに確認してから実装に入ること（SDD ルール）

## Key Decisions Made

- **設定ファイル**: `config.toml`（port, allowlist パスを定義）。git管理対象
- **CLI引数**: `--config`, `--port`, `--allowlist`。優先順位: CLI > config.toml > デフォルト
- **allowlist**: テキストファイル、1行1ドメイン、`#`コメント対応、完全一致のみ（v1）
- **依存クレート**: tokio, clap（derive）, serde + toml
- **403ボディ**: ブロックされたホスト名と `echo "<host>" >> allowlist.txt` のコマンドを明示。ユーザーがallowlistを育てられるように
- **行数目標**: ~50行はタイト → ~50–60行を許容範囲とする

## Blockers / Watch Out For

- 50行の制約と clap/serde の行数は競合する。derive マクロで圧縮する方針
- ワイルドカードマッチ（`*.example.com`）はスコープ外（将来対応）

## Changed Files

- `CLAUDE.md`: 新規作成（codebase概要、フック動作、SDD workflow）
- `.spec/PLAN.md`: テンプレートから実内容に更新
- `.spec/SPEC.md`: 新規作成（HTTP CONNECTプロキシの設計仕様）
