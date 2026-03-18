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

### フェーズ0の核心: AI によるフィードバックループ

**読み手は人間ではなく AI エージェント（tutus 内で動く Claude 等）。**

ブロック時のフロー:
1. AI がリクエストを送る → proxy が 403 を返す
2. **AI が 403 レスポンスを読み取り**、何がブロックされたかを把握する
3. AI がそのドメインへのアクセスが必要かどうかを判断する
4. 必要であれば `allowlist.txt` に追記して proxy を再設定
5. リトライ

このループにより、sandbox 環境で許可ドメインを AI 自身が育てていく。
403 レスポンスは AI が解釈できる構造化された内容でなければならない。

## フェーズ進行

| Phase | 内容 | 状態 |
|---|---|---|
| 0 | 基本 CONNECT proxy（~50行） | 完了 |
| 0.5 | TDD + エラーハンドリング改善 | 完了 |
| 0.6 | tutus 実運用（session-allowlist, wildcard, SIGHUP, static binary） | 完了 |
| 0.7 | `--port 0` auto-assign + graceful shutdown (SIGTERM) | 完了 |
| — | `--blacklist`, `--audit-log` | 未着手 |
| 1 | HTTPS インターセプト（パーソナルアーカイブ） | 将来 |

## Constraints（フェーズ0）

- domain allowlist で許可ドメインのみ通す
- 実装は ~50行 Rust に収める（現在は lib.rs ~640行 + main.rs ~100行に成長）

## Open Questions（フェーズ0時点で解決済み）

- allowlist の設定方法 → TOML + CLI引数（優先度: CLI > config > デフォルト）
- listenポートの設定方法 → 同上
- allowlist にマッチしない場合のレスポンス → 403（操作ガイド付き）
