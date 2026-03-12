# PLAN — HTTP CONNECT Proxy

## なぜ作るのか（本来のビジョン）

**自分の全ブラウザ通信を記録するパーソナルアーカイブプロキシ**が最終目標。

- Bookmark が面倒で、見たページを忘れる問題を解決したい
- URL だけでなく、コンテンツごと残したい（AI があるので全文検索・要約が現実的になった）
- HTTPS インターセプト（自前 CA + MITM）が技術的チャレンジ

## フェーズ0（現在）— sandbox 用途

tutus sandbox 内の AI エージェントが外部に出る通信を制御するプロキシとして先に実装する。
「将来拡張すればいいから、とりあえず今の目的のために作ろう」という判断（2026-03）。

Rust で HTTP CONNECT プロキシを実装する。約50行。

## Constraints（フェーズ0）

- domain allowlist で許可ドメインのみ通す
- 実装は ~50行 Rust に収める

## Open Questions（フェーズ0時点で解決済み）

- allowlist の設定方法 → TOML + CLI引数（優先度: CLI > config > デフォルト）
- listenポートの設定方法 → 同上
- allowlist にマッチしない場合のレスポンス → 403（操作ガイド付き）
