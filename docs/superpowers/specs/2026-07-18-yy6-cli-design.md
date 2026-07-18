# YY-6 CLI — design

Sprint 0, last MVP issue. `yy` (today view), `start`, `stop`, `status`; shared
entry-spec flags; `--yesterday`. Thin wrapper around `core::` (already built:
`start`, `stop`, `today` in `src/core/mod.rs`).

## Scope

In scope:
- `yy` — today's combined view (timeline + per-issue totals), text output
- `yy start [desc] [--issue KEY] [--tag T]... [--from t] [--to t]`
- `yy stop [--at t]`
- `yy status [--yesterday]`
- `--yesterday` = last working day (Mon→Fri, so Monday resolves to Friday)
- Issue key resolution: minimal `store/issues.rs` (currently an empty stub),
  auto-create on first use

Out of scope (later sprints, do not implement): `continue`, `edit`, `review`,
`gaps`, `aggregate`, `import`, `export`, `prompt`, config.toml, natural-language
time parsing, `--work-folder` flag, `$YY_WORK_FOLDER` env var, non-exclusive/
`--gap` overlap.

## Issue key format & resolution

Key format: `^[A-Z]{2,}-\d+$` (e.g. `YY-1`, `DFG-1234`, `KJJ-2`). Validated by a
clap `value_parser` on `--issue` — reject at parse time with a hint
(`issue key must be LETTERS-NUMBER, e.g. YY-1`).

`store/issues.rs` (new, was an empty stub):
- `issues.jsonl` in `work_folder`, same line-based JSONL pattern as
  `entries.rs` (reuse `store::jsonl` primitives — read all, append, atomic
  write).
- `resolve_or_create(work_folder, key: &str, desc: Option<&str>) -> anyhow::Result<Id>`:
  scan for an `Issue` with matching `key`; if found, return its `Id`. If not
  found, create `Issue { key: Some(key), title: desc.unwrap_or(key).into(),
  kind: IssueKind::Task, .. }`, append, return its `Id`.
- No update/delete in this issue — creation + lookup only.

## Time parsing

`--from` / `--to` / `--at` accept:
- `HH:MM` → today's date (or `--yesterday`'s date, if combined), local time
  converted to UTC
- Full RFC3339 timestamp

Natural language ("9am", "yesterday 14:00") explicitly deferred — no new crate
this issue.

`--from` + `--to` both given → write a completed entry directly (no active
run). `--from` alone → start running from that time. Neither given → `start`
begins now; `stop` closes now.

## Work folder

Hardcoded `$HOME/.yy_logs` for this issue. No `--work-folder` flag, no
`$YY_WORK_FOLDER` env var — both deferred to Sprint 1 config.toml work.

## Status target

`yy status` shows current task, worked-today total, remaining vs a hardcoded
8h/day target (matches the external system's 8h/day requirement). No config.

## CLI structure (`cli/mod.rs`)

Single `clap::Parser`, derive-based:

```rust
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Start { desc: Option<String>, #[command(flatten)] spec: EntrySpec },
    Stop { at: Option<String> },
    Status { #[arg(long)] yesterday: bool },
}

#[derive(Args)]
struct EntrySpec {
    #[arg(long, value_parser = parse_issue_key)]
    issue: Option<String>,
    #[arg(long)]
    tag: Vec<String>,
    #[arg(long)]
    from: Option<String>,
    #[arg(long)]
    to: Option<String>,
}
```

`command: None` → today view (`--yesterday` also applies here, so `Cli` itself
carries a top-level `--yesterday` flag, not just `Status`).

`main.rs` parses `Cli`, resolves work_folder (hardcoded), dispatches to a
`run(cli) -> anyhow::Result<()>` in `cli/mod.rs` that calls `core::` and
prints text. Errors bubble via `anyhow`, printed to stderr, non-zero exit.

## Testing

- Issue key parser: valid/invalid formats (unit test on the `value_parser`
  function directly, not through clap).
- `store::issues::resolve_or_create`: first call creates, second call with
  same key returns same `Id`, JSONL round-trips (same style as
  `entries.rs` tests).
- CLI integration: build the binary, run `start`/`stop`/status against a temp
  work_folder (e.g. via `assert_cmd`, or plain `std::process::Command` if we
  don't want a new dev-dependency — decide in the plan).
- No terminal required (per golden rule 6) — CLI tests use fixed args/temp
  dirs, no interactive input.
