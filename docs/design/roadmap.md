# Roadmap and risks

Part of the [design of record](../DESIGN.md).

## 9. Roadmap

| Step | Content | Done when |
|---|---|---|
| **Day 1** | Workspace; `yy-core` start/stop and time logic; `yy-store` with journal, projections and rebuild; `Backend` trait with `LocalBackend`; CLI `start`/`stop`/`status`/`today` | You track the rest of the project with it |
| **Day 2–3** | `yy-types` message types and the schema test; `yy-host` speaking JSON-RPC over the Unix socket; auto-spawn and the lock file; `RemoteBackend`; the status file and `yy prompt`; `undo`, `resume`, `--json` everywhere; JSONL export and import with the byte-identical round-trip test | Your data can leave the tool; the shell prompt shows the timer; two shells agree |
| **Week 1** | Terminal UI: panels, timeline, editing, key hints, `?` overlay | A day can be corrected entirely in the terminal |
| **Week 2** | Subscriptions and live updates, reminders, idle and sleep detection, review queue | Two terminals stay in sync; a laptop suspend produces a reviewable gap, not a lost hour |
| **Week 3–4** | Loopback listener with the token; the Topcoat application inside the host; then the installable phone version | Start a timer on the phone, the terminal shows it |
| **Ongoing** | Repository hygiene, documentation site, first tagged release | Someone else can build it from a clean clone |
| **Later** | Parallel entries, additional contexts in the UI, weekly and monthly reports, PDF export, remote access, immediate local updates ([§4.7](architecture.md#47-why-the-front-ends-do-not-implement-any-logic)) | — |

Repository hygiene is deliberately not step one. A `LICENSE`, a `README`, and CI
that runs the tests are there from the start; the full set of community files
arrives before the project is announced, not before it works.

Note that day two lost a whole toolchain relative to the superseded draft: no
`.proto` files, no code generation, no `protoc` question, no `buf`, and no
`xtask` — that crate existed only to run the code generator. What replaced all
of it is one test. This is why the schedule risk below is smaller than it was.

---

## 12. Risks

| Risk | Response |
|---|---|
| The one-day MVP slips into a week | Cut scope, not the deadline. Day one has no protocol, no host, no streaming, no TUI and no reminders — `LocalBackend` ([§4.7.1](architecture.md#471-and-why-the-process-boundary-arrives-on-day-two)) is what buys that |
| The host feels heavier than a single binary | Auto-start behind a lock file, Unix socket, and a `yy prompt` that never touches it |
| **Rule 8 rots without `buf breaking`** | The schema test in `yy-types`, asserting both no-drift and additive-only against the frozen `schema/` files ([§4.6](protocol.md#46-why-json-rpc-20-between-the-host-and-the-front-ends)). This is the one place the protocol change made something *weaker*, so it is the one that must be built on day two rather than deferred |
| Hand-written protocol plumbing has subtle bugs | The framing is `tokio_util`'s `LinesCodec`, not hand-rolled; what remains is a request-id map and a subscription registry. The live test and the crash test in [§13](verification.md#13-how-it-is-verified) cover the lifecycle, and disconnect-cancels-everything ([§6](protocol.md#6-the-protocol-surface)) is the rule that keeps it small |
| SQLite quietly costs the "readable data" property | Round-trip export test as a hard rule, in CI from day two — with the ordering, rendering and anchoring rules in [§4.3](storage.md#43-why-jsonl-for-export) that make it survive hand-edited files |
| Serving the browser quietly exposes the data | The loopback listener is a separate phase with its own token ([§6.1](protocol.md#61-where-the-host-listens-and-what-that-exposes)), not a side effect of building a web client |
| **Topcoat breaks under us** | It is 0.5.0 and says so. Bounded by phase (week three, not day one), by blast radius (`yy-host` only, and the web front-end holds no rules), and by a scheduled CI job that builds against `main` so breakage surfaces on a Monday rather than during a release ([§8.3](repository.md#83-dependencies)) |
| Four front-ends dilute the effort | The terminal UI is the reference; the CLI is narrow; the browser client is small and shares the host's process; no native mobile |
| Documentation drifts from the code | Protocol reference generated from the schema, never hand-written |
| `jj` makes the repository unfriendly to Git users | Colocated workspace, Git as the contract, and the invariant that no `jj`-only step may be required to build, test, or contribute ([§8.5](repository.md#version-control-jujutsu-and-git)) |
