# YY-6 Feedback Triage — Bug vs Feature

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Classify `.local/USER_FEEDBACK.md` items against `docs/superpowers/specs/2026-07-18-yy6-cli-design.md`, fix the one confirmed bug, park the rest as feature backlog.

**Architecture:** No new modules. Fix touches `src/cli/mod.rs::run()` only (`Start`/`Stop` arms), reusing the label-resolution pattern `render_today` already uses via `issue_labels()`.

**Tech Stack:** Rust 2024, existing `assert_fs`/`assertables`/`duct` e2e stack (`tests/cli.rs`).

## Global Constraints

- Pure bug fix — no unrelated behavior change (REFACTOR_PLAN.md Global Constraints apply: `rtk cargo test` count must stay ≥ current, all green).
- `cargo fmt` clean, `cargo clippy` warning-free (`just check`).
- Golden rule 4 (preserve, don't drop): raw ULID stays the on-disk `id` — this fix only changes *display*, never storage.

---

## Classification

| # | Feedback (`.local/USER_FEEDBACK.md`) | Spec says | Verdict |
|---|---|---|---|
| 1 | `start`/`stop` print raw ULID (`started 01KXW...`), user wants `started YY-6` | Design doc doesn't pin the success-message format, but `render_today`/`render_status` already resolve issue keys via `issue_labels()`/`find_by_id` — showing the raw id here is inconsistent with the rest of the UI and with the user's explicit invariant ("User must not see ULID, only on direct jsonl edit") | **BUG** — fixed below |
| 2 | `status` doesn't print `Issue: YY-6` | "Status target" spec only names current task / worked / remaining, no issue field | **Feature** — spec gap, needs a one-line design decision (add issue label to `render_status`), not a silent fix |
| 3 | `./yy` today view doesn't show the entry is stopped / no `end` time visible | Spec's `render_today` format (Task 2 of REFACTOR_PLAN.md) never specified an active/closed marker or end-time column | **Feature** — UX addition (active-state marker), out of scope for MVP format |
| 4 | `./yy` should say "keine active tasks" when nothing running | Same root as #3 — no active/inactive marker on the timeline | **Feature**, bundle with #3 |
| 5 | Today view lists both morning (YY-6) and evening (YY-7) entries; user expected to see only the active one | Spec: `yy` = *"today's combined view (timeline + per-issue totals)"* — showing the whole day is by design | **Not a bug** — behavior matches spec. Confusion traces back to #3/#4 (no active marker), not to over-inclusion |
| 6 | Bartib-style unified `status` (`list`/`last`/`report` merged) | User's own feedback file already flags this as a TODO requiring a fresh interview ("Ask User was interface... FRAGEN BOGEN PLAN TOOL") | **Feature**, large — needs `superpowers:brainstorming` + its own spec before planning, not bundled here |

Only #1 is a confirmed regression against already-built behavior (label resolution exists elsewhere, `start`/`stop` just don't use it). #2–#6 are product decisions — run them through brainstorming before writing code.

---

### Task 1: Fix `start`/`stop` to print the issue label, not the raw id

**Files:**
- Modify: `src/cli/mod.rs` (`run()`, `Commands::Start` and `Commands::Stop` arms, lines ~161–199)
- Modify: `tests/cli.rs` (extend the e2e test with label assertions)

**Interfaces:**
- Consumes: `issues::find_by_id(work_folder, id) -> anyhow::Result<Option<Issue>>` (existing, used today by `issue_labels()`).
- Produces: no new public fn — inline the same "key, else raw id" fallback `issue_labels()` already uses, since `Start`/`Stop` resolve a single id, not a list.

- [ ] **Step 1: Write the failing e2e assertions**

Edit `tests/cli.rs`, replacing the two `assert_starts_with!` lines in `start_status_today_stop_end_to_end`:

```rust
    let started = yy(&home, &["start", "manual smoke test", "--issue", "YY-6"]);
    assert_eq!(started, "started YY-6\n");
```

and:

```rust
    let stopped = yy(&home, &["stop"]);
    assert_eq!(stopped, "stopped YY-6\n");
```

Also add a new test for the no-issue case (label falls back to raw id, since there's nothing to resolve):

```rust
#[test]
fn start_stop_without_issue_prints_raw_id() {
    let home = TempDir::new().unwrap();

    let started = yy(&home, &["start", "no issue task"]);
    assert_starts_with!(started, "started ");
    assert!(!started.contains("no issue task"));

    let stopped = yy(&home, &["stop"]);
    assert_starts_with!(stopped, "stopped ");
}
```

- [ ] **Step 2: Run to verify RED**

Run: `rtk cargo test --test cli`
Expected: `start_status_today_stop_end_to_end` FAILS — `assert_eq!` left is `"started 01K...\n"`, right is `"started YY-6\n"`.

- [ ] **Step 3: Minimal fix in `src/cli/mod.rs`**

Replace the `Start` arm's return (around line 188):

```rust
            Ok(format!("started {}\n", entry.id))
```

with:

```rust
            let label = match issue_id {
                Some(id) => issues::find_by_id(work_folder, id)?
                    .and_then(|i| i.key)
                    .unwrap_or_else(|| entry.id.to_string()),
                None => entry.id.to_string(),
            };
            Ok(format!("started {label}\n"))
```

Replace the `Stop` arm's `Some(entry) =>` branch (around line 196):

```rust
                Some(entry) => Ok(format!("stopped {}\n", entry.id)),
```

with:

```rust
                Some(entry) => {
                    let label = match entry.issue_id {
                        Some(id) => issues::find_by_id(work_folder, id)?
                            .and_then(|i| i.key)
                            .unwrap_or_else(|| entry.id.to_string()),
                        None => entry.id.to_string(),
                    };
                    Ok(format!("stopped {label}\n"))
                }
```

- [ ] **Step 4: Run to verify GREEN**

Run: `rtk cargo test`
Expected: all tests pass, including the 3 in `tests/cli.rs` (2 existing + 1 new).

- [ ] **Step 5: Full check**

Run: `just check`
Expected: fmt clean, clippy warning-free, tests pass, build ok.

- [ ] **Step 6: Commit**

```bash
rtk git add src/cli/mod.rs tests/cli.rs
rtk git commit -m "fix: start/stop print issue key instead of raw ULID"
```

---

## Self-Review Notes

- **Scope:** only #1 gets code; #2–#6 are logged as feature backlog in `.local/architecture_notes.md` §16 candidates (do this by hand — not part of this plan's tasks, since none has an agreed interface yet).
- **No behavior change beyond the fix:** stored `Entry.id` (ULID) is untouched; only the printed string changes. Golden rule 4/5 unaffected — this touches display, not storage or write path.
- **Type consistency:** `issues::find_by_id` signature matches its existing use in `issue_labels()` (`src/cli/mod.rs:218`) — same `Option<Issue> -> and_then(|i| i.key)` pattern, just applied per-entry instead of per-list.
