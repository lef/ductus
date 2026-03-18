---
name: handover
description: Structured session handover — generate HANDOFF.md with complete session state for seamless continuation. Equivalent to starter-kit /handover. Use at session end or before context compaction.
allowed-tools: Read, Write, Edit, Bash, Grep, Glob
---

Generate a complete structured handover so the next session (or person) can continue exactly where you left off.
Compatible with cloudnative-co/claude-code-starter-kit `/handover` convention.

## Steps

### 1. Commit current work

```bash
cd "$CLAUDE_PROJECT_DIR"
git add -A
git diff --cached --quiet || git commit -m "checkpoint: handover auto-commit"
```

### 2. Update KNOWLEDGE.md

Append any technical discoveries or design decisions made this session to `.spec/KNOWLEDGE.md`.

### 3. Update TODO.md

Mark completed tasks `[x]`, add any newly discovered tasks.

### 4. Generate HANDOFF.md

Update `HANDOFF.md` in the project root with this structure:

```markdown
# Session Handover
## Generated: [ISO 8601 timestamp]

## Current State
- **Branch**: [current git branch]
- **Last Commit**: [hash + message]
- **Uncommitted Changes**: [list or "none"]

## What Was Done
[Numbered list of completed actions this session]

## What Remains
[Checklist in priority order]
- [ ] Next task (from TODO.md)

## Key Decisions Made
[Important decisions and rationale — brief]

## Known Issues / Blockers
[Anything discovered but not resolved]

## Context Files
[Files the next session should read first]
- .spec/TODO.md
- .spec/KNOWLEDGE.md
- [other relevant files]

## Recommended Next Steps
1. `make claude-ductus P=<project> REPOS=~/repos ARGS=--continue`
2. Read this handover: "Read HANDOFF.md and continue"
3. [Specific first action]
```

### 5. Update auto memory if needed

`~/.claude/projects/$(pwd | sed 's|/|-|g')/memory/` — write any user/feedback/project memories.

### 6. Final commit + session rename

```bash
git add HANDOFF.md .spec/
git commit -m "docs: handover — $(date '+%Y-%m-%d %H:%M')"
```

Then suggest session naming for easy resume:

```
/rename [feature-name: current-status]
```

Examples: `[sandbox-promote: implementing]`, `[trace-log: done]`, `[sprint-3: day-2]`

This enables targeted resume: `claude --resume`

## Notes

- `/handover` = structured format (this skill)
- `/handoff` = lightweight version (HANDOFF.md update + commit only)
- `/dumpmem` = comprehensive (includes memory/, full knowledge dump)
- Use `/handover` at natural session boundaries; `/dumpmem` before compaction
