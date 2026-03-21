#!/bin/bash
# SessionStart Banner — show user what was loaded (systemMessage only)
#
# Claude Code ignores SessionStart hook stderr on exit 0.
# Use JSON stdout with systemMessage to display to user.
# Only alphanumeric identifiers in output. No file contents. Injection-safe.

h=$(git -C "${CLAUDE_PROJECT_DIR:-$PWD}" rev-parse --short HEAD 2>/dev/null || echo "?")
printf '{"systemMessage":"[HANDOFF] context loaded @%s | SDD+TDD, megathink enforced"}\n' "$h"
