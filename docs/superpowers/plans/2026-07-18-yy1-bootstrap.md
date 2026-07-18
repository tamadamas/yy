# YY-1 Bootstrap Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Lay down the `src/` module tree from `.local/architecture.md` §7 so later
issues (YY-2..YY-6) have a place to land. No logic, no structs, no functions —
just modules that compile.

**Architecture:** Every new file is a module stub containing only an inner doc
comment (`//!`) describing its future purpose, copied from architecture.md §7.
Parent modules (`store/mod.rs`, `tui/mod.rs`) declare their children with
`mod x;`. `main.rs` declares the top-level modules and keeps its current
`println!("Hello, world!")` — no wiring beyond `mod` declarations.

**Tech Stack:** Rust 2024 edition, no new dependencies (Cargo.toml unchanged).

## Global Constraints

- Module tree must match `.local/architecture.md` §7 exactly (see spec:
  `docs/superpowers/specs/2026-07-18-yy1-bootstrap-design.md`).
- No structs, enums, functions, or logic — doc-comment-only stub files.
- `cargo build` must succeed.
- `cargo clippy --all-targets` must be warning-free.
- `cargo fmt --all --check` must pass.
- `core/` must not depend on `cli/` or `tui/` (not yet testable — no code
  exists in `core/` yet — but keep the doc comment's wording so YY-5 doesn't
  drift from it).

---

### Task 1: Module skeleton

**Files:**
- Modify: `src/main.rs`
- Create: `src/model.rs`
- Create: `src/store/mod.rs`
- Create: `src/store/jsonl.rs`
- Create: `src/store/entries.rs`
- Create: `src/store/issues.rs`
- Create: `src/store/active.rs`
- Create: `src/store/state.rs`
- Create: `src/core/mod.rs`
- Create: `src/pipeline.rs`
- Create: `src/cli/mod.rs`
- Create: `src/tui/mod.rs`
- Create: `src/tui/app.rs`
- Create: `src/tui/keymap.rs`
- Create: `src/tui/panes/mod.rs`
- Create: `src/watch/mod.rs`
- Create: `src/import/mod.rs`
- Create: `src/export/mod.rs`

**Interfaces:**
- Produces: the module tree itself (`crate::model`, `crate::store`,
  `crate::store::jsonl`, `crate::store::entries`, `crate::store::issues`,
  `crate::store::active`, `crate::store::state`, `crate::core`,
  `crate::pipeline`, `crate::cli`, `crate::tui`, `crate::tui::app`,
  `crate::tui::keymap`, `crate::tui::panes`, `crate::watch`, `crate::import`,
  `crate::export`) — every later issue (YY-2..YY-6) adds code inside these
  paths rather than creating new top-level modules.

- [ ] **Step 1: Create `src/model.rs`**

```rust
//! Issue, Entry, IssueKind, Id — plain data types (with `t`).
```

- [ ] **Step 2: Create the `store/` module files**

`src/store/mod.rs`:

```rust
//! Storage layer: JSONL read/write, entries, issues, active-set, state cache.

pub mod active;
pub mod entries;
pub mod issues;
pub mod jsonl;
pub mod state;
```

`src/store/jsonl.rs`:

```rust
//! Generic line read/write, atomic writes, type dispatch, error preservation.
```

`src/store/entries.rs`:

```rust
//! Monthly file resolution, date-range queries.
```

`src/store/issues.rs`:

```rust
//! Issue storage.
```

`src/store/active.rs`:

```rust
//! Active-set (the currently running entry).
```

`src/store/state.rs`:

```rust
//! state.json cache (last_seen, today total).
```

- [ ] **Step 3: Create `src/core/mod.rs` and `src/pipeline.rs`**

`src/core/mod.rs`:

```rust
//! Use cases: start, stop, assign, split, merge, report, gaps, audits.
//! Exposes a DTO/query layer. MUST NOT depend on `tui` or `cli`.
```

`src/pipeline.rs`:

```rust
//! Classifiers (desc -> kind/issue/tags/exclusive) + audits -> need_review.
```

- [ ] **Step 4: Create `src/cli/mod.rs`**

```rust
//! clap parsing; thin wrapper around `core`; shared entry-spec flags.
```

- [ ] **Step 5: Create the `tui/` module files**

`src/tui/mod.rs`:

```rust
//! Tokyo Night TUI: state machine, keymap, panes.

pub mod app;
pub mod keymap;
pub mod panes;
```

`src/tui/app.rs`:

```rust
//! State machine.
```

`src/tui/keymap.rs`:

```rust
//! Keymap.
```

`src/tui/panes/mod.rs`:

```rust
//! Issue list, timeline, detail, status bar panes.
```

- [ ] **Step 6: Create `src/watch/mod.rs`, `src/import/mod.rs`, `src/export/mod.rs`**

`src/watch/mod.rs`:

```rust
//! tick: break reminders, daily-limit warnings, idle/sleep detection.
```

`src/import/mod.rs`:

```rust
//! bartib plaintext, pasted HTML block import.
```

`src/export/mod.rs`:

```rust
//! json + formatters; invoice-style PDF (later).
```

- [ ] **Step 7: Wire the top-level modules into `src/main.rs`**

```rust
mod cli;
mod core;
mod export;
mod import;
mod model;
mod pipeline;
mod store;
mod tui;
mod watch;

fn main() {
    println!("Hello, world!");
}
```

- [ ] **Step 8: Verify the build**

Run: `cargo build`
Expected: `Compiling yy v0.1.0 (...)` then `Finished` with no errors and no
warnings (an empty `mod x;` with only a doc comment produces none — nothing
is unused because the module itself is reachable from `main`).

- [ ] **Step 9: Verify clippy is clean**

Run: `cargo clippy --all-targets`
Expected: `Finished` with no warnings.

- [ ] **Step 10: Verify formatting**

Run: `cargo fmt --all --check`
Expected: no output, exit code 0. If it fails, run `cargo fmt --all` and
re-check.

- [ ] **Step 11: Commit**

```bash
git add src/
git commit -m "feat: add module skeleton per architecture.md §7 (YY-1)"
```
