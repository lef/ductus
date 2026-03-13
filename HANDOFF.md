# HANDOFF — Session Transition Notes

**Last Updated**: 2026-03-13
**Previous Work**: フェーズ0.6完了 → ホスト動作確認 → tutus/contextus-dev-rust contrib

## Current State

### Completed (this session)

**フェーズ0.6 完了（前回 dumpmem まで）**:
- static binary / GitHub Actions / wildcard / SIGHUP / tutus 統合
- 全 21 テスト GREEN（単体 17 + 統合 4）

**動作確認・後始末（今回）**:
- ホスト上での curl テスト: `api.github.com` → 200, `evil.com` → 403 ✅
- GitHub Actions push は意図的に後回し（free 枠節約）
- tutus の HANDOFF に実動作確認タスクを投げ込み（tutus 側の仕事として委譲）
- contextus-dev-rust `rules/ai-agent-rust.md` に Rust コード編集時のモデル切り替えルールを追記

### Not Started (priority order)

1. **tutus からの依頼: hybrid allowlist 機能の Rust 実装**（下記参照）
2. **GitHub リリース**（低優先度）— `git tag v0.1.0 && git push && git push --tags`。急がない
3. **フェーズ1** — HTTPS インターセプト設計。当面先

---

## tutus からの依頼: hybrid allowlist 学習システム（Rust 実装）

**依頼元**: tutus（`lef/tutus`）セッション 2026-03-13
**背景**: tutus 側で shell スクリプト（`ductus-allow`, `ductus-review`, `ductus-session.sh`）を実装済み。これらのスクリプトが依存する ductus 側の Rust 機能が必要。

### 追加が必要な CLI フラグ

```
--session-allowlist <path>   第2の allowlist ファイル（セッション限定）
                              SIGHUP でメインと一緒にリロードする
--blocked-log <path>          ブロックされたドメインをログに書く
                              セッション内重複排除（in-memory HashSet）
                              形式: 2026-03-13T14:23:01Z domain.com（1行1件）
--pidfile <path>              起動時に自 PID をファイルに書く
                              shell スクリプトが SIGHUP のために使う
```

### 実装すべき構造体・関数

```rust
// lib.rs に追加
pub struct BlockedLog {
    seen: HashSet<String>,
    file: Option<std::fs::File>,
}
pub fn new_blocked_log(path: Option<&str>) -> Arc<Mutex<BlockedLog>>
pub fn load_merged_allowlist(permanent: &str, session: Option<&str>) -> Allowlist
pub fn reload_merged_allowlist(allowlist: &Arc<RwLock<Allowlist>>, permanent: &str, session: Option<&str>)
```

`Allowlist` 構造体の変更は不要（`load_merged_allowlist` が両ファイルをマージして同じ構造体を返す）。

### SIGHUP ハンドラの変更

```rust
// 現在: reload_allowlist(&al_sig, &path_sig)
// 変更後: reload_merged_allowlist(&al_sig, &permanent_path, session_path.as_deref())
```

`session_path` を `Arc<Option<String>>` として SIGHUP タスクにクローン渡しする。

### TDD: 追加すべきテスト（18件）

**unit tests (lib.rs) — 13件**:
1. `blocked_log_no_path_no_file` — `new_blocked_log(None)` → ファイルなし、record しても panic しない
2. `blocked_log_records_to_file` — ファイルあり、record 後にファイルが存在する
3. `blocked_log_deduplicates` — 同ドメインを2回 record → ファイルは1行
4. `blocked_log_timestamp_format` — 行が `^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z ` にマッチ
5. `blocked_log_multiple_domains` — a.com + b.com + a.com → 2行
6. `load_merged_no_session` — `load_merged_allowlist("perm", None)` == `load_allowlist("perm")`
7. `load_merged_adds_session_entries` — perm に a.com、session に b.com → 両方許可
8. `load_merged_session_missing_ok` — session パスが存在しない → エラーなし、perm のみ
9. `load_merged_session_wildcards` — session に `*.crates.io` → `static.crates.io` が許可される
10. `reload_merged_picks_up_session_change` — session ファイルに追記 → SIGHUP 相当の reload で反映
11. `reload_merged_session_none_ok` — `reload_merged_allowlist(arc, "perm", None)` panic しない
12. `blocked_log_creates_parent_dir_if_needed` — ネスとしたパスへ書けること（任意）
13. タイムスタンプ実装: `std::time::SystemTime` + 手書きフォーマット（chrono 依存なし）

**integration tests (proxy_test.rs) — 5件**:
14. `session_allowlist_domain_gets_200` — session allowlist のドメインに CONNECT → 200
15. `blocked_domain_not_in_either_gets_403` — 両 allowlist にないドメイン → 403
16. `blocked_domain_written_to_log` — ブロック後にログファイルにドメインが書かれている
17. `blocked_domain_logged_once` — 2回 CONNECT → ログは1行
18. `no_blocked_log_when_not_specified` — `--blocked-log` なし → ログファイル作成されない

### 開発環境の注意

**この実装は ductus repo を dev-rust 環境でやること**（tutus は dev-sh）。
contextus-dev-rust の rules に従うこと（`ai-agent-rust.md` のモデル切り替えルールも含む）。

### tutus 側の shell スクリプトとの連携

tutus の `scripts/ductus-session.sh` は現在、ductus を以下で起動している:
```bash
sudo -u "$REAL_USER" ~/.local/bin/ductus --port 8080 --allowlist ~/.config/ductus/allowlist.txt &
echo $! > /tmp/ductus.pid  # manual pidfile
```

Rust 実装が完了したら `ductus-session.sh` の起動コマンドを以下に変更するだけでよい（shell 側の変更最小）:
```bash
sudo -u "$REAL_USER" ~/.local/bin/ductus \
    --port 8080 \
    --allowlist ~/.config/ductus/allowlist.txt \
    --session-allowlist "$SESSION_FILE" \
    --blocked-log "$BLOCKED_LOG" \
    --pidfile "$PID_FILE" &
```

---

## Next Session: Read First

- `.spec/TODO.md` — 現状確認
- tutus の実動作確認が済んだら allowlist 整備フェーズへ

## Key Decisions Made

- **GitHub Actions は後回し**: free 枠節約。tag を打てばいつでも動く
- **tutus 実動作確認は tutus の仕事**: ductus HANDOFF ではなく tutus HANDOFF に記録
- **他リポジトリへの依頼パターン**: そのリポジトリの HANDOFF に直接「〜からだよ」と追記してコミットする

## Blockers / Watch Out For

- **GitHub Actions の release workflow 未実行**: リポジトリが GitHub に push されていないため未動作
- **tutus での実動作未確認**: proxy を実際に起動して sandbox から通信させるテストがまだ（tutus 側に委譲済み）

## Changed Files (this session, after previous dumpmem)

**ductus**:
- `.spec/TODO.md`: 動作確認・contrib 完了タスクを [x]、GitHub リリースタスクを追加

**tutus**:
- `HANDOFF.md`: ductus 実動作確認タスクを追記

**contextus-dev-rust**:
- `rules/ai-agent-rust.md`: Rust コード編集時のモデル切り替えルールを追記
