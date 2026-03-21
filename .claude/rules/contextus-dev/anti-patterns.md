# Anti-Pattern Prevention Rules (Development)

> L2-dev: extends L0 `anti-patterns.md` with development-specific rules.

## Completion Integrity

- Never claim "done" or "complete" unless ALL tests pass.
- After implementation, always run the test suite and report actual results.
- If tests fail, report the failures honestly — do not mark the task as complete.
- Include actual test output in your completion report.

## Drift Prevention (Development)

- Linter and type checker results override your confidence — fix all reported issues.
