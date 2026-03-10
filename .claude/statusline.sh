#!/bin/bash
# statusline.sh — Claude Code context usage display
#
# Reads context usage JSON from stdin and outputs "Context: XX%"
# Install: copy to ~/.claude/statusline.sh and add statusLine config
# to ~/.claude/settings.json (see contextus-claude README)

input=$(cat)

CACHE=/tmp/statusline-last-pct
PERCENT=$(printf '%s' "$input" \
  | grep -o '"used_percentage":[^,}]*' \
  | grep -o '[0-9]*' \
  | head -1)

if [ -n "$PERCENT" ]; then
    printf '%s' "$PERCENT" > "$CACHE"
elif [ -f "$CACHE" ]; then
    PERCENT=$(cat "$CACHE")
else
    PERCENT="--"
fi
printf "Context: %s%%\n" "$PERCENT"
