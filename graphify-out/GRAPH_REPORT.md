# Graph Report - yy  (2026-07-19)

## Corpus Check
- 28 files · ~13,856 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 240 nodes · 428 edges · 27 communities (26 shown, 1 thin omitted)
- Extraction: 100% EXTRACTED · 0% INFERRED · 0% AMBIGUOUS · INFERRED: 1 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `3a5a4abf`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- mod.rs
- mod.rs
- Entry
- entries.rs
- jsonl.rs
- active.rs
- issues.rs
- Global Constraints
- yy
- parse_time
- YY-6 CLI — design
- YY-1 Bootstrap — module skeleton
- yy
- work_folder
- Global Constraints
- setup-worktree.sh
- Architecture Conformance & SOLID / God-Structure Audit

## God Nodes (most connected - your core abstractions)
1. `Entry` - 25 edges
2. `Id` - 17 edges
3. `start()` - 16 edges
4. `Issue` - 13 edges
5. `stop()` - 11 edges
6. `resolve_or_create()` - 10 edges
7. `yy` - 10 edges
8. `run()` - 9 edges
9. `issue_labels()` - 9 edges
10. `close_and_archive()` - 9 edges

## Surprising Connections (you probably didn't know these)
- `render_today()` --references--> `TodayView`  [EXTRACTED]
  src/cli/mod.rs → src/core/mod.rs
- `render_today()` --references--> `Id`  [EXTRACTED]
  src/cli/mod.rs → src/model.rs
- `render_status()` --references--> `Entry`  [EXTRACTED]
  src/cli/mod.rs → src/model.rs
- `issue_labels()` --references--> `TodayView`  [EXTRACTED]
  src/cli/mod.rs → src/core/mod.rs
- `issue_labels()` --references--> `Id`  [EXTRACTED]
  src/cli/mod.rs → src/model.rs

## Import Cycles
- None detected.

## Communities (27 total, 1 thin omitted)

### Community 0 - "mod.rs"
Cohesion: 0.13
Nodes (23): Cli, Commands, EntrySpec, error_message_includes_hint(), format_duration(), issue_labels(), parse_issue_key(), render_status() (+15 more)

### Community 1 - "mod.rs"
Cohesion: 0.16
Nodes (24): close_and_archive(), IssueTotal, last_working_day(), DateTime, NaiveDate, Option, Path, PathBuf (+16 more)

### Community 2 - "Entry"
Cohesion: 0.13
Nodes (21): Default, Display, EntryTag, Formatter, Into, IssueTag, Self, custom_issue_kind_round_trips() (+13 more)

### Community 3 - "entries.rs"
Cohesion: 0.31
Nodes (15): append(), append_preserves_existing_comments_and_lines(), entries_in_range(), entries_in_range_spans_multiple_months(), entry_at(), missing_month_file_reads_no_entries(), month_path(), read_month() (+7 more)

### Community 4 - "jsonl.rs"
Cohesion: 0.30
Nodes (14): Line, parse(), parse_record(), read(), Record, render(), round_trips_comments_and_malformed_lines(), Path (+6 more)

### Community 5 - "active.rs"
Cohesion: 0.24
Nodes (13): Active, elapsed(), elapsed_of_running_entry_is_derived_from_now(), missing_file_reads_as_no_active_entry(), path(), read(), Option, PathBuf (+5 more)

### Community 6 - "issues.rs"
Cohesion: 0.35
Nodes (13): find_by_id(), find_by_id_returns_none_when_missing(), path(), read_all(), resolve_or_create(), resolve_or_create_creates_on_first_use(), resolve_or_create_falls_back_to_key_as_title_when_no_desc(), resolve_or_create_reuses_existing_key() (+5 more)

### Community 7 - "Global Constraints"
Cohesion: 0.18
Nodes (10): Global Constraints, Self-Review Notes, Task 1: `core::start` accepts `note` and `tags`, Task 2: `core::last_working_day`, Task 3: `store/issues.rs` — resolve-or-create by key, Task 4: CLI issue key validation, Task 5: CLI time parsing, Task 6: CLI commands, rendering, and `run()` (+2 more)

### Community 8 - "yy"
Cohesion: 0.18
Nodes (10): Acknowledgments, Data & config, Editing, Features, How it works, Install, License, Quick start (+2 more)

### Community 9 - "parse_time"
Cohesion: 0.27
Nodes (8): a_date(), parse_time(), parses_full_rfc3339_timestamp(), parses_hh_mm_on_reference_date_as_local_then_converts_to_utc(), DateTime, NaiveDate, Result, Utc

### Community 10 - "YY-6 CLI — design"
Cohesion: 0.20
Nodes (9): CLI structure (`cli/mod.rs`), Issue key format & resolution, Scope, Status target, Subcommand aliases, Testing, Time parsing, Work folder (+1 more)

### Community 11 - "YY-1 Bootstrap — module skeleton"
Cohesion: 0.29
Nodes (6): Done when, Goal, Out of scope, Scope, Testing, YY-1 Bootstrap — module skeleton

### Community 12 - "yy"
Cohesion: 0.53
Nodes (5): TempDir, String, start_status_today_stop_end_to_end(), status_with_nothing_running_reports_no_active_entry(), yy()

### Community 13 - "work_folder"
Cohesion: 0.40
Nodes (3): PathBuf, work_folder(), main()

### Community 14 - "Global Constraints"
Cohesion: 0.50
Nodes (3): Global Constraints, Task 1: Module skeleton, YY-1 Bootstrap Implementation Plan

### Community 38 - "Architecture Conformance & SOLID / God-Structure Audit"
Cohesion: 0.17
Nodes (11): Architecture Conformance & SOLID / God-Structure Audit, architecture.md vs. actual code — deltas found (and handled), Global Constraints, God structures vs. god modules — the distinction that matters here, Pre-Sprint-1 Refactor Implementation Plan, Self-Review Notes, SOLID assessment, Task 1: Extract `cli/parse.rs` (issue-key validation) and DRY the error string (+3 more)

## Knowledge Gaps
- **41 isolated node(s):** `setup-worktree.sh script`, `Why`, `Features`, `Install`, `Quick start` (+36 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **1 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Entry` connect `Entry` to `mod.rs`, `mod.rs`, `entries.rs`, `jsonl.rs`, `active.rs`?**
  _High betweenness centrality (0.158) - this node is a cross-community bridge._
- **Why does `Id` connect `Entry` to `mod.rs`, `mod.rs`, `issues.rs`?**
  _High betweenness centrality (0.078) - this node is a cross-community bridge._
- **Why does `start()` connect `mod.rs` to `Entry`?**
  _High betweenness centrality (0.034) - this node is a cross-community bridge._
- **What connects `setup-worktree.sh script`, `Why`, `Features` to the rest of the system?**
  _41 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `mod.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.13333333333333333 - nodes in this community are weakly interconnected._
- **Should `Entry` be split into smaller, more focused modules?**
  _Cohesion score 0.12962962962962962 - nodes in this community are weakly interconnected._