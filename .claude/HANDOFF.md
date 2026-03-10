# HANDOFF — contextus-claude

**Last Updated**: 2026-03-10
**Previous Work**: Initial repository creation + PR #1 (English hook tags)

## Current State

### Completed
- Initial repo structure: hooks, rules, agents, skills, settings.json
- PR #1: Replace Japanese tags in session-start.sh with `[session:handoff]`

### Pending
- PR #1 needs merge
- AGENTS.md and HANDOFF.md added (this PR)

## Next Session: Read First

- Check PR #1 status (fix/english-hook-messages)
- `.spec/` not yet initialized — add if planning larger changes

## Known Issues

- `rules/memory.md` references `~/.claude/projects/...` (Claude Code specific path) — acceptable for L1
- `settings.json` wires hooks to `$CLAUDE_PROJECT_DIR/.claude/hooks/...` — correct by design
