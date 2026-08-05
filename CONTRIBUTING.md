# Contributing to yy

Thanks for your interest. This is the short, human version of how a change gets
from your machine into the repository.

## Before you write code

**Fixes are welcome without asking.** A bug with a fix, a broken link, a
confusing paragraph, a missing test: open a pull request. You do not need to
file an issue first.

**Features need a conversation first.** `yy` has a
[design of record](docs/DESIGN.md) that states what it is and, more usefully,
what it deliberately is not. A feature has to fit that design or change it, and
finding out after you wrote the code is the worst outcome for both of us. Open
an issue describing what you want and why.

If your change contradicts something in the design, that is not automatically a
rejection — but say so explicitly in the issue, and expect the conversation to
be about the design document rather than the code.

**AI-assisted contributions are fine**, and are used here too. Treat the result
as your own work: read the code, read the diff, be ready to explain every line.
Reviewing is the expensive part and it does not scale. If nobody read the change
before it arrived, all that is left for a reviewer is work that could have been
prompted for directly. Say in the pull request which model you used and what it
did. Agent-specific instructions live in [AGENTS.md](AGENTS.md).

## Setting up

You need a Rust toolchain and a C compiler. `rust-toolchain.toml` pins the exact
compiler version, so rustup installs it for you on the first `cargo` command.

```sh
git clone https://github.com/tamadamas/yy
cd yy
cargo test
```

There is nothing else. No `protoc`, no Node.js, no `buf`, no code generation
step. `just` is a convenience for running the checks below; every recipe in the
`justfile` is a plain `cargo` command you can read and run by hand.

One extra toolchain is needed to *format*:

```sh
rustup toolchain install nightly --component rustfmt
```

## Running the checks

Run this before you push. It is what CI runs, so it saves you a round trip:

```sh
just check
```

or, by hand:

```sh
cargo +nightly fmt --all --check
cargo clippy --workspace --all-targets --all-features --locked
cargo test --workspace --all-features --locked
cargo doc --workspace --all-features --no-deps --locked
```

**Nothing runs these for you.** There is no pre-commit hook, on purpose: the
maintainer uses [Jujutsu](docs/jj/index.md), which does not execute Git hooks,
so a hook would silently protect some people and not others. Every check lives
in `just check` and in CI instead.

### Formatting

`cargo +nightly fmt`, not `cargo fmt`. `rustfmt.toml` uses options that only
exist on nightly — import grouping in particular — and a stable rustfmt ignores
them silently, producing a file that fails CI although you did run the
formatter.

This is the *only* thing in the project that needs nightly. Building, testing,
linting and documentation all run on the pinned stable toolchain.

### Warnings are errors

`.cargo/config.toml` sets `build.warnings = "deny"`, so a warning fails the
build locally exactly as it does in CI. While you are mid-refactor and do not
want to fix them yet:

```sh
CARGO_BUILD_WARNINGS=allow cargo check
```

Use the environment variable rather than editing the file, so the setting cannot
accidentally be committed in the off position.

## Making the change

Small, focused pull requests are much easier to accept than large ones. If a
change grows past a fix, that is usually the signal to open a conversation.

Match the code around you. Two conventions show up in nearly every diff:

- **No `unsafe`.** It is denied at the workspace root.
- **Tests live at the bottom of the file** they test, in `#[cfg(test)] mod
  tests`. `yy-core` has no I/O, so its tests need no files, no terminal and no
  network; keep it that way.

### Tests

Send a test with the change. A bug fix gets the test that would have caught it,
and a new behaviour gets the test that says what it does — that is what makes a
review about the change rather than about whether it works at all. `cargo test`
runs on every pull request and a failing test blocks the merge.

What matters is which guarantee a test makes, not how many there are.
[How it is verified](docs/design/verification.md) lists the ones the project
depends on: the export round-trip, the journal rebuild, and the schema
compatibility test are the load-bearing three, and a change near any of them
should say in the pull request what it does to that guarantee.

**Coverage is not measured yet.** §13 promises a `just coverage` recipe that
fails below an agreed floor; neither the recipe, the tool choice, nor a CI job
exists today, and the tests that exist are few because the implementation has
barely started. It is planned, including the GitHub Actions job, and tracked in
[TODO.md](TODO.md). Until it lands, nothing enforces a floor and a pull request
will not be rejected for missing one — which is exactly why sending the test
yourself matters now.

### Schema

If your change touches the protocol types in `yy-types`, the schema test will
fail until you regenerate the snapshot:

```sh
just schema
```

Commit the resulting `schema/current.json` diff. It is meant to be visible in
review — [rule 8](docs/design/rules.md#10-rules-that-must-not-be-broken) says
protocol changes are additive, and that diff is how anyone can tell.

If your change contradicts the design document, update the design document in
the same pull request. It is the design *of record*; a decision that only exists
in code is not a decision anyone can find later.

## Commits and pull requests

Pull request titles follow
[Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <subject>
```

The type is one of `feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`,
`build`, `ci`, `chore`, `revert`. The optional scope is the area you touched:
a crate name with the `yy-` prefix dropped (`core`, `store`, `host`, `cli`,
`tui`, `types`, `client`), or `design` / `jj` for documentation. The subject is
imperative present tense, lowercase, no trailing period:

```
fix(store): keep comments anchored when re-exporting
```

A CI job enforces this on pull request titles. **Pull requests are
squash-merged**, so the title becomes the commit on `main` and the changelog
entry, and your branch history is discarded — you do not need to tidy it up
before pushing.

For a breaking change, add `!` after the type or scope and end the body with a
`BREAKING CHANGE:` footer explaining how to migrate.

[The pull request template](.github/PULL_REQUEST_TEMPLATE.md) asks for the same
things a review actually needs: describe the whole diff rather than your last
commit, say how you verified it, and reference any issue it closes
(`Closes #123`).

## How a change reaches `main`

**`main` is never written to directly.** Branch protection rejects direct
pushes, force pushes and branch deletion for everyone, including the maintainer.
Every change arrives by a pull request that passed CI and was squash-merged.
This is [rule 12](docs/design/rules.md#10-rules-that-must-not-be-broken).

The normal loop:

1. Branch off `main`.
2. Make the change, with tests where it makes sense.
3. Run `just check`.
4. Push to your fork and open a pull request against `main`.
5. Keep it mergeable by rebasing on `main`, not by merging `main` into it.

## Using Git, or Jujutsu

**Use Git. Everything works.** The repository is a normal Git repository on
GitHub, and CI has a job whose only purpose is to prove that a plain `git clone`
builds and tests with no other tooling
([rule 11](docs/design/rules.md#10-rules-that-must-not-be-broken)).

The maintainer uses [Jujutsu](https://jj-vcs.dev) in a colocated workspace,
which is invisible from the outside. If you are curious, [`docs/jj/`](docs/jj/index.md)
is a step-by-step guide written for people who know Git and have never used
`jj`. It is optional reading and always will be.

## Where to find things

- [Design of record](docs/DESIGN.md) — what `yy` is and why, decision by decision.
- [Rules](docs/design/rules.md) — the twelve invariants. If a change breaks one,
  it needs a very good argument.
- [How it is verified](docs/design/verification.md) — what the tests actually
  guarantee.
- [AGENTS.md](AGENTS.md) — instructions for coding agents.

By contributing, you agree that your contributions are licensed under the
[MIT licence](LICENSE).
