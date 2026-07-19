# `yy status` shows issue label + HH:MM durations — design

Sprint 0 follow-up (feedback #2 from `.local/USER_FEEDBACK.md`, triaged in
`docs/superpowers/plans/2026-07-19-yy6-feedback-triage.md`). Started as a
label-only change; scope grew during review to a global duration-format
change (`format_duration` is shared by `render_today` and `render_status`,
so a status-only fix would have left the two views inconsistent).

## Problem

1. `yy status` prints `current: <desc>` but never the issue key — user
   can't tell which issue the active entry belongs to without checking
   `./yy`.
2. Durations render as `0h35m` everywhere. User wants `HH:MM`.
3. `remaining` can go negative (worked past the 8h target) with no visual
   cue beyond a bare negative number.

## Scope

In scope: `render_status`'s `current:`/`worked:`/`remaining:` lines,
`render_today`'s per-entry and per-issue-total duration formatting (both
consumers of the shared `format_duration`).

Out of scope (explicitly deferred, own feature later): color output for
negative `remaining` (needs a color crate, TTY detection, `NO_COLOR`
handling, and rework of every test that asserts on exact output strings —
too large to bundle here).

## Format

Line label renamed `current:` → `active:`.

```
active: [YY-6] test
worked: 00:06
remaining: 07:53
```

Overtime (worked > 8h target):

```
active: [YY-6] test
worked: 08:15(-00:15)
remaining: -00:15
```

Rules:
- `active:` — entry has an issue → `active: [<issue-key>] <desc>`; no issue
  (`--issue` not passed) → `active: (no issue) <desc>` (same placeholder
  `render_today`'s `label_for` already uses); no active entry → unchanged
  `active: no active entry`. `desc` missing → unchanged `(no desc)`
  fallback, e.g. `active: [YY-6] (no desc)`.
- `worked:` — plain `HH:MM` when `worked_today <= 8h`. When
  `worked_today > 8h`, append the overtime diff in parens:
  `worked: <HH:MM>(-<HH:MM>)`, where the parenthesized value equals
  `remaining` (always negative in that case).
- `remaining:` — `target - worked`, `HH:MM`, sign-prefixed when negative
  (`-00:15`). Never clamped to zero.
- `HH:MM` is zero-padded on both sides (`00:06`, not `0:6`; `08:15`, not
  `8:15`). Hours are not capped at 24 (a >24h duration would be unusual for
  a single day but the format doesn't special-case it — same as today's
  `0h35m` which already allowed arbitrary hour counts).
- Same `HH:MM` format applies to `render_today`'s per-entry durations and
  per-issue totals (`10:22 [no issue] test 00:35`), replacing `0h35m`.
  Those never go negative (elapsed time can't be negative), so no
  parens/sign case applies there.

## Interface changes

`src/cli/render.rs`:

```rust
fn format_duration(d: TimeDelta) -> String   // now HH:MM zero-padded, sign-prefixed if negative
pub fn render_today(view: &TodayView, issue_labels: &[(Option<Id>, String)]) -> String   // unchanged signature, new formatting via format_duration
pub fn render_status(active_entry: &Option<Entry>, current_label: Option<&str>, worked_today: TimeDelta) -> String
```

- `format_duration` becomes: split `d.num_minutes()` into
  `(sign, hours, minutes)`; format as `{sign}{hours:02}:{minutes:02}`. Used
  as-is by `render_today` (always non-negative input) and by
  `render_status` for `worked`/`remaining` (may be negative for
  `remaining`).
- `render_status` gains the overtime-parens branch: after computing
  `remaining = DAILY_TARGET - worked_today`, if `remaining` is negative,
  render `worked` as `{format_duration(worked_today)}({format_duration(remaining)})`;
  otherwise `format_duration(worked_today)` alone.
- `current_label` (new param): `None` when `active_entry` is `None` (label
  irrelevant); `Some(&str)` otherwise, already resolved by the caller —
  render_status stays pure formatting, no store access.
- Caller (`run()` in `src/cli/mod.rs`) resolves `current_label` before
  calling, reusing the fallback chain the `start`/`stop` fix (commit
  `7734aa4`) established: `issues::find_by_id(work_folder, id)?.and_then(|i| i.key)`
  → falls back to `entry.id.to_string()` for an unresolvable id, or the
  literal `"(no issue)"` when `entry.issue_id` is `None`.

## Testing

- `format_duration` unit tests (new, in `render.rs`): zero, sub-hour,
  multi-hour, negative (sign-prefixed), zero-padding on single-digit
  hours/minutes.
- `render_today_shows_entries_and_totals`: update expected substring from
  `"45m"`/`"0h45m"` to `"00:45"`.
- `render_status_shows_active_note_and_remaining`: pass
  `current_label: Some("YY-6")`, assert `"[YY-6] deep work"`; update
  duration assertions to `"02:00"`/`"06:00"`.
- New `render_status_shows_no_issue_placeholder`: `current_label: None`
  path already covered by `render_status_handles_nothing_running`; add a
  case with `active_entry: Some(..)` + `current_label: Some("(no issue)")`
  to cover the has-entry-no-issue branch.
- New `render_status_shows_overtime_parens`: `worked_today` > 8h, assert
  `worked:` line contains `(-` and `remaining:` line starts with `-`.
- `tests/cli.rs` e2e: extend `start_status_today_stop_end_to_end` — status
  after `start ... --issue YY-6` should contain `"[YY-6]"` in the `active:`
  line.

## Self-review

- Placeholder scan: no TODOs remain; brackets/padding/parens fully
  specified (all four were open questions in review, now resolved).
- Internal consistency: `format_duration`'s new negative-sign behavior is
  used by `remaining` and the `worked` overtime branch consistently; not
  reintroduced as a second helper.
- Scope: touches one shared function (`format_duration`) and its two
  existing callers (`render_today`, `render_status`) plus one new param on
  `render_status`. No decomposition needed — small enough for one
  implementation plan.
- Ambiguity check: overtime-parens value is defined as *equal to*
  `remaining`, not independently computed, so the two can't drift.
