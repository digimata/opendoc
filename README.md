---
path: projects/opendoc/README.md
outline: |
  • OpenDoc                   L13
    ◦ What it looks like      L19
    ◦ Design                  L60
    ◦ Formats                 L71
    ◦ Workspace               L81
    ◦ Status                  L89
    ◦ Specs                   L93
---

# OpenDoc

An agent-friendly structured document format.

OpenDoc (`.odoc`) represents documents as typed block trees with inline content — paragraphs, headings, tables, lists, code — in a JSON structure that machines can produce, query, and consume without lossy intermediate representations. Semantic enrichment (entity mentions, structured values, cross-references) lives in a separate annotation layer that targets content nodes by stable ID, so the content tree never changes shape regardless of how much analysis is layered on.

## What it looks like

```json
{
  "$schema": "https://opendoc.dev/schema/odoc/1.0.0.json",
  "odoc": "1.0.0",
  "id": "urn:odoc:acme/q3-earnings",
  "meta": {
    "title": "Acme Corp Q3 2024 Earnings Release",
    "created_at": "2024-11-01T00:00:00Z",
    "source": {
      "uri": "https://sec.gov/Archives/edgar/data/12345/q3-earnings.htm",
      "media_type": "text/html"
    }
  },
  "content": [
    {
      "id": "h1", "type": "heading", "level": 1,
      "content": [{"id": "t1", "type": "text", "text": "Financial Highlights"}]
    },
    {
      "id": "p1", "type": "paragraph",
      "content": [
        {"id": "t2", "type": "text", "text": "Revenue grew to "},
        {"id": "t3", "type": "text", "text": "$4.2B", "marks": [{"type": "strong"}]},
        {"id": "t4", "type": "text", "text": ", up 18% year-over-year."}
      ]
    }
  ],
  "annotations": [
    {
      "id": "a1", "type": "data:value",
      "target": {"kind": "text_range", "start": {"node": "t3", "offset": 0}, "end": {"node": "t3", "offset": 4}},
      "body": {"value": 4200000000, "unit": "USD", "concept": "revenue", "period": "2024-Q3"}
    }
  ]
}
```

A plain renderer sees readable text. A smart consumer extracts `revenue = $4.2B for 2024-Q3` without parsing natural language.

## Design

| Principle | What it means |
|---|---|
| **Self-describing** | Every document carries a schema reference — an agent with zero domain knowledge can parse the structure |
| **Content/annotation separation** | The content tree is pure structure. Entity links, structured values, and cross-references are annotations that target nodes by ID. Add or remove annotations without touching content. |
| **Addressable** | Every node has a stable document-local ID. Annotations, diffs, and references target nodes precisely — no brittle array indices. |
| **Versionable** | Documents are immutable snapshots. Corrections produce new versions with structured diffs and predecessor links. |
| **Extensible** | Domain vocabularies (SEC filings, legal docs, medical records) add custom block types and annotation types via namespaced extensions. Unknown types are skipped, not errors. |
| **Token-efficient** | Compact JSON — no redundant nesting. TOON serialization cuts tokens 30-60% for LLM context windows. |

## Formats

| Extension | Format | Use case |
|---|---|---|
| `.json` | Canonical JSON | Storage, APIs, interchange |
| `.toon` | TOON | LLM context windows (30-60% smaller) |
| `.jsonl` | Streaming JSON Lines | Large documents, incremental processing |

## Workspace

| Crate | Purpose |
|---|---|
| `odoc` | Core types, serde, renderers, builder API |
| `osheet` | OpenSheet — structured tabular data companion format |
| `cli` | `odoc render`, `odoc convert` CLI |

## Status

Early development. The spec is stable; the Rust implementation is stubbed out.

## Specs

- [OpenDoc specification](docs/odoc.md) — full format spec with examples
- [OpenSheet specification](docs/osheet.md) — tabular data companion format
