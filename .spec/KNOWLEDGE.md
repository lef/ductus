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

## contextus エコシステムへの contrib（2026-03-13 後半）

### setup.sh --update 実装
- `setup.sh --update` で L0→L1→L2+ を一括同期できるようにした
- L2+ プロファイルは `.claude/.contextus/layers` に記録（1行1プロファイル）
- L2 ルールのインストール先を `rules/<profile>/` に名前空間化（L1 との衝突防止）
- **注意**: tutus の symlink 構造には未対応。`.contextus/.claude/` 経由で直接操作が必要

### TDD HARD RULE の責任分界点
- HARD RULE（実装前に失敗するテスト必須）は L1 `tdd-guide` agent に集約
- L2（dev-rust, dev-sh）は言語固有の「どうやって」のみ担当
- 重複排除: L2 から HARD RULE セクションを削除

### /dumpmem と /handoff の使い分け（ユーザー指摘）
- `/handoff`: セッション終了時の軽量版
- `/dumpmem`: コンテキストが大きくなったとき、大量の作業後の包括保存
- 今回のように多くの contrib を行ったあとは `/handoff` ではなく `/dumpmem`

## フェーズ0.6 完了（2026-03-13）

### 実装内容
- **static binary**: `[profile.release]` に lto/strip/panic=abort。Makefile で `uname -m` 自動検出 → `$(ARCH)-unknown-linux-musl`
- **GitHub Actions**: CI（fmt/clippy/test on push/PR）+ release（tag v* → MUSL binary をリリースに添付）
- **ワイルドカード allowlist**: `Allowlist { exact: HashSet, wildcards: Vec }` + `wildcard_match()`。外部 crate 不要、`*.`prefix のみ対応（YAGNI）
- **SIGHUP リロード**: `Arc<RwLock<Allowlist>>`。`std::sync::RwLock`（write lock を `.await` の前に必ずドロップ）。`reload_allowlist()` を公開関数として切り出しユニットテスト可能に
- **tutus 統合**: `env.allowlist` に HTTP_PROXY 系追加。`claude-sandbox.sh`/`aider-sandbox.sh` に proxy 稼働確認ブロック（警告のみ、hard fail なし）

### 技術的発見
- **ホストが aarch64**: WSL2 環境が ARM。x86_64-unknown-linux-musl はクロスコンパイルになりリンカエラー（`cc: error: unrecognized command-line option '-m64'`）。Makefile で `uname -m` 自動検出が正解
- **`std::sync::RwLock` vs `tokio::sync::RwLock`**: SIGHUP ハンドラの write は非同期 I/O なし → `std::sync::RwLock` で十分。read guard も `.await` 前に必ずブロックスコープでドロップ
- **`--clap version` サポートなし**: clap の derive マクロで `version` を明示しないと `--version` フラグが生成されない（`Usage: ductus [OPTIONS]` のみ）
- **`.git/objects/` に root 所有ディレクトリ混入**: PreCompact フックが root で `git add -A` を実行したことで発生。`sudo chown -R lef:lef .git/objects/` で修正
- **contextus-dev-rust の layers 記録漏れ**: `.claude/.contextus/layers` が存在せず `setup.sh --update` で L2 が再同期されなかった。今回追加

### 設計決定
| 決定 | 理由 | 日付 |
|---|---|---|
| `Allowlist` struct（HashSet を置き換え）| 型変更で wildcard/exact を統一的に扱える | 2026-03-13 |
| `std::sync::RwLock`（tokio 版でなく） | write lock を `.await` またぎなし。シンプル | 2026-03-13 |
| Makefile で `uname -m` 自動検出 | 環境に応じたターゲットを自動選択 | 2026-03-13 |
| tutus 統合は「警告のみ」 | proxy が別のアドレスの可能性もあり hard fail は過剰 | 2026-03-13 |

## 複数 sandbox 同時起動とポート競合（2026-03-16 tutus 側で発見）

### 問題

tutus で複数 sandbox を同時起動すると、全 sandbox が同じ ductus（port 8080）を共有する。
各 sandbox が別の allowlist/設定で動くべき場合、ductus もセッション毎に独立すべき。

### tutus 側の暫定対策（実装済み）

`ductus-session.sh` でシェル側ポートスキャン:
```bash
DUCTUS_PORT=8080
while ss -tlnH "sport = :${DUCTUS_PORT}" | grep -q LISTEN; do
    DUCTUS_PORT=$((DUCTUS_PORT + 1))
done
```

### ductus 側で実装すべきこと

1. `--port 0`: OS に空きポートを割り当てさせる（`TcpListener::bind("0.0.0.0:0")`）
2. bind 成功後、実際のポート番号を stdout に出力（`listener.local_addr().port()`）
3. graceful shutdown: SIGTERM で clean に終了（現状 `kill` で止めてる）

理想:
```bash
DUCTUS_PORT=$(ductus --port 0 --allowlist ... --daemon)
export HTTP_PROXY="http://127.0.0.1:${DUCTUS_PORT}"
# ... sandbox 実行 ...
kill $(cat /tmp/ductus.pid)  # graceful shutdown
```

## Rejected Approaches

- `--permission-mode auto` on sandbox: サーバー依存・root 制限あり → `bypassPermissions` + non-root が正解（tutus 側で解決済み）
- lib.rs 分離なしで統合テスト: 統合テストから `ductus::run()` を呼ぶには lib ターゲットが必要。`main.rs` のみでは integration test からのアクセスが不可能
