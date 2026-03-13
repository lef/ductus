# TODO — ductus HTTP CONNECT proxy

## フェーズ0: sandbox 用途（完了）

- [x] proxy/Cargo.toml 作成（tokio, clap, serde, toml 依存）
- [x] proxy/src/main.rs 実装（69行）
- [x] proxy/config.toml サンプル作成
- [x] proxy/allowlist.txt サンプル作成
- [x] cargo build で動作確認
- [x] 手動テスト: curl --proxy でブロック・通過の両方を確認 ✅
- [x] load_allowlist のユニットテストを追加

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
- [ ] tutus の symlink 構造を実ディレクトリに移行

## 参照

- `.spec/PLAN.md`: 本来のビジョン（パーソナルウェブアーカイブ）
- `.spec/SPEC.md`: フェーズ0 + フェーズ0.5の確定仕様
