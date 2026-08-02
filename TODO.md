# TODO

Where things stand and what is left. Written 2026-08-01.

Nothing here is implementation of `yy` itself — that is the
[roadmap](docs/design/roadmap.md). This file tracks the repository setup that
was started today and the loose ends in it.

---

## Done

- `LICENSE` — MIT, held by "the yy authors".
- `docs/DESIGN.md` split into `docs/design/` (11 files, §-numbers preserved),
  rewritten for JSON-RPC 2.0, Topcoat, Jujutsu, MIT, and Rust 1.97.
- `docs/jj/` — index, setup, tutorial, github. Written for a Git user.
- `README.md`, `CONTRIBUTING.md`, `RELEASING.md`.
- `AGENTS.md`, `CLAUDE.md -> AGENTS.md`, `.claude -> .agents`,
  `.agents/skills/{check,style,prose,design,jj,commit,pr}`.
- `Cargo.toml` workspace lints, `.cargo/config.toml` (`build.warnings`),
  `rust-toolchain.toml` (pinned 1.97.1), `rustfmt.toml`, `deny.toml`, `justfile`.
- `.github/workflows/{ci,semantic-pr,dependencies}.yml`.
- `docs/book/` mdBook skeleton with stub chapters.

Verified: `cargo clippy` clean, manifest valid, `build.warnings = "deny"` fails
the build as intended, `self_named_module_files` enforces `foo/mod.rs`, and all
relative links in every markdown file resolve.

---

## Blocked on a GitHub remote

None of this can be done until the repository has a remote. It is listed first
because several other items quietly depend on it.

- [ ] Create the GitHub repository and push.
- [ ] **Branch protection on `main`** — this is [rule 12] and currently exists
      only as prose. Required: no direct push, no force push, no deletion,
      required status checks (`format`, `clippy`, `test`, `doc comments`,
      `book`, `cargo-deny`, `builds from a plain git clone`), squash-merge only.
      Apply to administrators too, or the rule is decorative.
- [ ] Enable private vulnerability reporting.
- [ ] Enable GitHub Pages for the mdBook output.
- [ ] Replace `<owner>` placeholders in `README.md`, `CONTRIBUTING.md`, and the
      `repository`/`edit-url-template` fields in `Cargo.toml` and
      `docs/book/book.toml`.
- [ ] Pin every `uses:` in `.github/workflows/` to a full commit SHA. They are
      on version tags now, which §8.5 says is not enough. There is a `TODO`
      comment at the top of `ci.yml`.

[rule 12]: docs/design/rules.md#10-rules-that-must-not-be-broken

## Missing repository files

- [ ] `CHANGELOG.md` — Keep a Changelog format, currently referenced by
      `RELEASING.md` and `CONTRIBUTING.md` but does not exist.
- [ ] `CODE_OF_CONDUCT.md` — referenced in the §8.2 layout.
- [ ] `SECURITY.md` — referenced in the §8.2 layout.
- [ ] `.github/ISSUE_TEMPLATE/` and a pull request template. `CONTRIBUTING.md`
      currently says there is no PR template, which is a valid choice; decide
      deliberately rather than by omission.
- [ ] `CODEOWNERS`.
- [ ] A release workflow. `RELEASING.md` step 6 says "the tag triggers the
      release workflow"; that workflow does not exist yet. It needs to build
      binaries, attach provenance attestations, and create the GitHub release.
- [ ] A Pages deploy workflow for `docs/book/`. `ci.yml` only proves the book
      builds.

## Decisions still open

- [ ] **`clippy.toml`** — not created. Topcoat has one only for
      `doc-valid-idents`. Add it when a lint actually needs configuring, not
      before.
- [ ] **Coverage floor.** §13 promises `just coverage` and a floor; neither the
      recipe nor a CI job exists. Pick a tool (`cargo-llvm-cov`) and a number,
      or drop the promise from §13. Do not leave it as an unbacked claim.
- [ ] **Does anything get published to crates.io?** §8.3 and `RELEASING.md`
      both hedge. `yy-host` cannot be (git dependency on Topcoat). Decide
      whether `yy-types` / `yy-core` / `yy-store` are published, because it
      changes what the release workflow does.
- [ ] **htmx** — currently declared unused in §7 with a stated reason. You said
      "mal sehen". Revisit when the web front-end is actually built, not before.
- [ ] **Is nightly-for-rustfmt worth it?** It is now the *only* nightly in the
      project (§8.6). The argument is import-grouping churn in diffs. If it
      annoys you in practice, dropping `group_imports` and `imports_granularity`
      removes the second toolchain entirely.

## Documentation still to write

The mdBook chapters are stubs. They are deliberately not written yet — each one
gets written when the feature it documents exists.

- [ ] `docs/book/src/guide/*` — installing, tracking a day, correcting a day,
      your data, shell prompt.
- [ ] `docs/book/src/spec/jsonl.md` — the record types and the canonical
      rendering rules from §4.3.
- [ ] `docs/book/src/spec/protocol.md` — generated from the `yy-types` doc
      comments and `schema/`, per §8.4. Needs the generator, which does not
      exist.
- [ ] `docs/book/src/spec/errors.md` — the JSON-RPC error code space.
- [ ] `docs/book/src/{contributing,design}.md` currently stubs; decide whether
      they include the root files or just link to them.

## Local setup (you, not the repository)

- [ ] **Install `jj`** — it is not on this machine yet:
      `cargo install --locked --bin jj jj-cli`.
- [ ] `jj git init --colocate` in this repository.
- [ ] Configure `user.name`, `user.email`, and fish completions —
      [`docs/jj/setup.md`](docs/jj/setup.md).
- [ ] Install `just`, `mdbook`, and `cargo-deny` if you want the full check set
      locally. None are required to build or test.
- [ ] Work through [`docs/jj/tutorial.md`](docs/jj/tutorial.md). Step 6 (`jj
      undo`) first, while the stakes are zero.

## First real code

Everything above is scaffolding. The actual first step is
[roadmap](docs/design/roadmap.md) **Day 1**: convert to a workspace, write
`yy-core` start/stop and time logic, `yy-store` with the journal and rebuild,
the `Backend` trait with `LocalBackend`, and the four CLI commands.

Done when you are tracking the rest of this project with it.
