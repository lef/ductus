# Git Workflow

## Commit Message Format

```
<type>: <description>
```

type: feat, fix, refactor, docs, test, chore, security

Examples:
- `feat: add domain allowlist to proxy`
- `fix: quote variable in cleanup function`
- `security: restrict bind mount to specific paths`
- `docs: update spec checklist`

## Implementation Workflow

1. Write SPEC / TODO first (see sdd.md)
2. Write tests before implementation (TDD)
3. Implement (minimal change to pass tests)
4. Review and commit

## Branch Strategy

- Main branch: stable
- Small changes: commit directly to main
- Large changes: use feature/* branch

## Pre-commit Checklist # コミット前チェックリスト

Correctness / 正しさ:
- Tests pass / テストが通る
- Acceptance criteria met (see TODO.md) / 受け入れ基準を満たす

Consistency / 一貫性:
- Consistent with SPEC.md and CONSTITUTION.md / SPEC・CONSTITUTION と矛盾しない
- No unrelated changes included / 無関係な変更が混ざっていない

Safety / 安全性:
- No secrets in diff / 差分に secrets がない
- No security regressions / セキュリティの後退がない

Quality / 品質:
- Linter / formatter clean (if applicable) / リンター・フォーマッターが通る
- KNOWLEDGE.md updated if discoveries were made / 発見があれば KNOWLEDGE.md を更新

Projects may extend this checklist in their own CONSTITUTION or AGENTS.md.
プロジェクト独自のチェック項目は CONSTITUTION や AGENTS.md に追加できる。
