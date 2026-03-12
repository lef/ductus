#!/bin/bash
# SessionStart Hook — inject previous session context into Claude's context window
#
# Outputs HANDOFF.md to stdout, which Claude Code injects into the context.
# Stderr messages are shown to the user.

set -euo pipefail

PROJECT_DIR="${CLAUDE_PROJECT_DIR:-$PWD}"
HANDOFF_FILE="${PROJECT_DIR}/HANDOFF.md"
SPEC_DIR="${PROJECT_DIR}/.spec"
SESSIONS_DIR="${HOME}/.claude/sessions"

# Inject HANDOFF.md into context (stdout)
if [ -f "$HANDOFF_FILE" ]; then
    echo "[session:handoff]"
    cat "$HANDOFF_FILE"
    echo ""
    echo "[/session:handoff]"
    echo "[SessionStart] Injected HANDOFF.md into context" >&2
else
    echo "[SessionStart] No HANDOFF.md found (first session)" >&2
fi

# Inject SDD files into context (stdout)
# TODO.md is the authoritative task tracker — always inject.
# PLAN.md has the "why" — needed to avoid losing motivation after Phase 0.
# KNOWLEDGE.md has accumulated decisions — prevents re-investigation.
for sdd_file in TODO.md PLAN.md KNOWLEDGE.md; do
    filepath="${SPEC_DIR}/${sdd_file}"
    if [ -f "$filepath" ]; then
        tag=$(echo "$sdd_file" | sed 's/\.md$//' | tr '[:upper:]' '[:lower:]')
        echo "[session:${tag}]"
        cat "$filepath"
        echo ""
        echo "[/session:${tag}]"
    fi
done
if [ -d "$SPEC_DIR" ]; then
    echo "[SessionStart] Injected .spec/ SDD files into context" >&2
fi

# Show recent compaction log entries (stderr only)
COMPACTION_LOG="${SESSIONS_DIR}/compaction-log.txt"
if [ -f "$COMPACTION_LOG" ]; then
    recent=$(tail -3 "$COMPACTION_LOG")
    if [ -n "$recent" ]; then
        echo "[SessionStart] Recent compactions:" >&2
        echo "$recent" >&2
    fi
fi
