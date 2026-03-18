---
path: projects/opendoc/CHANGELOG.md
outline: |
  • Changelog                    L8
    ◦ 0.1.0 — 2026-03-17        L10
---

# Changelog

## 0.2.0 — 2026-03-17

- Consumer API: `io` module — `from_str`/`from_path`/`to_string`/`to_path` and friends for reading and writing `.odoc` JSON
- Consumer API: `query` module — `DocumentIndex` for O(1) node/annotation lookup, `NodeRef` enum, inherent helpers on `Block`/`InlineNode`/`Annotation`/`Target`
- Consumer API: `render` module — `Render` trait and `MarkdownRenderer` for odoc → GFM markdown (feature-gated: `render-markdown`)
- Consumer API: `convert` module — `Convert` trait and `MarkdownConverter` for markdown → odoc via pulldown-cmark (feature-gated: `convert-markdown`)
- Golden test corpus for markdown render output
- Constants: `ODOC_VERSION`, `SCHEMA_URI`

## 0.1.0 — 2026-03-17

- Core types: Document, Block (9 variants), InlineNode, Mark (10 variants), Annotation, Target, Provenance
- Lossless JSON serde roundtrip for all types
- Spec simplifications: marks always objects (no bare strings), list items always `Vec<Block>`
- Project scaffold: workspace with `odoc`, `osheet`, and `odoc-cli` crates
