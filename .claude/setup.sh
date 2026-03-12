#!/bin/bash
# setup.sh — Bootstrap or update a contextus project
#
# Initial setup (run from project root after cloning contextus-claude):
#
#   gh repo clone lef/contextus-claude .claude -- --depth=1
#   bash .claude/setup.sh [--layer2 <repo>]
#
# Update all installed layers to latest:
#
#   bash .claude/setup.sh --update
#
# Options:
#   --layer2 <repo>   Install an L2+ profile (e.g. contextus-dev-rust, contextus-kw)
#   --update          Re-sync L0, L1, and all installed L2+ layers from GitHub
#   --work-dir <dir>  Work directory for .spec/ (default: .spec)

set -euo pipefail

# --- Parse arguments ---
LAYER2=""
UPDATE=false
WORK_DIR=".spec"
while [[ $# -gt 0 ]]; do
  case "$1" in
    --layer2)    LAYER2="$2";   shift 2 ;;
    --update)    UPDATE=true;   shift   ;;
    --work-dir)  WORK_DIR="$2"; shift 2 ;;
    *) echo "error: unknown option: $1" >&2; exit 1 ;;
  esac
done

# --- Resolve paths ---
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
CLAUDE_DIR="$SCRIPT_DIR"
LAYERS_FILE="$CLAUDE_DIR/.contextus/layers"

# --- Helpers ---

check_gh() {
  if ! command -v gh &>/dev/null; then
    echo "" >&2
    echo "error: gh (GitHub CLI) is not installed." >&2
    echo "  macOS:         brew install gh" >&2
    echo "  Ubuntu/Debian: sudo apt-get install gh" >&2
    echo "  Other:         https://cli.github.com/" >&2
    echo "" >&2
    exit 1
  fi
  if ! gh auth status &>/dev/null; then
    echo "" >&2
    echo "error: not logged in to GitHub. Run: gh auth login" >&2
    echo "" >&2
    exit 1
  fi
  gh auth setup-git
  echo ":: gh CLI OK" >&2
}

clone_tmp() {
  local repo="$1"
  local tmp
  tmp="$(mktemp -d)"
  git clone --quiet --depth=1 "https://github.com/lef/$repo" "$tmp"
  echo "$tmp"
}

# Apply L0 (contextus) files — only framework-owned rules, not project files.
apply_l0() {
  local src="$1"
  echo ":: applying L0 (contextus)..." >&2
  mkdir -p "$CLAUDE_DIR/rules"
  for rule in agent-security.md sdd.md security.md; do
    [ -f "$src/rules/$rule" ] && cp "$src/rules/$rule" "$CLAUDE_DIR/rules/$rule"
  done
}

# Apply L1 (contextus-claude) files — hooks, skills, agents, rules, scripts.
# Does NOT touch: settings.json, settings.local.json (project-owned).
apply_l1() {
  local src="$1"
  echo ":: applying L1 (contextus-claude)..." >&2
  for dir in hooks skills agents; do
    [ -d "$src/$dir" ] && cp -r "$src/$dir/." "$CLAUDE_DIR/$dir/"
  done
  # L1-owned rules (not namespaced)
  for rule in memory.md git-workflow.md agents.md; do
    [ -f "$src/rules/$rule" ] && cp "$src/rules/$rule" "$CLAUDE_DIR/rules/$rule"
  done
  [ -f "$src/statusline.sh" ] && cp "$src/statusline.sh" "$CLAUDE_DIR/statusline.sh"
  [ -f "$src/setup.sh" ]      && cp "$src/setup.sh"      "$CLAUDE_DIR/setup.sh"
  find "$CLAUDE_DIR/hooks" -name "*.sh" -exec chmod +x {} \;
  [ -f "$CLAUDE_DIR/statusline.sh" ] && chmod +x "$CLAUDE_DIR/statusline.sh"
  [ -f "$CLAUDE_DIR/setup.sh" ]      && chmod +x "$CLAUDE_DIR/setup.sh"
}

# Apply L2+ profile — rules go into rules/<profile>/ to avoid namespace collision.
apply_layer() {
  local repo="$1"
  local profile
  profile="$(basename "$repo")"
  local src="$2"
  echo ":: applying $repo -> rules/$profile/..." >&2
  if [ -d "$src/rules" ]; then
    mkdir -p "$CLAUDE_DIR/rules/$profile"
    cp -r "$src/rules/." "$CLAUDE_DIR/rules/$profile/"
  fi
  if [ -d "$src/agents" ]; then
    cp -r "$src/agents/." "$CLAUDE_DIR/agents/"
  fi
}

# Record an L2+ profile in the layers manifest.
record_layer() {
  local repo="$1"
  mkdir -p "$CLAUDE_DIR/.contextus"
  touch "$LAYERS_FILE"
  if ! grep -qxF "$repo" "$LAYERS_FILE"; then
    echo "$repo" >> "$LAYERS_FILE"
  fi
}

# Read installed L2+ layers from manifest.
read_layers() {
  if [ -f "$LAYERS_FILE" ]; then
    grep -v '^#' "$LAYERS_FILE" | grep -v '^[[:space:]]*$' || true
  fi
}

# ============================================================
# --update mode: re-sync all installed layers
# ============================================================
if $UPDATE; then
  echo ":: contextus update" >&2
  check_gh

  # L0
  TMP_L0="$(clone_tmp contextus)"
  trap 'rm -rf "$TMP_L0"' EXIT
  apply_l0 "$TMP_L0"

  # L1 — clone to temp, apply, then the script itself is replaced
  TMP_L1="$(clone_tmp contextus-claude)"
  trap 'rm -rf "$TMP_L0" "$TMP_L1"' EXIT
  # Clean L1 dogfooding files from the clone before applying
  rm -f  "$TMP_L1/AGENTS.md" "$TMP_L1/HANDOFF.md" "$TMP_L1/README.md"
  rm -rf "$TMP_L1/.spec" "$TMP_L1/.contextus" "$TMP_L1/.git"
  apply_l1 "$TMP_L1"

  # L2+ from manifest
  while IFS= read -r layer; do
    TMP_LN="$(clone_tmp "$layer")"
    apply_layer "$layer" "$TMP_LN"
    rm -rf "$TMP_LN"
  done < <(read_layers)

  echo "" >&2
  echo ":: Update complete." >&2
  echo "" >&2
  exit 0
fi

# ============================================================
# Initial setup mode
# ============================================================
echo ":: contextus setup" >&2
echo ":: project directory: $PROJECT_DIR" >&2

check_gh

# --- 1. Fetch and apply L0 ---
TMP_L0="$(mktemp -d)"
trap 'rm -rf "$TMP_L0"' EXIT
git clone --quiet --depth=1 https://github.com/lef/contextus "$TMP_L0"

apply_l0 "$TMP_L0"

# L0 project files — only create, never overwrite
if [ ! -f "$PROJECT_DIR/AGENTS.md" ]; then
  cp "$TMP_L0/AGENTS.md" "$PROJECT_DIR/AGENTS.md"
  echo ":: AGENTS.md created" >&2
fi
if [ ! -d "$PROJECT_DIR/$WORK_DIR" ]; then
  cp -r "$TMP_L0/.spec" "$PROJECT_DIR/$WORK_DIR"
  echo ":: $WORK_DIR/ created" >&2
fi

# --- 2. Clean up contextus-claude's own dogfooding files ---
if [ -d "$CLAUDE_DIR/.git" ]; then
  rm -rf "$CLAUDE_DIR/.git"
fi
rm -f  "$CLAUDE_DIR/AGENTS.md" "$CLAUDE_DIR/HANDOFF.md" "$CLAUDE_DIR/README.md"
rm -rf "$CLAUDE_DIR/.spec"
echo ":: .claude/ cleaned up" >&2

# --- 3. Install L2+ profile (optional) ---
if [ -n "$LAYER2" ]; then
  TMP_L2="$(mktemp -d)"
  git clone --quiet --depth=1 "https://github.com/lef/$LAYER2" "$TMP_L2"
  apply_layer "$LAYER2" "$TMP_L2"
  record_layer "$LAYER2"
  rm -rf "$TMP_L2"
fi

# --- 4. Make scripts executable ---
find "$CLAUDE_DIR/hooks" -name "*.sh" -exec chmod +x {} \;
[ -f "$CLAUDE_DIR/statusline.sh" ] && chmod +x "$CLAUDE_DIR/statusline.sh"
[ -f "$CLAUDE_DIR/setup.sh" ]      && chmod +x "$CLAUDE_DIR/setup.sh"
echo ":: hooks configured" >&2

# --- 5. Create HANDOFF.md if not present ---
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

# --- 6. Initialize git repository if needed ---
if [ ! -d "$PROJECT_DIR/.git" ]; then
  git -C "$PROJECT_DIR" init --quiet
  git -C "$PROJECT_DIR" add .
  git -C "$PROJECT_DIR" commit --quiet -m "chore: initialize contextus project"
  echo ":: git repository initialized" >&2
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
