# YY-1 Bootstrap — module skeleton

**Sprint:** Sprint 0 (MVP). **Issue:** YY-1.
**Source of truth:** `.local/architecture.md` §7 (module layout).

## Goal

Lay down the module tree so later issues (YY-2..YY-6) have a place to land.
No behavior, no logic — just the skeleton `cargo build` compiles against.

## Scope

Create empty modules matching architecture.md §7:

```
src/
├── main.rs         # wires mods; keeps current "Hello, world" print
├── model.rs        # Issue, Entry, IssueKind, Id — filled in YY-2
├── store/
│   ├── mod.rs
│   ├── jsonl.rs     # filled in YY-3
│   ├── entries.rs   # filled in YY-4
│   ├── issues.rs
│   ├── active.rs    # filled in YY-4
│   └── state.rs
├── core/mod.rs      # filled in YY-5
├── pipeline.rs      # later sprint
├── cli/mod.rs       # filled in YY-6
├── tui/
│   ├── mod.rs
│   ├── app.rs
│   ├── keymap.rs
│   └── panes/mod.rs # later sprint
├── watch/mod.rs      # later sprint
├── import/mod.rs     # later sprint
└── export/mod.rs     # later sprint
```

Each file gets a one-line doc comment stating its purpose (copied from §7). No
structs, no functions — an empty file with `//!` module comment is enough for
`mod x;` to compile.

## Out of scope

Any actual logic (model fields, store I/O, core use-cases, CLI parsing). Those
are YY-2 through YY-6.

## Done when

- `cargo build` succeeds.
- `cargo clippy --all-targets` is warning-free (empty modules produce none).
- `cargo fmt --all --check` passes.
- Module tree matches architecture.md §7 exactly.

## Testing

None needed — no logic exists yet. `cargo build` is the verification.
