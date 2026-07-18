# Graph Report - yy  (2026-07-18)

## Corpus Check
- 21 files · ~1,212 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 39 nodes · 18 edges · 21 communities (20 shown, 1 thin omitted)
- Extraction: 100% EXTRACTED · 0% INFERRED · 0% AMBIGUOUS
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `6a3bc595`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- yy
- YY-1 Bootstrap — module skeleton
- setup-worktree.sh

## God Nodes (most connected - your core abstractions)
1. `yy` - 10 edges
2. `YY-1 Bootstrap — module skeleton` - 6 edges
3. `Goal` - 1 edges
4. `Scope` - 1 edges
5. `Out of scope` - 1 edges
6. `Done when` - 1 edges
7. `Testing` - 1 edges
8. `setup-worktree.sh script` - 1 edges
9. `Why` - 1 edges
10. `Features` - 1 edges

## Surprising Connections (you probably didn't know these)
- None detected - all connections are within the same source files.

## Import Cycles
- None detected.

## Communities (21 total, 1 thin omitted)

### Community 1 - "yy"
Cohesion: 0.18
Nodes (10): Acknowledgments, Data & config, Editing, Features, How it works, Install, License, Quick start (+2 more)

### Community 2 - "YY-1 Bootstrap — module skeleton"
Cohesion: 0.29
Nodes (6): Done when, Goal, Out of scope, Scope, Testing, YY-1 Bootstrap — module skeleton

## Knowledge Gaps
- **15 isolated node(s):** `Goal`, `Scope`, `Out of scope`, `Done when`, `Testing` (+10 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **1 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **What connects `Goal`, `Scope`, `Out of scope` to the rest of the system?**
  _15 weakly-connected nodes found - possible documentation gaps or missing edges._