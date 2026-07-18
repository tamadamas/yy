# yy

A terminal-first time tracker for your real workday.

> **Status: in development.** The design is settled (see `architecture.md`); the MVP is
> being built. Commands below describe the intended tool. Expect rough edges.

Time is tracked against **issues** (tasks, meetings, recurring work) that can receive
many time **entries** — not one flat log line per activity. Start tracking immediately,
without a description or an issue, and link the entry later. Your data is plain,
human-editable **JSON Lines** — inspect it, hand-edit it, `grep` it, or put it in git.

## Why

- **Issue-based, not a flat log.** Many entries roll up under one issue, which matches how
  a day of parallel meetings and billable work actually looks.
- **No runaway timer.** `yy` never accumulates seconds in a background counter. Elapsed
  time is always computed from stored timestamps, so a crash — or a forgotten stop over
  the weekend — can't silently inflate your hours.
- **Plain text, local, yours.** No database, no cloud, no account. Just JSONL files you
  can edit by hand.
- **Sensible defaults.** Commands default to *today*. `yy --yesterday` resolves to the
  last working day (Friday on a Monday).
- **A fast TUI** for browsing and editing without leaving the keyboard.
- **Optional reminders** for breaks, your daily limit, and idle/sleep time.

## Features

- `start` / `stop` with derived time; start a task and the previous one stops.
- One unified *today* view (timeline **and** per-issue totals) — no separate `list` and
  `report`.
- Parallel meetings alongside one focus task (an *active-set*, not a single timer).
- Automatic classification rules (e.g. a "Meeting" gets tagged and queued for review).
- A `need_review` queue for anything that needs a second look.
- Idle/sleep detection that turns away-from-keyboard time into a reviewable pause.
- Shell-prompt integration (fish + starship/tide): current task and `5.32 / 8` at a
  glance, red past your target.
- Import from bartib and from a pasted block of your external system's HTML.
- Invoice-style PDF export to check your time before sending it to clients *(planned)*.

## Install

Requires a recent Rust toolchain. **Linux and macOS only.**

```bash
cargo install --path .     # from a clone
# or
cargo build --release      # binary at target/release/yy
```

## Quick start

```bash
yy start "Implement JSONL store" --issue YY-3   # begin (issue optional)
yy                                              # today: timeline + totals
yy status                                       # current task, worked, remaining to 8h
yy stop                                          # close the running entry
yy start "Urgent meeting with Paul"             # previous stops; auto-tagged, queued
yy --yesterday                                  # last working day
```

Every entry command accepts the same optional flags: `--desc`, `--issue`, `--tag`,
`--from`, `--to`. Give both `--from` and `--to` to record a finished entry; omit `--to`
to leave it running. Times accept `HH:MM`, a full timestamp, or natural language
("9am", "yesterday 14:00").

## How it works

Entries store intervals (`start`/`end`); elapsed is computed as `(end or now) − start`.
There is **no daemon** — each command, a periodic one-shot `tick`, and OS wake/unlock
hooks all reconcile the same on-disk state. That's why the tracker is robust to crashes
and why the shell prompt stays instant.

## Data & config

```
$HOME/.yy_logs/              # data (override with $YY_WORK_FOLDER or --work-folder)
├── issues.jsonl
├── entries/YYYY-MM.jsonl
├── active.json
└── state.json
$HOME/.config/yy/config.toml # settings: work hours, daily target, reminders, rules, theme
```

Files are line-oriented and safe to edit by hand — comments (`#…`) and even lines `yy`
can't parse are preserved, never dropped.

## Editing

Three equal ways to fix your time: open the `.jsonl` in your editor, use the TUI's edit
mode, or the CLI (`yy edit @id`, or backfill with `--from/--to`).

## Acknowledgments

`yy` is an independent implementation — no code is shared with either project — but owes
its ideas to:

- **[bartib](https://github.com/nikolassv/bartib)** (GPLv3) — the plain-text, CLI-first
  time tracker whose start/stop/list/report workflow and human-editable log inspired the
  core.
- **[lazygit](https://github.com/jesseduffield/lazygit)** (MIT) — the fast, keyboard-
  driven, panel-based terminal UI.

## License

TBD (add a `LICENSE` file before the first release).
