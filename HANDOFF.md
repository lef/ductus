# HANDOFF — Session Transition Notes

**Last Updated**: 2026-03-13
**Previous Work**: フェーズ0.5完了 + contextus エコシステム大規模 contrib

## Current State

### Completed (this session)

**フェーズ0.5 完了（TDD）**:
- RED: `parse_connect_target` 単体4件 + 統合4件（Compile RED 確認済み）
- GREEN: lib.rs 分離、anyhow 導入、400/502 実装、全13テスト パス
- REFACTOR: cargo fmt + clippy クリーン

**contextus L0/L1 同期**:
- safety-net.sh、dumpmem skill、agent-security.md、sdd.md 追加
- settings.json に safety-net PreToolUse フック追記

**contextus-dev-rust L2 インストール**:
- rules/rust/ に13ファイル配置（L1 との名前空間分離）

**contextus エコシステム contrib**:
- SessionStart フック: .spec/ ファイルを自動 inject（contextus-claude）
- setup.sh --update: L0/L1/L2+ 一括同期 + layers manifest（contextus-claude）
- L2 名前空間修正: rules/<profile>/ でインストール（contextus-claude）
- TDD HARD RULE: L1 tdd-guide に集約、L2 の重複削除（3 repos）
- contextus (L0): 層間互換性課題を TODO.md に記録
- contextus-dev-sh: testing.md 新規作成（bats TDD）
- tutus: session-start.sh 同期済み

### Not Started (priority order)

1. **setup.sh の bats テスト追加** — HARD RULE 適用: 実装したがテストなし（TODO.md に記録済み）
2. **tutus の symlink 構造を実ディレクトリに移行** — .claude -> .contextus/.claude を廃止
3. **フェーズ1** — HTTPS インターセプト設計（SPEC.md 作成から、SDD ワークフロー）

## Next Session: Read First

- `.spec/TODO.md` — contextus 改善タスクとフェーズ1が次
- `proxy/src/lib.rs` — 現在の実装（フェーズ0.5完了後）
- `proxy/README.md` — 403 仕様（BLOCKED:/ALLOWLIST: 形式）

## Key Decisions Made

- **TDD HARD RULE は L1 に集約**: L2 は言語固有の「どうやって」のみ
- **/dumpmem と /handoff の使い分け**: 大量作業後は /dumpmem、セッション終了時は /handoff
- **setup.sh --update**: layers manifest（.claude/.contextus/layers）で L2+ を記録
- **L2 名前空間**: rules/<profile>/ サブディレクトリでインストール（L1 との衝突防止）

## Blockers / Watch Out For

- **setup.sh に bats テストがない**: HARD RULE 違反。次回優先でテスト追加が必要
- **tutus の symlink**: .contextus/.claude/ 経由で直接 git add が必要（.claude/ 経由は失敗）
- **contextus-dev-rust の .spec/TODO.md**: レビュー待ちルールが残っている

## Changed Files (this session)

**ductus**:
- `proxy/src/lib.rs`: 新規（フェーズ0.5 — lib/bin 分離）
- `proxy/src/main.rs`: 薄い CLI ラッパーに変更
- `proxy/tests/proxy_test.rs`: 新規（統合テスト4件）
- `proxy/Cargo.toml`: anyhow 追加
- `.claude/agents/tdd-guide.md`: HARD RULE 追記（sync）
- `.claude/rules/rust/testing.md`: HARD RULE 削除（L1 に集約）
- `.claude/setup.sh`: --update フラグ追加（sync）
- `.spec/KNOWLEDGE.md`: セッション後半の発見を追記
- `.spec/TODO.md`: contextus 改善タスク追記

**contextus-claude**: session-start.sh、setup.sh、tdd-guide.md、TODO.md
**contextus-dev-rust**: testing.md（HARD RULE 削除）
**contextus-dev-sh**: testing.md 新規作成
**contextus (L0)**: .spec/TODO.md 新規作成（層間互換性課題）
**tutus**: session-start.sh 同期
