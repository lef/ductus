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

## Notes

- Write for your next self: assume zero context from this session
- HANDOFF.md is short-term handoff; MEMORY.md (if present) is long-term knowledge
- Commit so it persists across machines
