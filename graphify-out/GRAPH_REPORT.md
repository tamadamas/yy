# Graph Report - yy  (2026-07-18)

## Corpus Check
- 22 files · ~1,922 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 43 nodes · 21 edges · 22 communities (21 shown, 1 thin omitted)
- Extraction: 100% EXTRACTED · 0% INFERRED · 0% AMBIGUOUS
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `1d702a2e`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- yy
- YY-1 Bootstrap — module skeleton
- setup-worktree.sh
- Global Constraints

## God Nodes (most connected - your core abstractions)
1. `yy` - 10 edges
2. `YY-1 Bootstrap — module skeleton` - 6 edges
3. `YY-1 Bootstrap Implementation Plan` - 2 edges
4. `Global Constraints` - 2 edges
5. `setup-worktree.sh script` - 1 edges
6. `Why` - 1 edges
7. `Features` - 1 edges
8. `Install` - 1 edges
9. `Quick start` - 1 edges
10. `How it works` - 1 edges

## Surprising Connections (you probably didn't know these)
- None detected - all connections are within the same source files.

## Import Cycles
- None detected.

## Communities (22 total, 1 thin omitted)

### Community 1 - "yy"
Cohesion: 0.18
Nodes (10): Acknowledgments, Data & config, Editing, Features, How it works, Install, License, Quick start (+2 more)

### Community 2 - "YY-1 Bootstrap — module skeleton"
Cohesion: 0.29
Nodes (6): Done when, Goal, Out of scope, Scope, Testing, YY-1 Bootstrap — module skeleton

### Community 21 - "Global Constraints"
Cohesion: 0.50
Nodes (3): Global Constraints, Task 1: Module skeleton, YY-1 Bootstrap Implementation Plan

## Knowledge Gaps
- **16 isolated node(s):** `setup-worktree.sh script`, `Why`, `Features`, `Install`, `Quick start` (+11 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **1 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **What connects `setup-worktree.sh script`, `Why`, `Features` to the rest of the system?**
  _16 weakly-connected nodes found - possible documentation gaps or missing edges._