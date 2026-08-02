---
name: style
description: Always use this skill before writing or editing Rust code in the yy repository
---

# Code Style

## Module layout

**`foo/mod.rs`, never `foo.rs` beside `foo/`.** Enforced by
`clippy::self_named_module_files = "deny"`; the build fails with
`` `mod.rs` files are required ``. A leaf module with no submodules is a plain
`foo.rs`; the moment it gains a directory, the file moves into it as `mod.rs`.

Topcoat's repository uses the opposite convention and denies the opposite lint.
Do not carry its layout over when reading its source for reference.

## General

* Keep related code together: a struct is immediately followed by its inherent
  `impl` and then its trait impls, before the next struct in the file. Unit
  tests (`#[cfg(test)] mod tests`) go at the very bottom of the file.
* No `unsafe`. Denied at the workspace root.
* Free functions are allowed, but first consider whether a more idiomatic
  grouping onto a struct exists.

## Rules the compiler cannot check

These come from `docs/design/rules.md`. Ordinary-looking code violates them.

* **Never accumulate elapsed time.** Store a start and an optional end; compute
  the difference on read. A field that a timer adds to is always a bug.
* **Never delete.** A removal is an appended operation, not a `DELETE`.
* **`yy-core` performs no I/O.** No files, no terminal, no network, and no
  ambient clock: a caller passes the current time in. If a change to `yy-core`
  needs a fixture, the change is in the wrong crate.
* **`yy prompt` opens no socket and spawns no host.** It reads one file and
  exits.
* **No floating-point in stored or transmitted data.** Durations are integer
  milliseconds. `0.1 + 0.2` does not round-trip and the export guarantee is
  byte-identical.

## Types

* The types in `yy-types` are simultaneously the wire format, the JSONL export
  records, and the journal payloads. Do not add a second representation of any
  of them, and do not add a conversion layer between them.
* Field order in a serialised struct is the declaration order, and it is
  load-bearing: the export round-trip test compares bytes.
* Optional fields are omitted when absent, never serialised as `null`.

## Dependencies

Declare every dependency in the top-level `Cargo.toml` under
`[workspace.dependencies]` with a version and no features. Crates pull it in
with `workspace = true` and opt into features there.

## Documentation comments

Item docs say what something is and how to use it. The doc comments in
`yy-types` are the source of the published protocol reference, so treat them as
user-facing text rather than notes to yourself. Rustdoc lints are denied.
