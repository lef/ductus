# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What This Repo Is

**contextus-claude** is a Claude Code-specific layer for the [contextus](https://github.com/lef/contextus) framework. It is a template/toolkit that users install into their own projects. The files here are scaffolding — they become part of a user's `.claude/` directory, not a runnable application.

Layer hierarchy:
```
L0: contextus (base)    ← upstream, agent-agnostic
L1: contextus-claude    ← this repo (Claude-specific hooks, skills, rules, agents)
L2: contextus-*         ← downstream profiles (e.g. contextus-kw, contextus-dev-sh)
```

## Key Files and Their Roles

| Path | Role |
|---|---|
| `AGENTS.md` | Project-wide conventions for all AI agents (stable, not session-specific) |
| `HANDOFF.md` | Session handoff state — written by agent at session end, injected at session start |
| `.spec/` | Spec-Driven Development scaffold (PLAN → SPEC/DESIGN → TODO → KNOWLEDGE) |
| `.claude/settings.json` | Hook wiring and permission deny-list |
| `.claude/hooks/memory-persistence/` | SessionStart / Stop / PreCompact hook scripts |
| `.claude/skills/handoff/SKILL.md` | `/handoff` slash command |
| `.claude/skills/sos-recall/SKILL.md` | `/sos-recall` emergency context recovery |
| `.claude/rules/` | Rule files injected into Claude's context (memory, agents, git-workflow, security, rust-style) |
| `.claude/agents/` | Sub-agent definitions (planner, architect, tdd-guide, code-reviewer, security-reviewer) |
| `.claude/setup.sh` | Bootstrap script for installing contextus-claude into a new project |
| `.claude/statusline.sh` | Status line command for Claude Code's status bar |

## How Hooks Work

The three hook scripts in `.claude/hooks/memory-persistence/` are wired in `settings.json`:

- **SessionStart**: Reads `HANDOFF.md` and outputs it to stdout — Claude Code injects this into the context window. Also shows recent compaction log entries on stderr.
- **Stop**: Creates or updates a daily session log at `~/.claude/sessions/YYYY-MM-DD-session.md`.
- **PreCompact**: Logs the compaction event, appends a marker to the active session log, and reminds Claude to run `/handoff` before context is lost. Also auto-commits via `git add -A && git commit`.

Hook stdout is injected into Claude's context; stderr is shown to the user.

## Spec-Driven Development (SDD) Workflow

For any non-trivial change to this repo:

1. Human writes intent in `.spec/PLAN.md`
2. Agent asks clarifying questions, then writes `.spec/SPEC.md` (code) or `.spec/DESIGN.md` (docs/research)
3. Agent writes `.spec/TODO.md` (checkbox task list)
4. Human confirms TODO.md before implementation starts
5. Discoveries and decisions go into `.spec/KNOWLEDGE.md`

**TODO.md is the authoritative task tracker.** If HANDOFF.md's "next task" conflicts with TODO.md, TODO.md wins.

## Session Continuity

At the start of every session:
1. Read `HANDOFF.md` (auto-injected by SessionStart hook)
2. Cross-check against `.spec/TODO.md` — TODO.md takes precedence if they conflict
3. Confirm the next task with the user before starting

Before ending a session, run `/handoff` to update and commit `HANDOFF.md`.

## Installing contextus-claude (setup.sh)

```bash
gh repo clone lef/contextus-claude .claude -- --depth=1
bash .claude/setup.sh [--layer2 <repo>]
```

`setup.sh` requires `gh` CLI (authenticated). It fetches L0 base files, cleans up `.claude/` metadata, optionally installs an L2 profile, makes hooks executable, creates `HANDOFF.md`, and initializes git.

## Contribution Scope

- Hook, rule, agent, or skill improvements → PR to this repo (L1)
- Shell-specific conventions → PR to contextus-dev-sh (L2)
- Agent-agnostic conventions (HANDOFF format, `.spec/` workflow) → PR to contextus (L0)

All content must be generic and reusable (no project-specific content). Dependencies: `bash`, `date`, `sed` only.
