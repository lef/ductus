# HANDOFF — Session Transition Notes

**Last Updated**: 2026-03-13
**Previous Work**: フェーズ0.6完了（static binary / GitHub Actions / wildcard / SIGHUP / tutus 統合）

## Current State

### Completed (this session)

**contextus インフラ整備**:
- setup.sh の bats テスト追加（contextus-claude `tests/test-setup.sh`）
- tutus の symlink 構造を実ディレクトリに移行（`.claude -> .contextus/.claude` 廃止）
- `.claude/.contextus/layers` に contextus-dev-rust を記録（`setup.sh --update` 対応）

**フェーズ0.6 完了（TDD）**:
- static binary: `[profile.release]` + Makefile（`uname -m` で arch 自動検出）
- GitHub Actions: CI（push/PR で fmt/clippy/test）+ release（tag v* で MUSL binary）
- ワイルドカード allowlist: `Allowlist` struct + `wildcard_match()`（外部 crate 不要）
- SIGHUP リロード: `Arc<RwLock<Allowlist>>`、`reload_allowlist()` 公開関数
- tutus 統合: `env.allowlist` に HTTP_PROXY 系追加、sandbox script に proxy 確認
- aarch64 MUSL binary ビルド確認・`~/.local/bin/ductus` に配置
- 全 21 テスト GREEN（単体 17 + 統合 4）

### Not Started (priority order)

1. **フェーズ1** — HTTPS インターセプト設計（SPEC.md 作成から、SDD ワークフロー）
2. **GitHub への push・tag**（`git tag v0.1.0 && git push && git push --tags`）でリリース binary を生成
3. **tutus での実運用確認** — `HTTP_PROXY` を設定して sandbox を起動し実際に動作確認

## Next Session: Read First

- `.spec/TODO.md` — フェーズ0.6 完了確認、次はフェーズ1
- `proxy/src/lib.rs` — `Allowlist` struct + `reload_allowlist()` を含む現在の実装
- `proxy/README.md` — 403 仕様と使い方

## Key Decisions Made

- **ホストは aarch64**: x86_64-unknown-linux-musl はクロスコンパイルになりリンカエラー。Makefile で `uname -m` 自動検出
- **`std::sync::RwLock`**: SIGHUP ハンドラの write は非同期 I/O なし → tokio 版不要
- **tutus proxy 確認は警告のみ**: hard fail にしない（別 proxy の可能性もある）
- **Rust コードを書く前はモデル変更を確認**: Rust コードが不要になったら戻せると伝える

## Blockers / Watch Out For

- **GitHub Actions の release workflow 未実行**: リポジトリが GitHub に push されていないため未動作。tag を push して初めて確認できる
- **tutus での実動作未確認**: proxy を実際に起動して sandbox から通信させるテストがまだ

## Changed Files (this session)

**ductus**:
- `proxy/Cargo.toml`: `[profile.release]` 追加
- `proxy/Makefile`: 新規（build-static、verify-static、uname -m 自動検出）
- `.github/workflows/ci.yml`: 新規
- `.github/workflows/release.yml`: 新規
- `proxy/src/lib.rs`: `Allowlist` struct、`wildcard_match()`、`reload_allowlist()`、`Arc<RwLock<Allowlist>>`
- `proxy/src/main.rs`: `RwLock` ラップ、SIGHUP タスク追加
- `proxy/tests/proxy_test.rs`: `spawn_proxy()` を tempfile + `RwLock` ベースに更新
- `.claude/.contextus/layers`: 新規（contextus-dev-rust 記録）
- `.spec/KNOWLEDGE.md`: フェーズ0.6 の発見を追記
- `.spec/TODO.md`: フェーズ0.6 チェックリスト完了

**tutus**:
- `scripts/env.allowlist`: HTTP_PROXY 系追加
- `scripts/claude-sandbox.sh`: proxy 稼働確認ブロック追加
- `scripts/aider-sandbox.sh`: proxy 稼働確認ブロック追加
- `.claude/`: symlink から実ディレクトリに移行（`.contextus/.claude/` → `.claude/`）
- `.spec/`: 同上
- `HANDOFF.md`: 同上（symlink → 実ファイル）

**contextus-claude**:
- `tests/test-setup.sh`: 新規（record_layer/read_layers/apply_layer のユニットテスト）
