# TODO

Where things stand and what is left. Written 2026-08-01, updated 2026-08-03.

Nothing here is implementation of `yy` itself — that is the
[roadmap](docs/design/roadmap.md). This file tracks the repository setup that
was started today and the loose ends in it.

## To be done

- [ ] **`clippy.toml`** — not created. Topcoat has one only for
      `doc-valid-idents`. Add it when a lint actually needs configuring, not
      before.
- [ ] **Coverage floor.** §13 promises `just coverage` and a floor; neither the
      recipe nor a CI job exists. **§13 keeps the promise** — the gap is
      schedule, not design, and coverage arrives once there is code worth
      measuring. Three pieces, in this order: the `just coverage` recipe with
      `cargo-llvm-cov`, a number chosen when a number can mean something, and a
      job in `ci.yml` that runs it on every pull request. Until all three
      exist, `CONTRIBUTING.md` says plainly that coverage is not measured —
      that sentence is what has to be deleted when this is done.

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

- [ ] Work through [`docs/jj/tutorial.md`](docs/jj/tutorial.md). Step 6 (`jj
      undo`) first, while the stakes are zero.

## First real code

Everything above is scaffolding. The actual first step is
[roadmap](docs/design/roadmap.md) **Day 1**: convert to a workspace, write
`yy-core` start/stop and time logic, `yy-store` with the journal and rebuild,
the `Backend` trait with `LocalBackend`, and the four CLI commands.

Done when you are tracking the rest of this project with it.
