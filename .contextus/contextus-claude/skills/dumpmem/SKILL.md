---
name: dumpmem
description: Comprehensive context dump to persistent storage before memory is lost. Use when the user says "フル保存", "コンテキスト保存", "保存して", "全部保存", or "dumpmem".
allowed-tools: Read, Write, Edit, Bash, Grep, Glob
---

Dump all session context to persistent storage. AI-specific operation: we forget, so we write.

## Steps

### 1. git add -A && commit（作業ファイルをまず保存）

```bash
cd "$CLAUDE_PROJECT_DIR"
git add -A
git diff --cached --quiet || git commit -m "checkpoint: dumpmem auto-commit"
```

REPOS 内の dirty repos もコミット（sandbox で REPOS= 指定時のみ）:

```bash
if [ -n "${SANDBOX_REPOS_DIR:-}" ] && [ -d "$SANDBOX_REPOS_DIR" ]; then
    for repo in "$SANDBOX_REPOS_DIR"/*/; do
        [ -d "$repo/.git" ] || continue
        cd "$repo"
        # まず何が変更されているか確認
        git status --short
    done
fi
```

各 dirty repo について:
- `git diff --stat` で変更内容をレビュー
- `.claude/settings.local.json` 等のセンシティブなファイルは commit しない
- 変更内容に応じた意味のあるコミットメッセージで commit
- 不要な変更（ビルド成果物等）は `git checkout` で戻す

### 2. KNOWLEDGE.md 更新

今セッションで発見した技術的事実・設計判断を `.spec/KNOWLEDGE.md` に追記する。
- 新しく分かったこと（ライブラリの挙動、制約、etc.）
- 却下したアプローチとその理由
- 非自明な決定の根拠

**追記する前に、同じトピックの既存エントリを確認し、矛盾・古い記述があれば修正する。**

書かない: セッション固有の一時状態、推測・未検証の情報。

### 3. TODO.md 更新

`.spec/TODO.md` を更新する:
- 今セッションで完了したタスクに `[x]` をつける
- 今セッションで**発見した新しい将来タスク**を追記する（重要）

### 4. memory/ 更新（必要があれば）

`~/.claude/projects/$(pwd | sed 's|/|-|g')/memory/` に記録すべきものがあれば書く:
- **user**: ユーザーの役割・好み・知識レベルに関する発見
- **feedback**: 行動修正につながる指摘（「〜しないで」等）
- **project**: プロジェクトの状況・目標・制約の変化
- **reference**: 外部システムへのポインタ

### 5. HANDOFF.md 更新 + commit

`/handoff` スキルと同じ手順で HANDOFF.md を更新してコミットする。

### 6. trace.log 確認（任意）

今日のトレースログを確認して、想定外のツール操作がないか確認する:

```bash
cat ~/.claude/sessions/$(date '+%Y-%m-%d')-trace.log 2>/dev/null | tail -20
```

異常があればユーザーに報告する。

### 7. 全 repos 同期

`/sync-repos --all` の手順を実行する。
全リポジトリが ahead=0 dirty=0 であることを確認して報告。

## Notes

- コンテキストが大きくなってきたと感じたら自発的に実行してよい
- 肥大化対策は別途検討（TODO に記録済み）
- `/handoff` はセッション終了時の軽量版として独立して存在する
