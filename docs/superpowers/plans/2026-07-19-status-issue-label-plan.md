# Status Issue Label + HH:MM Duration Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement `docs/superpowers/specs/2026-07-19-status-issue-label-design.md`: `yy status` shows the active entry's issue key, and all durations render as zero-padded `HH:MM` (sign-prefixed when negative) instead of `0h35m`.

**Architecture:** All touched code lives in `src/cli/mod.rs` (the REFACTOR_PLAN.md split into `cli/render.rs` has not been applied yet, so `format_duration`/`render_today`/`render_status`/`run()` are all still in this one file). No new modules, no new dependencies.

**Tech Stack:** Rust 2024, existing `chrono`/`assert_fs`/`assertables`/`duct` stack.

## Global Constraints

- Pure feature addition per the approved spec — no unrelated behavior change. `rtk cargo test` count must only grow, never shrink; existing tests get their assertions updated for the new format, never deleted.
- `cargo fmt` clean, `cargo clippy` warning-free (`just check`).
- `format_duration` stays a private fn (only `render_today`/`render_status` in the same file call it) — no signature widening beyond what's needed.
- Color output for negative `remaining` is explicitly out of scope (see spec) — do not add it.

---

### Task 1: `format_duration` → zero-padded `HH:MM`, sign-prefixed

**Files:**
- Modify: `src/cli/mod.rs` (`format_duration`, line 87-90)
- Test: `src/cli/mod.rs` `#[cfg(test)] mod tests`

**Interfaces:**
- Consumes: nothing new.
- Produces: `fn format_duration(d: TimeDelta) -> String` — same signature, new output shape (`"08:15"`, `"-00:15"`, `"00:06"`) instead of `"8h15m"`. Called unchanged by `render_today` and `render_status` (Tasks 2-3 update their call sites' expectations, not their calls to this fn).

- [ ] **Step 1: Write the failing unit tests**

Add to the `tests` module in `src/cli/mod.rs` (near the other small unit tests, e.g. after `resolve_target_date_with_yesterday_uses_last_working_day`):

```rust
    #[test]
    fn format_duration_zero_pads_sub_hour() {
        assert_eq!(format_duration(TD::minutes(6)), "00:06");
    }

    #[test]
    fn format_duration_zero_pads_multi_hour() {
        assert_eq!(format_duration(TD::hours(8) + TD::minutes(15)), "08:15");
    }

    #[test]
    fn format_duration_handles_zero() {
        assert_eq!(format_duration(TD::zero()), "00:00");
    }

    #[test]
    fn format_duration_sign_prefixes_negative() {
        assert_eq!(format_duration(-TD::minutes(15)), "-00:15");
    }
```

- [ ] **Step 2: Run to verify RED**

Run: `rtk cargo test --lib format_duration`
Expected: 4 new tests FAIL — output still `"0h06m"` etc., not `"00:06"`.

- [ ] **Step 3: Minimal implementation**

Replace `format_duration` (`src/cli/mod.rs:87-90`):

```rust
fn format_duration(d: TimeDelta) -> String {
    let total_minutes = d.num_minutes();
    let sign = if total_minutes < 0 { "-" } else { "" };
    let hours = total_minutes.abs() / 60;
    let minutes = total_minutes.abs() % 60;
    format!("{sign}{hours:02}:{minutes:02}")
}
```

- [ ] **Step 4: Run to verify GREEN**

Run: `rtk cargo test --lib format_duration`
Expected: all 4 pass.

- [ ] **Step 5: Update existing callers' assertions**

`render_today_shows_entries_and_totals` (`src/cli/mod.rs:268-280`) currently asserts `out.contains("45m") || out.contains("0h45m")`. Change to:

```rust
        assert!(out.contains("00:45"));
```

`render_status_shows_active_note_and_remaining` (`src/cli/mod.rs:292-299`) will be replaced wholesale in Task 2 (its call site changes shape there) — skip editing it here to avoid double-editing; leave it as the old 2-arg call for now, it is superseded in Task 2 Step 1.

- [ ] **Step 6: Run full suite, full check**

Run: `rtk cargo test && just check`
Expected: `render_today` tests green. `render_status` tests still compile (old 2-arg signature untouched by this task) and pass with old assertions still using `"2h"`/`"6h"` OR-branches, which still match since `format_duration`'s new output `"02:00"`/`"06:00"` satisfies those `||` checks already written. Confirm with the run — no code change needed here if so.

- [ ] **Step 7: Commit**

```bash
rtk git add src/cli/mod.rs
rtk git commit -m "feat: render durations as zero-padded HH:MM"
```

---

### Task 2: `render_status` — issue label + overtime parens

**Files:**
- Modify: `src/cli/mod.rs` (`render_status`, lines 135-148)
- Test: `src/cli/mod.rs` `#[cfg(test)] mod tests`

**Interfaces:**
- Consumes: `format_duration` (Task 1, `HH:MM` output).
- Produces: `pub fn render_status(active_entry: &Option<Entry>, current_label: Option<&str>, worked_today: TimeDelta) -> String` — new middle parameter. Called by `run()`'s `Status` arm (Task 3 updates that call site).

- [ ] **Step 1: Write the failing tests**

Replace `render_status_shows_active_note_and_remaining` and `render_status_handles_nothing_running` in `src/cli/mod.rs`'s test module with:

```rust
    #[test]
    fn render_status_shows_issue_label_and_remaining() {
        let active = Some(sample_entry(None, Some("deep work")));
        let out = render_status(&active, Some("[YY-6]"), TD::hours(2));
        assert!(out.contains("active: [YY-6] deep work"));
        assert!(out.contains("worked: 02:00"));
        assert!(out.contains("remaining: 06:00"));
    }

    #[test]
    fn render_status_shows_no_issue_placeholder() {
        let active = Some(sample_entry(None, Some("deep work")));
        let out = render_status(&active, Some("(no issue)"), TD::hours(2));
        assert!(out.contains("active: (no issue) deep work"));
    }

    #[test]
    fn render_status_handles_nothing_running() {
        let out = render_status(&None, None, TD::zero());
        assert!(out.contains("active: no active entry"));
    }

    #[test]
    fn render_status_shows_overtime_parens() {
        let active = Some(sample_entry(None, Some("deep work")));
        let out = render_status(&active, Some("[YY-6]"), TD::hours(8) + TD::minutes(15));
        assert!(out.contains("worked: 08:15(-00:15)"));
        assert!(out.contains("remaining: -00:15"));
    }
```

- [ ] **Step 2: Run to verify RED**

Run: `rtk cargo test --lib render_status`
Expected: FAIL to compile — `render_status` still takes 2 args, not 3, and old tests calling it with 2 args now conflict. (A compile error counts as RED: the new tests describe a target interface that doesn't exist yet.)

- [ ] **Step 3: Minimal implementation**

Replace `render_status` (`src/cli/mod.rs:135-148`):

```rust
/// Render `yy status`: active task (with issue label, already bracketed by
/// the caller), worked so far, remaining vs an 8h target (parenthesized
/// overtime diff on `worked` when negative).
pub fn render_status(
    active_entry: &Option<Entry>,
    current_label: Option<&str>,
    worked_today: TimeDelta,
) -> String {
    let active = match active_entry {
        Some(entry) => {
            let desc = entry.note.as_deref().unwrap_or("(no desc)");
            match current_label {
                Some(label) => format!("{label} {desc}"),
                None => desc.to_string(),
            }
        }
        None => "no active entry".to_string(),
    };

    let remaining = DAILY_TARGET - worked_today;
    let worked = if remaining < TimeDelta::zero() {
        format!(
            "{}({})",
            format_duration(worked_today),
            format_duration(remaining)
        )
    } else {
        format_duration(worked_today)
    };

    format!(
        "active: {active}\nworked: {worked}\nremaining: {}\n",
        format_duration(remaining)
    )
}
```

`current_label` arrives pre-formatted (`"[YY-6]"` or `"(no issue)"`) from the caller (Task 3) — `render_status` only concatenates it with `desc`, it doesn't add brackets itself.

- [ ] **Step 4: Fix the now-broken old test / run() call site so the crate compiles**

Delete the old `render_status_shows_active_note_and_remaining` test entirely (superseded by `render_status_shows_issue_label_and_remaining` from Step 1 — don't leave both, the old one calls the 2-arg form and won't compile).

`run()`'s `Status` arm (`src/cli/mod.rs:214-222`) still calls `render_status(&active_entry, worked_today)` with 2 args — this won't compile either. Temporarily fix it minimally so the build succeeds before Task 3 does the real wiring:

```rust
            Ok(render_status(&active_entry, None, worked_today))
```

(Task 3 replaces this whole arm properly — this is a throwaway placeholder just to keep the build green between Task 2 and Task 3 commits.)

- [ ] **Step 5: Run to verify GREEN**

Run: `rtk cargo test && just check`
Expected: all green, fmt clean, clippy warning-free.

- [ ] **Step 6: Commit**

```bash
rtk git add src/cli/mod.rs
rtk git commit -m "feat: render_status shows issue label and overtime parens"
```

---

### Task 3: Wire `run()` to resolve and pass the real `current_label`

**Files:**
- Modify: `src/cli/mod.rs` (`run()`, `Commands::Status` arm, lines 214-222)
- Test: `tests/cli.rs`

**Interfaces:**
- Consumes: `render_status` (Task 2, 3-arg form), `issues::find_by_id` (existing).
- Produces: no new public interface — `run()`'s output text changes.

- [ ] **Step 1: Write the failing e2e assertion**

In `tests/cli.rs`, extend `start_status_today_stop_end_to_end`'s status check:

```rust
    let status = yy(&home, &["status"]);
    assert_contains!(status, "manual smoke test");
    assert_contains!(status, "active: [YY-6] manual smoke test");
```

- [ ] **Step 2: Run to verify RED**

Run: `rtk cargo test --test cli`
Expected: FAIL — actual `status` output has `active: deep work`/no label (Task 2's placeholder `None` from Step 4), not `active: [YY-6] manual smoke test`.

- [ ] **Step 3: Minimal implementation**

Replace the `Commands::Status` arm (`src/cli/mod.rs:214-222`, the placeholder from Task 2 Step 4):

```rust
        Some(Commands::Status) => {
            let view = core::today(work_folder, target_date)?;
            let worked_today = view
                .totals
                .iter()
                .fold(TimeDelta::zero(), |acc, t| acc + t.elapsed);
            let active_entry = active::read(work_folder)?.entry;
            let current_label = match &active_entry {
                Some(entry) => Some(match entry.issue_id {
                    Some(id) => {
                        let key = issues::find_by_id(work_folder, id)?
                            .and_then(|i| i.key)
                            .unwrap_or_else(|| entry.id.to_string());
                        format!("[{key}]")
                    }
                    None => "(no issue)".to_string(),
                }),
                None => None,
            };
            Ok(render_status(&active_entry, current_label.as_deref(), worked_today))
        }
```

- [ ] **Step 4: Run to verify GREEN**

Run: `rtk cargo test`
Expected: all tests pass.

- [ ] **Step 5: Full check**

Run: `just check`
Expected: fmt clean, clippy warning-free, tests pass, build ok.

- [ ] **Step 6: Commit**

```bash
rtk git add src/cli/mod.rs tests/cli.rs
rtk git commit -m "feat: yy status shows active entry's issue label"
```

---

## Self-Review Notes

- **Spec coverage:** `active:` rename + brackets → Task 3's `format!("[{key}]")`. `(no issue)` placeholder → Task 3. HH:MM zero-padding → Task 1. Overtime parens on `worked` + negative `remaining` → Task 2. `render_today` HH:MM → Task 1 (shared `format_duration`). Color explicitly deferred → not implemented, matches spec's "out of scope."
- **Placeholder scan:** no TBD/TODO; every step has real code. Task 2 Step 4's `None` is a real, explicit, temporary value (not a vague placeholder) needed only to keep the build compiling between two commits — replaced by real logic one task later.
- **Type consistency:** `render_status(&Option<Entry>, Option<&str>, TimeDelta) -> String` matches across Task 2's definition and Task 3's call site. `format_duration(TimeDelta) -> String` unchanged signature, matches Task 1 definition and its two callers.
- **Build-stays-green:** every task ends with a full `rtk cargo test && just check` pass — Task 2 Step 4 exists specifically so Task 2's commit alone is buildable, not just Task 2+3 combined.
