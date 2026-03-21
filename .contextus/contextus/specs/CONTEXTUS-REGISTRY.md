# CS-REGISTRY: Contextus Registry

> Type: SPEC
> Version: 0.1.0
> Updated: 2026-03-22
> Status: draft
> Provenance: L0/contextus

## Abstract

CS-REGISTRY defines the trust anchor for CS-MD and CS-INDEX vocabulary.
It is the single source of truth for header field names, document types,
section names, and inline markers used across the contextus ecosystem.

Analogous to [IANA](https://www.iana.org/) registries for Internet protocols,
CS-REGISTRY guarantees minimal identifiers so that all tools and agents
agree on the meaning of `> Type: HANDOFF` or `## Task`.

## Status of This Document

This is a draft specification.

## 1. Introduction

### 1.1 Motivation

When multiple layers (L0-L3), tools (Spec Kit, GSD), and agents (Claude,
Codex, Gemini) interact with CS-MD documents, they need to agree on what
field names and values mean. CS-REGISTRY provides this agreement.

### 1.2 Design Principles

- **Minimal identifiers guaranteed.** The registry defines the meaning of
  each field/section/marker. If it's in the registry, it's canonical.
- **AI-readable.** JSONL format — parseable by any LLM or program.
- **grep-friendly.** One entry per line. Category comments for human scanning.
- **Postel's Law.** Unknown entries are ignored. The registry is extensible.
- **No process overhead.** No formal registration process for now.
  L0 maintains core entries. L2/L3 add domain-specific entries with
  `registered_by` attribution.

### 1.3 Terminology

The key words "MUST", "MUST NOT", "SHOULD", and "MAY" are per RFC 8174.

## 2. Registry File

### 2.1 Location

```
registry.jsonl
```

This file is the trust anchor. It is maintained in the contextus (L0) repository.

### 2.2 Format

JSONL (JSON Lines): one JSON object per line.

Lines starting with `#` are comments (for human readability).
Parsers MUST skip lines starting with `#`.
Parsers MUST skip empty lines.

```jsonl
# --- header fields ---
{"kind":"header","field":"Type","level":"MUST",...}
# --- sections ---
{"kind":"section","field":"Task","level":"MUST","types":["HANDOFF"],...}
```

### 2.3 Sorting

Entries SHOULD be sorted by `kind` (header → section → marker),
then by `level` (MUST → SHOULD → MAY), then alphabetically by `field`.

## 3. Entry Schema

### 3.1 Common Fields

Every registry entry MUST include:

| Field | Type | Description |
|---|---|---|
| kind | enum: `header`, `section`, `marker`, `type` | Entry category |
| field | string | The identifier being registered |
| level | enum: `MUST`, `SHOULD`, `MAY` | Requirement level (RFC 8174) |
| description | string | Human-readable meaning |
| registered_by | string | Who registered this (e.g., `L0/contextus`, `L2/contextus-dev`) |

### 3.1.1 Per-Type Level Override

A section field MAY have different requirement levels for different document types.
This is expressed as multiple registry entries with the same `field` but different
`types` and `level` values.

The normative → informative gradient determines the default:

| Document category | Default level for optional fields |
|---|---|
| Normative (CONSTITUTION, SPEC, DESIGN) | MUST |
| Deliverable (TASK, TODO) | SHOULD / MAY |
| Informative (KNOWLEDGE) | SHOULD |
| Exploratory (DRAFT, PLAN) | MAY |
| Ephemeral (HANDOFF) | MAY |

Example: `References` section is MUST for normative types, SHOULD for
informative, MAY for exploratory. This is expressed as three registry entries
with `field: "References"` at different levels for different `types` arrays.

### 3.1.2 Why Multiple Lines Per Field (Design Rationale)

Three formats were considered for per-type level overrides:

| Format | Example | Pros | Cons |
|---|---|---|---|
| **A: Multiple lines** (chosen) | 3 lines with same `field`, different `types`/`level` | Each line is self-contained. Read one line → know the behavior. grep-friendly | Duplicate field names |
| **B: Nested JSON** | `{"field":"References","levels":{"CONSTITUTION":"MUST","DRAFT":"MAY"}}` | 1 field = 1 line | Nested JSON, harder to grep |
| **C: Gradient lookup** | `{"field":"References","level":"gradient"}` | Minimal registry | Must read spec to know behavior. Format doesn't show behavior |

**Format A was chosen because the format itself shows the behavior.**
You can read one line of registry.jsonl and know "References is MUST for
CONSTITUTION, SPEC, DESIGN" without consulting any other document.

This follows the CS-MD design principle: **self-describing**.
If registry entries are not self-describing, they violate the same principle
that CS-MD headers are designed to uphold.

### 3.2 Header-Specific Fields

| Field | Type | Description |
|---|---|---|
| format | string | Value format (e.g., `ISO 8601`, `SemVer`, `enum`) |
| values | string[] | Allowed values (for enums) |
| reference | string | External standard this field aligns with |

### 3.3 Section-Specific Fields

| Field | Type | Description |
|---|---|---|
| types | string[] | Which document types this section applies to (`["HANDOFF"]`, `["*"]`) |

### 3.4 Marker-Specific Fields

| Field | Type | Description |
|---|---|---|
| types | string[] | Which document types this marker is allowed in |

## 4. Registration

### 4.1 Core Entries (L0)

Core entries are maintained by the contextus (L0) repository maintainers.
These define the CS-MD baseline that all projects share.

### 4.2 Domain Extensions (L2/L3)

L2 and L3 layers MAY register additional entries with `registered_by`
set to their layer identifier (e.g., `L2/contextus-dev`).

Extensions MUST NOT redefine core entries. If a field name conflicts
with an existing registration, the earlier registration wins
(first-come, first-served).

### 4.3 Deprecation

To deprecate an entry, add `"deprecated": true` and `"deprecated_by": "..."`.
Deprecated entries MUST NOT be removed from the registry (Postel's Law:
existing documents may still use them).

## 5. Reading the Registry

### 5.1 For Programs

```bash
# All MUST header fields
grep '^{' registry.jsonl | grep '"kind":"header"' | grep '"level":"MUST"'

# All sections for HANDOFF
grep '^{' registry.jsonl | grep '"kind":"section"' | grep '"HANDOFF"'
```

### 5.2 For AI Agents

Read `registry.jsonl` at session start or when generating CS-MD
documents. Use MUST fields to ensure validity. Use MAY fields as available.

### 5.3 For Humans

Comment lines (`#`) group entries by category for visual scanning.
The JSONL itself is not designed for human editing — modify via
tooling or careful line editing.

## 6. Relationship to Other Specs

```
CS-MD    → defines document format, references CS-REGISTRY for valid values
CS-INDEX → defines index format, references CS-REGISTRY for field names
CS-REGISTRY → defines the identifiers (this document)
```

CS-REGISTRY is independent. CS-MD and CS-INDEX depend on it.

## 7. Security Considerations

- The registry is the trust anchor. Tampering with it changes the meaning
  of all CS-MD documents. It SHOULD be git-tracked and changes reviewed.
- `registered_by` provides attribution but not authentication.
  Git commit signatures are the appropriate authentication mechanism.

## References

- [CS-MD](CS-MD.md) — Document format specification
- [CS-INDEX](CS-INDEX.md) — Index specification
- [IANA](https://www.iana.org/) — Internet Assigned Numbers Authority (model)
- [RFC 8174](https://datatracker.ietf.org/doc/html/rfc8174) — RFC 2119 Keywords

## Authors

This specification was developed through collaborative sessions between
a human (HTTP/1.1bis RFC contributor) and AI agents (Claude Opus 4.6).
