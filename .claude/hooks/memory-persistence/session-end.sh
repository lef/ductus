#!/bin/bash
# Stop Hook — create/update session log at session end

SESSIONS_DIR="${HOME}/.claude/sessions"
TODAY=$(date '+%Y-%m-%d')
SESSION_FILE="${SESSIONS_DIR}/${TODAY}-session.md"
PROJECT_NAME=$(basename "${CLAUDE_PROJECT_DIR:-$PWD}")

mkdir -p "$SESSIONS_DIR"

if [ -f "$SESSION_FILE" ]; then
    if command -v sed &>/dev/null; then
        sed -i "s/\*\*Last Updated:\*\*.*/\*\*Last Updated:\*\* $(date '+%H:%M')/" "$SESSION_FILE" 2>/dev/null
    fi
    echo "[SessionEnd] Updated: $SESSION_FILE" >&2
else
    cat > "$SESSION_FILE" << EOF
# Session: ${TODAY}
**Date:** $TODAY
**Started:** $(date '+%H:%M')
**Last Updated:** $(date '+%H:%M')
**Project:** $PROJECT_NAME

---

## Work Done

[Auto-generated session log]

## Handoff Notes

-
EOF
    echo "[SessionEnd] Created: $SESSION_FILE" >&2
fi
