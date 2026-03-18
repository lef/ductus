---
name: sos-recall
description: Emergency context recovery — extract recent decisions from this session's conversation log. Use when you've lost track of what was decided earlier in the session.
allowed-tools: Bash
---

Extract recent assistant messages from this session's jsonl log to recover lost context.

## Steps

1. Find the current session's jsonl file:
```bash
ls -t ~/.claude/projects/$(pwd | sed 's|/|-|g')/*.jsonl 2>/dev/null | head -1
```

2. Extract the last 20 assistant text messages using grep (no python3 required):
```bash
JSONL=$(ls -t ~/.claude/projects/$(pwd | sed 's|/|-|g')/*.jsonl 2>/dev/null | head -1)
grep -o '"text":"[^"]*"' "$JSONL" \
  | sed 's/^"text":"//; s/"$//' \
  | sed 's/\\n/\n/g; s/\\"/"/g; s/\\\\/\\/g' \
  | grep -v '^$' \
  | tail -60
```

3. Show the extracted messages to the user so they can identify the lost decision.

## Notes

- python3 is not available on host (host minimization principle) — use grep only
- This recovers assistant text blocks, not the full conversation
- Most useful for recovering design decisions, proposals, and conclusions
- If the session jsonl is not found, check: `ls ~/.claude/projects/`
- For targeted search, use keyword grep directly:
  ```bash
  grep -o '"text":"[^"]*keyword[^"]*"' ~/.claude/projects/SLUG/*.jsonl | head -5
  ```
