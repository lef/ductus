# contextus-claude

Claude Code-specific layer for [contextus](https://github.com/lef/contextus).

Provides automation for the HANDOFF.md workflow:
- Session start: injects HANDOFF.md into Claude's context automatically
- Session end: creates session log
- Pre-compact: saves state before context compression, reminds to run `/handoff`
- `/handoff` skill: updates HANDOFF.md and commits it

## Installation

```bash
mkdir my-project && cd my-project
gh repo clone lef/contextus-claude .claude -- --depth=1
bash .claude/setup.sh
```

For a knowledge work project, add `--layer2 contextus-kw`:

```bash
bash .claude/setup.sh --layer2 contextus-kw
```

`setup.sh` handles everything:
- Verifies gh CLI is installed and authenticated
- Configures git to use gh as credential helper (safe HTTPS auth)
- Fetches contextus (L0) base files: `AGENTS.md`, `HANDOFF.md`, `.spec/`
- Cleans up `.claude/` (removes clone metadata)
- Installs L2 profile rules/agents if `--layer2` is specified
- Makes hook scripts executable
- Initializes a git repository

**Requirements:** [gh CLI](https://cli.github.com/) (`brew install gh` / `sudo apt-get install gh`)

## What's Inside

| Path | Purpose |
|---|---|
| `hooks/memory-persistence/` | SessionStart / Stop / PreCompact hooks |
| `skills/handoff/SKILL.md` | `/handoff` slash command |
| `skills/sos-recall/SKILL.md` | `/sos-recall` — emergency context recovery from session log |
| `rules/memory.md` | Memory persistence rules for Claude |
| `rules/agents.md` | Sub-agent usage conventions |
| `rules/git-workflow.md` | Conventional commits, branch strategy |
| `rules/security.md` | Prompt injection defense |
| `agents/` | planner, architect, tdd-guide, code-reviewer, security-reviewer |
| `settings.json` | Hook wiring + permission defaults |

## Requirements

- `bash`, `date`, `sed` (standard on Linux/macOS)
- No external dependencies

## Hook Wiring

`settings.json` wires the hooks automatically. The hooks use `$CLAUDE_PROJECT_DIR` (set by Claude Code) to find `HANDOFF.md` relative to the project.

## Skills

### `/handoff`

Run `/handoff` before ending a session. It will:
1. Update `HANDOFF.md` with this session's work
2. Commit it to git

Claude's `PreCompact` hook reminds you to run it before context compression.

### `sos-*` series — Emergency recovery skills

Skills prefixed with `sos-` are for emergency situations when context is lost.

### `/sos-recall`

Run `/sos-recall` when you (or Claude) can't remember what was decided earlier in the session. It extracts the last 20 assistant messages from the session's `.jsonl` log — no keyword needed.

## Layer Position

```
L0: contextus (base)    ← upstream
L1: contextus-claude    ← this repo
L2: contextus-*         ← downstream (e.g. contextus-sh-dev)
```

## Contribution Policy

- Improvements to hooks, rules, agents, or skills: PR to this repo
- Shell-specific conventions: PR to contextus-sh-dev
- Agent-agnostic conventions (HANDOFF, .spec/): PR to contextus

## Key Principles (for contributors)

- Language-neutral: hook messages and tags must be in English
- No project-specific content: everything must be generic and reusable
- Minimal dependencies: bash, date, sed only — no external tools
