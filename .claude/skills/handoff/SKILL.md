---
name: handoff
description: Update HANDOFF.md and commit before ending a session. Use when the user says "done", "see you later", or "handoff", or before context compaction.
allowed-tools: Read, Write, Bash
---

Update HANDOFF.md with this session's work and commit it to git.

## Steps

1. Review this session's work from the conversation history
2. Overwrite HANDOFF.md with this structure:

```markdown
# HANDOFF — Session Transition Notes

**Last Updated**: YYYY-MM-DD HH:MM
**Previous Work**: [One-line summary of this session]

## Current State

### Completed (this session)
- [Finished tasks]

### In Progress
- [Started but not finished]

### Not Started (priority order)
1. [Next task]
2. [After that]

## Next Session: Read First

- [Files or sections to check]

## Key Decisions Made

- [Important choices and rationale]

## Blockers / Watch Out For

- [Anything that slowed things down]

## Changed Files

- `path/to/file`: [what changed]
```

3. Commit:
```bash
cd "$CLAUDE_PROJECT_DIR"
git add HANDOFF.md
git diff --cached --quiet || git commit -m "docs: update HANDOFF.md"
```

4. REPOS 内の dirty repos もコミット（sandbox で REPOS= 指定時のみ）:
```bash
if [ -n "${SANDBOX_REPOS_DIR:-}" ] && [ -d "$SANDBOX_REPOS_DIR" ]; then
    for repo in "$SANDBOX_REPOS_DIR"/*/; do
        [ -d "$repo/.git" ] || continue
        cd "$repo"
        git status --short
    done
fi
```
各 dirty repo について変更内容をレビューし、意味のあるメッセージで commit。
`.claude/settings.local.json` 等のセンシティブなファイルは commit しない。

5. 全 repos 同期: `/sync-repos --all` の手順を実行する。

## Notes

- Write for your next self: assume zero context from this session
- HANDOFF.md is short-term handoff; MEMORY.md (if present) is long-term knowledge
- Commit so it persists across machines
