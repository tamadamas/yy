# Repository and open-source practice

Part of the [design of record](../DESIGN.md).

## 8.1 Starting point

The repository today is a single package: `Cargo.toml`, `src/main.rs` printing
"Hello, world!", one commit. The first implementation step converts it into a
workspace root with `[workspace.dependencies]`, and moves `src/main.rs` into
`crates/yy-cli/`.

Edition 2024 is already in use. The minimum supported Rust version is **1.97**,
and `rust-toolchain.toml` pins **1.97.1** exactly.

Two things worth separating, because they are usually conflated. A *library*
keeps its MSRV low, because every bump is a cost imposed on its users. `yy` is
an **application**: its users install a binary and never compile it, so a
conservative MSRV buys nothing and costs the newer language and cargo features.
The floor is therefore set by what the project wants, not by caution — Topcoat
needs 1.95, and cargo's `build.warnings` (§8.6) needs 1.97.

The toolchain is pinned to an exact patch version rather than `stable`. This
matters more than it usually would: because warnings are denied, a new lint in
a future stable release turns into a build failure on a day nobody changed
anything. Bumping the pin is then a deliberate, reviewable commit, which is the
only way "warnings are errors" stays sustainable. There is consequently no
separate MSRV job in CI — everyone, including CI, is on the pinned version, so
there is nothing for a second job to discover.

**No git remote is configured.** Everything in §8.4 and §8.5 that assumes GitHub
— Pages, Actions, issue templates, private vulnerability reporting, release
provenance — is blocked until one exists. This is not a design problem, but it
is the reason those items cannot simply be ticked off in order.

## 8.2 Layout

```
yy/
├─ Cargo.toml              # workspace; dependency versions declared once
├─ Cargo.lock              # committed: it is what pins Topcoat (§8.3)
├─ rust-toolchain.toml     # the only place a compiler version is written
├─ rustfmt.toml            # nightly-only options; see §8.6
├─ .cargo/config.toml      # build.warnings = "deny"; see §8.7
├─ deny.toml               # license / advisory / duplicate checks
├─ README.md  CHANGELOG.md  CONTRIBUTING.md  LICENSE (MIT)
├─ CODE_OF_CONDUCT.md  SECURITY.md  RELEASING.md
├─ AGENTS.md               # instructions for coding agents
├─ CLAUDE.md -> AGENTS.md  # symlink; one file, every agent
├─ .agents/skills/         # task-scoped agent instructions
├─ .claude -> .agents      # symlink
├─ justfile                # every recipe is a plain cargo command
├─ .github/workflows/      # ci.yml, semantic-pr.yml, dependencies.yml
├─ schema/                 # frozen JSON Schema, one file per released
│                          #   protocol version — rule 8's evidence (§4.6)
├─ crates/
│  ├─ yy-types/            # the shared serde types — the contract
│  ├─ yy-core/             # domain rules and time logic — no I/O
│  ├─ yy-store/            # SQLite, migrations, JSONL import/export
│  ├─ yy-host/             # the process: JSON-RPC server, subscriptions,
│  │                       #   reminders, and the Topcoat web application
│  ├─ yy-client/           # Backend trait + Local and Remote impls (§4.7.1)
│  ├─ yy-tui/              # terminal UI
│  └─ yy-cli/              # command line
└─ docs/
   ├─ DESIGN.md            # index of the design of record
   ├─ design/              # the design itself, split by area
   ├─ book/                # mdBook: the guide and the specification (§8.4)
   └─ jj/                  # the Jujutsu guide; optional reading (§8.5)
```

`yy-core` contains the rules and touches nothing external — no files, no
terminal, no network. Its tests need none of those either, which is what keeps
them fast and honest.

**`yy-types`, not `yy-proto`.** The old name meant protobuf, and there is no
protobuf. The crate is plain hand-written serde types with no build script and
no generated code, which is the whole point of
[§4.6](protocol.md#46-why-json-rpc-20-between-the-host-and-the-front-ends).
Naming it after the wire would have been wrong too, because the same types are
the JSONL records and the journal payloads
([§5.1](storage.md#51-the-journal--the-truth)); being one set of types for all
three is the property worth protecting, so the name says only that.

**There is no `xtask`.** It existed for one reason — running `protox` so that
nobody needed `protoc` — and that reason is gone with the code generation.
Rule 8's enforcement does not need a binary: it is a test in `yy-types` that
serialises the message types with `schemars` and compares the result against
`schema/`, regenerating on `UPDATE_SCHEMA=1`. That is strictly better than an
xtask plus a CI job, because drift now fails `cargo test` on the contributor's
own machine instead of in CI afterwards. See
[§13](verification.md#13-how-it-is-verified) for what the test actually asserts.

## 8.3 Dependencies

Declared once at the workspace root, with a version and no features; crates opt
into features at the use site.

| Crate | Version | Used by |
|---|---|---|
| `rusqlite` (`bundled`) | 0.40 | `yy-store` |
| `rusqlite_migration` | 2.6 | `yy-store` |
| `jiff` | 0.2 | `yy-core` |
| `ulid` | 3.0 | `yy-core` |
| `serde` / `serde_json` | 1 | everywhere |
| `thiserror` | 2 | everywhere |
| `clap` (`derive`) | 4.6 | `yy-cli` |
| `tokio` | 1.53 | `yy-host`, `yy-client` |
| `tokio-util` (`codec`) | 0.7 | `yy-host`, `yy-client` — newline framing |
| `ratatui` | 0.30 | `yy-tui` |
| `topcoat` | git, `main` | `yy-host` |
| `schemars` | 1 | `yy-types`, dev-dependency only |

Day one needs only the first seven rows. `cc` is required for `rusqlite`'s
bundled SQLite and is assumed present on any machine with a Rust toolchain.

**Topcoat is tracked from `main`, not crates.io.** The framework is moving fast
(0.1.0 to 0.5.0 in eleven days), fixes land upstream well before a release, and
`yy` is small enough to absorb the churn. The dependency is therefore:

```toml
topcoat = { git = "https://github.com/tokio-rs/topcoat", branch = "main" }
```

Three consequences, all accepted deliberately:

- **`Cargo.lock` is committed and is the pin.** A git dependency without a
  lockfile is not reproducible. `yy` ships binaries, so the lockfile belongs in
  the repository regardless; here it is load-bearing. Upgrading is
  `cargo update -p topcoat`, which is a reviewable commit.
- **`yy-host` cannot be published to crates.io** while this holds, because
  crates.io rejects git dependencies. This is fine: `yy-host` is a binary crate.
  It does constrain [§8.5](#85-releases)'s release story, so releases ship
  binaries and `yy-core` / `yy-store` / `yy-types` are what get published, if
  anything does.
- **CI must schedule a `main` build**, not only a lockfile build, or upstream
  breakage is discovered at the worst possible moment. A weekly job that runs
  `cargo update -p topcoat` and the test suite turns "Topcoat broke us" into a
  failing scheduled run rather than a surprise during a release.

## 8.4 Documentation

**mdBook is the publication target; rustdoc is a lint.** These are different
jobs and the distinction is worth stating, because the obvious move is to copy
Topcoat, which embeds its prose guides into rustdoc with
`#![doc = include_str!(...)]`. That is right for Topcoat and wrong here.

Topcoat is a library: its users read docs.rs, so the guide belongs where the API
is. `yy` is an application. Its users run a CLI and will never open `cargo doc`,
so the same technique would file the user guide somewhere no user visits.

The specification matters more. Its audience is someone writing a third-party
front-end or reading their own JSONL export, and
[§4.6](protocol.md#46-why-json-rpc-20-between-the-host-and-the-front-ends)
argues explicitly that such a person need not be a Rust programmer — "a client
is fifty lines in any language" was a reason for choosing JSON-RPC. Publishing
the protocol specification inside rustdoc would make it Rust-only and quietly
give back what that decision bought.

So: **mdBook**, published to GitHub Pages, holding a guide (what it is, how to
use it, how to write a front-end) and a specification (the JSONL format, the
JSON-RPC methods, the error codes). The design of record lives alongside them,
split across [`docs/design/`](.).

**rustdoc still runs in CI**, for one reason: the doc comments on `yy-types` are
the *source* of the protocol reference, which is generated from them and from
the frozen JSON Schema in `schema/`. A broken intra-doc link there is a broken
specification later, so the lints are denied in `[workspace.lints.rustdoc]` and
`cargo doc` runs on every pull request. This keeps [§12](roadmap.md#12-risks)'s
answer to "documentation drifts from the code": the reference is derived from
the types, never hand-written, exactly as it was when the source was `.proto`
comments.

Nothing here needs `--cfg docsrs` feature badges, so nothing here needs nightly.

## 8.5 Process

### Version control: Jujutsu and Git

**Git is the contract.** The remote is GitHub, the workflow is pull requests,
and `CONTRIBUTING.md` describes both. A contributor who has never heard of `jj`
clones with `git`, branches, pushes, and opens a pull request, and nothing in
the repository asks them to do otherwise. No `jj`-only file, hook, or CI step
may ever be required to build, test, or contribute — that is the invariant.

**`main` is written only by merging a pull request.** Direct pushes are rejected
by GitHub branch protection, for everyone including the maintainer, with force
pushes and branch deletion disabled. Every pull request must pass the required
checks below before the merge button unlocks, and pull requests are
squash-merged so the title becomes the commit on `main`.

This costs nothing and buys the property that the history of `main` is exactly
the set of things CI approved. It also removes the only realistic way a
`jj` mistake could reach anyone else: the operations that would be dangerous are
the ones the server refuses. `jj` reinforces it locally, since its default
`immutable_heads()` revset is `trunk() | tags() | untracked_remote_bookmarks()`
and it therefore declines to rewrite anything on `main` before a push is even
attempted.

**The maintainer uses [Jujutsu](https://jj-vcs.dev) in a colocated workspace.**
`jj git init --colocate` puts `.jj` and `.git` side by side over the same
working copy, so the repository is simultaneously a normal Git repository. `jj`
imports and exports Git state on every command, which is what makes the choice
invisible to everyone else.

Why it is worth a line in a design document rather than being a personal habit:
`jj`'s operation log is the same idea as
[§4.4](storage.md#44-why-record-every-change-and-why-that-means-no-confirmations)'s
journal, and `jj undo` is the same idea as
[rule 4](rules.md#10-rules-that-must-not-be-broken). The tool is being built by
someone using a tool built on its own premise, which is a decent daily check
that the premise holds.

Two practical constraints that follow, both already reflected above:

- **Git hooks cannot be relied on.** `jj` does not run them
  (jj-vcs/jj#405). Every check must live in CI and in `just check`, never only
  in a `pre-commit` hook. This is good practice anyway; here it is mandatory.
- **`.gitattributes` is not honoured by `jj`** (jj-vcs/jj#53), so nothing in the
  build may depend on it — in particular, no line-ending normalisation. §4.3
  already requires `\n` and no BOM, enforced by the round-trip test rather than
  by the VCS.

The step-by-step guide is in [`docs/jj/`](../jj/index.md); it assumes Git
experience and no `jj` experience.

### Releases

Conventional Commits, checked in CI on pull request titles. `CHANGELOG.md` in
the Keep a Changelog format, generated from commits rather than maintained by
hand. The specification, the Rust crates, and the web front-end version
independently. Published artifacts carry provenance attestations. The full
procedure, in both `jj` and Git form, is in `RELEASING.md`.

### Community files

MIT licence held by "the yy authors"
([§4.10](smaller-decisions.md#410-smaller-decisions)), contributing guide, code
of conduct, security policy with private vulnerability reporting enabled, issue
and pull request templates. `CONTRIBUTING.md` states that a clean clone needs
only a Rust toolchain and a C compiler — plus a nightly toolchain for `cargo
fmt` and nothing else, which §8.6 explains.

### CI

Actions pinned by commit hash with minimal permissions. `.github/workflows/`
holds three files:

- **`ci.yml`** — required on every pull request: format check (nightly), clippy,
  tests, `cargo doc`, the mdBook build, `cargo deny`, and a "builds from a plain
  `git clone`" job that is [rule 11](rules.md#10-rules-that-must-not-be-broken)'s
  evidence.
- **`semantic-pr.yml`** — the pull request title must be a Conventional Commit,
  since squash-merging makes it the commit on `main`.
- **`dependencies.yml`** — the scheduled `cargo update -p topcoat` build from
  §8.3. Named for what it is; `yy` is not a fork of anything, so "upstream"
  would have been the wrong word.

Two things CI does *not* contain, both deliberately:

- **No `RUSTFLAGS: -D warnings`.** Warnings are denied through
  `build.warnings` in `.cargo/config.toml` (§8.6), which does not invalidate the
  build cache, so one warm cache serves clippy, test, and doc instead of each
  job recompiling the workspace under different flags.
- **No toolchain version.** `rust-toolchain.toml` is the only place a compiler
  version is written, and rustup honours it on the runner, so CI and a
  contributor's machine cannot drift apart.

The schema check needs no job of its own either: it is part of `cargo test`.

## 8.6 Formatting, and why nightly is required for it

`rustfmt.toml` is modelled on Topcoat's and jj's, because both solve the same
problem and their choices are worth copying rather than re-deriving:

```toml
edition = "2024"
max_width = 100

comment_width = 100
wrap_comments = true
format_code_in_doc_comments = true
doc_comment_code_block_width = 100

group_imports = "StdExternalCrate"
imports_granularity = "Crate"
reorder_imports = true
```

**Everything below `max_width` is a nightly-only rustfmt option.** A stable
rustfmt does not apply them; it warns and carries on, which means a
stable-formatted file and a nightly-formatted file differ and CI fails on a
contributor who did nothing wrong. There is no way to have these options and a
single toolchain, so the choice has to be made deliberately:

- `group_imports = "StdExternalCrate"` and `imports_granularity = "Crate"` are
  the reason to accept it. Without them, import blocks drift in every file and
  a meaningful share of every review diff is import churn that no one decided.
  `"Crate"` rather than jj's `"Item"` because it produces fewer lines and
  matches Topcoat, whose source this project reads a lot of.
- `wrap_comments` and `format_code_in_doc_comments` keep prose and doc examples
  inside the same 100 columns as the code.

**This does not weaken the "a fresh clone builds" promise**, and the distinction
matters: `cargo build`, `cargo test`, and `cargo clippy` all run on the pinned
stable toolchain from `rust-toolchain.toml`. Nightly is needed only to *format*.
A contributor who never runs `cargo +nightly fmt` gets a CI failure with an
exact command to fix it, not a broken build. `CONTRIBUTING.md` says so in those
words.

## 8.7 Warnings are errors, without the usual cost

Lint levels are declared in the manifest, not passed on a command line:

```toml
[workspace.lints.rust]
unsafe_code = "deny"

[workspace.lints.rustdoc]
broken_intra_doc_links = "deny"
private_intra_doc_links = "deny"
invalid_html_tags = "deny"

[workspace.lints.clippy]
pedantic = { level = "warn", priority = -1 }
self_named_module_files = "deny"
```

`unsafe_code = "deny"` because nothing in a local time tracker needs it, and a
denied lint is a better guarantee than a convention. The rustdoc lints because
those comments are the specification's source (§8.4). `pedantic` as a warning
rather than an error so it informs without blocking, with individual lints
allowed at the workspace root as they prove noisy — each `allow` being a
decision someone made once, in a reviewable line.

**Module layout is `foo/mod.rs`, never `foo.rs` beside `foo/`.** That is what
`self_named_module_files` enforces; the build fails with `` `mod.rs` files are
required``, naming the file and the destination. A leaf module with no
submodules stays a plain `foo.rs` — the rule applies from the moment a module
acquires a directory, which is also the moment the ambiguity appears.

Worth flagging because it is a trap: Topcoat denies `mod_module_files`, the
*opposite* lint, and therefore uses the opposite layout. The two cannot both be
enabled. Since this project reads Topcoat's source for reference, the difference
is written into `AGENTS.md` and the `style` skill rather than left to be
noticed.

Everything else is denied wholesale, through `.cargo/config.toml`:

```toml
[build]
warnings = "deny"
```

This is the conventional `RUSTFLAGS: -D warnings` replaced by a cargo setting
stabilised in Rust 1.97, and the difference is not cosmetic. **Setting
`RUSTFLAGS` invalidates the build cache.** In the usual arrangement — the flag
in CI, absent locally — the clippy job, the test job and the developer's own
`cargo check` each compile the workspace under different flags and cannot share
artifacts. `build.warnings` does not participate in the cache key, so one warm
cache serves every job, and a local build is the same configuration CI runs.

It also applies to local packages only, leaving dependencies alone, which is
what one actually wants and what `-D warnings` never quite got right.

The escape hatch is an environment variable rather than an edit, so nobody is
tempted to comment the setting out and forget:

```sh
CARGO_BUILD_WARNINGS=allow cargo check    # while working; never committed
```

This is the reason `rust-toolchain.toml` pins an exact patch version (§8.1). A
denied-warnings project on a floating `stable` breaks the day a new lint ships,
for no reason anyone in the repository caused.
