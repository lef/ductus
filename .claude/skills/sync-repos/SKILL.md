---
name: sync-repos
description: Check and push all git repositories. Use at session end, after dumpmem/handoff, or when the user says "sync", "push all", "全repo同期".
allowed-tools: Bash
---

Ensure all git repositories are committed and pushed to remote.

## Modes

- **Default (--all)**: All repos under `~/repos/*/`
- **--project**: Only `$CLAUDE_PROJECT_DIR` and `$SANDBOX_REPOS_DIR` (if set)

## Steps

### 1. Check all repos

```bash
for repo in ~/repos/*/; do
    [ -d "$repo/.git" ] || continue
    cd "$repo"
    name="$(basename "$repo")"
    branch="$(git rev-parse --abbrev-ref HEAD 2>/dev/null)"
    ahead="$(git log --oneline "origin/${branch}..HEAD" 2>/dev/null | wc -l)"
    dirty="$(git status --short 2>/dev/null | wc -l)"
    echo "$name: ahead=$ahead dirty=$dirty"
done
```

### 2. Handle dirty repos

For each repo with dirty > 0:
- Run `git status --short` and `git diff --stat` to review changes
- Commit with a meaningful message (NOT `git add -A` — add specific files)
- Skip `.claude/settings.local.json` and other sensitive files

### 3. Push ahead repos

For each repo with ahead > 0:
- `git push`

### 4. Final verification

Re-run the check from step 1. All repos must show `ahead=0 dirty=0`.
Report the final status to the user.

## Notes

- This skill is called automatically by `/dumpmem` and `/handoff`
- Do NOT push without reviewing what's being pushed
- If a push fails (e.g., diverged branch), report to the user — do not force push
