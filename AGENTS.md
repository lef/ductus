# AGENTS.md — Project Instructions

> This file is read by AI agents (Claude Code, Codex, Gemini CLI, Aider, etc.).
> Edit this file to define project-wide conventions. Keep it stable — it is not for session notes.

## Project Overview

[Describe your project here]

## Key Principles

- [Your core design principles]

## Session Continuity

AI agents lose context between sessions. This project uses the following convention:

- `HANDOFF.md` (project root): Written by the agent at session end. Contains work summary, next steps, blockers.
- Read `HANDOFF.md` at the start of every session to restore context.
- Update `HANDOFF.md` before ending a session.
- Commit `HANDOFF.md` to git so it persists across machines.

## Spec-Driven Development (SDD)

Before implementing anything non-trivial:

1. Write what you want in `.spec/PLAN.md` (human, free-form)
2. Structure it into `.spec/SPEC.md` or `.spec/DESIGN.md` (agent, after clarifying questions)
3. Break it down into `.spec/TODO.md` (agent, task list with checkboxes)
4. Record decisions and discoveries in `.spec/KNOWLEDGE.md`

Do not start implementation until `.spec/TODO.md` is confirmed.

## `.spec/` Variants

| Workflow | Files |
|---|---|
| Software development | PLAN → SPEC → TODO → KNOWLEDGE |
| Research / knowledge work | PLAN → DESIGN → TODO → KNOWLEDGE |

Use `SPEC.md` when output is code or a system. Use `DESIGN.md` when output is a document, paper, or analysis.

## First Session Bootstrap

When starting a session in a project that has no `.spec/PLAN.md`:

1. Ask the human what they want to accomplish
2. Ask clarifying questions until intent is clear
3. Create `.spec/PLAN.md` summarizing the discussion
4. Proceed with the SDD workflow above

Do not assume the intent from `AGENTS.md` alone. The bootstrap questions surface project-specific goals that belong in `.spec/`, not `AGENTS.md`.

> **TODO**: This bootstrap step could eventually be automated in a session-start hook
> or a `make bootstrap-spec` command that prompts the human and generates `.spec/` files.

## Sub-Agent Delegation

When delegating to a sub-agent (specialized agent, tool, or another AI session):

**Always pass:**
- What already exists (relevant file contents, current state)
- What must NOT change — constraints that were deliberately set, and why
- The rationale behind existing decisions, so the sub-agent doesn't "fix" intentional choices

> Without this context, sub-agents apply generic best practices that may conflict with
> project-specific decisions. The calling agent is responsible for providing context.

**Also check TODO.md before starting any task.** If HANDOFF.md's "next task" conflicts
with TODO.md, TODO.md takes precedence — it is the authoritative task tracker.

## Memory and Knowledge

- Do not write session-specific state into `AGENTS.md`
- `AGENTS.md` contains only stable, project-wide directives
- Use `HANDOFF.md` for current session state
- Use `.spec/KNOWLEDGE.md` for accumulated project discoveries
