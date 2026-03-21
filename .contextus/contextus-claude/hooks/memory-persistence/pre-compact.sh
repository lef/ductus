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

# Plan A: Auto-generate HANDOFF snapshot from git log + pending TODOs
# Runs mechanically without Claude intelligence — safety net if /dumpmem was missed.
# If /dumpmem was already run, this is redundant but harmless.
HANDOFF_FILE="${PROJECT_DIR}/HANDOFF.md"
SPEC_DIR="${PROJECT_DIR}/.spec"
if git -C "$PROJECT_DIR" rev-parse --git-dir > /dev/null 2>&1; then
    {
        echo ""
        echo "---"
        echo "<!-- pre-compact-snapshot -->"
        echo "## Pre-Compaction Snapshot (auto-generated $(date '+%Y-%m-%d %H:%M'))"
        echo ""
        echo "### Recent Commits"
        git -C "$PROJECT_DIR" --no-pager log --oneline -10 2>/dev/null | sed 's/^/- /' || true
        echo ""
        if [ -f "${SPEC_DIR}/TODO.md" ]; then
            echo "### Pending TODOs"
            grep '^\- \[ \]' "${SPEC_DIR}/TODO.md" | head -20 | sed 's/^/  /' || true
            echo ""
        fi
        echo "**Note**: Run /dumpmem to replace this with Claude-authored context."
    } >> "$HANDOFF_FILE" || true
fi

# REPOS 内の dirty repos もコミット（sandbox で REPOS= 指定時のみ）
if [ -n "${SANDBOX_REPOS_DIR:-}" ] && [ -d "$SANDBOX_REPOS_DIR" ]; then
    for repo in "$SANDBOX_REPOS_DIR"/*/; do
        [ -d "$repo/.git" ] || continue
        cd "$repo"
        git add -u  # 追跡済みファイルのみ（新規ファイルは skill 経由で Claude がレビューして commit）
        git diff --cached --quiet || git commit -m "checkpoint: pre-compact auto-commit" || true
    done
fi

echo "[PreCompact] git commit + HANDOFF snapshot done."
echo ""
echo "================================================================"
echo "STOP. Run /dumpmem NOW before compaction erases this context."
echo "================================================================"
