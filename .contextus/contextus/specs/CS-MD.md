# CS-MD: Contextus Structured Markdown

> Type: SPEC
> Version: 0.1.0
> Updated: 2026-03-21
> Status: draft
> Provenance: L0/contextus

## Abstract

CS-MD (Contextus Structured Markdown) is a document specification for
markdown files used in AI-assisted collaborative work. It defines header
fields, section names, and inline markers that give markdown documents
machine-parseable semantics while remaining human-readable.

CS-MD is not a new format — it IS markdown. It is not a protocol —
how documents are exchanged is defined elsewhere. It is not a schema —
what makes a document valid is defined by CS-SCHEMA.

CS-MD は AI 協業で使う markdown 文書の仕様。header fields, section names,
inline markers を定義し、markdown 文書に機械可読な意味論を与える。
新しいフォーマットではない（markdown そのもの）。プロトコルでもスキーマでもない。
文書の書き方を定義する。妥当性は CS-SCHEMA が定義する。

### Related Specifications

| Spec | Role |
|---|---|
| **CS-MD** (this document) | Document specification: how to write |
| [CS-SCHEMA](CS-SCHEMA.md) | Validation schema: what makes it valid |
| [CS-INDEX](CS-INDEX.md) | Index: how to search and link documents |
| [CONTEXTUS-REGISTRY](CONTEXTUS-REGISTRY.md) | Registry: trust anchor for identifiers |

## Status of This Document

This is a draft specification. It is subject to change based on
implementation experience (dogfooding in tutus, faber, and other
contextus-based projects).

## 1. Introduction

### 1.1 Motivation

There is no universal structured markdown body specification.
Multiple tools (Spec Kit, GSD, OpenSpec, AGENTS.md) have invented
their own conventions independently, creating silos.
(Surveyed 2026-03-21: 11 projects, none universal.)

CS-MD fills this gap with a minimal, grep-friendly convention.

> "Tool calling was designed for machines but we had to teach it to them.
> Markdown was designed for humans and they already knew it."
> — Fabian Kubler

### 1.2 What CS-MD Is

CS-MD makes markdown files **self-describing** and **self-contained**.
A regular markdown file is just content.
A CS-MD file knows WHAT it is (Type), WHERE it came from (Provenance),
WHEN it changed (Updated), and WHAT it relates to (Tags).

CS-MD is the **HTTP headers of markdown** — metadata that describes the payload.

CS-MD is also the **microformats of markdown**. Just as microformats (2004)
and microdata (2009) added semantic conventions to HTML without changing
the HTML format, CS-MD adds semantic conventions to markdown without
changing the markdown format. Parsers that don't understand CS-MD
still render the document normally (graceful degradation).

```
HTML  + microformats (2004) = structured HTML (class="vcard")
HTML  + microdata (2009)    = structured HTML (itemscope, itemprop)
HTML  + JSON-LD             = structured HTML (<script type="application/ld+json">)
Markdown + CS-MD (2026)     = structured Markdown (> Type:, ## Task)
```

Any markdown file can become a CS-MD document by adding `> Type:`.
CS-MD is not limited to Structured Flow documents.
README, AGENTS.md, rule files — all can be CS-MD if they opt in.

### Why Self-Containment Matters for LLMs

AI agents lose context between sessions. When an agent reads a CS-MD file,
it gets **everything it needs** from that single file:

- **Type** tells the agent what role this document plays
- **Tags** tell the agent what topics are covered and which other documents are related
- **Provenance** tells the agent where this document came from and how trustworthy it is
- **Context** tells the agent where the discussion behind this document happened

Without CS-MD, an agent must read multiple files, run git commands, and infer
relationships from prose — consuming context budget and risking errors.

**CS-MD makes each file a self-contained unit of knowledge that an agent
can understand without external context.** This is not optional for
multi-agent orchestration where sub-agents receive only focused task prompts,
not full project context.

### 1.3 Scope and Prerequisites

Prerequisites:

- **MUST**: Documents are managed in a **git** repository
  (Provenance uses git hashes, references rely on git rename tracking,
  file size and history are queried from git — NOT from filesystem)
- **SHOULD**: The project follows Structured Flow for work artifacts
  (PLAN, SPEC, TODO, TASK, KNOWLEDGE, DRAFT)
- **MAY**: The contextus layer system (L0-L3) is in use

CS-MD depends on **git** and **markdown parsers** (grep/awk).
CS-MD does NOT depend on any specific filesystem, OS, or tool.

### 1.4 What CS-MD Is NOT

- **Not a new document format.** CS-MD IS markdown.
- **Not a protocol.** How documents are exchanged is defined elsewhere.
- **Not a schema.** What makes a document valid is defined by CS-SCHEMA.
- **Not general-purpose.** It assumes git-managed projects.

### 1.5 Philosophy

#### Why markdown?

Markdown is LLM-friendly **not because it is a good specification, but because
LLMs are trained on massive amounts of it** (GitHub, documentation, READMEs).

```
English is human-friendly  ← not because the grammar is clean,
                              but because humans grew up with it
Markdown is LLM-friendly   ← not because the spec is unambiguous,
                              but because LLMs grew up on GitHub
```

In fact, markdown is a notoriously difficult format to parse. The original
specification (John Gruber, 2004) was informal prose, not a formal grammar.
CommonMark (2014) attempted to formalize it but the spec is 600+ pages with
hundreds of edge cases. Different parsers produce different output for the
same input. Writing a fully correct markdown parser is considered one of the
harder parsing challenges in practice.

**CS-MD does not depend on markdown being a good specification.**
CS-MD uses a **safe subset** of markdown that is believed to be unambiguous.

> **WARNING: This safe subset analysis is INCOMPLETE (TBD).**
> The table below is a preliminary assessment. A thorough analysis against
> MacFarlane's 6 pain points (emphasis, indented code, raw HTML, link syntax,
> list item membership, block-level interruption) and CommonMark's 600+ edge
> cases is required before this section can be considered final.
> Markdown is not context-free — there may be ambiguities in constructs
> we currently assume are safe. Evidence (testing against multiple parsers)
> is needed. See KNOWLEDGE for the full parser difficulty survey.

| CS-MD uses | Ambiguous? | Why safe |
|---|---|---|
| `# Title` (H1) | No | One per document, first heading |
| `> key: value` (blockquote) | No | grep `'^>'` — no parser needed |
| `## Section` (H2) | No | Flat structure, no nesting |
| `- item` (list) | Edge cases exist | CS-MD uses simple flat lists only |
| `[text](path)` (link) | No | Standard syntax, well-defined |

CS-MD avoids: deeply nested lists, indentation-sensitive constructs,
HTML-in-markdown, complex table formatting, and other ambiguous areas.

**The design principle**: use markdown because LLMs already know it,
but restrict to the subset that is unambiguous and grep-parseable.
Do not depend on a markdown parser for CS-MD header extraction.

#### Visible metadata

Unlike HTML's `<meta>` tags (hidden from users) or JSON-LD `<script>` blocks
(invisible), CS-MD metadata is **visible**. `> Type: HANDOFF` renders as a
blockquote that humans can read.

This is a deliberate choice, not a limitation. In the world of AI-assisted work,
**both humans and agents read the same document**. Hidden metadata creates
a split: humans see one thing, machines see another. CS-MD rejects this split.
The metadata IS content. What agents see, humans see. What humans write,
agents read.

#### Documents as the interface

CS-MD believes that **documents are the interface between humans and AI agents**.
Not APIs. Not tool calls. Not chat messages. Documents.

- A HANDOFF.md is how an agent tells the next agent (or human) what happened
- A TASK.md is how a human (or orchestrator) tells an agent what to do
- A KNOWLEDGE.md is how discoveries persist across sessions and agents

These documents must be readable by the smallest agent (4K context) and the
largest (1M+ context). They must be writable by humans in a text editor and
by agents through file operations. They must survive git push, pull, merge,
and decades of format changes.

Markdown — despite its flaws — is the only format that meets all these
requirements today. CS-MD makes it work by staying in the safe subset
and adding just enough structure to be machine-useful.

#### The structure spectrum is intentional

Every CS-MD header field has a declared structure level (§3.0):
structured, semi-structured, or free-form. This is not accidental.

HTTP learned this lesson the hard way: `User-Agent` was free-form for 25 years,
became unparseable, and had to be replaced with Structured Fields (RFC 8941).
CS-MD declares structure level on day one, so there is no future pressure
to "fix" a field's format. Free-form fields (Context, Description) are
**intentionally** free-form. Their value is in human readability. Forcing
structure on them would make them harder to write and no more useful to
machines (which can read the structured fields instead).

### 1.6 Design Principles

#### Foundational (the 4 pillars)

These four principles define what CS-MD IS. They are not optional guidelines —
they are the architectural decisions that make CS-MD work.

1. **Semantic conventions on an existing format.**
   CS-MD adds meaning to standard markdown. No new syntax is invented.
   `> Type:` is a standard blockquote. `## Task` is a standard heading.
   The MEANING is new. The FORMAT is not. (Microformats principle, 2004.)

2. **Graceful degradation.**
   A parser that does NOT understand CS-MD still renders the document normally.
   `> Type: HANDOFF` renders as a blockquote. `## Task` renders as a heading.
   Nothing breaks. Agents that don't know CS-MD read it as regular markdown.

3. **Postel's Law (Jon Postel, RFC 761).**
   Be conservative in what you send, be liberal in what you accept.
   Unknown header fields MUST be ignored, not rejected.
   New extensions MUST NOT break existing parsers.
   This is how CS-MD stays extensible without coordination between tools.

4. **Registry-managed vocabulary.**
   Field names, section names, and markers are registered in a central
   trust anchor (`registry.jsonl`). This guarantees that `> Type: HANDOFF`
   means the same thing everywhere. (IANA principle.)

#### Derived

- **grep/awk parseable.** No special parser required. `grep '^>' file.md` extracts all headers.
- **EBP (Evidence-Based Practice).** This spec is validated by running code (dogfooding), not by reasoning.
- **Self-describing.** Each file carries its own metadata. No external lookup needed to understand what it is.
- **Self-contained.** Each file carries enough context for an agent to act on it without reading other files.

### 1.3 Terminology

The key words "MUST", "MUST NOT", "SHOULD", "SHOULD NOT", and "MAY"
in this document are to be interpreted as described in RFC 8174
when, and only when, they appear in ALL CAPITALS.

## 2. Document Structure

### 2.1 Metadata and Content

A CS-MD document has two layers, analogous to HTML's `<head>` and `<body>`:

```
HTML:                          CS-MD:
  <head>                         > Type: HANDOFF        ← metadata (header block)
    <title>Page</title>          > Updated: 2026-03-21
    <meta name="author"...>      > Tags: finding, mcp
  </head>                        # Document Title       ← title (serves both roles)
  <body>                         ## Section Name         ← content (body)
    <h1>Page</h1>                (markdown body)
    <p>content</p>
  </body>
```

**Key difference from HTML**: CS-MD metadata is **visible**.
HTML's `<meta>` is hidden from users. CS-MD's `> key: value` renders as a
blockquote — humans read it naturally. This is a feature, not a limitation.
It follows from Pillar 2 (graceful degradation): the metadata IS content
when rendered by a non-CS-MD parser.

`# Title` serves both `<title>` (metadata: document name) and `<h1>`
(content: first heading). In markdown there is no separate head/body split,
so one element serves both roles.

### 2.2 Parts of a CS-MD Document

1. **Title** (H1, exactly one)
2. **Header** (blockquote metadata lines)
3. **Sections** (H2 headings, some fixed per document type)
4. **Body** (standard markdown within sections)

```markdown
# Document Title

> Type: HANDOFF
> Updated: 2026-03-21

## Section Name
(body content)
```

## 3. Header Fields

Header fields are encoded as blockquote lines (`> Key: Value`).
They are extractable with `grep '^>' document.md`.

### 3.0 Metadata Structure Spectrum

Header fields have different levels of structure, analogous to HTTP headers:

```
              HTTP header              CS-MD header
Structured:   Content-Type: text/html  > Type: HANDOFF           ← enum, machine parses directly
              Last-Modified: <date>    > Updated: 2026-03-21     ← ISO 8601, machine parses
              Via: proxy1, proxy2      > Provenance: x@m:abc1234 ← identity format, machine resolves

Semi-struct:  Accept-Language: ja,en   > Tags: finding, fetch-mcp ← comma list, grep searchable
              Cache-Control: no-cache  > Status: draft            ← enum, but simpler

Free-form:    Server: Apache/2.4       > Context: MINUTES 2026-03-21 — fetch MCP の議論
                                       > Description: sandbox 内から fetch MCP で URL 取得を実証
```

**Structured** fields have machine-parseable formats (enums, dates, identity syntax).
Parsers can validate and process them mechanically.

**Semi-structured** fields have conventions (comma-separated, enum-like) but
are primarily for search and filtering, not mechanical processing.

**Free-form** fields are human-readable descriptions. They are grep-searchable
but not mechanically parsed. Their value is in human understanding, not machine processing.

**Design rationale**: this spectrum exists because metadata serves two audiences.
Machine-oriented fields (Type, Updated, Provenance) need strict formats for
tooling to work. Human-oriented fields (Context, Description) need to be
readable and writable without consulting a format spec. Forcing strict format
on human fields reduces usability. Allowing free-form on machine fields
breaks tooling. The spectrum balances both.

**Lesson from HTTP**: HTTP's `User-Agent` header was historically free-form
(`Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/...`), making it
notoriously difficult to parse. IETF developed Structured Fields (RFC 8941)
and Client Hints (RFC 8942) to replace free-form headers with structured ones.
CS-MD avoids this mistake by **declaring structure level upfront** for each field.
Fields that are free-form (Context, Description) are intentionally so — there
will be no future pressure to "structure" them, because the spec explicitly
documents that their value is in human readability, not machine parsing.

### 3.1 MUST Fields

Every CS-MD document MUST include:

| Field | Format | Structure | Semantics |
|---|---|---|---|
| Type | enum (see §4) | structured | Document type identifier |
| Updated | ISO 8601 (`YYYY-MM-DD` or `YYYY-MM-DDTHH:MM`) | structured | Last modification timestamp |

### 3.2 MAY Fields

The following fields MAY appear. Parsers MUST NOT fail on their absence.

| Field | Format | Structure | Semantics |
|---|---|---|---|
| Version | SemVer `X.Y.Z` | structured | Document version. Major = breaking section change |
| Provenance | identity or chain (see §3.4) | structured | Origin and lineage |
| Status | enum: `draft`, `discussion`, `in_progress`, `confirmed`, `archived` | semi-structured | Lifecycle state |
| Tags | comma-separated | semi-structured | Classification, search, and **implicit document relationships** (see §3.5) |
| Context | human-readable description | free-form | Where the discussion behind this document happened |
| Description | single line | free-form | Short summary for indexing |

### 3.2.1 Context Field

Context links a document to the discussion that produced it (provenance of reasoning).

**Context is free-form, NOT a resolvable path.** It is metadata for humans:

```markdown
> Context: MINUTES 2026-03-21 — fetch MCP sandbox 化の議論
> Context: Slack #arch-review 2026-03-20
> Context: PR #42 review comments
```

Machine-resolvable links to specific discussions belong in `## References`,
not in the Context header. Context tells a human "where to look."
References tells a machine "what to fetch."

This separation follows the metadata structure spectrum:
Context = free-form (human), References = file-relative path (machine).
| Description | single line | Short summary for indexing |

### 3.4 Provenance Format

Provenance identifies WHERE a document comes from and THROUGH WHICH layers.

**Identity format** (single origin):

```
repo@branch:short-hash
```

Example: `contextus@main:abc1234`

- `repo`: git repository name (human-readable locator)
- `branch`: git branch name
- `short-hash`: 7-character git commit hash (as in `git log --oneline`)

The short hash is usually unique within a project ecosystem.
Full SHA is NOT included in CS-MD headers (context conservation).
Tools that need the full hash resolve via `git log <short-hash> --format=%H`.

**Chain format** (multi-layer origin, most specific → most general):

```
repo@branch:hash-->repo@branch:hash-->repo@branch:hash
```

Example: `contextus-dev-sh@main:abc1234-->contextus-dev@main:def5678-->contextus@main:ghi9012`

Chain delimiter: `-->` (ASCII, mermaid flowchart compatible, grep-friendly).
Direction: left = most specific (L3), right = most general (L0).
No spaces around `-->`.

**Design rationale**:

- Hash is the identifier (globally unique via SHA-1/SHA-256)
- repo@branch is the locator (where to find it, human context)
- Short hash balances human readability and machine precision
- git is the resolver for full hash when needed
- Chain uses `-->` from mermaid graph syntax for familiarity

### 3.5 Tags as Implicit Relationships

Tags serve a dual role: classification AND implicit document relationships.

Documents sharing tags are implicitly related — without explicit links.
This is **editorial judgment** (curated by human/agent), not mechanical extraction.

```
Document A: > Tags: security, gh-auth
Document B: > Tags: security, oauth
→ Shared tag "security" → implicitly related
```

**Why Tags replace See-Also:**

- See-Also (explicit link to specific docs) requires maintenance
- Tags (shared vocabulary) create relationships that **emerge** from the vocabulary
- INDEX.jsonl can compute relatedness from shared tags
- No new header field needed — Tags already registered as MAY

**Tag vocabulary is not centrally controlled.** Tags are free-form.
Projects SHOULD develop consistent tag conventions over time.
KNOWLEDGE entries SHOULD use class tags: `decision`, `finding`, `lesson`.

### 3.6 Rejected Header Fields

The following fields were considered and rejected (YAGNI):

| Field | Rejected because |
|---|---|
| Lines / Size | git provides this (`git show HEAD:file \| wc -l`). No FS dependency |
| Valid-Until | git history provides freshness. Manual expiry dates rot |
| Priority | Manual priority is a dirty hack. Importance emerges from graph (refs_in count) |
| See-Also | Tags provide curated implicit relationships. Explicit links are in body text |
| Language | Modern LLMs detect language from content. Project context disambiguates |

These can be reconsidered if a concrete use case emerges (Postel's Law: adding later is not breaking).

### 3.7 Extensibility

Parsers MUST ignore unknown header fields (Postel's Law).
Adding a new MAY field is NOT a breaking change (no major version bump).

Domain-specific fields (L2/L3) MAY be added.
The `registry.jsonl` file serves as the IANA-style trust anchor
for all registered field names.

## 4. Document Types

### 4.1 Classification

```
Constraints:   CONSTITUTION          — normative (absolute rules)
Deliverables:  PLAN → SPEC → TODO → TASK  — Structured Flow artifacts
References:    DRAFT ↔ KNOWLEDGE     — informative (symmetric pair)
State:         HANDOFF               — session state (ephemeral)
```

### 4.1.1 Normative vs Informative — Requirements Gradient

Document types form a gradient from normative (binding) to informative (contextual).
This gradient determines how strictly optional fields (like References) are required.

| Category | Types | Nature | References | [NEEDS CLARIFICATION] |
|---|---|---|---|---|
| **Normative** | CONSTITUTION, SPEC, DESIGN | Binding. Must be followed | **MUST** | Must be resolved before confirmation |
| **Deliverable** | TASK, TODO | Actionable. Must be executable | **SHOULD** (TASK) / MAY (TODO) | Must NOT remain in TASK |
| **Informative** | KNOWLEDGE (individual file) | Verified facts. Trusted reference | **SHOULD** | Must NOT remain |
| **Exploratory** | DRAFT, PLAN | Unresolved. Work in progress | **MAY** | MAY remain |
| **Ephemeral** | HANDOFF | Session state. Short-lived | **MAY** | Must NOT remain |

**Design principle**: normative documents MUST cite their sources (References).
Informative documents SHOULD. Exploratory and ephemeral documents MAY.

This gradient applies to ALL optional fields and sections — not just References.
When adding a new MAY field to the registry, its requirement level per document
type SHOULD follow this gradient.

### 4.2 Type Registry

| Type | Role | MUST sections | File location |
|---|---|---|---|
| HANDOFF | session state | ## Task, ## Context | project root |
| TODO | backlog | phase/category H2s | $FLOW_DIR/ |
| TASK | execution unit | ## Goal, ## Context | $FLOW_DIR/tasks/ |
| PLAN | intent | free-form | $FLOW_DIR/ |
| SPEC | structured requirements (dev) | L2-defined | $FLOW_DIR/ |
| DESIGN | structured design (kw) | L2-defined | $FLOW_DIR/ |
| KNOWLEDGE | verified discoveries | topic H2s | $FLOW_DIR/ |
| DRAFT | exploration, unresolved | ## 問題, ## 未解決 | $FLOW_DIR/ |
| CONSTITUTION | absolute constraints | ## Inherited Principles | $FLOW_DIR/ |

### 4.3 DRAFT ↔ KNOWLEDGE Symmetry

DRAFT and KNOWLEDGE are a symmetric pair (informative references):

- DRAFT: unresolved, hypothetical, exploratory. `[NEEDS CLARIFICATION]` MAY remain.
- KNOWLEDGE: resolved, verified, accumulated. `[NEEDS CLARIFICATION]` MUST NOT remain.
- Resolution flow: DRAFT → resolves → KNOWLEDGE. New questions: KNOWLEDGE → DRAFT.

### 4.4 KNOWLEDGE: Human Index + Inbox

KNOWLEDGE.md serves a dual role as **human-curated index** and **quick capture inbox**.

```markdown
# KNOWLEDGE

> Type: KNOWLEDGE
> Updated: 2026-03-22

## Quick Notes
- small finding here (2026-03-22)

## Entries
- [fetch MCP e2e](knowledge/001-fetch-mcp-e2e.md) — finding
```

- **Quick Notes**: rapid capture of small findings. CS-MD compliant (it's just body content)
- **Entries**: curated, importance-ranked links to individual `knowledge/*.md` files
- When a Quick Note matures → promoted to individual CS-MD file in `knowledge/`
- INDEX.jsonl is the **machine** index (auto-generated, complete)
- KNOWLEDGE.md is the **human** index (curated, editorial)

Both are valid CS-MD documents. Both are derived from individual files.

### 4.5 TODO vs TASK

- TODO = backlog (planning level, output of Structured Flow "Tasks" step)
- TASK = execution unit (runtime level, handed to an agent)
- Orchestrator selects from TODO → creates TASK → agent executes → TODO marked [x]

## 5. References

### 5.1 Syntax

References MUST use standard markdown link syntax:

```markdown
Internal: [text](relative/path.md#heading)
External: [text](https://example.com)
```

No new syntax is invented. `#heading` anchors work in standard markdown.

### 5.2 Internal References

Internal references MUST use **file-relative paths** (standard markdown behavior).

```markdown
From .spec/knowledge/001-fetch-mcp.md:
  [design-doc](../../design-doc.md#section)     ← file-relative (CORRECT)
  [design-doc](/design-doc.md)                   ← absolute (PROHIBITED)
  [design-doc](design-doc.md)                    ← git-root-relative (NON-STANDARD)
```

**File-relative paths are the only portable option.**
Absolute paths have zero portability (different machines, sandbox vs host).
Git-root-relative paths are not standard markdown — GitHub and most renderers
resolve links relative to the file's location.

This follows Pillar 1: use existing format conventions, don't invent new ones.

When files move, references break. Repair via git rename tracking:
`git log --follow` detects renames, tooling can update links.

### 5.3 External References

External references use URLs. Permalink stability is assumed but not guaranteed.

### 5.4 Inline Markers

| Marker | Meaning | Allowed in |
|---|---|---|
| `[NEEDS CLARIFICATION]` | Unresolved ambiguity | DRAFT, SPEC, DESIGN |
| `[NEEDS CLARIFICATION: detail]` | Unresolved with context | DRAFT, SPEC, DESIGN |

Markers MUST be resolved before a SPEC/DESIGN is confirmed.
Markers MAY remain in DRAFT documents.

## 6. KNOWLEDGE Classification

KNOWLEDGE entries fall into three classes with different stability:

| Class | When | Stability | Archive strategy |
|---|---|---|---|
| Decision | Structure step | High (ADR-like, immutable) | Archive directly |
| Finding | Execute step | Low (may become stale) | Needs validity check |
| Lesson | Record step | High (transferable) | Candidate for L0/L2 promotion |

Tags SHOULD indicate the class: `> Tags: decision, ...` / `> Tags: finding, ...` / `> Tags: lesson, ...`

## 7. Domain Customization

CS-MD core is defined at L0. L2 layers customize domain-specific aspects:

| Aspect | L0 (common) | L2-dev | L2-kw |
|---|---|---|---|
| Flow directory | $FLOW_DIR | `.spec/` | `.design/` |
| Structure doc | SPEC or DESIGN | SPEC.md | DESIGN.md |
| Evidence method | (L2 decides) | test-first (TDD) | evidence-first |

## 8. Compatibility

CS-MD has been verified (2026-03-21) to coexist with:

| Tool | Metadata format | Conflict |
|---|---|---|
| Spec Kit (GitHub) | `**Key**: value` inline | None |
| GSD | YAML frontmatter + XML tags | None |
| OpenSpec (Fission AI) | heading patterns | None |
| AGENTS.md (AAIF) | free-form | None |

Coexistence strategy: "if both exist, read both."

## 9. Registry

The file `registry.jsonl` is the trust anchor for CS-MD vocabulary,
analogous to IANA registries. It defines all header fields, section names,
and markers with their MUST/SHOULD/MAY levels and registered-by attribution.

## 10. Security Considerations

- CS-MD documents MAY contain `[NEEDS CLARIFICATION]` markers that indicate
  incomplete specifications. Implementations SHOULD NOT proceed to execution
  with unresolved markers in normative documents (SPEC, CONSTITUTION).
- Header fields from untrusted sources SHOULD be validated against the registry.
- The Provenance field provides lineage but does not guarantee authenticity.
  Git commit signatures are the appropriate mechanism for authentication.

## References

### Normative

- [RFC 761](https://datatracker.ietf.org/doc/html/rfc761) — Postel's Law (Robustness Principle)
- [RFC 8174](https://datatracker.ietf.org/doc/html/rfc8174) — RFC 2119 Keywords Update
- [CommonMark](https://commonmark.org/) — Markdown specification
- [ISO 8601](https://www.iso.org/iso-8601-date-and-time-format.html) — Date and time format
- [SemVer 2.0](https://semver.org/) — Semantic Versioning
- [CONTEXTUS-REGISTRY](CONTEXTUS-REGISTRY.md) — Field name registry specification

### Informative — Provenance and Identity

- [SPDX 2.3 VCS Locator](https://spdx.github.io/spdx-spec/v2.3/external-repository-identifiers/) — `git+https://host/repo@ref#subpath` (ISO/IEC 5962:2021)
- [purl (Package URL)](https://github.com/package-url/purl-spec) — `pkg:github/owner/repo@commitish`
- [OCI Distribution Reference](https://github.com/distribution/reference) — `name:tag@digest` pattern
- [gitrevisions(7)](https://git-scm.com/docs/gitrevisions) — Git revision syntax
- [git-clone URL format](https://git-scm.com/docs/git-clone) — Git URL schemes

### Informative — Metadata Structure Lessons

- [RFC 8941 (Structured Fields)](https://datatracker.ietf.org/doc/html/rfc8941) — IETF's retrofit of structured formats onto HTTP headers. CS-MD avoids this need by declaring structure level upfront (§3.0)
- [RFC 8942 (Client Hints)](https://datatracker.ietf.org/doc/html/rfc8942) — Structured replacement for free-form `User-Agent`. Cautionary tale: free-form metadata that should have been structured from the start

### Informative — Prior Art (Structured Metadata on Existing Formats)

CS-MD follows the same pattern as these HTML structured data approaches:
add semantic conventions on top of an existing format without changing the format.

- [Microformats](https://microformats.org/) (2004) — semantic class names in HTML (`class="vcard"`, `class="hentry"`). Community-maintained vocabulary wiki. Graceful degradation. Directly inspired CS-MD's "conventions on top of existing format" approach
- [Microdata](https://html.spec.whatwg.org/multipage/microdata.html) (2009, HTML5) — `itemscope`/`itemprop` attributes. W3C standardized. More formal than microformats but same principle
- [JSON-LD](https://json-ld.org/) — structured data as `<script>` block in HTML. Machine-readable, invisible to users. Analogous to CS-MD's `> header` block (visible but distinct from body)
- [RDFa](https://rdfa.info/) — RDF attributes in HTML. Heavier than microformats, less adopted

### Informative — Design Influences

- [Mermaid Flowchart](https://mermaid.js.org/syntax/flowchart.html) — `-->` link syntax (Provenance chain delimiter)
- [OpenSpec](https://thedocs.io/openspec/) — Heading pattern conventions
- [AGENTS.md (AAIF)](https://agents.md/) — "Just a markdown file" philosophy
- [Fabian Kubler](https://fabian-kuebler.com/posts/markdown-agentic-ui/) — Markdown as agentic protocol
- [Rob Pike's Rules](https://users.ece.utexas.edu/~adnan/pike.html) — EBP influence (Rule 2: Measure)
- [Jon Postel / RFC 761](https://datatracker.ietf.org/doc/html/rfc761) — Robustness Principle
- [ADR (Architecture Decision Records)](https://adr.github.io/) — KNOWLEDGE Decision class
- [Dublin Core](https://www.dublincore.org/specifications/dublin-core/dces/) — Metadata field naming
- [Schema.org CreativeWork](https://schema.org/CreativeWork) — dateModified, version, status

## Acknowledgments

This specification builds on the work of:

- **Jon Postel** — the Robustness Principle (RFC 761, 1980) is the foundation
  of CS-MD's extensibility and compatibility design. "Be conservative in what
  you send, be liberal in what you accept" governs every design decision in
  this spec: unknown fields are ignored, new extensions don't break parsers,
  tools coexist without coordination.

- **Tantek Çelik and the microformats community** — the "use existing format,
  add conventions" approach that CS-MD applies to markdown was pioneered by
  microformats for HTML in 2004. CS-MD is microformats for the AI agent era.

- **Rob Pike** — Rule 2 ("Measure. Don't tune for speed until you've measured")
  became the EBP principle. This spec was validated by running code, not by
  reasoning.

## Appendix A: Markdown Parser Difficulty (Survey 2026-03-22)

This appendix documents why markdown parsing is difficult and how CS-MD
mitigates the risks. **This analysis is incomplete — see WARNING in §1.5.**

### A.1 Why Markdown Is Hard to Parse

Markdown is not context-free. Key issues (MacFarlane, "Beyond Markdown" 2022):

1. **Emphasis**: `*a**b*` — 17 rules needed (CommonMark). Doubled characters
   (`**`) create exponential ambiguity. 20+ parsers produce different output.
2. **Indented code blocks**: force complex indentation rules for list items.
3. **Raw HTML pass-through**: parser must recognize HTML tag names.
4. **Reference links**: `[foo]` interpretation depends on definitions anywhere in document.
   Cannot parse incrementally.
5. **List item membership**: "how far to indent?" has no simple answer.
6. **Block-level interruption**: can a list start without a blank line?

The original Markdown (Gruber, 2004) was a prose description + buggy Perl script.
No formal grammar. No test suite. CommonMark (2014) is a 164-page spec with
600+ examples — an after-the-fact RFC for a format already in production for 10 years.

### A.2 CS-MD's Mitigation Strategy

CS-MD uses constructs believed to be in the "safe" area of markdown.
**This table is preliminary and requires validation against multiple parsers.**

| CS-MD uses | MacFarlane pain point? | Risk |
|---|---|---|
| `# Title` (H1) | No | Low — unambiguous |
| `> key: value` (blockquote) | No | Low — grep, no parser needed |
| `## Section` (H2) | Interruption (#6) | Low if preceded by blank line |
| `- item` (flat list) | Membership (#5) | Low for flat (no nesting) |
| `- [ ] task` (checkbox) | GFM extension | Low — widely supported |
| `[text](path)` (inline link) | Not #4 (inline, not reference) | Low |
| `[MARKER]` (no link def) | Could be confused with #4 | **Needs verification** |

### A.3 What CS-MD Avoids

- Emphasis (`*`, `**`, `_`) — pain point #1, not used in headers or structure
- Indented code blocks — pain point #2, only fenced blocks if any
- Raw HTML — pain point #3, never used
- Reference links (`[foo]` with separate definition) — pain point #4, inline only
- Nested lists — pain point #5, flat lists only
- Ambiguous block starts — pain point #6, blank lines between all blocks

### A.4 Open Questions (TBD)

- Is `[NEEDS CLARIFICATION]` ever confused with a reference link by any parser?
- Do blockquote lines (`> key: value`) interact with lazy continuation in any parser?
- Are there edge cases where `## Heading` immediately after `> blockquote` is ambiguous?
- Should CS-MD mandate blank lines before `##` headings (CommonMark recommends but does not require)?
- Djot (MacFarlane's clean-slate redesign) — relevant for future CS-MD evolution?

### A.5 References

- [John MacFarlane — Beyond Markdown](https://johnmacfarlane.net/beyond-markdown.html) (2022)
- [CommonMark](https://commonmark.org/)
- [GFM Spec](https://github.github.com/gfm/)
- [Babelmark 3](https://babelmark.github.io/) — compare 20+ parsers
- [Djot](https://github.com/jgm/djot) — clean-slate markdown alternative
- [Why Markdown is not context-free](https://roopc.net/posts/2014/markdown-cfg/)

## Authors

This specification was developed through collaborative sessions between
a human (HTTP/1.1bis RFC contributor) and AI agents (Claude Opus 4.6).
