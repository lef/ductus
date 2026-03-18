# SPEC — HTTP CONNECT Proxy (Rust)

## 概要

ドメイン allowlist を持つ HTTP CONNECT プロキシ。
設定ファイル + CLI引数で動作し、git 管理できる構成を持つ。

## ファイル構成

```
proxy/
├── Cargo.toml
├── src/
│   └── main.rs          # ~50行目標
├── config.toml          # 設定ファイル（git管理対象）
└── allowlist.txt        # 許可ドメイン一覧（git管理対象、育てる）
```

## 設定ファイル (config.toml)

```toml
port = 8080
allowlist = "allowlist.txt"
```

## CLI引数（config.toml の値を上書き）

```
proxy [--config <path>] [--port <port>] [--allowlist <path>]
```

- `--config`: 設定ファイルパス（デフォルト: `config.toml`）
- `--port`: リスニングポート（config.toml を上書き）
- `--allowlist`: allowlist ファイルパス（config.toml を上書き）

優先順位: CLI引数 > config.toml > デフォルト値

## allowlist.txt フォーマット

```
# コメント行は # で始まる
example.com
api.github.com
*.example.org   # ワイルドカード（スコープ外: 最初は完全一致のみ）
```

最初のバージョンは完全一致のみ。ワイルドカードは将来対応。

## プロキシ動作

1. TCP 接続を受け付ける
2. `CONNECT host:port HTTP/1.1` を読む
3. `host` が allowlist にあれば → ターゲットに TCP 接続、`200 Connection established` を返してトンネル
4. なければ → 403 を返す

## 403 レスポンスのボディ

```
BLOCKED: <host>
ALLOWLIST: <allowlist_path>
```

- `BLOCKED:` — ブロックされたドメイン名（ホスト部分のみ）
- `ALLOWLIST:` — allowlist ファイルの絶対パス

**読み手は AI エージェント。** tutus 内の AI が 403 を受け取り、ブロックされたドメインを把握し、
必要と判断すれば `<allowlist_path>` にドメインを追記する。
コマンドは含まない — パース・判断・操作は読み手の責任。

## 依存クレート

- `tokio` (async runtime, features: full)
- `clap` (CLI引数)
- `serde` + `toml` (設定ファイル)

## 行数見積もり

| 箇所 | 行数 |
|---|---|
| CLI引数 (clap) | ~8 |
| config.toml 読み込み | ~10 |
| allowlist 読み込み | ~5 |
| TCP accept ループ + CONNECT パース | ~15 |
| allowlist チェック + 403/200 分岐 | ~10 |
| トンネル (tokio::io::copy) | ~5 |
| **合計** | **~53行** |

50行はタイトだが、clap の derive マクロを使えば CLI部分を圧縮できる。
許容範囲として ~50–60行 とする。

## スコープ外（将来対応）

- ~~ワイルドカード / glob マッチ~~ → フェーズ0.6 で実装済み（`*.example.com` 形式）
- ~~allowlist のホットリロード（SIGHUP等）~~ → フェーズ0.6 で実装済み
- HTTPS 証明書検証（フェーズ1）
- 認証（Proxy-Authorization）

---

## フェーズ0.5: コード品質改善（TDD + エラーハンドリング）

**目的**: 外部から観察できる振る舞いを変えずに、コードを TDD 水準に引き上げる。

### 抽出する関数

- `parse_connect_target(line: &str) -> Option<String>`: CONNECT 行パーサー（純粋関数、テスタブル）
- `run(listener, allowlist, allowlist_path)`: accept ループ（統合テスト用に main から切り出す）

### エラーハンドリング仕様

| 状況 | 現在の動作 | 新しい動作 |
|---|---|---|
| bind 失敗 | panic | `anyhow::Error` で終了（ログ付き） |
| accept エラー | panic | ログして継続（トランジェントエラー対応） |
| CONNECT 以外のメソッド | サイレントドロップ | `HTTP/1.1 400 Bad Request` を返す |
| ターゲット接続失敗 | サイレントドロップ | `HTTP/1.1 502 Bad Gateway` を返す |

### テスト追加

**単体テスト** (`proxy/src/main.rs` の `#[cfg(test)]` に追加):
- `parse_connect_target`: 正常 / 非CONNECT / 空文字 / 不完全 の4ケース

**統合テスト** (`proxy/tests/proxy_test.rs` — 新規ファイル):
- port-0 bind で実 TCP 接続（`run()` を tokio::task で起動）
- allowed domain → `200 Connection established`
- blocked domain → `403 Forbidden`
- CONNECT 以外のメソッド → `400 Bad Request`
- 到達不能ターゲット → `502 Bad Gateway`

### 依存追加

- `anyhow = "1"` → `[dependencies]`（バイナリ用エラー処理、`?` で伝播）

### コメント

- すべての `pub(crate)` アイテムに `///` doc comment
- 非自明なロジックに `//`（CONNECT ヘッダードレイン、domain 抽出の `:` 分割）

---

## フェーズ0.6: tutus 実運用改良

**目的**: tutus sandbox での実運用に耐えるレベルに引き上げる。

### 追加 CLI フラグ

| フラグ | 用途 |
|---|---|
| `--session-allowlist <path>` | 第2の allowlist（セッション固有、SIGHUP でリロード） |
| `--blocked-log <path>` | ブロックドメインをタイムスタンプ付きでログ |
| `--pidfile <path>` | 起動時に PID をファイルに書く |

### 追加機能

- **ワイルドカード allowlist**: `*.github.com` 形式。`*.`prefix のみ（外部 crate 不要）
- **SIGHUP リロード**: `Arc<RwLock<Allowlist>>`。permanent + session を同時リロード
- **static binary**: MUSL ターゲット、`[profile.release]` で lto/strip/panic=abort
- **GitHub Actions**: CI（fmt/clippy/test）+ release（tag v* → MUSL binary）

---

## フェーズ0.7: ポート自動割り当て + graceful shutdown

**目的**: 複数 sandbox 同時起動時のポート競合を解消し、clean な停止を実現する。

### `--port 0`

- OS に空きポートを割り当てさせる（`TcpListener::bind("0.0.0.0:0")`）
- bind 成功後、実際のポート番号を **stdout** に出力（`println!("{actual_port}")`）
- `--port 0` の場合のみ stdout に出力（既存スクリプト互換性維持）
- stderr のログ（`:: ductus listening on :XXXX`）は常に actual_port を使用

**使用例**:
```bash
DUCTUS_PORT=$(ductus --port 0 --allowlist /tmp/allow.txt --pidfile /tmp/ductus.pid &)
export HTTP_PROXY="http://127.0.0.1:${DUCTUS_PORT}"
```

### graceful shutdown

- `run()` に `shutdown: impl Future<Output = ()>` 引数を追加
- accept loop を `tokio::select!` に変更:
  - `listener.accept()` と `shutdown` を同時待ち
  - shutdown が resolve → `break`（accept loop 停止）
- `main.rs` で SIGTERM ハンドラを作成し `run()` に渡す
- `run()` 戻り後に pidfile を自動削除
- in-flight 接続は drain しない（tokio runtime 終了時に drop）

### テスト

| テスト名 | 内容 |
|---|---|
| `run_returns_on_shutdown` | oneshot で shutdown signal → `run()` が返る |
| `proxy_serves_then_shuts_down` | リクエスト処理後に shutdown → 正常終了 |
| `port_zero_prints_actual_port` | バイナリ起動 → stdout からポート読み取り |
