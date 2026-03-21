# TDD — Test-Driven Development Rules

## The Technique: Use t_wada's Name

When instructing Claude to do TDD, say **"t_wada の推奨する進め方に従って"** instead of just "TDD".
Person names anchor the concept and dramatically improve Claude's instruction-following accuracy.

Similarly:
- Refactoring → **"Martin Fowler のRefactoring"**
- Small cleanups → **"Kent Beck のTidyings"**
- SOLID → **"Robert C. Martin のSOLID原則"**

Ref: https://memory-lovers.blog/entry/2025/06/27/102550

## t_wada's TDD Cycle (RED → GREEN → REFACTOR)

1. **RED**: Write a failing test first. Run it. Confirm it fails.
2. **GREEN**: Write the minimum code to make it pass. Nothing more.
3. **REFACTOR**: Clean up while keeping tests green.

Never skip RED. A test that was never seen failing proves nothing.

## Rules

- **Test first, always** — no implementation before a failing test exists
- **One failing test at a time** — don't write multiple failing tests ahead
- **Minimum code for GREEN** — resist the urge to over-implement
- **Refactor only on GREEN** — never refactor on RED

## Mandatory Checkpoint Before Implementation

Before writing ANY implementation code, perform this check:

1. Does a failing test exist for the behavior I'm about to implement?
2. Have I run the tests and seen them FAIL (RED)?
3. Have I noted the RED output (shown to human or recorded in response)?

If the answer to any of these is "no" → **stop and write the test first**.

This is mechanical, not a judgment call. "This seems trivial" is not grounds to skip.
Self-judgment that leads to skipping TDD is the most common failure mode.

**Self-check:**
> "Do I have a failing test for this? If not, I must write one before proceeding."

## When to Use tdd-guide Agent

Always use the `tdd-guide` sub-agent when:
- Adding a new feature
- Fixing a bug (write a test that reproduces it first)
- Any change that touches business logic

## Exceptions

- Pure config/documentation changes: skip
- Trivial one-liners with no logic: **confirm with human first, do not self-judge**
- Explicit human override: "no tests needed" / "直接実装して"
