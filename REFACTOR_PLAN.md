# Pre-Sprint-1 Refactor Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Split the two lowest-cohesion god-files (`src/cli/mod.rs`, `src/core/mod.rs`) into focused modules that match architecture §7, and DRY one duplicated error string — pure structural cleanup, zero behavior change, before Sprint 1 grows `core/` and `cli/`.

**Architecture:** Graphify's `cluster-only` pass (commit `7849b35`) flags community 0 `cli/mod.rs` at cohesion **0.13** (23 nodes mixing clap surface + key validation + text rendering + orchestration) and community 1 `core/mod.rs` at **0.16** (24 nodes mixing tracking use-cases + the DTO/query layer). Architecture §7 already names the intended seams: `cli/` is a *"thin wrapper around core/; shared entry-spec flags"* and `core/` *"Exposes a DTO/query layer."* This plan extracts `cli/parse.rs` + `cli/render.rs` and `core/query.rs`, leaving each `mod.rs` as a slim orchestration/re-export hub. `cli/time.rs` already exists and is untouched.

**Tech Stack:** Rust 2024, clap 4 (derive), chrono, anyhow, ulid, serde_json — no new dependencies. `duct` + `assert_fs` + `assertables` dev-deps for the existing e2e suite.

## Global Constraints

- **Pure refactor — no behavior change.** The existing suite (48 tests: lib unit + `core`/`cli`/`store` unit + `tests/cli.rs` e2e) is the safety net. After every task, `rtk cargo test` MUST show the same count green. Tests move with the code they cover; none are deleted or weakened.
- **Golden rule 6 (core independence):** `core/` MUST NOT depend on `cli/` or `tui/`. The `core/query.rs` extraction must keep every `use` inside `core::` or `crate::{model,store}` — never `crate::cli`.
- **Golden rules 1–5, 7 hold unchanged:** derived time, plain-text/local, one-shot/no-daemon, preserve-don't-drop, atomic writes, `t` discriminator on every record. No storage/timing/format code is touched by this plan.
- **Tooling (user feedback [[feedback-use-just-rtk-commands]]):** use `just check` and `rtk cargo <cmd>` / `rtk git <cmd>` — never bare `cargo`/`git`. `just check` runs fmt-check + clippy + test + build.
- `cargo fmt` clean and `cargo clippy` warning-free before any task is done (`just check` enforces both).
- Module layout target is `.local/architecture.md` §7 (updated this session to document the `lib.rs` bin/lib split and the `core/query.rs` + `cli/{parse,render}.rs` seams this plan carves). Do not invent new top-level modules; only split the two flagged files along the seams §7 already names.

---

## Architecture Conformance & SOLID / God-Structure Audit

Ran before writing the tasks, to confirm the refactor targets the real problems and nothing else.

### architecture.md vs. actual code — deltas found (and handled)

| Spec (`.local/architecture.md`) | Actual code | Verdict |
|---|---|---|
| Status: "planning — no code written yet" | YY-1…YY-6 built, 48 tests green | **Stale — fixed this session** (status now says Sprint 0 MVP built). |
| §7 layout: `main.rs` only | `main.rs` **+ `lib.rs`** (bin/lib split for `tests/cli.rs`) | **Undocumented — fixed this session** (§7 now shows `lib.rs` + `tests/`). |
| §7: `core/` one unit; `cli/` "thin wrapper" | `core/mod.rs` 297 L (tracking **+** query/DTO); `cli/mod.rs` 329 L (clap **+** parse **+** render **+** run) | **Real SRP violation → Tasks 1–3.** §7 updated to name the `query`/`parse`/`render` seams. |
| §6 active-set: exclusive + parallel meetings + `--gap` | `active.rs` = single `Option<Entry>` | **MVP deferral (Sprint 1 YY-8), not a defect.** Noted in §7 MVP-delta. No action. |
| §10 config: XDG + `$YY_WORK_FOLDER` + `--work-folder` | `lib.rs::work_folder()` hardcodes `$HOME/.yy_logs` | **MVP deferral (Sprint 1 YY-7), not a defect.** No action. |
| §11 CLI: `--desc` flag; `status [date\|--week]` | `start` takes positional `[desc]`; `status` no args | **MVP subset, intentional.** No action. |

### SOLID assessment

- **SRP (the one that bites):** `cli/mod.rs` and `core/mod.rs` each carry multiple reasons to change → **exactly the two files this plan splits.** After the split every file has one axis of change (clap surface, key validation, rendering, tracking, query).
- **OCP:** `store/jsonl.rs` already models this well — `Record::Unknown(Value)` preserves an unrecognized `t` and `Line::Malformed` preserves unparseable lines, so a new record type round-trips through an older binary without editing existing arms. **No change; keep as the pattern for future record types.**
- **DIP:** `core/` calls concrete `store::{entries,active,issues}` functions directly (no trait seam). For a single-developer, single-backend, no-daemon app this is **correct YAGNI** — introducing repository traits now would add indirection with zero second implementation. Left as-is deliberately; revisit only if a second store backend ever appears (none is planned — §12/§13).
- **LSP / ISP:** not applicable — no inheritance and no wide trait interfaces in the codebase.

### God structures vs. god modules — the distinction that matters here

Graphify's "god nodes" (`Entry` 25 edges, `Id` 17, `start()` 16) are **high fan-in domain types, not god objects.** `Entry`/`Id` are pure data (`Id` is a `ULID` newtype; `Entry` is fields + `start_now`) — a central domain entity *should* be widely referenced, and splitting it would scatter one cohesive concept. **Explicitly out of scope: do not touch `model.rs`, `Entry`, `Id`, or `jsonl.rs`.** The genuine god *modules* are `cli/mod.rs` and `core/mod.rs` (many unrelated symbols in one file, low cohesion 0.13 / 0.16) — those, and only those, are what Tasks 1–3 break up.

---

### Task 1: Extract `cli/parse.rs` (issue-key validation) and DRY the error string

**Files:**
- Create: `src/cli/parse.rs`
- Modify: `src/cli/mod.rs` (remove `parse_issue_key` + its 6 tests; add `mod parse;` + re-export)

**Interfaces:**
- Consumes: nothing new.
- Produces: `pub fn parse_issue_key(s: &str) -> Result<String, String>` — re-exported from `cli` so `EntrySpec`'s `#[arg(value_parser = parse_issue_key)]` (still in `mod.rs`) resolves it, and `main`/tests keep the same path.

- [ ] **Step 1: Create `src/cli/parse.rs` with the moved function, error string DRY'd**

The current code repeats the identical `format!(...)` in both the `else` guard and the final branch. Collapse it into one `reject` closure. Move the 6 existing tests verbatim.

```rust
//! Issue-key validation for the clap `value_parser` on `EntrySpec::issue`.

/// Validates an issue key: two or more uppercase letters, a dash, then one
/// or more digits (e.g. `YY-1`, `DFG-1234`, `KJJ-2`).
pub fn parse_issue_key(s: &str) -> Result<String, String> {
    let reject = || format!("issue key must be LETTERS-NUMBER, e.g. YY-1 (got \"{s}\")");

    let Some((prefix, suffix)) = s.split_once('-') else {
        return Err(reject());
    };

    let prefix_ok = prefix.len() >= 2 && prefix.chars().all(|c| c.is_ascii_uppercase());
    let suffix_ok = !suffix.is_empty() && suffix.chars().all(|c| c.is_ascii_digit());

    if prefix_ok && suffix_ok {
        Ok(s.to_string())
    } else {
        Err(reject())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_valid_keys() {
        assert_eq!(parse_issue_key("YY-1"), Ok("YY-1".to_string()));
        assert_eq!(parse_issue_key("DFG-1234"), Ok("DFG-1234".to_string()));
        assert_eq!(parse_issue_key("KJJ-2"), Ok("KJJ-2".to_string()));
    }

    #[test]
    fn rejects_single_letter_prefix() {
        assert!(parse_issue_key("Y-1").is_err());
    }

    #[test]
    fn rejects_lowercase() {
        assert!(parse_issue_key("yy-1").is_err());
    }

    #[test]
    fn rejects_missing_number() {
        assert!(parse_issue_key("YY-").is_err());
        assert!(parse_issue_key("YY").is_err());
    }

    #[test]
    fn rejects_non_numeric_suffix() {
        assert!(parse_issue_key("YY-1a").is_err());
    }

    #[test]
    fn error_message_includes_hint() {
        let err = parse_issue_key("bad").unwrap_err();
        assert!(
            err.contains("YY-1"),
            "error should hint at valid format: {err}"
        );
    }
}
```

- [ ] **Step 2: Remove `parse_issue_key` and its 6 tests from `src/cli/mod.rs`**

Delete the `parse_issue_key` fn (currently lines ~14–33) and the six tests (`accepts_valid_keys`, `rejects_single_letter_prefix`, `rejects_lowercase`, `rejects_missing_number`, `rejects_non_numeric_suffix`, `error_message_includes_hint`) from the `#[cfg(test)] mod tests` block. Then add the module declaration and re-export near the top of `src/cli/mod.rs`, alongside the existing `mod time;`:

```rust
mod parse;
mod time;

pub use parse::parse_issue_key;
```

Leave the `#[arg(long, value_parser = parse_issue_key)]` attribute on `EntrySpec::issue` unchanged — the re-export keeps `parse_issue_key` in scope.

- [ ] **Step 3: Verify the suite is still green**

Run: `rtk cargo test`
Expected: same total as before this task (48), all green. The 6 parse tests now run from `cli::parse::tests` instead of `cli::tests`.

- [ ] **Step 4: Full check**

Run: `just check`
Expected: fmt clean, clippy warning-free, tests pass, build ok.

- [ ] **Step 5: Commit**

```bash
rtk git add src/cli/parse.rs src/cli/mod.rs
rtk git commit -m "refactor: extract cli::parse, DRY issue-key error string"
```

---

### Task 2: Extract `cli/render.rs` (duration + today/status rendering)

**Files:**
- Create: `src/cli/render.rs`
- Modify: `src/cli/mod.rs` (remove `format_duration`, `DAILY_TARGET`, `render_today`, `render_status` + their 4 tests + `sample_entry` helper; add `mod render;` + `use`)

**Interfaces:**
- Consumes: `core::{IssueTotal, TodayView}`, `model::{Entry, Id}`, `store::active` (all unchanged).
- Produces:
  - `pub fn render_today(view: &TodayView, issue_labels: &[(Option<Id>, String)]) -> String`
  - `pub fn render_status(active_entry: &Option<Entry>, worked_today: TimeDelta) -> String`
  Both are called by `run()` in `mod.rs`. `format_duration` and `DAILY_TARGET` become private to `render.rs` (only used there).

- [ ] **Step 1: Create `src/cli/render.rs` with the moved rendering code and its tests**

Move `format_duration`, `DAILY_TARGET`, `render_today`, `render_status` verbatim; they only ever get called from `render.rs` and `run()`, so `format_duration`/`DAILY_TARGET` drop to private. Move the 4 render tests and the `sample_entry` helper they share.

```rust
//! Text rendering for CLI output: durations, the today/yesterday view, and
//! `yy status`. Pure formatting over `core` DTOs — no I/O, no clock reads
//! except the derived-elapsed call on a running entry.

use chrono::TimeDelta;

use crate::core::{IssueTotal, TodayView};
use crate::model::{Entry, Id};
use crate::store::active;

fn format_duration(d: TimeDelta) -> String {
    let total_minutes = d.num_minutes();
    format!("{}h{:02}m", total_minutes / 60, (total_minutes % 60).abs())
}

/// Render the today/yesterday combined view. `issue_labels` maps each entry's
/// `issue_id` to a display label (its key, or the raw id if unresolved) —
/// built by `run()` via `store::issues::find_by_id` before calling this.
pub fn render_today(view: &TodayView, issue_labels: &[(Option<Id>, String)]) -> String {
    let label_for = |issue_id: Option<Id>| -> String {
        issue_labels
            .iter()
            .find(|(id, _)| *id == issue_id)
            .map(|(_, label)| label.clone())
            .unwrap_or_else(|| "(no issue)".to_string())
    };

    if view.entries.is_empty() {
        return "no entries today\n".to_string();
    }

    let mut out = String::new();
    for entry in &view.entries {
        let desc = entry.note.as_deref().unwrap_or("(no desc)");
        let elapsed = active::elapsed(entry);
        out.push_str(&format!(
            "{}  {}  {}  {}\n",
            entry.start.format("%H:%M"),
            label_for(entry.issue_id),
            desc,
            format_duration(elapsed)
        ));
    }

    out.push_str("\ntotals:\n");
    for IssueTotal { issue_id, elapsed } in &view.totals {
        out.push_str(&format!(
            "  {}  {}\n",
            label_for(*issue_id),
            format_duration(*elapsed)
        ));
    }

    out
}

const DAILY_TARGET: TimeDelta = TimeDelta::hours(8);

/// Render `yy status`: current task, worked so far, remaining vs an 8h target.
pub fn render_status(active_entry: &Option<Entry>, worked_today: TimeDelta) -> String {
    let current = match active_entry {
        Some(entry) => entry.note.as_deref().unwrap_or("(no desc)").to_string(),
        None => "no active entry".to_string(),
    };

    let remaining = DAILY_TARGET - worked_today;
    format!(
        "current: {current}\nworked: {}\nremaining: {}\n",
        format_duration(worked_today),
        format_duration(remaining)
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeDelta as TD;

    fn sample_entry(issue_id: Option<Id>, note: Option<&str>) -> Entry {
        let mut e = Entry::start_now(issue_id);
        e.note = note.map(str::to_string);
        e
    }

    #[test]
    fn render_today_shows_entries_and_totals() {
        let issue = Id::new();
        let view = TodayView {
            entries: vec![sample_entry(Some(issue), Some("wrote tests"))],
            totals: vec![IssueTotal {
                issue_id: Some(issue),
                elapsed: TD::minutes(45),
            }],
        };
        let out = render_today(&view, &[]);
        assert!(out.contains("wrote tests"));
        assert!(out.contains("45m") || out.contains("0h45m"));
    }

    #[test]
    fn render_today_handles_empty_day() {
        let view = TodayView {
            entries: Vec::new(),
            totals: Vec::new(),
        };
        let out = render_today(&view, &[]);
        assert!(out.contains("no entries") || out.contains("nothing"));
    }

    #[test]
    fn render_status_shows_active_note_and_remaining() {
        let active = Some(sample_entry(None, Some("deep work")));
        let out = render_status(&active, TD::hours(2));
        assert!(out.contains("deep work"));
        assert!(out.contains("2h") || out.contains("02:00"));
        assert!(out.contains("6h") || out.contains("06:00")); // 8h target - 2h worked
    }

    #[test]
    fn render_status_handles_nothing_running() {
        let out = render_status(&None, TD::zero());
        assert!(out.contains("no active entry") || out.contains("nothing running"));
    }
}
```

- [ ] **Step 2: Remove the moved items from `src/cli/mod.rs` and wire the module**

Delete `format_duration`, `DAILY_TARGET`, `render_today`, `render_status`, and — from the test module — `sample_entry` plus the four `render_*` tests. Add `mod render;` next to `mod parse;`/`mod time;`, and bring the two public fns into scope for `run()`:

```rust
mod parse;
mod render;
mod time;

pub use parse::parse_issue_key;
use render::{render_status, render_today};
```

`run()`'s call sites (`render_today(&view, &labels)`, `render_status(&active_entry, worked_today)`) stay byte-for-byte identical. `mod.rs` keeps its own `use chrono::{NaiveDate, TimeDelta, Utc};` — `TimeDelta` is still needed for the `worked_today` fold in the `Status` arm, and `active` for `active::read`.

- [ ] **Step 3: Verify the suite is still green**

Run: `rtk cargo test`
Expected: 48 green. The 4 render tests now run from `cli::render::tests`.

- [ ] **Step 4: Full check**

Run: `just check`
Expected: fmt clean, clippy warning-free (watch for a now-unused `IssueTotal`/`Id` import left in `mod.rs` — if clippy flags one, remove it), tests pass, build ok.

- [ ] **Step 5: Commit**

```bash
rtk git add src/cli/render.rs src/cli/mod.rs
rtk git commit -m "refactor: extract cli::render (duration + today/status)"
```

---

### Task 3: Extract `core/query.rs` (the DTO/query layer)

**Files:**
- Create: `src/core/query.rs`
- Modify: `src/core/mod.rs` (remove `IssueTotal`, `TodayView`, `last_working_day`, `today` + their 6 tests; add `mod query;` + re-export)

**Interfaces:**
- Consumes: `model::{Entry, Id}`, `store::{active, entries}`, and `crate::core::{start, stop}` (test-only, for the `today_*` setup) — all unchanged.
- Produces (re-exported from `core` so `cli::run()` and tests keep `core::TodayView` etc. working unchanged):
  - `pub struct IssueTotal { pub issue_id: Option<Id>, pub elapsed: TimeDelta }`
  - `pub struct TodayView { pub entries: Vec<Entry>, pub totals: Vec<IssueTotal> }`
  - `pub fn last_working_day(date: NaiveDate) -> NaiveDate`
  - `pub fn today(work_folder: &Path, date: NaiveDate) -> anyhow::Result<TodayView>`

- [ ] **Step 1: Create `src/core/query.rs` with the moved DTOs, query, and their tests**

This is architecture §7's *"DTO/query layer."* Move `IssueTotal`, `TodayView`, `last_working_day`, `today` and their 6 tests. Keep every dependency inside `core`/`crate::{model,store}` (golden rule 6). The `today_*` tests set up state via `start`/`stop`, imported from the parent module.

```rust
//! The DTO / query layer (architecture §7): read-only aggregation over stored
//! entries. Returns plain data (`TodayView`, `IssueTotal`) for render-agnostic
//! consumers — MUST NOT depend on `cli/` or `tui/`.

use std::collections::BTreeMap;
use std::path::Path;

use chrono::{Datelike, NaiveDate, TimeDelta, Weekday};

use crate::model::{Entry, Id};
use crate::store::{active, entries};

/// Per-issue total elapsed time for a set of entries.
#[derive(Debug, Clone, PartialEq)]
pub struct IssueTotal {
    pub issue_id: Option<Id>,
    pub elapsed: TimeDelta,
}

/// Today's combined view: the chronological timeline and per-issue totals.
#[derive(Debug, Clone, PartialEq)]
pub struct TodayView {
    pub entries: Vec<Entry>,
    pub totals: Vec<IssueTotal>,
}

/// The last working day before `date`: Monday resolves to the preceding
/// Friday; Saturday/Sunday resolve to the preceding Friday; any other day
/// resolves to `date - 1 day`.
pub fn last_working_day(date: NaiveDate) -> NaiveDate {
    match date.weekday() {
        Weekday::Mon => date - TimeDelta::days(3),
        Weekday::Sun => date - TimeDelta::days(2),
        _ => date - TimeDelta::days(1),
    }
}

/// Aggregate `date`'s entries: closed entries from the monthly file plus the active
/// entry if it started on `date`, sorted chronologically, with per-issue totals.
pub fn today(work_folder: &Path, date: NaiveDate) -> anyhow::Result<TodayView> {
    let mut day_entries = entries::read_month(work_folder, date)?
        .into_iter()
        .filter(|e| e.start.date_naive() == date)
        .collect::<Vec<_>>();

    if let Some(running) = active::read(work_folder)?.entry
        && running.start.date_naive() == date
    {
        day_entries.push(running);
    }

    day_entries.sort_by_key(|e| e.start);

    let mut totals: BTreeMap<Option<Id>, TimeDelta> = BTreeMap::new();
    for entry in &day_entries {
        *totals.entry(entry.issue_id).or_insert_with(TimeDelta::zero) += active::elapsed(entry);
    }

    Ok(TodayView {
        entries: day_entries,
        totals: totals
            .into_iter()
            .map(|(issue_id, elapsed)| IssueTotal { issue_id, elapsed })
            .collect(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{start, stop};
    use chrono::{TimeZone, Utc};

    fn tmp_dir(label: &str) -> std::path::PathBuf {
        std::env::temp_dir().join(format!("yy-query-test-{label}-{}", ulid::Ulid::generate()))
    }

    #[test]
    fn today_combines_closed_entries_and_running_entry() {
        let work_folder = tmp_dir("today");
        let date = NaiveDate::from_ymd_opt(2026, 3, 7).unwrap();
        let issue = Id::new();

        let closed_start = Utc.with_ymd_and_hms(2026, 3, 7, 9, 0, 0).unwrap();
        let closed_stop = Utc.with_ymd_and_hms(2026, 3, 7, 9, 30, 0).unwrap();
        start(
            &work_folder,
            Some(issue),
            None,
            Vec::new(),
            Some(closed_start),
        )
        .unwrap();
        stop(&work_folder, Some(closed_stop)).unwrap();

        let running_start = Utc.with_ymd_and_hms(2026, 3, 7, 10, 0, 0).unwrap();
        start(
            &work_folder,
            Some(issue),
            None,
            Vec::new(),
            Some(running_start),
        )
        .unwrap();

        let view = today(&work_folder, date).unwrap();
        assert_eq!(view.entries.len(), 2);
        assert_eq!(view.entries[0].start, closed_start);
        assert_eq!(view.entries[1].start, running_start);

        assert_eq!(view.totals.len(), 1);
        assert_eq!(view.totals[0].issue_id, Some(issue));
        assert!(view.totals[0].elapsed >= TimeDelta::minutes(30));

        std::fs::remove_dir_all(&work_folder).unwrap();
    }

    #[test]
    fn today_ignores_entries_from_other_days() {
        let work_folder = tmp_dir("today-filter");
        let date = NaiveDate::from_ymd_opt(2026, 3, 7).unwrap();

        let other_day = Utc.with_ymd_and_hms(2026, 3, 6, 9, 0, 0).unwrap();
        start(&work_folder, None, None, Vec::new(), Some(other_day)).unwrap();
        stop(
            &work_folder,
            Some(Utc.with_ymd_and_hms(2026, 3, 6, 9, 30, 0).unwrap()),
        )
        .unwrap();

        let view = today(&work_folder, date).unwrap();
        assert_eq!(view.entries, Vec::new());
        assert_eq!(view.totals, Vec::new());

        std::fs::remove_dir_all(&work_folder).unwrap();
    }

    #[test]
    fn last_working_day_from_monday_is_friday() {
        let monday = NaiveDate::from_ymd_opt(2026, 3, 9).unwrap(); // a Monday
        assert_eq!(
            last_working_day(monday),
            NaiveDate::from_ymd_opt(2026, 3, 6).unwrap() // the preceding Friday
        );
    }

    #[test]
    fn last_working_day_from_tuesday_is_monday() {
        let tuesday = NaiveDate::from_ymd_opt(2026, 3, 10).unwrap();
        assert_eq!(
            last_working_day(tuesday),
            NaiveDate::from_ymd_opt(2026, 3, 9).unwrap()
        );
    }

    #[test]
    fn last_working_day_from_sunday_is_friday() {
        let sunday = NaiveDate::from_ymd_opt(2026, 3, 8).unwrap();
        assert_eq!(
            last_working_day(sunday),
            NaiveDate::from_ymd_opt(2026, 3, 6).unwrap()
        );
    }

    #[test]
    fn last_working_day_from_saturday_is_friday() {
        let saturday = NaiveDate::from_ymd_opt(2026, 3, 7).unwrap();
        assert_eq!(
            last_working_day(saturday),
            NaiveDate::from_ymd_opt(2026, 3, 6).unwrap()
        );
    }
}
```

- [ ] **Step 2: Slim `src/core/mod.rs` to the tracking use-cases + re-export**

Delete `IssueTotal`, `TodayView`, `last_working_day`, `today`, and the two `today_*` tests from `core/mod.rs`. Keep `start`, `stop`, `close_and_archive` and their tests (`start_then_stop_produces_closed_interval`, `starting_again_stops_the_previous_entry`, `stop_with_nothing_running_returns_none`, `start_sets_note_and_tags`). Add the module + re-export at the top, and drop now-unused imports.

The top of `core/mod.rs` becomes:

```rust
//! Use cases: start, stop, assign, split, merge, report, gaps, audits.
//! Read-only aggregation lives in the `query` submodule (the DTO/query layer).
//! MUST NOT depend on `tui` or `cli`.

use std::path::Path;

use chrono::{DateTime, Utc};

use crate::model::{Entry, Id};
use crate::store::{active, entries};

mod query;
pub use query::{IssueTotal, TodayView, last_working_day, today};
```

Note the import diff versus today's `core/mod.rs`: `start`/`stop`/`close_and_archive` no longer use `Datelike`, `NaiveDate`, `TimeDelta`, `Weekday`, or `BTreeMap`, so those leave `mod.rs` (they moved to `query.rs`). `DateTime` and `Utc` stay (used by `start`/`stop`/`close_and_archive` signatures and `Utc::now()`). Clippy will flag any that are wrong — let it guide the final import set.

- [ ] **Step 3: Verify the suite is still green**

Run: `rtk cargo test`
Expected: 48 green. `today_*` and `last_working_day_*` now run from `core::query::tests`; the `cli` re-export path (`core::TodayView`, `core::last_working_day`) is unchanged, so `cli/mod.rs` and `tests/cli.rs` compile untouched.

- [ ] **Step 4: Full check**

Run: `just check`
Expected: fmt clean, clippy warning-free, tests pass, build ok. Confirm no `crate::cli` appears anywhere under `src/core/` (golden rule 6): `rtk grep "crate::cli" src/core` should return nothing.

- [ ] **Step 5: Commit**

```bash
rtk git add src/core/query.rs src/core/mod.rs
rtk git commit -m "refactor: extract core::query DTO/query layer (arch §7)"
```

---

### Task 4: Refresh the graph and record the refactor

**Files:**
- Modify: `.local/architecture_notes.md` (decision-log entry)
- Regenerate: `graphify-out/` (no source change)
- Already done this session (do NOT redo): `.local/architecture.md` §7 was updated to
  document the `lib.rs` bin/lib split, the `tests/` dir, and the `core/query.rs` +
  `cli/{parse,render}.rs` seams; the stale "no code written yet" status line was
  corrected. Verify these are present rather than re-editing.

**Interfaces:** none — documentation + graph hygiene only.

- [ ] **Step 1: Rebuild the graph incrementally**

The four moved/renamed files are code-only, so no LLM key is needed.

Run: `graphify . --update --code-only`
Then: `graphify cluster-only .`
Expected: `cli/mod.rs` and `core/mod.rs` communities report higher cohesion than the 0.13 / 0.16 baseline (the mixed concerns are now separate communities). Capture the new numbers.

- [ ] **Step 2: Append a decision-log note**

Add a short dated entry to `.local/architecture_notes.md` recording: (a) the pre-Sprint-1 split of `cli/mod.rs` → `cli/{parse,render}.rs` and `core/mod.rs` → `core/query.rs`; (b) why — graphify cohesion 0.13/0.16, SRP, align to §7 seams before Sprint 1 grows both; (c) the before/after cohesion numbers from Step 1; (d) the architecture-conformance fixes already applied to `.local/architecture.md` this session (status line + §7 `lib.rs`/`tests/`/seams). Confirm the SOLID audit's "no action" calls (OCP via `Record::Unknown`, DIP-as-YAGNI, `Entry`/`Id`/`jsonl` out of scope) so a future reader doesn't re-litigate them. One or two paragraphs — match the existing note style.

- [ ] **Step 3: Verify nothing else drifted**

Run: `just check`
Expected: still green (this task changed no source).

- [ ] **Step 4: Commit**

```bash
rtk git add .local/architecture.md .local/architecture_notes.md graphify-out/
rtk git commit -m "docs: reconcile architecture.md with built MVP; record pre-Sprint-1 refactor; refresh graph"
```

---

## Self-Review Notes

- **Scope coverage:** The two files graphify flagged (`cli/mod.rs` 0.13, `core/mod.rs` 0.16) are both split along architecture §7's named seams (`cli/` thin wrapper + entry-spec; `core/` DTO/query layer). `store/*` (cohesion 0.22–0.35) and `jsonl.rs` (0.30) are already focused and are intentionally left alone — YAGNI. `core/mod.rs`'s remaining tracking trio (`start`/`stop`/`close_and_archive`) is cohesive and stays put; splitting it further would be premature before Sprint 1 defines `assign`/`split`/`merge`.
- **No behavior change:** every task's safety net is the existing 48-test suite; no test is deleted, only relocated with its code. Public paths (`core::TodayView`, `core::last_working_day`, `cli::parse_issue_key`, `cli::run`) are preserved via re-exports, so `main.rs`, `cli/mod.rs`, and `tests/cli.rs` compile unchanged.
- **Type consistency:** re-exported signatures in Task 3 (`today`, `last_working_day`, `TodayView`, `IssueTotal`) match their current definitions byte-for-byte; `render_today`/`render_status` signatures in Task 2 match `run()`'s existing call sites; `parse_issue_key` in Task 1 keeps its `Result<String, String>` shape for the clap `value_parser`.
- **Golden rule 6:** Task 3 Step 4 explicitly greps `src/core/` for `crate::cli` to prove core stays independent.
- **One real simplification beyond moves:** Task 1 collapses the duplicated error `format!` in `parse_issue_key` into a single `reject` closure — the only logic change in the plan, and it is covered by the existing `error_message_includes_hint` test.
```
