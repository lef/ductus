# Git Workflow

## Commit Message Format

```
<type>: <description>
```

Types: feat, fix, refactor, docs, test, chore, security

Examples:
- `feat: add retry logic to API client`
- `fix: quote variable in cleanup function`
- `docs: update HANDOFF.md`
- `test: add integration tests for auth flow`

## Implementation Workflow

1. **planner** agent: create implementation plan
2. **tdd-guide** agent: write tests first
3. Implement (minimum change to pass tests)
4. **code-reviewer** agent: review
5. **security-reviewer** agent: review if security-relevant
6. Commit

## Branch Strategy

- `main`: stable
- `master`: development default (if no main)
- `feature/*`: feature branches for larger changes

For solo development, small changes go directly to the default branch.

## Pre-Commit Checklist

- Tests pass
- No secrets in diff
- Consistent with design docs
