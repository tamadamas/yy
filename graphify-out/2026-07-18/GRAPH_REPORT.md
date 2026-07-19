# Graph Report - yy  (2026-07-18)

## Corpus Check
<<<<<<< HEAD
- 21 files · ~1,212 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 39 nodes · 18 edges · 21 communities (20 shown, 1 thin omitted)
=======
- 24 files · ~8,935 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 169 nodes · 283 edges · 30 communities (23 shown, 7 thin omitted)
>>>>>>> a084715 (Update graphify)
- Extraction: 100% EXTRACTED · 0% INFERRED · 0% AMBIGUOUS
- Token cost: 0 input · 0 output

## Graph Freshness
<<<<<<< HEAD
- Built from commit: `6a3bc595`
=======
- Built from commit: `3e488f2c`
>>>>>>> a084715 (Update graphify)
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- yy
- YY-1 Bootstrap — module skeleton
- setup-worktree.sh
<<<<<<< HEAD

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
=======
- mod.rs
- model.rs
- active.rs
- entries.rs
- jsonl.rs
- Global Constraints
- NaiveDate
- Vec
- YY-6 CLI — design
- Global Constraints
- Result
- String
- Result
- Vec

## God Nodes (most connected - your core abstractions)
1. `start()` - 16 edges
2. `stop()` - 11 edges
3. `Id` - 10 edges
4. `Issue` - 10 edges
5. `Entry` - 10 edges
6. `yy` - 10 edges
7. `close_and_archive()` - 9 edges
8. `YY-6 CLI — design` - 9 edges
9. `month_path()` - 9 edges
10. `read_month()` - 9 edges
>>>>>>> a084715 (Update graphify)

## Surprising Connections (you probably didn't know these)
- `Record` --references--> `Issue`  [EXTRACTED]
  src/store/jsonl.rs → src/model.rs
- `Record` --references--> `Entry`  [EXTRACTED]
  src/store/jsonl.rs → src/model.rs

## Import Cycles
- None detected.

<<<<<<< HEAD
## Communities (21 total, 1 thin omitted)
=======
## Communities (30 total, 7 thin omitted)
>>>>>>> a084715 (Update graphify)

### Community 1 - "yy"
Cohesion: 0.18
Nodes (10): Acknowledgments, Data & config, Editing, Features, How it works, Install, License, Quick start (+2 more)

### Community 2 - "YY-1 Bootstrap — module skeleton"
Cohesion: 0.29
Nodes (6): Done when, Goal, Out of scope, Scope, Testing, YY-1 Bootstrap — module skeleton

<<<<<<< HEAD
## Knowledge Gaps
- **15 isolated node(s):** `Goal`, `Scope`, `Out of scope`, `Done when`, `Testing` (+10 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **1 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.
=======
### Community 5 - "mod.rs"
Cohesion: 0.16
Nodes (25): DateTime, Entry, Id, NaiveDate, Option, Path, PathBuf, Result (+17 more)

### Community 8 - "model.rs"
Cohesion: 0.14
Nodes (18): Default, Display, EntryTag, Formatter, Into, IssueTag, Self, custom_issue_kind_round_trips() (+10 more)

### Community 10 - "active.rs"
Cohesion: 0.26
Nodes (13): Active, elapsed(), elapsed_of_running_entry_is_derived_from_now(), missing_file_reads_as_no_active_entry(), path(), read(), Entry, PathBuf (+5 more)

### Community 11 - "entries.rs"
Cohesion: 0.31
Nodes (16): append(), append_preserves_existing_comments_and_lines(), entries_in_range(), entries_in_range_spans_multiple_months(), entry_at(), missing_month_file_reads_no_entries(), month_path(), read_month() (+8 more)

### Community 13 - "jsonl.rs"
Cohesion: 0.32
Nodes (13): Line, parse(), parse_record(), read(), Record, render(), round_trips_comments_and_malformed_lines(), Result (+5 more)

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
- **32 isolated node(s):** `Task 1: `core::start` accepts `note` and `tags``, `Task 2: `core::last_working_day``, `Task 3: `store/issues.rs` — resolve-or-create by key`, `Task 4: CLI issue key validation`, `Task 5: CLI time parsing` (+27 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **7 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.
>>>>>>> a084715 (Update graphify)

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

<<<<<<< HEAD
- **What connects `Goal`, `Scope`, `Out of scope` to the rest of the system?**
  _15 weakly-connected nodes found - possible documentation gaps or missing edges._
=======
- **Why does `Entry` connect `model.rs` to `jsonl.rs`, `mod.rs`?**
  _High betweenness centrality (0.033) - this node is a cross-community bridge._
- **Why does `Issue` connect `model.rs` to `jsonl.rs`, `mod.rs`?**
  _High betweenness centrality (0.033) - this node is a cross-community bridge._
- **What connects `Task 1: `core::start` accepts `note` and `tags``, `Task 2: `core::last_working_day``, `Task 3: `store/issues.rs` — resolve-or-create by key` to the rest of the system?**
  _32 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `model.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.14 - nodes in this community are weakly interconnected._
>>>>>>> a084715 (Update graphify)
