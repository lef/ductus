#!/bin/bash
# setup.sh — Bootstrap a contextus project
#
# Usage (run from your project root after cloning contextus-claude):
#
#   gh repo clone lef/contextus-claude .claude -- --depth=1
#   bash .claude/setup.sh [--layer2 <repo>]
#
# Options:
#   --layer2 <repo>   Install an L2 profile (e.g. contextus-kw, contextus-sh-dev)
#
# What this does:
#   1. Checks gh CLI is installed and authenticated
#   2. Configures git to use gh as credential helper (safe HTTPS auth)
#   3. Fetches contextus (L0) base files into your project
#   4. Cleans up .claude/ (removes .git and contextus-claude's own project files)
#   5. Installs L2 profile rules/agents if specified
#   6. Makes hook scripts executable
#   7. Creates HANDOFF.md if not present
#   8. Initializes a git repository if not already one

set -euo pipefail

# --- Parse arguments ---
LAYER2=""
WORK_DIR=".spec"   # default; L2 can override (e.g. contextus-kw uses .design)
while [[ $# -gt 0 ]]; do
  case "$1" in
    --layer2)    LAYER2="$2";    shift 2 ;;
    --work-dir)  WORK_DIR="$2";  shift 2 ;;
    *) echo "error: unknown option: $1" >&2; exit 1 ;;
  esac
done

# --- Resolve paths ---
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
CLAUDE_DIR="$SCRIPT_DIR"

echo ":: contextus setup" >&2
echo ":: project directory: $PROJECT_DIR" >&2

# --- 1. Check gh CLI ---
if ! command -v gh &>/dev/null; then
  echo "" >&2
  echo "error: gh (GitHub CLI) is not installed." >&2
  echo "" >&2
  echo "  Please install it first:" >&2
  echo "    macOS:         brew install gh" >&2
  echo "    Ubuntu/Debian: sudo apt-get install gh" >&2
  echo "    Other:         https://cli.github.com/" >&2
  echo "" >&2
  exit 1
fi
echo ":: gh CLI found" >&2

# --- 2. Check gh auth ---
if ! gh auth status &>/dev/null; then
  echo "" >&2
  echo "error: not logged in to GitHub." >&2
  echo "" >&2
  echo "  Please run: gh auth login" >&2
  echo "  Then re-run this script." >&2
  echo "" >&2
  exit 1
fi
echo ":: GitHub authentication OK" >&2

# --- 3. Configure git credential helper ---
gh auth setup-git
echo ":: git credential helper configured" >&2

# --- 4. Fetch L0 (contextus base files) ---
echo ":: fetching contextus base files..." >&2
TMP_L0="$(mktemp -d)"
trap 'rm -rf "$TMP_L0"' EXIT

git clone --quiet --depth=1 https://github.com/lef/contextus "$TMP_L0"

# AGENTS.md — skip if already exists
if [ ! -f "$PROJECT_DIR/AGENTS.md" ]; then
  cp "$TMP_L0/AGENTS.md" "$PROJECT_DIR/AGENTS.md"
  echo ":: AGENTS.md created" >&2
else
  echo ":: AGENTS.md already exists, skipping" >&2
fi

# Work directory (.spec by default, overridden by --work-dir)
if [ ! -d "$PROJECT_DIR/$WORK_DIR" ]; then
  cp -r "$TMP_L0/.spec" "$PROJECT_DIR/$WORK_DIR"
  echo ":: $WORK_DIR/ created" >&2
else
  echo ":: $WORK_DIR/ already exists, skipping" >&2
fi

# --- 5. Remove .claude/.git and contextus-claude's own project files ---
if [ -d "$CLAUDE_DIR/.git" ]; then
  rm -rf "$CLAUDE_DIR/.git"
fi
# Remove contextus-claude's own dogfooding files (not for consumer projects)
rm -f  "$CLAUDE_DIR/AGENTS.md"
rm -f  "$CLAUDE_DIR/HANDOFF.md"
rm -f  "$CLAUDE_DIR/README.md"
rm -rf "$CLAUDE_DIR/.spec"
rm -rf "$CLAUDE_DIR/.contextus"
echo ":: .claude/ cleaned up" >&2

# --- 6. Install L2 profile (optional) ---
if [ -n "$LAYER2" ]; then
  echo ":: fetching $LAYER2 (L2 profile)..." >&2
  TMP_L2="$(mktemp -d)"
  git clone --quiet --depth=1 "https://github.com/lef/$LAYER2" "$TMP_L2"

  if [ -d "$TMP_L2/rules" ]; then
    cp -r "$TMP_L2/rules/." "$CLAUDE_DIR/rules/"
    echo ":: $LAYER2 rules installed" >&2
  fi
  if [ -d "$TMP_L2/agents" ]; then
    cp -r "$TMP_L2/agents/." "$CLAUDE_DIR/agents/"
    echo ":: $LAYER2 agents installed" >&2
  fi

  rm -rf "$TMP_L2"
fi

# --- 7. Make hook scripts executable ---
find "$CLAUDE_DIR/hooks" -name "*.sh" -exec chmod +x {} \;
echo ":: hooks configured" >&2

# --- 8. Create HANDOFF.md if not present ---
HANDOFF_FILE="$PROJECT_DIR/HANDOFF.md"
if [ ! -f "$HANDOFF_FILE" ]; then
  cat > "$HANDOFF_FILE" << 'EOF'
# HANDOFF — Session Transition Notes

**Last Updated**: —
**Previous Work**: (first session)

## Current State

### Completed (this session)
-

### In Progress
-

### Not Started (priority order)
1.

## Next Session: Read First

-

## Key Decisions Made

-

## Blockers / Watch Out For

-

## Changed Files

-
EOF
  echo ":: HANDOFF.md created" >&2
fi

# --- 9. Initialize git repository if needed ---
if [ ! -d "$PROJECT_DIR/.git" ]; then
  git -C "$PROJECT_DIR" init --quiet
  git -C "$PROJECT_DIR" add .
  git -C "$PROJECT_DIR" commit --quiet -m "chore: initialize contextus project"
  echo ":: git repository initialized" >&2
else
  echo ":: git repository already exists, skipping" >&2
fi

# --- Done ---
echo "" >&2
echo ":: Setup complete." >&2
echo "" >&2
echo "   Next steps:" >&2
echo "   1. Edit AGENTS.md — describe your project" >&2
echo "   2. Write your first plan in .spec/PLAN.md" >&2
echo "   3. Start Claude Code: claude" >&2
echo "" >&2
