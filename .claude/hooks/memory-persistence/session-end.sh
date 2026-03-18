#!/bin/bash
# Stop Hook — create/update session log at session end

SESSIONS_DIR="${HOME}/.claude/sessions"
TODAY=$(date '+%Y-%m-%d')
SESSION_FILE="${SESSIONS_DIR}/${TODAY}-session.md"
PROJECT_NAME=$(basename "${CLAUDE_PROJECT_DIR:-$PWD}")
PROJECT_DIR="${CLAUDE_PROJECT_DIR:-$PWD}"
SPEC_DIR="${PROJECT_DIR}/.spec"

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

# Append Git Summary to WORKLOG.md (skip if today's entry already exists)
WORKLOG_FILE="${SPEC_DIR}/WORKLOG.md"
if [ -d "$SPEC_DIR" ] && git -C "$PROJECT_DIR" rev-parse --git-dir > /dev/null 2>&1; then
    if grep -q "^## Git Summary (${TODAY})" "$WORKLOG_FILE" 2>/dev/null; then
        echo "[SessionEnd] WORKLOG.md already has entry for ${TODAY}, skipping" >&2
    else
        {
            echo ""
            echo "## Git Summary (${TODAY})"
            echo "### Commits"
            git -C "$PROJECT_DIR" --no-pager log --oneline --since="00:00" 2>/dev/null | sed 's/^/- /' || true
            echo "### Changed Files"
            git -C "$PROJECT_DIR" --no-pager diff --stat HEAD 2>/dev/null || true
        } >> "$WORKLOG_FILE" || true
        echo "[SessionEnd] Appended Git Summary to WORKLOG.md" >&2
    fi
fi

# Record [SessionEnd] to trace.log
TRACE_FILE="${SESSIONS_DIR}/${TODAY}-trace.log"
mkdir -p "$SESSIONS_DIR"
echo "$(date -Iseconds) [SessionEnd] project=${PROJECT_DIR}" >> "$TRACE_FILE" || true
