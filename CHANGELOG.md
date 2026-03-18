# Changelog

## 0.1.0 — 2026-03-17

- Core types: Document, Block (9 variants), InlineNode, Mark (10 variants), Annotation, Target, Provenance
- Lossless JSON serde roundtrip for all types
- Spec simplifications: marks always objects (no bare strings), list items always `Vec<Block>`
- Project scaffold: workspace with `odoc`, `osheet`, and `odoc-cli` crates
