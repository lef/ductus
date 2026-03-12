# HANDOFF — Session Transition Notes

**Last Updated**: 2026-03-13
**Previous Work**: contextus 同期 + contextus-dev-rust 導入 + フェーズ0.5 SDD 文書整備

## Current State

### Completed (this session)

**contextus L0/L1 同期**:
- `safety-net.sh` + `safety-net-fragment.json` → `.claude/hooks/`（実行可能）
- `dumpmem/SKILL.md` → `.claude/skills/`（`/dumpmem` コマンド有効）
- `agent-security.md` + `sdd.md` → `.claude/rules/`（L0 ルール追加）
- `settings.json` に safety-net を PreToolUse フックとして追記

**contextus-dev-rust L2 インストール**:
- 旧 `.claude/rules/rust-style.md` 削除 → `.claude/rules/rust/` に13ファイル配置
- `CLAUDE.md` Key Files テーブル更新

**contextus-dev-rust フィードバック**:
- `contextus-dev-rust/.spec/TODO.md` に dogfooding 観点で4件の課題を追記

**フェーズ0.5 SDD 文書**:
- `.spec/SPEC.md` にフェーズ0.5仕様追記（抽出関数・エラーハンドリング・テスト計画）
- `.spec/TODO.md` にフェーズ0.5チェックリスト追記（RED/GREEN/REFACTOR）

**PLAN.md + SPEC.md の目的記録**:
- フェーズ0の核心（AI フィードバックループ）を PLAN.md に追記
- 403 レスポンス仕様を AI 読み手前提に修正（SPEC.md）
- `proxy/README.md` 新規作成（人間・AI 両対応のシンプルな API doc）
- 403 ボディを `BLOCKED: <host>` / `ALLOWLIST: <path>` の key-value 形式に変更

### Not Started (priority order)

1. **フェーズ0.5 RED フェーズ** — `parse_connect_target` 単体テスト4件 + `tests/proxy_test.rs` 統合テスト4件を書く（Compile RED を確認してから実装へ）
2. **フェーズ0.5 GREEN フェーズ** — `anyhow` 追加、関数抽出、400/502 実装
3. **フェーズ0.5 REFACTOR フェーズ** — `main()` 薄く、doc comments、clippy
4. **フェーズ1** — HTTPS インターセプト設計（SPEC.md 作成から）

## Next Session: Read First

- `.spec/TODO.md` — フェーズ0.5チェックリスト（RED フェーズから開始）
- `.spec/SPEC.md` — フェーズ0.5仕様（抽出する関数・エラーハンドリング表）
- `proxy/src/main.rs` — 現在の実装（124行）
- `proxy/README.md` — 403 レスポンス形式（実装と一致させること）

## Key Decisions Made

- **contextus-dev-rust rules を `rules/rust/` サブディレクトリに配置**: フラットな `rules/` と分離
- **403 ボディをコマンドなし key-value 形式に変更**: `echo` コマンド埋め込みはコマンド注入に見えるため廃止。`BLOCKED:` / `ALLOWLIST:` の2行のみ
- **フェーズ0の読み手は AI**: 403 レスポンスは tutus 内の AI エージェントが読んで allowlist を育てるための設計
- **実装前にモデル切り替え済み**: フェーズ0.5 RED フェーズは Opus で開始

## Blockers / Watch Out For

- `proxy/src/main.rs` の現在の 403 ボディ実装は古い形式（`"is not in the allowlist."` メッセージ）。フェーズ0.5 REFACTOR で新 key-value 形式に合わせること
- contextus-dev-rust の `setup.sh` 連携が未定義（L2 インストール方法が手動コピーのまま）

## Changed Files

- `.claude/hooks/safety-net.sh`: 新規（L1 から追加）
- `.claude/hooks/safety-net-fragment.json`: 新規（参照用テンプレート）
- `.claude/skills/dumpmem/SKILL.md`: 新規（L1 から追加、memory パスを ductus 用に修正）
- `.claude/rules/agent-security.md`: 新規（L0 から追加）
- `.claude/rules/sdd.md`: 新規（L0 から追加）
- `.claude/rules/rust-style.md`: 削除（L2 の `rust/rust-style.md` に置き換え）
- `.claude/rules/rust/*.md`: 新規13ファイル（contextus-dev-rust L2）
- `.claude/settings.json`: safety-net フック追記
- `CLAUDE.md`: Key Files テーブル更新
- `.spec/PLAN.md`: フェーズ0核心（AI フィードバックループ）追記
- `.spec/SPEC.md`: フェーズ0.5仕様追記、403 ボディ形式変更
- `.spec/TODO.md`: フェーズ0.5チェックリスト追記
- `proxy/README.md`: 新規（API doc）
- `contextus-dev-rust/.spec/TODO.md`: dogfooding フィードバック4件追記
