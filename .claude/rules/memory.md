# Memory Persistence Rules

Claude loses context on compression. Follow these rules to prevent information loss.

## Session Start Checklist

1. Read HANDOFF.md
2. **Read `.spec/TODO.md` and cross-check against HANDOFF.md's "next task"**
   - If priorities conflict, TODO.md takes precedence
   - TODO.md is the authoritative task tracker with recorded rationale
3. Confirm the next task with the user before starting

> HANDOFF.md's "next task" can drift from TODO.md when written carelessly.
> Always verify against TODO.md before acting.

## When Writing HANDOFF.md — "Next Task" Rule

Copy the task line directly from TODO.md. Do not rephrase or renumber.

- ❌ "Aider sandbox test (Priority 4)" ← self-assigned number, may be wrong
- ✅ "Priority 4: rename sandbox-agent → sandbox-claude (see TODO.md)" ← exact copy

## Before Context Compaction (Critical)

When context is large or a PreCompact hook message appears, **in this order**:

1. Run `/handoff` to update and commit HANDOFF.md
2. Write important technical discoveries to auto memory (`~/.claude/projects/.../memory/`)
3. Then allow compression

> The PreCompact hook only does mechanical work. Only Claude can write meaningful HANDOFF content.
> Context is lost after compression — HANDOFF.md must be updated **before**.

## When to Write to Auto Memory

1. **Important technical discovery** — e.g., "Library X requires Y at runtime"
   → Write to a topic file in memory/ immediately

2. **Significant design decision** — e.g., "Chose flat file over DB for simplicity"
   → Write to MEMORY.md or decisions.md

3. **Long work in progress**
   → Update MEMORY.md with: current task, progress, next step

4. **User says "remember this"**
   → Write immediately. No confirmation needed.

## HANDOFF.md vs MEMORY.md

| | HANDOFF.md | MEMORY.md (auto memory) |
|---|---|---|
| Purpose | Next session handoff | Long-term knowledge |
| Written | Every session end | When discoveries happen |
| Scope | What's happening now | Stable patterns and facts |
| Committed | Yes (project repo) | Yes (auto memory dir) |

## What Not to Do

- Do not write session-specific state to MEMORY.md
- Do not write unverified information
- Do not let MEMORY.md exceed 200 lines (split to topic files)
