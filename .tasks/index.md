---
path: projects/opendoc/.tasks/index.md
outline: |
  • OpenDoc — Tasks          L9
    ◦ Dependency Graph      L19
    ◦ Notes                 L41
---

# OpenDoc — Tasks

| Task | Title | Phase | Status | Blocked on |
|---|---|---|---|---|
| [T-0082](T-0082.md) | Core types matching the spec | 1 | done | — |
| [T-0083](T-0083.md) | Serde — lossless JSON roundtrip | 1 | done | T-0082 |
| [T-0084](T-0084.md) | Markdown renderer | 2 | backlog | T-0082 |
| [T-0085](T-0085.md) | Builder API | 2 | backlog | T-0082 |
| [T-0086](T-0086.md) | CLI — render and convert commands | 3 | backlog | T-0083, T-0084 |

## Dependency Graph

```
Phase 1 — Types + Serialization (done)
────────────────────────────────────────
T-0082 (core types) ✓
   |
   +-> T-0083 (serde roundtrip) ✓

Phase 2 — Rendering + Construction
────────────────────────────────────────
T-0082 -> T-0084 (markdown renderer)  ┐
                                      │ parallel
T-0082 -> T-0085 (builder API)        ┘

Phase 3 — CLI
────────────────────────────────────────
T-0083 + T-0084 -> T-0086 (CLI)

Critical path: T-0082 -> T-0084 -> T-0086
```

## Notes

- `odoc` crate is domain-agnostic; SEC-specific parsing (HTML → .odoc) lives in hermaeus
- `osheet` is a stub until GAAP standardization needs it (hermaeus T-0068+)
- T-0077 (hermaeus) tracks the scaffold; these tasks track implementation
