# CS-INDEX: Contextus Structured Markdown Index

> Type: SPEC
> Version: 0.1.0
> Updated: 2026-03-21
> Status: draft
> Provenance: L0/contextus

## Abstract

CS-INDEX defines a JSONL-based index format for CS-MD documents.
The index provides machine-readable lookup, bidirectional reference
tracking, and implicit graph structure without requiring a database.

CS-INDEX は CS-MD 文書の JSONL ベースインデックス形式を定義する。
データベースなしで検索、双方向参照追跡、暗黙的グラフ構造を提供する。

## Status of This Document

This is a draft specification, dependent on CS-MD (specs/CS-MD.md).

## 1. Introduction

### 1.1 Relationship to CS-MD

CS-INDEX depends on CS-MD. CS-MD does NOT depend on CS-INDEX.

```
CS-MD (independent) ← CS-INDEX (depends on CS-MD)
```

CS-MD documents are the source of truth. CS-INDEX is a **derived cache**
— it can be regenerated at any time from the source .md files.

### 1.2 Two Indexes: Machine and Human

A project has **two** indexes for its CS-MD documents:

| Index | Format | Generated | Audience | Role |
|---|---|---|---|---|
| INDEX.jsonl | JSONL | auto-generated from CS-MD headers | machine | Search, graph, validation |
| KNOWLEDGE.md | CS-MD | human-curated | human + LLM | Quick capture inbox, importance-ranked entry list |

INDEX.jsonl is the machine index — complete, mechanical, regenerable.
KNOWLEDGE.md is the human index — curated, editorial, importance-ranked.

Both are derived from the individual CS-MD files in `knowledge/`,
but they serve different purposes and different audiences.

KNOWLEDGE.md is itself a CS-MD document (`> Type: KNOWLEDGE`).
It contains:
- **Quick Notes** section: rapid capture of small findings (inbox)
- **Entries** section: curated links to individual `knowledge/*.md` files (index)

When a Quick Note matures, it is promoted to an individual CS-MD file.
When it becomes stale, it is archived or deleted.

### 1.3 Design Principles

- **Cache, not source of truth.** INDEX.jsonl is derived from .md files.
  If it breaks, regenerate it (`make index`).
- **grep-friendly.** JSONL (one JSON object per line) is greppable.
- **No database.** For project-scoped knowledge (< 10,000 entries),
  flat files + grep are sufficient. SQLite is the escalation path.
- **Postel's Law.** Unknown fields in index entries MUST be ignored.

## 2. File Format

### 2.1 Location

```
$FLOW_DIR/INDEX.jsonl
```

The file MAY be git-tracked or gitignored (it is a regenerable cache).

### 2.2 Format

One JSON object per line (JSONL). Each line represents one CS-MD document
**or one section within a multi-entry document** (see §2.4).

```jsonl
{"id":"001","file":"knowledge/001-fetch-mcp.md","title":"fetch MCP e2e","tags":["finding","fetch-mcp"],"provenance":"tutus/master","context":"MINUTES:2026-03-21#fetch","refs_out":["CONSTITUTION#security"],"refs_in":["003-audit"],"updated":"2026-03-21","status":"active"}
```

### 2.3 Fields

#### MUST Fields

| Field | Type | Description |
|---|---|---|
| id | string | Unique identifier (number prefix or slug) |
| file | string | Relative path to the .md file, optionally with `#heading` anchor |
| title | string | H1 title of the document, or H2 title of the section |
| updated | string | ISO 8601 date from CS-MD header |

#### SHOULD Fields

| Field | Type | Description |
|---|---|---|
| tags | string[] | From CS-MD `> Tags:` header |
| status | string | From CS-MD `> Status:` header |
| refs_out | string[] | Documents this entry references (outgoing edges) |
| refs_in | string[] | Documents referencing this entry (incoming edges, derived) |

#### MAY Fields

| Field | Type | Description |
|---|---|---|
| provenance | string | From CS-MD `> Provenance:` header |
| context | string | From CS-MD `> Context:` header (link to MINUTES) |
| description | string | From CS-MD `> Description:` header |
| class | string | KNOWLEDGE class: `decision`, `finding`, or `lesson` |

### 2.4 Sub-Document Indexing (#heading References)

A single CS-MD file MAY contain multiple entries as H2 sections
(e.g., weekly findings grouped in one file). INDEX.jsonl can reference
individual sections using `#heading` anchors in the `file` field.

```jsonl
{"id":"w3-001","file":"knowledge/2026-03-w3.md#bwrap-path","title":"bwrap PATH に /sbin 必要","tags":["finding","bwrap"],...}
{"id":"w3-002","file":"knowledge/2026-03-w3.md#socat-timeout","title":"socat -t120 タイムアウト","tags":["finding","socat"],...}
```

**Rules for sub-document indexing:**

- `file` field includes `#heading-slug` (GitHub-style heading anchor)
- `title` is the H2 heading text, not the H1
- `tags` MAY be document-level (from `> Tags:` header) or entry-specific
- `id` is unique per entry, not per file
- Standard markdown `[text](file.md#heading)` links can point to individual entries
- `#heading` anchors work in GitHub, Obsidian, and most markdown renderers

**When to use sub-document indexing:**

- Small findings that don't warrant individual files (2-3 lines each)
- Grouped by time (weekly), context (session), or topic
- The document itself is CS-MD compliant (H1 + header block)
- INDEX provides per-entry searchability despite grouping

## 3. Generation

### 3.1 Source Extraction

INDEX.jsonl is generated by reading CS-MD headers and body references:

```bash
for f in "$FLOW_DIR"/knowledge/*.md "$FLOW_DIR"/DRAFT-*.md; do
    [ -f "$f" ] || continue
    # Extract > header fields
    # Extract [text](path) references → refs_out
    # Output JSONL line
done
# Derive refs_in by inverting all refs_out
```

### 3.2 Regeneration

```bash
make index    # regenerate INDEX.jsonl from .md files
```

If INDEX.jsonl is missing or corrupt, regeneration recovers all data.
No information is lost because the .md files are the source of truth.

## 4. Graph Structure

### 4.1 Implicit Graph

The combination of `refs_out` and `refs_in` defines a directed graph:

- **Nodes**: CS-MD documents (each JSONL entry)
- **Outgoing edges**: `refs_out` (what this document references)
- **Incoming edges**: `refs_in` (what references this document)

No explicit graph database is needed. The graph is implicit in the index.

### 4.2 Bidirectional Lookup

```bash
# What does entry 002 reference?
grep '"002"' INDEX.jsonl    # read refs_out

# What references entry 002?
grep '"002"' INDEX.jsonl    # read refs_in (or grep all refs_out)
```

### 4.3 Provenance Chains

Provenance forms a DAG (directed acyclic graph):

```
KNOWLEDGE: "gh OAuth fallback"
  ← MINUTES: 2026-03-21 (context)
  ← CONSTITUTION: security (refs_out)
  ← KNOWLEDGE: "FG-PAT setup" (refs_out)
```

## 5. KNOWLEDGE-Specific Features

### 5.1 Three-Class System

KNOWLEDGE entries are classified by the `class` field:

| Class | Stability | Archive Strategy |
|---|---|---|
| decision | High (ADR-like) | Archive directly |
| finding | Low (may stale) | Needs validity check |
| lesson | High (transferable) | L0/L2 promotion candidate |

### 5.2 MINUTES Linkage

The `context` field links a KNOWLEDGE entry to the MINUTES discussion
that produced it. This provides the "WHY" behind decisions without
bloating the KNOWLEDGE entry itself.

## 6. Security Considerations

- INDEX.jsonl is a cache. Tampering with it does not affect source of truth.
- Regeneration from .md files is the recovery mechanism.
- Graph traversal should not follow external URLs automatically
  (prompt injection risk via fetched content).

## References

- [CS-MD](CS-MD.md) — The document format this index is built on
- [JSONL](https://jsonlines.org/) — JSON Lines format
- [ADR](https://adr.github.io/) — Architecture Decision Records (KNOWLEDGE class inspiration)

## Authors

This specification was developed through collaborative sessions between
a human (HTTP/1.1bis RFC contributor) and AI agents (Claude Opus 4.6).
