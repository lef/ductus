# Agent Collaboration Rules

## Available Sub-Agents

| Agent | Purpose | When to Use |
|---|---|---|
| planner | Implementation planning | Before complex features or refactors |
| architect | Design review | Architecture changes, key design decisions |
| tdd-guide | Test-driven development | New features, bug fixes |
| code-reviewer | Code review | After code changes |
| security-reviewer | Security analysis | Before commits with auth/permission changes |

## Auto-Launch (no user prompt needed)

1. **Complex feature request** → planner
2. **Code changes complete** → code-reviewer
3. **Security-relevant changes** → security-reviewer
   - Authentication / authorization changes
   - File permission or capability changes
   - Network configuration changes
   - Secrets handling changes
4. **Architecture proposal** → architect

## Passing Context to Sub-Agents

Before launching a sub-agent, always include existing design decisions in the prompt:

- **What already exists**: current file contents, previous decisions
- **What must NOT change**: constraints that were deliberately set (e.g., "no jq", "use $HOME not absolute path")
- **Why it was done that way**: rationale, so the agent doesn't "fix" intentional choices

> Without this context, sub-agents will apply generic best practices that may conflict
> with deliberate project decisions. The calling agent is responsible for providing context.

Example (bad):
> "Check the statusline config and fix any issues."

Example (good):
> "Check the statusline config. Existing decisions: (1) use grep not jq — shell-only principle,
> (2) use $HOME not absolute path — must work across environments. Fix only what is broken,
> do not change these."

## Parallel Execution

Run independent agents concurrently:

```
Good: Launch code-reviewer and security-reviewer in the same message

Bad: Run code-reviewer, wait for result, then run security-reviewer
```
