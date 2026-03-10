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
BLOCKED: <host> is not in the allowlist.
To allow this domain, add it to <allowlist_path>:
  echo "<host>" >> <allowlist_path>
```

ユーザーが何をブロックされたかを把握し、allowlist を育てられるようにする。

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

- ワイルドカード / glob マッチ
- allowlist のホットリロード（SIGHUP等）
- HTTPS 証明書検証
- 認証（Proxy-Authorization）
