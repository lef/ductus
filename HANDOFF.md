# HANDOFF — Session Transition Notes

**Last Updated**: 2026-03-18 (session 2)
**Previous Work**: SIGHUP fix + pidfile default + --bind + dot-domain allowlist

## Current State

### Completed (this session)

- **SIGHUP race condition 修正**: signal 登録を `tokio::spawn` の外に移動。旧コードでは登録前に SIGHUP が届くとプロセス終了 → sandbox 全断
- **signal `.expect()` 除去**: SIGHUP は match+warning、SIGTERM は `?` 伝播
- **pidfile デフォルト化**: `/tmp/ductus.pid` をデフォルトで作成。`--no-pidfile` で無効化、`--pidfile <path>` で上書き
- **`--bind` オプション**: デフォルト `127.0.0.1`（安全側）。`--bind 0.0.0.0` で全インターフェース Listen
- **`.example.com` dot-domain 記法**: root + 全サブドメイン（Squid/Nginx 慣習）。`*.example.com` は RFC 6125 準拠でサブドメインのみ
- **allowlist.txt にドキュメント追記**: 3種類の記法の説明
- **テスト追加**: 合計52テスト（unit 35 + integration 17）
- **リリースバイナリビルド済み**: `proxy/target/aarch64-unknown-linux-musl/release/ductus` (1.1MB)
- **L1 (contextus-claude) 同期**: hooks/rules/skills/agents/settings.json を最新に同期（前セッション）

### In Progress

- **ductus バイナリ更新**: リリースビルド完了済み。ホスト側で旧バイナリを差し替えて再起動が必要

### Not Started (priority order)

1. **gh CLI 導入**: allowlist に `.github.com` + `.githubusercontent.com` 追加済み（ホスト側）。バイナリ更新後にインストール可能
2. **git push**: ローカルに 5+ コミットが溜まっている。gh 導入後に push
3. **tutus `ductus-session.sh` 更新**: `--bind`, `--no-pidfile`, `--port 0` 対応。現在は ss ループでポートスキャン中
4. **`--blacklist`** — 永久ブロックリスト
5. **`--audit-log`** — 全 CONNECT リクエストを記録
6. **透過プロキシモード** — HTTP_PROXY 不要で全通信キャプチャ
7. **フェーズ1** — HTTPS インターセプト設計

## Next Session: Read First

- `.spec/TODO.md` — 現状確認
- `ductus --help` で全フラグ確認
- ホスト側でバイナリ更新が完了したか確認

## Next Session: Host-Side Actions Required

ductus バイナリ更新手順（ホスト側で実行）:
```bash
kill -TERM $(pgrep ductus)
cp ~/repos/ductus/proxy/target/aarch64-unknown-linux-musl/release/ductus ~/.local/bin/ductus
# sandbox 再起動で ductus-session.sh が新バイナリを自動起動
make claude-ductus P=~/repos/ductus REPOS=~/repos
```

注意:
- `--bind` デフォルトが `127.0.0.1` に変更。sandbox は `--net=host` なので問題なし
- `ductus-session.sh` は `--bind` を渡していないが、`127.0.0.1` で OK
- ただし `ductus-session.sh` が `--port 0` に未対応（まだ ss ループ）。将来更新予定

## Key Decisions Made

- **SIGHUP ハンドラは spawn の外**: `tokio::signal::unix::signal()` は即座にOSレベルのハンドラを登録する。spawn 内だと task poll 待ちになり race が生じる
- **pidfile デフォルト ON**: `pgrep` の誤爆で sandbox が死ぬのを防止
- **`--bind` デフォルト `127.0.0.1`**: 外部からプロキシとして使われるリスクを排除
- **`.example.com` 新記法**: `*.example.com` の RFC 6125 セマンティクスは変えない。Squid/Nginx 慣習の新記法で UX 改善
- **chrono 非依存の UTC フォーマット**: `SystemTime` + 手計算。外部 crate 追加不要
- **`BlockedLog` は `Arc<Mutex<>>`（tokio 版でなく）**: ファイル書き込みは sync、`.await` またがない
- **Opus 4.6 1M でモデル切り替え不要**: contextus-dev-rust のルールは「コスト最適化モード」として残した

## Rust 開発環境（2026-03-18 確認済み）

**sandbox 内で完全動作**:
- rustc 1.94.0 + aarch64-unknown-linux-musl ターゲット
- `source ~/.cargo/env` で PATH 設定
- ductus 自身がプロキシとして動作中（HTTP_PROXY=http://127.0.0.1:8080）
- `~/repos/` に contextus-dev-rust 等あり — 直接 contribution 可能

## Blockers / Watch Out For

- **ホスト側バイナリ更新が必要**: 新機能（--bind, dot-domain）は新バイナリでのみ動作
- **旧 ductus の SIGHUP は危険**: kill -HUP すると sandbox ごと死ぬ。kill -TERM は安全
- `Text file busy` エラー: ductus が起動中のまま `cp` すると失敗する。`kill -TERM` してから
- tutus の `ductus-session.sh` は `--port 0` 未対応（まだ ss ループでポートスキャン中）
- GitHub Actions release は未実行（push していない）

## Changed Files (this session)

- `proxy/src/main.rs`: SIGHUP 修正、pidfile デフォルト化、`--bind`/`--no-pidfile` 追加
- `proxy/src/lib.rs`: `dot_domains` フィールド、`dot_domain_match()`、doc comment 更新
- `proxy/tests/proxy_test.rs`: テスト8件追加（pidfile 2件、SIGHUP 1件、dot-domain 1件、bind 1件、etc.）
- `proxy/Cargo.toml`: libc dev-dependency 追加
- `proxy/allowlist.txt`: 記法ドキュメント追記
- `.spec/KNOWLEDGE.md`: SIGHUP race, wildcard サーベイ, 新決定事項
- `.spec/TODO.md`: フェーズ0.8 セクション追加

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
