---
name: sos-recall
description: Emergency context recovery — extract recent decisions from this session's conversation log. Use when you've lost track of what was decided earlier in the session.
allowed-tools: Bash
---

Extract recent assistant messages from this session's jsonl log to recover lost context.

## Steps

1. Find the current session's jsonl file:
```bash
ls -t ~/.claude/projects/$(pwd | sed 's|/|-|g; s|^-||')/*.jsonl 2>/dev/null | head -1
```

2. Extract the last 20 assistant text messages (decisions, summaries, proposals):
```bash
JSONL=$(ls -t ~/.claude/projects/$(pwd | sed 's|/|-|g; s|^-||')/*.jsonl 2>/dev/null | head -1)
python3 - "$JSONL" <<'EOF'
import json, sys
with open(sys.argv[1]) as f:
    lines = f.readlines()
msgs = []
for line in lines:
    try:
        d = json.loads(line)
        if d.get('type') == 'assistant':
            for block in d.get('message', {}).get('content', []):
                if block.get('type') == 'text' and block.get('text', '').strip():
                    msgs.append(block['text'])
    except:
        pass
for i, m in enumerate(msgs[-20:], 1):
    print(f"\n[{i}] " + "─"*60)
    print(m[:500])
EOF
```

3. Show the extracted messages to the user so they can identify the lost decision.

## Notes

- This recovers what the assistant said, not the full conversation
- Most useful for recovering design decisions, proposals, and conclusions
- If the session jsonl is not found, check: `ls ~/.claude/projects/`
