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

## インクリメンタル HANDOFF 更新（コンパクション損失の最小化）

「セッション終了時にまとめて書く」ではなく、**決定・完了のたびに HANDOFF.md を更新する**。

更新タイミング（いずれかに該当したら）:
- git commit 直後（何を・なぜ変えたかを一行追記）
- 重要な設計決定をした直後
- ユーザーが「OK」「次へ」と言ってタスクが区切れた直後
- 新しい未解決タスクを発見した直後

手順: HANDOFF.md の In Progress / Next Session セクションを更新 → `git commit -m "docs: update HANDOFF.md"`

> PreCompact hook の自動スナップショット（git log + 未完 TODO の機械的転写）は保険。
> Claude が書いた内容より低品質なので、インクリメンタル更新が優先。

## Before Context Compaction (Critical)

When context is large or a PreCompact hook message appears, **run `/dumpmem` immediately**.
Do not skip. Do not defer. Run it before any other response.

> The PreCompact hook auto-writes a HANDOFF snapshot (git log + pending TODOs) as a safety net.
> Only Claude can write meaningful narrative content. `/dumpmem` must run **before** compaction.

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
| Written | Every task completion + session end | When discoveries happen |
| Scope | What's happening now | Stable patterns and facts |
| Committed | Yes (project repo) | Yes (auto memory dir) |

## Context Window Best Practices

- **CLAUDE.md < 150 lines** — adherence drops significantly beyond this
- **Avoid the last 20% of context** for complex multi-file work (single-file edits are safe)
- Each subtask should be completable within < 50% of the context window
- Load files only when needed — do not front-load "just in case"
- Small atomic commits per task; do not batch across features

## What Not to Do

- Do not write session-specific state to MEMORY.md
- Do not write unverified information
- Do not let MEMORY.md exceed 200 lines (split to topic files)
