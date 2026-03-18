# HANDOFF — Session Transition Notes

**Last Updated**: 2026-03-18
**Previous Work**: --port 0 auto-assign + graceful shutdown (SIGTERM) + contextus-dev-rust 同期

## Current State

### Completed (this session)

- **`--port 0` + stdout 返却**: OS に空きポート割り当て、実ポートを stdout に出力
- **graceful shutdown**: SIGTERM で accept loop 停止 + pidfile 自動削除
- **`run()` 署名変更**: `shutdown: impl Future<Output = ()>` 引数追加、`tokio::select!` で accept loop
- **テスト3件追加**（合計42テスト: unit 30 + integration 12）
- **contextus-dev-rust 同期**: Model Switching ルール更新（Opus 4.6 1M で基本不要に）
- **sandbox 環境確認**: Rust 1.94.0 が sandbox 内で完全動作（persistent overlay 解消済み）

### In Progress

なし

### Not Started (priority order)

1. **`--blacklist`** — 永久ブロックリスト。allowlist にあっても拒否
2. **`--audit-log`** — 全 CONNECT リクエストを記録（許可・ブロック両方）
3. **GitHub リリース**（低優先度）— `git tag v0.1.0 && git push && git push --tags`
4. **tutus 側 `ductus-session.sh` 更新** — `--port 0` + graceful shutdown 対応
5. **透過プロキシモード** — HTTP_PROXY 不要で全通信キャプチャ
6. **フェーズ1** — HTTPS インターセプト設計。当面先

## Next Session: Read First

- `.spec/TODO.md` — 現状確認
- `ductus --help` で全フラグ確認
- tutus HANDOFF に `--port 0` + SIGTERM 対応の依頼を入れる

## Key Decisions Made

- **chrono 非依存の UTC フォーマット**: `SystemTime` + 手計算。外部 crate 追加不要
- **`BlockedLog` は `Arc<Mutex<>>`（tokio 版でなく）**: ファイル書き込みは sync、`.await` またがない
- **`run()` に `shutdown: impl Future<Output = ()>`**: テスタビリティ + 柔軟性。テストは `oneshot` や `pending()` を渡せる
- **port 0 時のみ stdout 出力**: 既存スクリプトの互換性維持。stderr のログは常に actual_port
- **in-flight 接続の drain なし**: shutdown 時は accept loop のみ停止。spawned tasks は runtime 終了時に drop（現状十分）
- **Opus 4.6 1M でモデル切り替え不要**: contextus-dev-rust のルールは「コスト最適化モード」として残した

## Rust 開発環境（2026-03-18 確認済み）

**sandbox 内で完全動作**:
- rustc 1.94.0 + aarch64-unknown-linux-musl ターゲット
- `source ~/.cargo/env` で PATH 設定
- ductus 自身がプロキシとして動作中（HTTP_PROXY=http://127.0.0.1:8080）
- `~/repos/` に contextus-dev-rust 等あり — 直接 contribution 可能

## Blockers / Watch Out For

- GitHub Actions release は未実行（push していない）
- `Text file busy` エラー: ductus が起動中のまま `cp` すると失敗する。`pkill ductus` してから
- tutus の `ductus-session.sh` は `--port 0` 未対応（まだ ss ループでポートスキャン中）

## Changed Files (this session)

- `proxy/src/lib.rs`: `run()` に `shutdown` 引数追加、`tokio::select!` で accept loop
- `proxy/src/main.rs`: SIGTERM ハンドラ、`--port 0` stdout 出力、pidfile クリーンアップ
- `proxy/tests/proxy_test.rs`: shutdown テスト2件 + port 0 テスト1件追加
- `.claude/rules/rust/ai-agent-rust.md`: Model Switching セクション更新
- `.spec/KNOWLEDGE.md`: 新決定事項 + sandbox 環境確認 + 実装記録
- `.spec/TODO.md`: フェーズ0.7 セクション追加

## Feature Request: 全通信の audit log（2026-03-13）

**要求**: `--audit-log` オプション — ブロックしたものだけでなく、全 CONNECT リクエストを記録する。

**ユースケース**:
- 新規ツールを sandbox で初めて動かす際、そのツールが接続を試みた全ドメインを把握する
- 「怪しいツールが密かに通信している先」を洗い出す（拡張機能のデータ盗難検知等）
- allowlist 登録前の審査フローとして使う

**現状との差分**:
- `--blocked-log`: ブロックされたドメインのみ記録（既存）
- `--audit-log`: 許可・ブロック両方の全 CONNECT を記録（新規）

**tutus 側の対応**: `ductus-review` の上位版として全通信レビューフローを追加予定。

## Feature Request: --blacklist フラグ（2026-03-14）

**要求**: `--blacklist <path>` — 永久ブロックリスト。allowlist にあっても拒否する。

**ユースケース（tutus 側）**:
- `ductus-review` の `[i]gnore` コマンドで `~/.config/ductus/review-ignore.txt` に追記
- ductus 起動時に `--blacklist ~/.config/ductus/review-ignore.txt` を渡す
- → ignore したドメインがプロキシレベルで強制ブロックされる（UI スキップだけでなく実効性あり）

**tutus 側の対応**:
- `ductus-session.sh` で `DUCTUS_IGNORE_FILE` を `--blacklist` に渡す（フラグ実装後）
- allowlist + session-allowlist に入れていてもここにあれば弾く動作を想定

---

## Feature Request: 透過プロキシモード（2026-03-13）

**要求**: HTTP_PROXY 設定不要で全通信をキャプチャする透過プロキシ。

**ユースケース**:
- proxy 設定を無視するツールの通信も捕捉できる
- 「怪しいツールが密かに通信している先」を完全に洗い出す
