#!/bin/bash
# SessionStart Hook — inject previous session context into Claude's context window
#
# Outputs HANDOFF.md to stdout, which Claude Code injects into the context.
# User-visible messages are handled by session-banner.sh (systemMessage JSON).

set -euo pipefail

PROJECT_DIR="${CLAUDE_PROJECT_DIR:-$PWD}"
HANDOFF_FILE="${PROJECT_DIR}/HANDOFF.md"
SPEC_DIR="${PROJECT_DIR}/.spec"
SESSIONS_DIR="${HOME}/.claude/sessions"

# Inject enforcement rules (always, before any other context)
cat << 'ENFORCE'
[session:enforcement]
MANDATORY — 以下は全セッションで遵守する義務:
1. 非自明なタスクでは必ず extended thinking (megathink) を使用すること
2. 実装コードを書く前に必ずテストを先に書くこと（TDD: RED → GREEN → REFACTOR）
3. 非自明な機能実装の前に .spec/TODO.md を確認し、人間の承認を得ること（SDD）
違反はユーザーの時間を浪費する。
[/session:enforcement]
ENFORCE

# Inject HANDOFF.md into context (stdout)
if [ -f "$HANDOFF_FILE" ]; then
    echo "[session:handoff]"
    cat "$HANDOFF_FILE"
    echo ""
    echo "[/session:handoff]"
fi

# Inject CONSTITUTION.md into context (stdout) — project constraints
CONSTITUTION_FILE="${SPEC_DIR}/CONSTITUTION.md"
if [ -f "$CONSTITUTION_FILE" ]; then
    echo "[session:constitution]"
    cat "$CONSTITUTION_FILE"
    echo ""
    echo "[/session:constitution]"
fi

# Inject SDD files into context (stdout)
for sdd_file in TODO.md PLAN.md KNOWLEDGE.md; do
    filepath="${SPEC_DIR}/${sdd_file}"
    if [ -f "$filepath" ]; then
        tag=$(echo "$sdd_file" | sed 's/\.md$//' | tr '[:upper:]' '[:lower:]')
        echo "[session:${tag}]"
        if [ "$sdd_file" = "TODO.md" ]; then
            # Filter out completed tasks (- [x]) and their indented children
            awk '
                /^- \[x\]/ { skip = 1; next }
                /^[^ ]/ || /^- / { skip = 0 }
                !skip { print }
            ' "$filepath"
        else
            cat "$filepath"
        fi
        echo ""
        echo "[/session:${tag}]"
    fi
done

# Append meeting header to MINUTES.md and inject into context
MINUTES_FILE="${SPEC_DIR}/MINUTES.md"
if [ -d "$SPEC_DIR" ]; then
    TODAY=$(date '+%Y-%m-%d')
    NOW=$(date '+%H:%M')
    {
        echo ""
        echo "## 会議: ${TODAY} ${NOW}"
        echo "**出席者**: $(whoami)（人間）、Claude（AI）"
        echo ""
        echo "---"
        echo ""
    } >> "$MINUTES_FILE" || true

    # Inject only recent 5 sessions (file retains full history)
    echo "[session:minutes]"
    _total=$(grep -c '^## 会議:' "$MINUTES_FILE" 2>/dev/null || echo 0)
    _skip=$(( _total > 5 ? _total - 5 : 0 ))
    if [ "$_skip" -eq 0 ]; then
        cat "$MINUTES_FILE"
    else
        awk -v skip="$_skip" '
            /^## 会議:/ { count++ }
            count > skip { print }
        ' "$MINUTES_FILE"
    fi
    echo ""
    echo "[/session:minutes]"
fi

# Record [SessionStart] to trace.log
TODAY=$(date '+%Y-%m-%d')
TRACE_FILE="${SESSIONS_DIR}/${TODAY}-trace.log"
mkdir -p "$SESSIONS_DIR"
echo "$(date -Iseconds) [SessionStart] project=${PROJECT_DIR}" >> "$TRACE_FILE" || true
