# Agent instructions

`yy` is a local time tracker: you start a task, you stop it, and at the end of
the month you can prove where the hours went. It is a Rust workspace.

`CLAUDE.md` is a symlink to this file, and `.claude/` is a symlink to
`.agents/`. There is one set of instructions, and every agent reads it.

## Read this first

This project has a **design of record** at [`docs/DESIGN.md`](docs/DESIGN.md),
split across [`docs/design/`](docs/design). It is not background reading. It
states what was decided, what each decision costs, and where a later decision
reversed an earlier one.

Two documents are load-bearing for almost any task:

- [`docs/design/rules.md`](docs/design/rules.md) — twelve invariants. **Check a
  change against these before writing it.** Several are non-obvious and easy to
  violate with ordinary-looking code: time is never accumulated, nothing is ever
  deleted, `yy-core` performs no I/O, `yy prompt` never opens a socket.
- [`docs/design/verification.md`](docs/design/verification.md) — what the tests
  actually guarantee, and therefore what breaking one means.

If a change contradicts the design, **update the design document in the same
change** and say so. A decision that exists only in code is a decision nobody
can find later. If you think a rule is wrong, say so explicitly rather than
quietly working around it.

## Project structure

The repository is currently a single package (`src/main.rs`) that day one splits
into `crates/`. Write new code where it will live, not where it currently
compiles.

- `yy-types`: the shared serde types. One set of definitions serves three
  purposes at once — the JSON-RPC wire format, the JSONL export records, and the
  operation journal payloads. That unification is deliberate and load-bearing;
  do not introduce a second representation for any of them. Named `yy-types` and
  not `yy-proto` because there is no protobuf anywhere in this project.
- `yy-core`: the domain rules and all time logic. **Touches nothing external**:
  no files, no terminal, no network, and no clock beyond what is passed in. Its
  tests need no fixtures.
- `yy-store`: SQLite, migrations, JSONL import and export.
- `yy-host`: the background process. JSON-RPC server, subscriptions, reminders,
  and the Topcoat web application.
- `yy-client`: the `Backend` trait plus `LocalBackend` (links the store) and
  `RemoteBackend` (talks to the host).
- `yy-tui`: the terminal UI, in ratatui.
- `yy-cli`: the command line.

There is no `xtask` and no code generation. If you find yourself wanting one,
that is a signal to re-read
[§4.6](docs/design/protocol.md#46-why-json-rpc-20-between-the-host-and-the-front-ends).

## Documentation index

### The design

- [`docs/design/purpose.md`](docs/design/purpose.md): what `yy` is for, and the
  one-day MVP constraint that decides scope.
- [`docs/design/architecture.md`](docs/design/architecture.md): why a background
  host exists, why front-ends hold no logic, and the `Backend` trait that lets
  the process boundary arrive late.
- [`docs/design/storage.md`](docs/design/storage.md): SQLite, the JSONL export
  format and its canonical rendering rules, the operation journal, intervals
  rather than counters, and how timestamps are stored.
- [`docs/design/protocol.md`](docs/design/protocol.md): JSON-RPC 2.0 over a Unix
  socket, why not gRPC, typed subscriptions, the method surface, and the two
  listeners.
- [`docs/design/frontends.md`](docs/design/frontends.md): the terminal UI, the
  CLI, `yy prompt`, and the Topcoat browser view.
- [`docs/design/repository.md`](docs/design/repository.md): layout,
  dependencies, documentation, version control, releases, CI, formatting, and
  the warnings-are-errors setup.
- [`docs/design/roadmap.md`](docs/design/roadmap.md),
  [`docs/design/glossary.md`](docs/design/glossary.md),
  [`docs/design/smaller-decisions.md`](docs/design/smaller-decisions.md).

### Working on the repository

- [`CONTRIBUTING.md`](CONTRIBUTING.md): setup, checks, commit format, how a
  change reaches `main`.
- [`RELEASING.md`](RELEASING.md): the release procedure.
- [`docs/jj/`](docs/jj/index.md): the Jujutsu guide. Read
  [`docs/jj/github.md`](docs/jj/github.md) before running any VCS command.

## Version control

> ## ALWAYS USE `jj`. NEVER USE `git` TO COMMIT.
>
> Git is reached only through `jj git ...` (`jj git fetch`, `jj git push`).
> Bare `git commit`, `git add`, `git rebase`, `git merge`, `git switch`,
> `git checkout`, `git worktree`, `git push` are all forbidden for agents.
> Read-only inspection (`git log`, `git show`, `gh`) is fine.

The repository is a **colocated Jujutsu/Git workspace**: `.jj/` and `.git/` over
one working copy, so both tools *can* run — which is exactly why the rule above
has to be explicit. Human contributors may use Git and the project guarantees
they can ([rule 11](docs/design/rules.md#10-rules-that-must-not-be-broken)).
Agents have no reason to: `jj` does the same jobs, and its operation log makes a
mistake one `jj undo` away instead of an investigation. Mixing the two also
produces states that are hard to reason about, since every `jj` command imports
and exports Git state underneath you.

There is deliberately **no `git` skill**. If you are reaching for a Git command
that changes something, stop and find the `jj` equivalent in
[`docs/jj/tutorial.md`](docs/jj/tutorial.md) — it has a full translation table.

The one documented exception is annotated, signed tags at release time, which
`jj` cannot create. That is a maintainer step in
[`RELEASING.md`](RELEASING.md), not an agent one.

**`main` is never written to directly.** Branch protection rejects direct
pushes, force pushes, and deletion, for everyone. Every change reaches `main` by
a squash-merged pull request that passed CI. Never run `jj bookmark move main`,
`jj git push --bookmark main`, or `git push origin main`.

`jj` enforces part of this locally: its default `immutable_heads()` is
`trunk() | tags() | untracked_remote_bookmarks()`, so it refuses to rewrite
anything on `main`.

**Git hooks do not run**, because `jj` does not execute them. Nothing catches an
unformatted file for you. Run the checks yourself.

The [`jj`](.agents/skills/jj/SKILL.md) skill has the rules and points at the
guides; do not restate `jj` commands elsewhere.

## Verifying a change

Always, before proposing a change as finished:

```sh
just check
```

The [`check`](.agents/skills/check/SKILL.md) skill lists what that runs and when
each part is needed.

Two things that will surprise you:

- **Formatting needs nightly.** `cargo +nightly fmt --all`, never `cargo fmt`.
  `rustfmt.toml` uses nightly-only options and stable ignores them silently,
  producing a file that fails CI although the formatter ran. This is the only
  nightly in the project.
- **Warnings are errors,** via `build.warnings` in `.cargo/config.toml`. Do not
  add `RUSTFLAGS: -D warnings` anywhere; that is what this setting replaced, and
  reintroducing it invalidates the build cache. To silence warnings while
  working, use `CARGO_BUILD_WARNINGS=allow cargo check` and never commit code
  that needs it.

## Conventions

- **Module layout: `foo/mod.rs`, never `foo.rs` beside `foo/`.** This is
  enforced, not advised: `clippy::self_named_module_files` is denied at the
  workspace root and the build fails with `` `mod.rs` files are required ``. A
  leaf module with no submodules stays a plain `foo.rs`; the rule applies the
  moment the module gains a directory, and then the file moves into it.
  (Topcoat's repository uses the opposite convention. Do not carry it over.)
- **No `unsafe`.** Denied at the workspace root.
- **No floating-point numbers in stored or transmitted data, ever.** Durations
  are integer milliseconds. This is a correctness rule, not a preference: `0.1 +
  0.2` does not round-trip, and the export guarantee is byte-identical.
- **Tests at the bottom of the file** they cover, in `#[cfg(test)] mod tests`.
- **Declare every dependency at the workspace root** with a version and no
  features; opt into features at the use site.
- Prose in this repository uses ordinary typography including em dashes. That is
  a deliberate difference from Topcoat's ASCII-only rule; match the file you are
  editing.

The [`style`](.agents/skills/style/SKILL.md) skill covers code, and
[`prose`](.agents/skills/prose/SKILL.md) covers documentation.

## Skills

| Skill | Use it when |
|---|---|
| [`check`](.agents/skills/check/SKILL.md) | Verifying a change before proposing it |
| [`style`](.agents/skills/style/SKILL.md) | Writing or editing Rust |
| [`prose`](.agents/skills/prose/SKILL.md) | Writing markdown documentation |
| [`design`](.agents/skills/design/SKILL.md) | A change touches or contradicts the design of record |
| [`jj`](.agents/skills/jj/SKILL.md) | Any Jujutsu or Git operation |
| [`commit`](.agents/skills/commit/SKILL.md) | Authoring a commit message |
| [`pr`](.agents/skills/pr/SKILL.md) | Opening a pull request |

## Safety

Only safe Rust. `unsafe_code` is denied at the workspace root, and no exception
has been needed.
