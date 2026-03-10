# AGENTS.md — Project Instructions

> This file is read by Claude Code.
> Edit this file to define project-wide conventions. Keep it stable — it is not for session notes.

## Project Overview

[Describe your project here]

## Key Principles

- [Your core design principles]

## Session Continuity

AI agents lose context between sessions. This project uses the following convention:

- `HANDOFF.md` (project root): Written by the agent at session end. Contains work summary, next steps, blockers.
- A `SessionStart` hook automatically injects HANDOFF.md into context at the start of each session.
- Run `/handoff` before ending a session to update HANDOFF.md and commit it.
- A `PreCompact` hook reminds you to run `/handoff` before context compression.

## Spec-Driven Development (SDD)

Before implementing anything non-trivial:

1. Write what you want in `.spec/PLAN.md` (human, free-form)
2. Structure it into `.spec/SPEC.md` or `.spec/DESIGN.md` (agent, after clarifying questions)
3. Break it down into `.spec/TODO.md` (agent, task list with checkboxes)
4. Record decisions and discoveries in `.spec/KNOWLEDGE.md`

Do not start implementation until `.spec/TODO.md` is confirmed by the human.

## Sub-agents

The following sub-agents are available. Use them proactively per `rules/agents.md`:

| Agent | When to use |
|---|---|
| **planner** | Complex features, refactoring before implementation |
| **architect** | Architecture changes, design decisions |
| **tdd-guide** | New features, bug fixes — write tests first |
| **code-reviewer** | After any code change |
| **security-reviewer** | Before commits involving permissions, auth, or external input |

## Memory

- `memory/MEMORY.md`: Persistent memory across sessions (auto-loaded, 200 line limit)
- Write important technical discoveries to topic files under `memory/`
- See `rules/memory.md` for detailed guidelines

## First Session Bootstrap

When starting a session in a project that has no `.spec/PLAN.md`:

1. Ask the human what they want to accomplish
2. Ask clarifying questions until intent is clear
3. Create `.spec/PLAN.md` summarizing the discussion
4. Proceed with the SDD workflow above

Do not assume the intent from `AGENTS.md` alone. The bootstrap questions surface project-specific goals.
