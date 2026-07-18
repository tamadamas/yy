# Graph Report - yy  (2026-07-19)

## Corpus Check
- 24 files · ~9,233 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 178 nodes · 321 edges · 24 communities (23 shown, 1 thin omitted)
- Extraction: 100% EXTRACTED · 0% INFERRED · 0% AMBIGUOUS
- Token cost: 0 input · 0 output

## Graph Freshness
<<<<<<< HEAD
- Built from commit: `d7a583d2`
=======
- Built from commit: `a084715f`
>>>>>>> 195853b (Add coverage tool)
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- yy
- YY-1 Bootstrap — module skeleton
- setup-worktree.sh
- mod.rs
- Entry
- active.rs
- entries.rs
- issues.rs
- jsonl.rs
- Global Constraints
- YY-6 CLI — design
- Global Constraints

## God Nodes (most connected - your core abstractions)
1. `Entry` - 23 edges
2. `start()` - 16 edges
3. `Id` - 14 edges
4. `Issue` - 13 edges
5. `stop()` - 11 edges
6. `resolve_or_create()` - 10 edges
7. `yy` - 10 edges
8. `close_and_archive()` - 9 edges
9. `month_path()` - 9 edges
10. `read_month()` - 9 edges

## Surprising Connections (you probably didn't know these)
- `start()` --references--> `Entry`  [EXTRACTED]
  src/core/mod.rs → src/model.rs
- `start()` --references--> `Id`  [EXTRACTED]
  src/core/mod.rs → src/model.rs
- `stop()` --references--> `Entry`  [EXTRACTED]
  src/core/mod.rs → src/model.rs
- `close_and_archive()` --references--> `Entry`  [EXTRACTED]
  src/core/mod.rs → src/model.rs
- `IssueTotal` --references--> `Id`  [EXTRACTED]
  src/core/mod.rs → src/model.rs

## Import Cycles
- None detected.

## Communities (24 total, 1 thin omitted)

### Community 1 - "yy"
Cohesion: 0.18
Nodes (10): Acknowledgments, Data & config, Editing, Features, How it works, Install, License, Quick start (+2 more)

### Community 2 - "YY-1 Bootstrap — module skeleton"
Cohesion: 0.29
Nodes (6): Done when, Goal, Out of scope, Scope, Testing, YY-1 Bootstrap — module skeleton

### Community 5 - "mod.rs"
Cohesion: 0.16
Nodes (24): close_and_archive(), IssueTotal, last_working_day(), DateTime, NaiveDate, Option, Path, PathBuf (+16 more)

<<<<<<< HEAD
### Community 8 - "Entry"
=======
### Community 8 - "model.rs"
>>>>>>> 195853b (Add coverage tool)
Cohesion: 0.13
Nodes (21): Default, Display, EntryTag, Formatter, Into, IssueTag, Self, custom_issue_kind_round_trips() (+13 more)

### Community 10 - "active.rs"
Cohesion: 0.24
Nodes (13): Active, elapsed(), elapsed_of_running_entry_is_derived_from_now(), missing_file_reads_as_no_active_entry(), path(), read(), Option, PathBuf (+5 more)

### Community 11 - "entries.rs"
Cohesion: 0.31
Nodes (15): append(), append_preserves_existing_comments_and_lines(), entries_in_range(), entries_in_range_spans_multiple_months(), entry_at(), missing_month_file_reads_no_entries(), month_path(), read_month() (+7 more)

### Community 12 - "issues.rs"
Cohesion: 0.35
Nodes (13): find_by_id(), find_by_id_returns_none_when_missing(), path(), read_all(), resolve_or_create(), resolve_or_create_creates_on_first_use(), resolve_or_create_falls_back_to_key_as_title_when_no_desc(), resolve_or_create_reuses_existing_key() (+5 more)

### Community 13 - "jsonl.rs"
Cohesion: 0.30
Nodes (14): Line, parse(), parse_record(), read(), Record, render(), round_trips_comments_and_malformed_lines(), Path (+6 more)

### Community 21 - "Global Constraints"
Cohesion: 0.50
Nodes (3): Global Constraints, Task 1: Module skeleton, YY-1 Bootstrap Implementation Plan

### Community 24 - "YY-6 CLI — design"
Cohesion: 0.20
Nodes (9): CLI structure (`cli/mod.rs`), Issue key format & resolution, Scope, Status target, Subcommand aliases, Testing, Time parsing, Work folder (+1 more)

### Community 25 - "Global Constraints"
Cohesion: 0.18
Nodes (10): Global Constraints, Self-Review Notes, Task 1: `core::start` accepts `note` and `tags`, Task 2: `core::last_working_day`, Task 3: `store/issues.rs` — resolve-or-create by key, Task 4: CLI issue key validation, Task 5: CLI time parsing, Task 6: CLI commands, rendering, and `run()` (+2 more)

## Knowledge Gaps
- **32 isolated node(s):** `setup-worktree.sh script`, `Why`, `Features`, `Install`, `Quick start` (+27 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **1 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

<<<<<<< HEAD
- **Why does `Entry` connect `Entry` to `jsonl.rs`, `active.rs`, `entries.rs`, `mod.rs`?**
  _High betweenness centrality (0.203) - this node is a cross-community bridge._
- **Why does `Id` connect `Entry` to `issues.rs`, `mod.rs`?**
  _High betweenness centrality (0.091) - this node is a cross-community bridge._
- **Why does `start()` connect `mod.rs` to `Entry`?**
  _High betweenness centrality (0.054) - this node is a cross-community bridge._
- **What connects `setup-worktree.sh script`, `Why`, `Features` to the rest of the system?**
  _32 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Entry` be split into smaller, more focused modules?**
=======
- **Why does `Entry` connect `model.rs` to `jsonl.rs`, `active.rs`, `entries.rs`, `mod.rs`?**
  _High betweenness centrality (0.203) - this node is a cross-community bridge._
- **Why does `Id` connect `model.rs` to `issues.rs`, `mod.rs`?**
  _High betweenness centrality (0.091) - this node is a cross-community bridge._
- **Why does `start()` connect `mod.rs` to `model.rs`?**
  _High betweenness centrality (0.054) - this node is a cross-community bridge._
- **What connects `setup-worktree.sh script`, `Why`, `Features` to the rest of the system?**
  _32 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `model.rs` be split into smaller, more focused modules?**
>>>>>>> 195853b (Add coverage tool)
  _Cohesion score 0.12962962962962962 - nodes in this community are weakly interconnected._