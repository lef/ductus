# .spec/ — Spec-Driven Development Scaffold

This folder manages the lifecycle of features and tasks.

## Workflow

```
Human writes PLAN.md
    ↓
Agent asks clarifying questions
    ↓
Agent writes SPEC.md (or DESIGN.md)
    ↓
Human confirms
    ↓
Agent writes TODO.md
    ↓
Implementation
    ↓
Discoveries → KNOWLEDGE.md
```

Do not start implementation until TODO.md is confirmed by the human.

## File Roles

| File | Author | Content |
|---|---|---|
| `PLAN.md` | Human | Free-form intent. Rough is fine. |
| `SPEC.md` | Agent | Structured requirements. For code/systems. |
| `DESIGN.md` | Agent | Structured plan. For documents/research. |
| `TODO.md` | Agent | Checkbox task list, derived from SPEC/DESIGN. |
| `KNOWLEDGE.md` | Agent | Discoveries, decisions, rationale. Accumulates. |

## Variants

**Software / System Development** (`SPEC.md`):
> PLAN → SPEC → TODO → KNOWLEDGE

**Research / Knowledge Work** (`DESIGN.md`):
> PLAN → DESIGN → TODO → KNOWLEDGE

Use `SPEC.md` when the output is running code or a system.
Use `DESIGN.md` when the output is a document, analysis, or paper.

The folder name is `.spec/` regardless of variant.

## Multiple Features

For large projects with multiple concurrent features, name files with a prefix:

```
.spec/
├── auth-PLAN.md
├── auth-SPEC.md
├── auth-TODO.md
└── KNOWLEDGE.md       ← shared across features
```
