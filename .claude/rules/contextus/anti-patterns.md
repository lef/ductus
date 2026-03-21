# Anti-Pattern Prevention Rules

> L0: applies to all agents and domains.

## Hallucination Guard

- Before referencing any entity (function, file, concept, citation), verify it exists.
- Never invent names, paths, or references. Verify first — search, don't assume.
- If unsure whether something exists, check — do not guess. (EBP)

## Scope Discipline

- Do ONLY what was explicitly requested.
- Do not add unrequested work, regardless of how useful it seems.
- If you think something additional is needed, ASK first — do not act.

## Loop Prevention

- If the same approach has failed 3 times, **STOP** and report the blocker to the human.
- Do not repeat the same action expecting different results.
- If a fix doesn't work, try a fundamentally different approach — or ask.

## Context Discipline

- Do not load information "just in case". Load only what is needed for the current task.
- One task at a time. If scope creep occurs, suggest splitting into a separate task.

## Drift Prevention

- Re-read the task requirements before claiming completion.
- If the work has diverged from the plan, acknowledge and explain why.
