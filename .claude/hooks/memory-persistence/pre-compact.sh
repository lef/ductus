#!/bin/bash
# PreCompact Hook — save state before context compression

SESSIONS_DIR="${HOME}/.claude/sessions"
COMPACTION_LOG="${SESSIONS_DIR}/compaction-log.txt"
PROJECT_DIR="${CLAUDE_PROJECT_DIR:-.}"

mkdir -p "$SESSIONS_DIR"

echo "[$(date '+%Y-%m-%d %H:%M:%S')] Context compaction triggered (project: $PROJECT_DIR)" >> "$COMPACTION_LOG"

ACTIVE_SESSION=$(ls -t "$SESSIONS_DIR"/*.md 2>/dev/null | head -1)
if [ -n "$ACTIVE_SESSION" ] && [ -f "$ACTIVE_SESSION" ]; then
    echo "" >> "$ACTIVE_SESSION"
    echo "---" >> "$ACTIVE_SESSION"
    echo "**[Compaction at $(date '+%H:%M')]** — Context compressed. Above content may be summarized." >> "$ACTIVE_SESSION"
fi

echo "[PreCompact] State saved before compaction" >&2
echo "[PreCompact] IMPORTANT: run /handoff to update HANDOFF.md before context is lost" >&2
