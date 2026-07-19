# `yy status` shows issue label — design

Sprint 0 follow-up (feedback #2 from `.local/USER_FEEDBACK.md`, triaged in
`docs/superpowers/plans/2026-07-19-yy6-feedback-triage.md`). Small, additive
change to `render_status`.

## Problem

`yy status` prints `current: <desc>` but never the issue key. User can't
tell which issue the active entry belongs to without checking `./yy`.

## Scope

In scope: add the resolved issue label to `render_status`'s `active:` line.

Out of scope: `render_today`'s per-line format (unrelated — that already
resolves labels), any change to storage or `active.rs`.

TODO: Rename "current:" to "active:"

## Format

```
active: [YY-6] test
worked: 0h 06m
remaining: 7h 53m
```

- Active entry has an issue → `active: <issue-key> <desc>`.
- Active entry has no issue (`--issue` not passed) → `active: (no issue) <desc>`,
  reusing the same `"(no issue)"` placeholder `render_today`'s `label_for`
  already uses (`src/cli/render.rs`), so the two views stay visually
  consistent.
- No active entry → unchanged: `active: no active entry`. No label shown —
  there's no entry to resolve one for.
- `desc` missing (`(no desc)` case, already existing behavior) is untouched;
  label prefixes it the same way: `active: YY-6 (no desc)`.

## Interface change

`src/cli/render.rs`:

```rust
pub fn render_status(active_entry: &Option<Entry>, current_label: Option<&str>, worked_today: TimeDelta) -> String
```

- `render_status` stays pure formatting — no store access, no new deps.
  `current_label` is `None` when `active_entry` is `None` (label irrelevant);
  `Some(&str)` otherwise, already resolved by the caller.
- Callers of `render_status` (`run()` in `src/cli/mod.rs`) resolve the label
  before calling, using the same fallback chain the `start`/`stop` fix
  (commit `7734aa4`) already established:
  `issues::find_by_id(work_folder, id)?.and_then(|i| i.key)` → falls back to
  `entry.id.to_string()` for an unresolvable id, or to the literal
  `"(no issue)"` when `entry.issue_id` is `None`.

## Testing

- `render.rs` unit tests: extend `render_status_shows_active_note_and_remaining`
  to pass `Some("YY-6")` and assert `"YY-6 deep work"` appears; add a case for
  `current_label: Some("(no issue)")`; `render_status_handles_nothing_running`
  passes `None` and is otherwise unchanged.
- `tests/cli.rs` e2e: extend the existing `start_status_today_stop_end_to_end`
  smoke test — `status` after `start ... --issue YY-6` should contain
  `"YY-6"` in the `active:` line (not just anywhere in output, to catch a
  regression where the label leaks into the wrong field).

## Self-review

- No placeholders — format, signature, and fallback chain are fully specified.
- No contradiction with the `start`/`stop` fix — same resolution pattern,
  reused rather than reinvented.
- Scoped to one file's interface change (`render_status`) plus its one
  caller (`run()`); no decomposition needed.
