# HANDOFF — Session Transition Notes

**Last Updated**: 2026-03-13
**Previous Work**: フェーズ0.6完了 → ホスト動作確認 → tutus/contextus-dev-rust contrib

## Current State

### Completed (this session)

**フェーズ0.6 完了（前回 dumpmem まで）**:
- static binary / GitHub Actions / wildcard / SIGHUP / tutus 統合
- 全 21 テスト GREEN（単体 17 + 統合 4）

**動作確認・後始末（今回）**:
- ホスト上での curl テスト: `api.github.com` → 200, `evil.com` → 403 ✅
- GitHub Actions push は意図的に後回し（free 枠節約）
- tutus の HANDOFF に実動作確認タスクを投げ込み（tutus 側の仕事として委譲）
- contextus-dev-rust `rules/ai-agent-rust.md` に Rust コード編集時のモデル切り替えルールを追記

### Not Started (priority order)

1. **tutus での実動作確認** — tutus HANDOFF に記録済み。tutus セッションで拾われる
2. **GitHub リリース**（低優先度）— `git tag v0.1.0 && git push && git push --tags`。急がない
3. **フェーズ1** — HTTPS インターセプト設計。当面先

## Next Session: Read First

- `.spec/TODO.md` — 現状確認
- tutus の実動作確認が済んだら allowlist 整備フェーズへ

## Key Decisions Made

- **GitHub Actions は後回し**: free 枠節約。tag を打てばいつでも動く
- **tutus 実動作確認は tutus の仕事**: ductus HANDOFF ではなく tutus HANDOFF に記録
- **他リポジトリへの依頼パターン**: そのリポジトリの HANDOFF に直接「〜からだよ」と追記してコミットする

## Blockers / Watch Out For

- **GitHub Actions の release workflow 未実行**: リポジトリが GitHub に push されていないため未動作
- **tutus での実動作未確認**: proxy を実際に起動して sandbox から通信させるテストがまだ（tutus 側に委譲済み）

## Changed Files (this session, after previous dumpmem)

**ductus**:
- `.spec/TODO.md`: 動作確認・contrib 完了タスクを [x]、GitHub リリースタスクを追加

**tutus**:
- `HANDOFF.md`: ductus 実動作確認タスクを追記

**contextus-dev-rust**:
- `rules/ai-agent-rust.md`: Rust コード編集時のモデル切り替えルールを追記
