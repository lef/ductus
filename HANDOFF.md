# HANDOFF — Session Transition Notes

**Last Updated**: 2026-03-13
**Previous Work**: hybrid allowlist Rust 実装（--session-allowlist / --blocked-log / --pidfile）

## Current State

### Completed (this session)

- **hybrid allowlist 実装**: tutus からの依頼を TDD で実装（t_wada 方式）
  - `--session-allowlist <path>` — 第2の allowlist、SIGHUP で永続と一緒にリロード
  - `--blocked-log <path>` — ブロックドメインをタイムスタンプ付きでログ（in-memory 重複排除）
  - `--pidfile <path>` — 起動時に自 PID をファイルに書く
  - `BlockedLog` 構造体（`Arc<Mutex<>>`で共有）
  - `load_merged_allowlist` / `reload_merged_allowlist` 追加
  - chrono 不使用の手書き UTC フォーマット（`SystemTime` + 手計算）
- **全 39 テスト GREEN**（unit 30 + integration 9）— 既存21件維持、新規18件追加
- **static binary 再ビルド＋インストール**（`~/.local/bin/ductus`）
  - `--help` で3フラグが確認済み

### In Progress

なし

### Not Started (priority order)

1. **tutus 側の `ductus-session.sh` 更新** — 新フラグを使う起動コマンドに変更（tutus repo の仕事）
2. **GitHub リリース**（低優先度）— `git tag v0.1.0 && git push && git push --tags`
3. **ポート指定 localhost 許可**（低優先度）— `localhost:3000` 形式の allowlist エントリ対応
4. **フェーズ1** — HTTPS インターセプト設計。当面先

## Next Session: Read First

- `.spec/TODO.md` — 現状確認
- tutus HANDOFF に `ductus-session.sh` 更新依頼を入れる（まだなら）
- `ductus --help` で `--session-allowlist`, `--blocked-log`, `--pidfile` が出ることを確認

## Key Decisions Made

- **chrono 非依存の UTC フォーマット**: `SystemTime` + 手計算。外部 crate 追加不要
- **`BlockedLog` は `Arc<Mutex<>>`（tokio 版でなく）**: ファイル書き込みは sync、`.await` またがない
- **`run()` に `blocked_log` を渡す設計**: 各 `handle_inner()` 呼び出しで直接ログを取る
- **既存 `reload_allowlist` はそのまま残す**: `reload_merged_allowlist` を追加し、`main.rs` は後者を使う

## Blockers / Watch Out For

- GitHub Actions release は未実行（push していない）
- tutus の `ductus-session.sh` はまだ旧コマンド（`--session-allowlist` 等なし）で動作中
  - 現状でも動く（新フラグはオプション）。tutus セッションで更新すればよい
- `Text file busy` エラー: ductus が起動中のまま `cp` すると失敗する。`pkill ductus` してから

## Changed Files

- `proxy/src/lib.rs`: `BlockedLog`, `load_merged_allowlist`, `reload_merged_allowlist`, `format_utc_now`, `is_leap` 追加。`run()`/`handle()`/`handle_inner()` に `blocked_log` 引数追加。unit tests 13件追加
- `proxy/src/main.rs`: `--session-allowlist`, `--blocked-log`, `--pidfile` フラグ追加。`load_merged_allowlist` 使用。SIGHUP ハンドラを `reload_merged_allowlist` に変更
- `proxy/tests/proxy_test.rs`: `spawn_proxy_with_opts()` 追加。integration tests 5件追加

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
