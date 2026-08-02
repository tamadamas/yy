# How it is verified

Part of the [design of record](../DESIGN.md).

## 13. How it is verified

- `just check` — formatting (`cargo +nightly fmt --check`), clippy with warnings
  denied, tests, workspace build.
- `just coverage` — fails below the agreed floor.
- **Schema test** — part of `cargo test`, living in `yy-types`. It serialises
  every request, response, and notification type to a JSON Schema and asserts
  that it matches `schema/current.json`, and that it is an additive extension of
  each frozen `schema/vN.json`: no removed method, no removed or retyped field,
  no field that changed from optional to required. `UPDATE_SCHEMA=1 cargo test`
  regenerates `current.json`, so the diff lands in the pull request where a
  reviewer sees it. This is
  [rule 8](rules.md#10-rules-that-must-not-be-broken)'s enforcement and replaces
  what `buf breaking` used to do
  ([§4.6](protocol.md#46-why-json-rpc-20-between-the-host-and-the-front-ends)).
  There is no separate CI job and no `xtask`; a check you can forget to run is a
  check you will forget to run.
- **Round trip** — export to JSONL, import into an empty database, export again:
  byte-identical. Hand-written comments survive, and survive *in place*, which
  the [§4.3](storage.md#43-why-jsonl-for-export) anchoring rule is what makes
  possible.
- **Crash test** — kill the host with a timer running, restart, and confirm the
  entry is intact and still running with no time lost.
- **Rebuild test** — delete the projection tables, rebuild from the journal, and
  confirm the result is identical. Meaningful because undo appends rather than
  mutates ([§5.3](storage.md#53-undo-is-an-append-not-a-removal)), so the
  journal being replayed is the real history.
- **Lock test** — with a host running, `LocalBackend` refuses the database and
  the CLI reaches the host instead; with no host, it opens it directly.
- **Undo test** — an operation from the CLI is undone from the terminal UI.
- **Live test** — two front-ends open; a change in one appears in the other with
  no input.
- **Subscription lifecycle test** — a subscriber that stops reading is dropped
  and resynced rather than buffered without bound, and closing a connection
  cancels every subscription on it. This covers the plumbing that
  [§4.6](protocol.md#46-why-json-rpc-20-between-the-host-and-the-front-ends)
  admits is hand-written.
- **Handshake test** — a client announcing an unsupported protocol version gets
  a named-versions error on `initialize`, not a parse failure later.
- **Prompt budget** — `yy prompt` measured under 10 ms with a host running, and
  returning immediately with none.
- **Time tests** — a day containing a DST transition totals 23 or 25 hours
  correctly; an entry recorded at `+02:00` exports as `+02:00` from a machine
  running in UTC; an end before its start is kept and tagged `need_review`; a
  timestamp round-trips through JSON without precision loss
  ([§4.9](storage.md#49-how-time-itself-is-stored)).
- **Doc comments** — `cargo doc` with the rustdoc lints denied. Not because
  anyone reads `yy` on docs.rs, but because those comments are the protocol
  reference's source ([§8.4](repository.md#84-documentation)).
- **Book build** — `mdbook build docs/book`, so the guide and specification
  cannot break silently.
- **Dependency build** — a scheduled job runs `cargo update -p topcoat` and the
  full suite, so breakage in a fast-moving dependency is a failing Monday run
  rather than a surprise during a release
  ([§8.3](repository.md#83-dependencies)).

There is deliberately **no MSRV job**: `rust-toolchain.toml` pins one exact
version and CI honours it, so every build everywhere is already that version
([§8.1](repository.md#81-starting-point)).
- **Git-friendliness test** — a plain `git clone` followed by `just check`
  succeeds with no `jj` installed. This is
  [rule 11](rules.md#10-rules-that-must-not-be-broken), and it is the only thing
  standing between a personal tool preference and a contributor barrier.
- **The real test** — the project is tracked in `yy` from day one. Anything that
  annoys you daily gets fixed; anything you never notice was not needed.
