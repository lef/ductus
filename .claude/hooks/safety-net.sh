#!/usr/bin/env bash
set -euo pipefail

# Safety Net — PreToolUse hook for Claude Code
# Blocks destructive commands before execution.

deny() {
  local reason="$1"
  local cmd="$2"
  cat <<EOF
{"hookSpecificOutput":{"permissionDecision":"deny","permissionDecisionReason":"BLOCKED by Safety Net\n\nReason: ${reason}\n\nCommand: ${cmd}\n\nIf this operation is truly needed, ask the user for explicit permission."}}
EOF
  exit 2
}

input=$(cat)

# Extract "tool_name":"<value>" — [^"]* stops at first closing quote
tool_name=$(echo "$input" | sed -n 's/.*"tool_name"[[:space:]]*:[[:space:]]*"\([^"]*\)".*/\1/p' | head -1)

if [[ "$tool_name" != "Bash" ]]; then
  exit 0
fi

# Extract command value, matching up to closing "}} to handle escaped quotes
cmd=$(echo "$input" | sed -n 's/.*"command"[[:space:]]*:[[:space:]]*"\(.*\)"[[:space:]]*}.*/\1/p' | head -1)
cmd=$(echo "$cmd" | sed 's/\\"/"/g; s/\\\\/\\/g')

if [[ -z "$cmd" ]]; then
  exit 0
fi

check_git_rules() {
  local cmd="$1"

  case "$cmd" in
    *"git reset --hard"*)
      deny "Use 'git stash' first to preserve changes" "$cmd" ;;
    *"git reset --merge"*)
      deny "git reset --merge can lose uncommitted changes" "$cmd" ;;
    *"git checkout -- "*)
      deny "Use 'git stash' first to preserve changes" "$cmd" ;;
  esac

  # git restore: block unless --staged is present
  if [[ "$cmd" == *"git restore"* ]] && [[ "$cmd" != *"--staged"* ]]; then
    deny "Use 'git stash' first to preserve changes" "$cmd"
  fi

  # git clean: block -f unless -n or --dry-run present
  if [[ "$cmd" == *"git clean"* ]] && echo "$cmd" | grep -q "\-[a-zA-Z]*f"; then
    if [[ "$cmd" != *"-n"* ]] && [[ "$cmd" != *"--dry-run"* ]]; then
      deny "Use 'git clean -n' first to preview what will be deleted" "$cmd"
    fi
  fi

  # git push --force: block unless --force-with-lease
  if echo "$cmd" | grep -qE "git push.*(--force|-f)"; then
    if [[ "$cmd" == *"--force-with-lease"* ]]; then
      return
    fi
    deny "Use '--force-with-lease' instead of '--force'" "$cmd"
  fi

  if [[ "$cmd" == *"git branch -D"* ]]; then
    deny "Use '-d' for safe delete (checks merge status)" "$cmd"
  fi

  if echo "$cmd" | grep -qE "git stash (drop|clear)"; then
    deny "Stash entries cannot be recovered after drop/clear" "$cmd"
  fi
}

check_rm_rules() {
  local cmd="$1"

  if ! echo "$cmd" | grep -qE "rm -rf|rm -r -f|rm -fr"; then
    return
  fi

  # Block root
  if echo "$cmd" | grep -qE 'rm -rf\s+/(\*?\s*$|$)'; then
    deny "Refusing to delete root filesystem" "$cmd"
  fi

  # Block home
  if echo "$cmd" | grep -qE 'rm -rf\s+(~|\$HOME)'; then
    deny "Refusing to delete home directory" "$cmd"
  fi

  # Block absolute paths
  local path
  path=$(echo "$cmd" | sed 's/.*rm -r[f ]*//; s/^-[a-zA-Z]* //' | tr -s ' ')
  if [[ "$path" == /* ]]; then
    deny "rm -rf with absolute path is blocked. Use a relative path within your project" "$cmd"
  fi
}

check_find_rules() {
  local cmd="$1"
  if [[ "$cmd" == *"find "* ]] && [[ "$cmd" == *"-delete"* ]]; then
    deny "find -delete is irreversible. Use -print first to verify" "$cmd"
  fi
}

check_git_rules "$cmd"
check_rm_rules "$cmd"
check_find_rules "$cmd"

exit 0
