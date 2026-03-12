# KNOWLEDGE — Accumulated Discoveries

> Written by the agent. Accumulates across sessions.
> Record decisions, rationale, and non-obvious findings here.
> Do not write session-specific state — use HANDOFF.md for that.

## Decisions

| Decision | Rationale | Date |
|---|---|---|
| tokio + clap + serde + toml | SPEC.md 確定済み。軽量で十分 | 2026-03-10 |
| 完全一致のみ（ワイルドカードなし v1） | YAGNI。育てる設計なので後から追加 | 2026-03-10 |
| anyhow（thiserror なし） | バイナリ専用。ライブラリエラー型は不要 | 2026-03-13 |
| lib.rs + main.rs 分離 | 統合テストから `ductus::run()` を呼ぶために必要。main.rs は薄い CLI ラッパーに | 2026-03-13 |
| 403 ボディを key-value 形式に | `BLOCKED: <host>` / `ALLOWLIST: <path>` の2行。echo コマンド埋め込みはコマンド注入に見えるため廃止 | 2026-03-13 |
| contextus-dev-rust rules を `rules/rust/` に配置 | L1 の汎用ルールと分離するためサブディレクトリに | 2026-03-13 |

## Technical Findings

- `TcpStream::split()` で ReadHalf/WriteHalf に分割後、NLL により最終使用後にborrowが解放される。`copy_bidirectional(&mut stream)` は両 half の最終使用後に呼べる
- `build-essential`（cc リンカー）がホストにないと `cargo build` 失敗。WSL2 初期状態では未インストールの場合あり
- Cargo は `src/lib.rs` と `src/main.rs` が両方あれば自動的に lib + bin ターゲットとして認識する。Cargo.toml への明示的な `[lib]` セクション追加は不要
- `cargo test` は lib のユニットテスト → bin のユニットテスト → `tests/` の統合テスト → doc-tests の順で実行される

## TDD フェーズ0.5 完了（2026-03-13）

**フェーズ0.5 で TDD 負債を解消した。**

実施した TDD フロー（t_wada 方式）:
1. Compile RED: `parse_connect_target` 単体テスト4件 + 統合テスト4件を書く → `cargo test --no-run` で失敗を確認
2. GREEN: `parse_connect_target`/`run()`/`handle_inner()` を実装 → 全13テスト パス
3. REFACTOR: `cargo fmt` + `cargo clippy -- -D warnings` クリーン

**結果**: 単体テスト9件 + 統合テスト4件 = 13件。エラーハンドリングも改善（400/502 レスポンス追加、accept loop の panic 排除）。

## contextus エコシステムの構造的課題（2026-03-13 発見）

### SessionStart フックが HANDOFF.md しか inject しなかった

**問題**: `.spec/TODO.md`、`PLAN.md`、`KNOWLEDGE.md` がセッション開始時に自動で読み込まれず、エージェントが読み忘れる。
**修正**: SessionStart フックで `.spec/` の3ファイルも inject するようにした（contextus-claude に contrib 済み）。

### tutus の symlink 構造と git の非互換

**問題**: tutus は `.claude -> .contextus/.claude` のシンボリックリンクを使用。git は symlink 越しのファイル追加を拒否する（`fatal: pathspec '...' is beyond a symbolic link`）。
**回避策**: `git add .contextus/.claude/...` と実パス経由で操作する必要がある。
**根本課題**: setup.sh は `.claude/` を実ディレクトリとして想定しており、tutus の `.contextus/` パターンに対応していない。upstream 同期も手動 cp のまま。

### L2 ルールのインストール先が未定義

**問題**: contextus-dev-rust の rules は `rules/*.md`（フラット）だが、インストール先は `rules/rust/*.md`（サブディレクトリ）。setup.sh の L2 インストールロジック（line 114: `cp -r "$TMP_L2/rules/." "$CLAUDE_DIR/rules/"`)はフラットにコピーするため、L1 の同名ファイル（例: `rust-style.md`）と衝突する。
**対策案**: L2 側で `rules/<profile>/` サブディレクトリ構造にするか、setup.sh に名前空間の概念を追加する。

## 参照すべきリソース（2026-03-12 記録）

- **cloudnative-co/claude-code-starter-kit**: Claude Code 開発環境の事実上の標準。TDD・エージェント構成・safety hook の参考に。Rust 固有の内容はないが workflow は参考になる
- **contextus-dev-rust/rules/**: Rust ベストプラクティス（linting, testing, error-handling 等）。実装前に必ず参照する

## Rejected Approaches

- `--permission-mode auto` on sandbox: サーバー依存・root 制限あり → `bypassPermissions` + non-root が正解（tutus 側で解決済み）
- lib.rs 分離なしで統合テスト: 統合テストから `ductus::run()` を呼ぶには lib ターゲットが必要。`main.rs` のみでは integration test からのアクセスが不可能
