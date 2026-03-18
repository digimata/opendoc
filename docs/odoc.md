---
path: projects/opendoc/docs/odoc.md
outline: |
  • OpenDoc :: Specification                        L48
    ◦ 1. Design Principles                          L56
    ◦ 2. Document Envelope                          L74
      ▪ 2.1 — Required fields                      L119
      ▪ 2.2 — Optional fields                      L129
    ◦ 3. Content Model                             L140
      ▪ 3.0 — Node identity                        L144
      ▪ 3.1 — Blocks                               L153
        · paragraph                                L159
        · heading                                  L173
        · list                                     L187
        · blockquote                               L208
        · code                                     L223
        · table                                    L234
        · image                                    L252
        · divider                                  L264
        · embed                                    L270
      ▪ 3.2 — Inline content                       L284
      ▪ 3.3 — Marks                                L316
    ◦ 4. Annotations                               L352
      ▪ 4.1 — Annotation structure                 L356
      ▪ 4.2 — Targets                              L382
      ▪ 4.3 — Example: annotated paragraph         L431
    ◦ 5. Provenance                                L492
      ▪ 5.1 — Attachments                          L496
      ▪ 5.2 — Provenance record structure          L527
      ▪ 5.3 — Core provenance kinds                L564
        · source                                   L574
        · assertion                                L591
        · derived                                  L607
    ◦ 6. Vocabulary Extensions                     L632
      ▪ 6.1 — Extension rules                      L636
      ▪ 6.2 — Bundled standard vocabularies        L647
        · odoc.entities.v1 — entity linking        L651
        · odoc.data.v1 — structured data           L680
        · odoc.xref.v1 — cross-references          L713
      ▪ 6.3 — Domain vocabularies                  L747
    ◦ 7. Versioning                                L785
      ▪ 7.1 — Version types                        L810
      ▪ 7.2 — Version resolution                   L819
    ◦ 8. Serialization                             L830
    ◦ 9. Open Questions                            L854
---

# OpenDoc :: Specification

Last updated: `2026.03.12`

> OpenDoc is a general-purpose structured document format — JSON-native, schema-aware, designed for machine consumption. Documents are typed block trees with inline nodes. Semantic enrichment (entity linking, structured data, cross-references) lives in a separate annotation layer that targets nodes by stable ID. Think "what if Word documents were JSON with semantic structure."

---

## 1. Design Principles

| # | Principle | Implication |
|---|-----------|-------------|
| P1 | Self-describing | Every document carries a schema reference. An agent with zero domain knowledge can parse the structure. |
| P2 | Annotatable | Semantic enrichment is layered on via annotations, not embedded in the content tree. The content tree is stable regardless of what annotations exist. |
| P3 | Layered | The core spec is pure structure plus attachment registries. Entity linking, structured data, and cross-references are vocabulary extensions — opt-in, not forced. |
| P4 | Consumption-first | Optimized for reading and querying, not authoring. |
| P5 | Schema-evolvable | New fields/block types can be added without breaking existing consumers. Unknown types are skipped, not errors. |
| P6 | Token-efficient | Compact enough for LLM context windows. No redundant nesting. |
| P7 | Addressable | Every node has a stable document-local ID so annotations, diffs, and references do not depend on array positions. |
| P8 | Versionable | Corrections and updates are first-class — immutable snapshots with predecessor links. |
| P9 | Dual-render | Every document supports both structured (machine) and presentation (human) reads. |

Reference models: Portable Text (block/span shape), ProseMirror (typed node tree), JSON-LD (context/vocabulary), W3C Web Annotation Data Model (annotation targeting), Frictionless Data (schema-in-band).

---

## 2. Document Envelope

Core field names use `snake_case`. Exceptions are reserved external fields such as `$schema`, `@context`, URI-like values, and namespaced vocabulary types.

```json
{
  "$schema": "https://opendoc.dev/schema/odoc/1.0.0.json",

  "odoc": "1.0.0",

  "id": "urn:odoc:example/api-v2-migration-guide",

  "meta": {
    "title": "API v2 Migration Guide",
    "created_at": "2024-11-01T00:00:00Z",
    "modified_at": "2024-11-15T09:30:00Z",
    "producer": "docs-pipeline/1.2.0",
    "language": "en",
    "source": {
      "uri": "https://example.com/docs/api-v2-migration.html",
      "media_type": "text/html",
      "retrieved_at": "2024-11-01T00:00:00Z"
    },
    "tags": ["api", "migration"]
  },

  "content": [
    // ... blocks (see §3)
  ],

  "annotations": [
    // ... annotations (see §4)
  ],

  "provenance": [
    // ... provenance records (see §5)
  ],

  "version": {
    "number": "1.0.0",
    "type": "original"
  }
}
```

### 2.1 — Required fields

| Field | Type | Description |
|-------|------|-------------|
| `$schema` | `string` | Schema URI for this document shape. Required for self-describing validation. |
| `odoc` | `string` | Spec version this document conforms to. |
| `id` | `string` | Globally unique document identifier (URN, URI, or UUID). |
| `meta` | `object` | Document metadata. `title` and `created_at` required; everything else optional. |
| `content` | `Block[]` | Ordered array of top-level blocks. |

### 2.2 — Optional fields

| Field | Type | Description |
|-------|------|-------------|
| `@context` | `string \| array` | JSON-LD context(s) for vocabulary extensions (see §6). Core vocabulary is implicit if omitted. |
| `annotations` | `Annotation[]` | Annotation layer (see §4). Omit if no annotations. |
| `provenance` | `ProvenanceRecord[]` | Top-level provenance registry (see §5). Omit if no provenance is attached. |
| `version` | `object` | Version metadata (see §7). Omit for unversioned documents. |

---

## 3. Content Model

The content tree is pure structure — blocks, text nodes, and formatting marks. No domain semantics. A renderer that ignores the `annotations` array produces a clean, readable document.

### 3.0 — Node identity

Every addressable node carries a stable document-local `id`.

- Every block MUST have an `id`.
- Every inline node MUST have an `id`.
- IDs MUST be unique within a document snapshot.
- IDs SHOULD be stable across non-destructive edits so annotations, version diffs, and cross-references survive reflow and reordering.

### 3.1 — Blocks

A block is a typed node in the document tree. Every block has an `id` and a `type`. Blocks may contain other blocks (nesting) or inline content (leaf).

Core block types:

#### `paragraph`

```json
{
  "id": "p1",
  "type": "paragraph",
  "content": [
    {"id": "t1", "type": "text", "text": "All endpoints now require "},
    {"id": "t2", "type": "text", "text": "Bearer token authentication", "marks": [{"type": "strong"}]},
    {"id": "t3", "type": "text", "text": " — API keys are deprecated."}
  ]
}
```

#### `heading`

```json
{
  "id": "h1",
  "type": "heading",
  "level": 2,
  "content": [{"id": "t4", "type": "text", "text": "Breaking Changes"}],
  "anchor": "breaking-changes"
}
```

`level`: 1–6. `anchor`: optional stable identifier for cross-referencing.

#### `list`

```json
{
  "id": "l1",
  "type": "list",
  "ordered": true,
  "items": [
    {"id": "li1", "content": [{"id": "p-li1", "type": "paragraph", "content": [{"id": "t5", "type": "text", "text": "Update your auth headers"}]}]},
    {"id": "li2", "content": [{"id": "p-li2", "type": "paragraph", "content": [
      {"id": "t6", "type": "text", "text": "Replace "},
      {"id": "t7", "type": "text", "text": "/v1/", "marks": [{"type": "code"}]},
      {"id": "t8", "type": "text", "text": " paths with "},
      {"id": "t9", "type": "text", "text": "/v2/", "marks": [{"type": "code"}]}
    ]}]}
  ]
}
```

List item `content` is always `Block[]`. Single-paragraph items wrap their inline content in a paragraph block.

#### `blockquote`

```json
{
  "id": "q1",
  "type": "blockquote",
  "content": [
    {"id": "p2", "type": "paragraph", "content": [
      {"id": "t10", "type": "text", "text": "This endpoint will be removed on 2025-06-01."}
    ]}
  ],
  "attribution": "v1 deprecation notice"
}
```

#### `code`

```json
{
  "id": "c1",
  "type": "code",
  "language": "bash",
  "text": "curl -H 'Authorization: Bearer TOKEN' https://api.example.com/v2/users"
}
```

#### `table`

Simple inline tables. For structured data with typed cells or provenance, use an OpenSheet embed instead.

```json
{
  "id": "tbl1",
  "type": "table",
  "caption": "Endpoint mapping",
  "columns": ["v1 path", "v2 path", "Notes"],
  "rows": [
    ["/v1/users", "/v2/users", "Unchanged"],
    ["/v1/items", "/v2/inventory", "Renamed"],
    ["/v1/search", "/v2/query", "New parameters"]
  ]
}
```

#### `image`

```json
{
  "id": "img1",
  "type": "image",
  "src": "https://example.com/docs/arch-diagram.png",
  "alt": "Architecture diagram showing the v2 request flow",
  "caption": "Figure 1: v2 request lifecycle"
}
```

#### `divider`

```json
{"id": "d1", "type": "divider"}
```

#### `embed`

Reference to external content — an OpenSheet, another OpenDoc, a video, etc.

```json
{
  "id": "emb1",
  "type": "embed",
  "src": "urn:osheet:example/endpoint-comparison",
  "media_type": "application/vnd.osheet+json",
  "title": "Full endpoint comparison matrix"
}
```

### 3.2 — Inline content

Inline elements live inside block `content` arrays. Core inline nodes are `text` nodes and `hard_break` markers.

```json
{"id": "t11", "type": "text", "text": "plain text"}
{"id": "t12", "type": "text", "text": "bold text", "marks": [{"type": "strong"}]}
{"id": "t13", "type": "text", "text": "linked text", "marks": [{"type": "link", "href": "https://example.com/docs/auth"}]}
{"id": "br1", "type": "hard_break"}
```

All marks are objects with a `type` field. Marks with extra fields include those fields alongside `type`:

```json
{
  "id": "p3",
  "type": "paragraph",
  "content": [
    {"id": "t14", "type": "text", "text": "See the "},
    {
      "id": "t15",
      "type": "text",
      "text": "authentication guide",
      "marks": [
        {"type": "link", "href": "https://example.com/docs/auth", "title": "Auth guide"}
      ]
    },
    {"id": "t16", "type": "text", "text": " for details."}
  ]
}
```

### 3.3 — Marks

Marks are inline formatting applied to text nodes. Core mark vocabulary:

| Mark | Description | Extra fields |
|------|-------------|--------------|
| `strong` | Bold / strong emphasis | — |
| `em` | Italic / emphasis | — |
| `code` | Inline code | — |
| `strikethrough` | Struck-through text | — |
| `underline` | Underlined text | — |
| `link` | Hyperlink | `href`, optional `title` |
| `highlight` | Highlighted text | optional `color` |
| `sup` | Superscript | — |
| `sub` | Subscript | — |
| `footnote` | Footnote reference | `note` (plain string) |

Encoding rules:

- All marks MUST be written as objects with a `type` field.
- Consumers MUST preserve mark order within a text node.
- `link` is for navigational/rendered hyperlinks. Typed semantic relationships between documents or sections belong in `xref:link` annotations, not marks.
- `footnote.note` is plain text in core v0. Structured or richly formatted footnotes SHOULD be modeled as appendix content plus `xref:link`.

Examples:

```json
{"id": "t17", "type": "text", "text": "important", "marks": [{"type": "strong"}, {"type": "underline"}]}
{"id": "t18", "type": "text", "text": "docs", "marks": [{"type": "link", "href": "https://example.com/docs"}]}
{"id": "t19", "type": "text", "text": "beta", "marks": [{"type": "highlight", "color": "yellow"}]}
{"id": "t20", "type": "text", "text": "1", "marks": [{"type": "footnote", "note": "Legacy endpoints remain available until 2025-06-01."}]}
```

---

## 4. Annotations

Annotations are the semantic enrichment layer. They target nodes in the content tree by ID and carry typed payloads. The content tree never changes shape because of annotations — adding, removing, or modifying annotations leaves the content untouched.

### 4.1 — Annotation structure

```json
{
  "id": "a1",
  "type": "entities:mention",
  "target": {
    "kind": "text_range",
    "start": {"node": "t1", "offset": 0},
    "end": {"node": "t1", "offset": 9}
  },
  "body": {
    "uri": "https://example.com/entities/acme-corp",
    "role": "subject"
  }
}
```

| Field | Type | Description |
|-------|------|-------------|
| `id` | `string` | Unique annotation ID within the document. |
| `type` | `string` | Namespace-qualified annotation type. Defined by vocabularies. |
| `target` | `object` | What this annotation targets (see §4.2). |
| `body` | `object` | Annotation payload. Schema depends on `type`. |
| `prov_refs` | `string[]` | Optional — IDs of provenance records in the top-level `provenance` registry. |

### 4.2 — Targets

An annotation target identifies what part of the content tree is being annotated. Every target object MUST include a `kind` field.

| `kind` | Meaning | Required fields |
|---|---|---|
| `node` | Entire block or inline node | `node` |
| `text_range` | Text span across one or more text nodes | `start`, `end` |
| `node_range` | Contiguous sequence of sibling nodes | `start`, `end` |

**Whole node** — annotates an entire block or inline node:

```json
{"kind": "node", "node": "p1"}
```

**Character range** — annotates a substring within text content:

```json
{
  "kind": "text_range",
  "start": {"node": "t1", "offset": 0},
  "end": {"node": "t1", "offset": 9}
}
```

`start` and `end` are boundary points. Each boundary point has:

```json
{"node": "t1", "offset": 0}
```

`offset` is a zero-indexed character position against that node's `text` string. `start` is inclusive, `end` is exclusive. `text_range` may stay within one text node or span multiple adjacent text nodes in document order.

Rules:

- `text_range` boundary points MUST reference text nodes, not block nodes.
- `text_range` MUST NOT cross block boundaries.
- `text_range` MAY cross `hard_break` nodes in document order; `hard_break` nodes do not contribute characters or offsets.
- Future non-text inline nodes introduced by extensions do not contribute characters and MUST NOT be addressed by `offset`.

**Node range** — annotates a contiguous sequence of sibling blocks:

```json
{"kind": "node_range", "start": "p1", "end": "p3"}
```

Covers all sibling nodes from `start` to `end` inclusive. Both must share the same parent.

### 4.3 — Example: annotated paragraph

Content:

```json
{
  "id": "p5",
  "type": "paragraph",
  "content": [
    {"id": "t20", "type": "text", "text": "Acme "},
    {"id": "t21", "type": "text", "text": "Corp", "marks": [{"type": "strong"}]},
    {"id": "t22", "type": "text", "text": " migrated "},
    {"id": "t23", "type": "text", "text": "14,000"},
    {"id": "t24", "type": "text", "text": " endpoints to v2 in under "},
    {"id": "t25", "type": "text", "text": "3 weeks"},
    {"id": "t26", "type": "text", "text": "."}
  ]
}
```

Annotations:

```json
[
  {
    "id": "a1",
    "type": "entities:mention",
    "target": {
      "kind": "text_range",
      "start": {"node": "t20", "offset": 0},
      "end": {"node": "t21", "offset": 4}
    },
    "body": {"uri": "https://example.com/entities/acme-corp"}
  },
  {
    "id": "a2",
    "type": "data:value",
    "target": {
      "kind": "text_range",
      "start": {"node": "t23", "offset": 0},
      "end": {"node": "t23", "offset": 6}
    },
    "body": {"value": 14000, "unit": "endpoints"}
  },
  {
    "id": "a3",
    "type": "data:value",
    "target": {
      "kind": "text_range",
      "start": {"node": "t25", "offset": 0},
      "end": {"node": "t25", "offset": 7}
    },
    "body": {"value": 3, "unit": "weeks"}
  }
]
```

A dumb renderer shows: "Acme Corp migrated 14,000 endpoints to v2 in under 3 weeks." A smart consumer knows Acme Corp is an entity and can extract the structured values.

---

## 5. Provenance

OpenDoc keeps provenance outside the content tree. Content nodes and annotations attach to provenance records via `prov_refs`.

### 5.1 — Attachments

Any block, inline node, or annotation MAY include `prov_refs`:

```json
{
  "id": "p6",
  "type": "paragraph",
  "content": [
    {"id": "t30", "type": "text", "text": "The migration reduced p99 latency to 240ms."}
  ],
  "prov_refs": ["pr1"]
}
```

```json
{
  "id": "a4",
  "type": "data:value",
  "target": {
    "kind": "text_range",
    "start": {"node": "t30", "offset": 37},
    "end": {"node": "t30", "offset": 42}
  },
  "body": {"value": 240, "unit": "ms", "concept": "p99-latency"},
  "prov_refs": ["pr1", "pr2"]
}
```

`prov_refs` attaches reusable provenance records without repeating the annotation target or mutating the content tree.

### 5.2 — Provenance record structure

Each provenance record lives in the top-level `provenance` array and can be referenced by multiple nodes or annotations.

```json
{
  "id": "pr1",
  "kind": "source",
  "body": {
    "source": "https://example.com/docs/api-v2-migration.html",
    "location": "§3, para.2",
    "method": "extracted",
    "agent": "docs-pipeline/1.2.0",
    "confidence": 0.98
  }
}
```

| Field | Type | Description |
|-------|------|-------------|
| `id` | `string` | Unique provenance record ID within the document. |
| `kind` | `string` | Provenance category. Core v0 kinds are `source`, `assertion`, and `derived`. |
| `body` | `object` | Provenance payload. Schema depends on `kind`. |

Common `body` fields:

| Field | Type | Description |
|-------|------|-------------|
| `source` | `string` | URI of the source material. |
| `location` | `string` | Human-readable location within the source (page, section, paragraph, cell). |
| `method` | `string` | How the content or annotation was produced: `extracted`, `derived`, `manual`, `imputed`, `transcribed`, `resolved`. |
| `agent` | `string` | What produced this record (pipeline version, model ID, human identifier). |
| `confidence` | `number` | Confidence score, 0–1, if available. |
| `input_refs` | `string[]` | Optional links to earlier provenance records this one depends on. |

`meta.source` remains the document-level default source. Use provenance records when a specific node or annotation needs more precise lineage.

### 5.3 — Core provenance kinds

OpenDoc v0 standardizes 3 provenance kinds:

| `kind` | Meaning | Typical attachment |
|---|---|---|
| `source` | Where content came from in the original artifact | blocks, inline nodes |
| `assertion` | How a semantic claim or annotation was produced | annotations |
| `derived` | How content or annotations were computed from earlier records | blocks, annotations |

#### `source`

Use when the record points back to the origin of extracted or transcribed content.

```json
{
  "id": "pr1",
  "kind": "source",
  "body": {
    "source": "https://www.sec.gov/Archives/.../aapl-20240928.htm",
    "location": "Note 1, Revenue Recognition, paragraph 2",
    "method": "extracted",
    "agent": "html-parser/0.1.0"
  }
}
```

#### `assertion`

Use when the record explains how an annotation or semantic claim was made.

```json
{
  "id": "pr2",
  "kind": "assertion",
  "body": {
    "method": "entity_resolver",
    "agent": "resolver/0.1.0",
    "confidence": 0.97
  }
}
```

#### `derived`

Use when the record represents a transformation, normalization, or calculation based on other provenance records.

```json
{
  "id": "pr3",
  "kind": "derived",
  "body": {
    "method": "xbrl_normalizer",
    "agent": "normalizer/0.1.0",
    "input_refs": ["pr1", "pr2"]
  }
}
```

Rules:

- `source` SHOULD include `source` and SHOULD include `location` when available.
- `assertion` SHOULD include `method` or `agent`, and SHOULD include `confidence` when the claim is probabilistic.
- `derived` SHOULD include `input_refs`.
- Additional provenance kinds MAY be added later; non-core kinds SHOULD be namespace-prefixed.

---

## 6. Vocabulary Extensions

The core spec defines blocks, text nodes, marks, the annotation mechanism, provenance attachment, and versioning. Everything else — entity linking, structured data, cross-references, domain-specific block types — is added via vocabularies.

### 6.1 — Extension rules

1. Custom annotation types MUST be namespace-prefixed (`entities:mention`, not `mention`).
2. Custom block types MUST be namespace-prefixed (`med:finding`, not `finding`).
3. Custom marks MUST be namespace-prefixed (`legal:defined-term`).
4. Consumers MUST ignore unknown namespaced types without error — skip the block or annotation, preserve children if they're core types.
5. Extensions MAY add custom fields to core blocks (also namespace-prefixed).
6. Extensions MUST NOT redefine the semantics of core types or fields.

Vocabularies are versioned independently from the core spec.

### 6.2 — Bundled standard vocabularies

These ship with the spec. They define annotation types that are generally useful across domains.

#### `odoc.entities.v1` — entity linking

Annotation type: `entities:mention`

```json
{
  "id": "a1",
  "type": "entities:mention",
  "target": {
    "kind": "text_range",
    "start": {"node": "t1", "offset": 0},
    "end": {"node": "t1", "offset": 9}
  },
  "body": {
    "uri": "https://example.com/entities/acme-corp",
    "role": "subject",
    "label": "Acme Corp"
  }
}
```

Body fields:

| Field | Type | Description |
|-------|------|-------------|
| `uri` | `string` | Resolvable entity identifier. Any URI scheme — HTTPS, URN, Wikidata, etc. |
| `role` | `string` | Optional semantic role: `subject`, `author`, `mentioned`. Extensible by domain vocabs. |
| `label` | `string` | Optional canonical display name for the entity. |

#### `odoc.data.v1` — structured data

Annotation type: `data:value`

```json
{
  "id": "a2",
  "type": "data:value",
  "target": {
    "kind": "text_range",
    "start": {"node": "t1", "offset": 33},
    "end": {"node": "t1", "offset": 38}
  },
  "body": {
    "value": 240,
    "unit": "ms",
    "concept": "p99-latency",
    "period": "2024-Q3"
  }
}
```

Body fields:

| Field | Type | Description |
|-------|------|-------------|
| `value` | `number \| string \| boolean` | The structured value. |
| `unit` | `string` | Unit of measure — any string (`ms`, `%`, `kg`, `req/s`, etc.). |
| `concept` | `string` | What this value represents — plain label or vocabulary-qualified term. |
| `period` | `string` | Temporal reference, if applicable. |
| `scale` | `number` | Display scale factor. Default `1`. |
| `precision` | `number` | Significant digits in the source. |

#### `odoc.xref.v1` — cross-references

Annotation type: `xref:link`

Semantic cross-references between documents, sections, and embedded resources. For navigational hyperlinks, use the `link` mark. `xref:link` adds typed, machine-readable relationships that consumers can reason over even if there is no visible hyperlink in the rendered document.

Rule of thumb:

- `link` answers: "where can a reader go?"
- `xref:link` answers: "what relationship is this document asserting?"

```json
{
  "id": "a3",
  "type": "xref:link",
  "target": {"kind": "node", "node": "p1"},
  "body": {
    "href": "urn:odoc:example/api-v1-guide#auth",
    "rel": "supersedes",
    "title": "v1 authentication docs"
  }
}
```

`link` and `xref:link` MAY coexist on the same text or block when a document needs both a clickable hyperlink and an explicit semantic relationship.

Body fields:

| Field | Type | Description |
|-------|------|-------------|
| `href` | `string` | Target document URI. Supports `urn:odoc:`, `urn:osheet:`, `https://`, `#fragment`. |
| `rel` | `string` | Relationship type: `references`, `supersedes`, `extends`, `contradicts`, `cites`. |
| `title` | `string` | Optional human-readable description of the target. |

### 6.3 — Domain vocabularies

Domain vocabularies add custom block types, annotation types, marks, and entity roles for specific fields. They are versioned and declared in `@context`.

Example — a medical vocabulary adding a custom block type:

```json
{
  "odoc": "1.0.0",
  "@context": [
    "https://opendoc.dev/ns/v1",
    {"med": "https://opendoc.dev/ns/med/v1"}
  ],
  "content": [
    {
      "id": "b1",
      "type": "med:finding",
      "severity": "moderate",
      "content": [
        {"id": "p1", "type": "paragraph", "content": [
          {"id": "t1", "type": "text", "text": "Mild pleural effusion noted in the right lung."}
        ]}
      ]
    }
  ]
}
```

Example registrations (not part of core spec — published separately):

| Namespace | Domain | Adds |
|-----------|--------|------|
| `fin.sec` | SEC / EDGAR filings | `fin.sec:risk-factor`, `fin.sec:mda-section` blocks; `fin.sec:filer` entity role; XBRL concept scheme for `data:value` |
| `legal` | Legal documents | `legal:clause`, `legal:definition` blocks; `legal:defined-term` mark; `legal:plaintiff` entity role |
| `research` | Academic / research | `research:abstract`, `research:finding` blocks; `research:citation` annotation type |

---

## 7. Versioning

Documents are **immutable snapshots**. Edits produce new versions, not mutations.

```json
{
  "version": {
    "number": "1.1.0",
    "type": "correction",
    "supersedes": "urn:odoc:example/api-v2-migration-guide@1.0.0",
    "date": "2024-12-01T00:00:00Z",
    "reason": "Fixed incorrect endpoint path in §3",
    "changes": [
      {
        "node_id": "tbl1",
        "path": "rows[1][1]",
        "from": "/v2/items",
        "to": "/v2/inventory",
        "reason": "Typo — wrong v2 path"
      }
    ]
  }
}
```

### 7.1 — Version types

| Type | Meaning | Semver impact |
|------|---------|---------------|
| `original` | First version | `1.0.0` |
| `update` | New content added | Minor bump |
| `correction` | Fix (typo, error) | Patch bump |
| `retraction` | Document withdrawn | Major bump |

### 7.2 — Version resolution

| URI | Resolves to |
|-----|-------------|
| `urn:odoc:example/guide` | Latest version |
| `urn:odoc:example/guide@1.0.0` | Specific version |

The `changes` array is an explicit structured diff — `node_id`, `path`, `from`, `to`, `reason`. `node_id` scopes a change to a stable node so diffs do not depend on top-level array positions. Optional — the `supersedes` link alone is sufficient for version chains.

---

## 8. Serialization

**Canonical:** JSON (`.odoc.json`). MIME type: `application/vnd.odoc+json`.

**Token-efficient:** TOON (`.odoc.toon`). MIME type: `application/vnd.odoc+toon`. Lossless JSON roundtrip, 30–60% token reduction for LLM context windows.

**Streaming:** JSON Lines (`.odoc.jsonl`) using explicit record wrappers. Each line MUST be an object with `record_kind` and `value`. Allowed `record_kind` values are `document`, `block`, `annotation`, and `provenance`.

```
{"record_kind":"document","value":{"$schema":"https://opendoc.dev/schema/odoc/1.0.0.json","odoc":"1.0.0","id":"...","meta":{...}}}
{"record_kind":"block","value":{"id":"h1","type":"heading","level":1,"content":[{"id":"t1","type":"text","text":"Introduction"}]}}
{"record_kind":"block","value":{"id":"p1","type":"paragraph","content":[{"id":"t2","type":"text","text":"..."}]}}
{"record_kind":"annotation","value":{"id":"a1","type":"entities:mention","target":{"kind":"text_range","start":{"node":"t2","offset":0},"end":{"node":"t2","offset":9}},"body":{...},"prov_refs":["pr1"]}}
{"record_kind":"provenance","value":{"id":"pr1","kind":"source","body":{...}}}
```

| Extension | Format |
|-----------|--------|
| `.odoc.json` | Canonical JSON |
| `.odoc.toon` | TOON serialization |
| `.odoc.jsonl` | Streaming JSON Lines |

---

## 9. Open Questions

1. **Top-level mixed content.** Current spec: top level is blocks only, inline lives inside blocks. Simpler, but single-paragraph documents need a wrapping paragraph block.

2. **Table model.** Inline tables are intentionally simple (string cells). Should they support typed cells and per-cell provenance, or always defer to OpenSheet?

3. **Mark nesting rules.** Should the spec define which marks can nest (e.g., bold inside a link) or leave it to consumers?

4. **Fragment granularity.** Currently only headings have `anchor`. Should all blocks support anchors for fine-grained cross-referencing? (Node IDs partially solve this — annotations can target any node by ID, but `anchor` is a human-friendly alias.)

5. **Binary content.** Images use `src` URIs. Should the spec support inline base64 for self-contained documents?

6. **Localization.** `meta.language` is document-level. Should blocks support per-block `lang` for multilingual documents?

7. **Overlapping annotations.** Two annotations can target overlapping character ranges on the same text node. The spec currently allows this silently. Should it define precedence or merging rules?

8. **Annotation ordering.** Does the order of annotations in the array carry semantics, or is it arbitrary?

9. **Provenance as core vs extension.** Provenance is currently a first-class top-level field alongside `content` and `annotations`. An alternative is to model it as a vocabulary extension — provenance records would live in the annotation layer rather than having their own top-level registry. The argument for core: lineage tracking is fundamental infrastructure. The argument for extension: it adds weight to the envelope for documents that don't need it, and `prov_refs` pollutes every block/inline/annotation type.
