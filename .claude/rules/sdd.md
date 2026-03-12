# Spec-Driven Development — Core Rules

Agent-agnostic SDD rules. Language- and tool-specific extensions belong in L2+ layers.

## New Repository Rule

**When creating a new repository, always record WHY in `.spec/PLAN.md` upfront.**

The first section of PLAN.md must be:

```markdown
## なぜ作るのか（本来のビジョン）

[Long-term motivation, problem to solve, how far this should eventually go]

## フェーズ0 / Phase 0 (current scope)

[What we're building now, and why this narrower scope first]
```

**Why this matters**: The original motivation disappears once implementation begins.
After Phase 0 is done, the "why" is the only guide for what to build next.
Git log and HANDOFF.md do not preserve motivation — only PLAN.md does.
