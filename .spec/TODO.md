# TODO — ductus HTTP CONNECT proxy

## フェーズ0: sandbox 用途（完了）

- [x] proxy/Cargo.toml 作成（tokio, clap, serde, toml 依存）
- [x] proxy/src/main.rs 実装（69行）
- [x] proxy/config.toml サンプル作成
- [x] proxy/allowlist.txt サンプル作成
- [x] cargo build で動作確認
- [x] 手動テスト: curl --proxy でブロック・通過の両方を確認 ✅
- [x] load_allowlist のユニットテストを追加

## フェーズ0.6: フェーズ0 改良（tutus 向け）（完了）

> フェーズ1は当面先。まずフェーズ0を tutus での実運用に耐えるレベルに改良する。（2026-03-13）

- [x] static binary ビルド（Cargo.toml release profile + Makefile で uname -m 自動検出）
- [x] GitHub Actions（CI: fmt/clippy/test、release: tag v* → MUSL binary）
- [x] ワイルドカード allowlist（`*.github.com` 形式、外部 crate 不要）
- [x] SIGHUP リロード（`Arc<RwLock<Allowlist>>`、kill -HUP で再起動不要）
- [x] tutus 統合（env.allowlist に HTTP_PROXY 系、sandbox script に proxy 確認）
- [x] aarch64 MUSL binary ビルド確認・`~/.local/bin/ductus` に配置
- [x] contextus-dev-rust を layers manifest に記録（`setup.sh --update` 対応）
- [x] ホスト上での動作確認（curl で allow/block 両方確認）
- [x] tutus HANDOFF に実動作確認タスクを投げ込み（tutus 側の仕事として記録）
- [x] contextus-dev-rust に Rust コード編集時のモデル切り替えルールを追記

## フェーズ0.7: ポート自動割り当て + graceful shutdown（完了）

> tutus で複数 sandbox 同時起動時のポート競合を解消する。（2026-03-18）

- [x] `--port 0` で OS に空きポート割り当て + stdout に実ポート出力
- [x] graceful shutdown: SIGTERM で accept loop 停止 + pidfile クリーンアップ
- [x] `run()` に `shutdown: impl Future<Output = ()>` 引数追加
- [x] テスト3件追加（shutdown 2件 + port 0 1件）— 合計42テスト
- [x] contextus-dev-rust 同期（Model Switching ルール更新: Opus 4.6 1M で不要に）
- [x] cargo fmt + clippy クリーン

## GitHub リリース（低優先度）

- [ ] `git tag v0.1.0 && git push && git push --tags` で GitHub Actions release 初起動（急がない、free 枠節約）

## フェーズ1: パーソナルウェブアーカイブ（将来）

**ビジョン**: 自分の全ブラウザ通信を記録するプロキシ。詳細は `.spec/PLAN.md` 参照。

### 設計・調査
- [ ] HTTPS インターセプト方式の調査（自前 CA + MITM）
- [ ] 自前 CA 証明書の生成と各ブラウザへのインストール方法を確認
- [ ] SSL termination ライブラリの選定（rustls? openssl?)
- [ ] フェーズ1の SPEC.md 作成（SDD ワークフロー）

### 記録機能
- [ ] リクエスト/レスポンスのログ形式を設計（WARC? SQLite? plain files?）
- [ ] ストレージ戦略（全コンテンツ保存 vs URL + メタデータのみ）
- [ ] AI 検索・要約インターフェースの設計

### 実装
- [ ] HTTPS インターセプト実装
- [ ] ログ記録機能実装
- [ ] allowlist → blocklist モデルへの転換（パーソナル用途では全通過がデフォルト）

## フェーズ0.5: コード品質改善

> ⚠️ テストを書いてから実装（t_wada TDD）。**実装開始前にモデルを切り替えること。**

### 文書・設定（完了）
- [x] SPEC.md にフェーズ0.5セクション追記
- [x] TODO.md にフェーズ0.5チェックリスト追記
- [x] contextus (L0/L1) + contextus-dev-rust (L2) を ductus に導入

### RED フェーズ（Compile RED 確認済み ✅）
- [x] `parse_connect_target` 単体テスト4件を書く（Compile RED）
- [x] `proxy/tests/proxy_test.rs` に統合テスト4件を書く（`run()` 未存在で RED）
- [x] `cargo test --no-run` で Compile RED を確認

### GREEN フェーズ（完了 ✅）
- [x] `anyhow = "1"` を `proxy/Cargo.toml` に追加
- [x] `parse_connect_target` を抽出 → 単体テスト GREEN
- [x] `run()` を抽出 → 統合テストが compile する状態に
- [x] `handle_inner()` で 400/502 を実装 → 統合テスト GREEN

### REFACTOR フェーズ（完了 ✅）
- [x] `main()` を薄くする（bind + run の呼び出しのみ）
- [x] accept ループエラーハンドリング（panic → log + continue）
- [x] doc comments 追加（`load_allowlist`, `parse_connect_target`, `run`）
- [x] `cargo fmt` + `cargo clippy -- -D warnings` をパス

### コミット計画
```
test: add unit tests for parse_connect_target
test: add integration tests for 200/403/400/502 paths
refactor: extract parse_connect_target and run()
fix: return 400 on non-CONNECT, 502 on target connect failure
fix: replace unwrap() with anyhow error handling
docs: add doc comments to all public items
chore: add anyhow dependency
```

## contextus エコシステム改善（発見したもの）

- [x] SessionStart フックで .spec/ ファイルを inject（contextus-claude contrib 済み）
- [x] setup.sh --update 実装（L0/L1/L2+ 一括同期、contextus-claude contrib 済み）
- [x] L2 ルール名前空間修正（rules/<profile>/、contextus-claude contrib 済み）
- [x] TDD HARD RULE を L1 に集約、L2 の重複削除（3 repos contrib 済み）
- [x] contextus (L0) に層間互換性課題を記録
- [x] setup.sh の bats テスト追加（HARD RULE 適用: 実装したが tests なし）← tests/test-setup.sh 追加済み（2026-03-13）
- [x] tutus の symlink 構造を実ディレクトリに移行（2026-03-13）

## 参照

- `.spec/PLAN.md`: 本来のビジョン（パーソナルウェブアーカイブ）
- `.spec/SPEC.md`: フェーズ0 + フェーズ0.5の確定仕様
