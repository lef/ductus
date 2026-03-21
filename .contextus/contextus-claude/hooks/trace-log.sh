#!/bin/bash
# PreToolUse: trace-log.sh — audit trace + strategic compact warnings
#
# 1. Appends one line per tool use to audit trace log
#    Format: ISO8601_TIMESTAMP [TOOL_NAME] KEY_INPUT
#    Key input priority: file_path > url > command (truncated to 80 chars)
#
# 2. Strategic Compact: warns at 50% and 100% of tool-call threshold
#    Uses $PPID (Claude Code process) as session identifier
#    Threshold: COMPACT_THRESHOLD env var (default 50)
#
# Always exits 0 — never blocks tools.

set -uo pipefail

SESSIONS_DIR="${HOME}/.claude/sessions"
TODAY=$(date '+%Y-%m-%d')
TRACE_FILE="${SESSIONS_DIR}/${TODAY}-trace.log"

INPUT=$(cat)

# Extract tool name
TOOL=$(echo "$INPUT" | grep -o '"tool_name":"[^"]*"' | sed 's/"tool_name":"//;s/".*//')

# Extract key input field: file_path > url > command
KEY=$(echo "$INPUT" | grep -o '"file_path":"[^"]*"' | sed 's/"file_path":"//;s/".*//' | head -1)
if [ -z "$KEY" ]; then
    KEY=$(echo "$INPUT" | grep -o '"url":"[^"]*"' | sed 's/"url":"//;s/".*//' | head -1)
fi
if [ -z "$KEY" ]; then
    KEY=$(echo "$INPUT" | grep -o '"command":"[^"]*"' | sed 's/"command":"//;s/".*//' | head -1 | cut -c1-80)
fi

mkdir -p "$SESSIONS_DIR"
echo "$(date -Iseconds) [${TOOL:-unknown}] ${KEY}" >> "$TRACE_FILE"

# ── Strategic Compact: tool-call counter per session (keyed by PPID) ──
THRESHOLD="${COMPACT_THRESHOLD:-50}"
COUNTER_FILE="${SESSIONS_DIR}/tool-count-${PPID}"

# Clean stale counters from previous days
find "$SESSIONS_DIR" -maxdepth 1 -name 'tool-count-*' -not -newer "$TRACE_FILE" -delete 2>/dev/null || true

if [ -f "$COUNTER_FILE" ]; then
    count=$(cat "$COUNTER_FILE" 2>/dev/null || echo 0)
    count=$((count + 1))
else
    count=1
fi
echo "$count" > "$COUNTER_FILE"

FIC_THRESHOLD=$((THRESHOLD * 3 / 5))
if [ "$count" -eq "$FIC_THRESHOLD" ]; then
    echo "[FIC] Context ~50% used (${count} tool calls). Consider /compact at next phase transition." >&2
fi
if [ "$count" -eq "$THRESHOLD" ]; then
    echo "[StrategicCompact] ${THRESHOLD} tool calls — good time for /compact before starting next phase." >&2
fi
if [ "$count" -gt "$THRESHOLD" ] && [ $(( count % 25 )) -eq 0 ]; then
    echo "[StrategicCompact] ${count} tool calls — context getting stale, consider /compact." >&2
fi

exit 0
