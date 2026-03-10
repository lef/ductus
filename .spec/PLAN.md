# PLAN — HTTP CONNECT Proxy

## What I Want

Rust で HTTP CONNECT プロキシを実装する。約50行。

## Constraints

- domain allowlist で許可ドメインのみ通す
- 実装は ~50行 Rust に収める

## Open Questions

- allowlist の設定方法（CLI引数？設定ファイル？環境変数？）
- listenポートの設定方法
- allowlist にマッチしない場合のレスポンス（407? 403?）
