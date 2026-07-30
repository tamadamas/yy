# `yy` — a time tracker you can trust

> This is the design of record. It states what `yy` is, what was decided, and
> why. Where a decision has a cost, the cost is written down next to it. When a
> decision changes, this document changes with it.

## 1. What this is

`yy` records how you spend your working day. You start a task, you stop it, and
at the end of the month you can prove where the hours went.

Two goals, in this order:

1. **A tool used daily.** It must be good enough to track real work within days,
   not months. If it is not usable early, it will never be finished.
2. **A presentable open-source project.** Clear documentation, a stable format,
   a workflow other people can contribute to.

Everything in this plan is judged against those two goals. Anything that serves
neither is cut.

---

## 2. The one-day MVP

The first version must be usable at the end of day one, so that `yy` can track
the work of building `yy`. This is the single most important constraint in the
plan, and it decides what goes into version one and what waits.

Day one delivers exactly this:

```
yy start "writing the storage layer"    # begin
yy status                                # what am I on, how long
yy stop                                  # end
yy today                                 # list of today's entries + total
```

Stored in one SQLite file. No terminal UI, no browser, no synchronization, no
reminders — and, per §4.7.1, **no background process and no protocol either**: on
day one the command line links the storage layer directly. Those come later, and
the design is arranged so they can come later without a rewrite.

Realistically this is one focused day if things go well and two if the code
generation setup fights back. That is acceptable. What is not acceptable is
spending two weeks on architecture before the first entry is recorded.

---

## 3. What existing tools already figured out

Before designing anything, here is what the field has settled on. These are the
lessons `yy` copies deliberately.

**From plain-text trackers ([klog](https://klog.jotaen.net/),
[bartib](https://github.com/nikolassv/bartib), Watson, Timewarrior):**

- The data outlives the program. Every one of these tools stores something you
  can read with your eyes and fix in a text editor. Users pick them *because* of
  that, not despite it. → `yy` must always be able to hand you your data in a
  readable form.
- A file you can copy is a backup, a sync mechanism, and a migration path in
  one, with no code.
- Machine-readable export (`--json`) is what makes a tool scriptable and turns
  users into contributors.
- Timewarrior's lesson specifically: **tags, not a hierarchy.** Rigid
  project trees stop matching reality within a month.
- Bartib's lesson: **resuming the last task must be one command.** It is the
  single most frequent action after a break.

**From terminal UIs (lazygit, k9s, gitui):**

- The real advantage of a terminal UI over a CLI is *discoverability* — you see
  what you can do. People discover features in lazygit they never knew existed
  in git. → The TUI is not a prettier CLI; it is where you learn the tool.
- A fixed set of panels, with the focused one obvious at a glance, beats a
  flexible layout.
- A key-hint bar plus a `?` overlay, always. No hidden shortcuts.
- Do not ask "are you sure?" — see the next point.

**From jujutsu (`jj`):**

- Record every operation, and no operation is dangerous. Undo always works, so
  confirmation prompts become unnecessary friction. This matters *more* for time
  tracking than for version control: lost code usually exists somewhere else,
  but a lost afternoon exists only in your memory, and by Friday it does not.

**From local-first tools (ActivityWatch, atuin):**

- A small local background process with a clear API is a proven pattern for
  "many front-ends, one truth", and it keeps everything on your machine.
- SQLite is the boring, correct choice for local state that several processes
  touch.

**From starship / shell prompts:**

- Anything that runs on every shell prompt has a hard budget of about 10 ms,
  and must degrade to silence rather than block.

---

## 4. Decisions, and why

Each decision below states the question, the realistic options, the choice, and
what it costs. Nothing here is inherited from another project's vocabulary.

### 4.1 Why a background process at all?

**Question:** Should `yy` be one program that runs and exits (like `ls`), or
should something stay running?

The simple version — every command opens the file, does its job, exits — is
attractive. It is what the plain-text trackers do, and it has no moving parts.
It is also what day one ships (§4.7.1), because it is the fastest route to a
working tool.

But three requirements break it:

- **Reminders.** "You have worked 90 minutes without a break" requires someone
  to be awake and counting. A command that has exited cannot notice anything.
- **Idle detection.** If the laptop sleeps for two hours with a timer running,
  someone has to notice the gap and offer to correct it.
- **More than one front-end.** A terminal UI, a browser tab, and a phone
  showing the same day must agree. If each one edits the file directly, they
  will overwrite each other and none of them will see the others' changes.

**Choice:** one small local process (the *host*) owns the data from the moment
the second front-end exists. Every front-end talks to it and never touches the
database.

**Cost:** a process must be started, supervised, and shut down. That is real
complexity, and it is mitigated: the CLI and TUI start the host automatically if
it is not running. You never type "start the server". If the host is not running
and cannot be started, read-only commands fail instantly instead of hanging.

**How auto-start works, concretely.** The client connects to the socket
(§6.1). On `ENOENT` or `ECONNREFUSED` it spawns `yy host serve --detach` and
retries with backoff for up to two seconds. The host takes an exclusive `flock`
on `$XDG_RUNTIME_DIR/yy/host.lock` before binding; if the lock is already held,
the loser exits silently, so two clients racing to spawn a host is harmless. The
lock holder is also responsible for removing a stale socket file it finds on
startup — holding the lock is what proves the previous owner is gone.

**Rejected alternative:** letting each front-end open the SQLite file directly,
permanently. SQLite would handle the locking correctly, but nothing would tell
the terminal UI that the browser just stopped the timer, and the reminder logic
would have no home. The host exists mainly so there is exactly one place where
the rules live.

### 4.2 Why SQLite instead of a text file?

**Question:** The plain-text trackers store a text file, and section 3 says the
readable format is why people choose them. So why a database?

Because the moment several processes read and write at the same time, a text
file stops being safe. Two writers append at once and one entry is lost — not
theoretically, but on the first laptop-wakes-up-during-a-write. Rebuilding
transactions, locking, and crash recovery on top of a text file means writing a
worse database.

There is a second reason: the questions you actually ask ("total per issue for
last month", "which days have gaps") are queries. Answering them over a text
file means loading everything into memory every time. That is fine at one year
of data and unpleasant at five.

**Choice:** one SQLite file at `$YY_DATA/yy.db`, in WAL mode, with exactly one
writer at a time (§4.7.1, rule 2).

**Why SQLite and not something else:** it is a single file you can copy, it has
no server and no configuration, it is the most-tested piece of software of its
kind, and it survives power loss by design. A client-server database would
contradict "local and self-contained". A document store buys nothing here — the
data is small, relational, and boringly shaped.

**Cost:** the file is no longer editable with your eyes. That loss is
unacceptable, which is why the next decision exists.

### 4.3 Why JSONL for export?

**Question:** If the store is a database, in what form do you get your data
back?

The readable format is not a nice-to-have. It is your backup, your escape hatch
if this project is abandoned, and your proof to yourself that nothing is locked
away. So `yy` guarantees a lossless export — and the format matters.

- **CSV**: cannot express a list of tags or an optional field without
  conventions. Rejected.
- **One big JSON file**: the whole file must be rewritten to add one entry, and
  a one-line change produces a diff that touches everything. Rejected.
- **YAML/TOML**: pleasant to read, unpleasant to generate and parse
  reliably, and no better at diffing. Rejected.
- **JSONL** (one JSON object per line): every line is independent, so appending
  is one write, a change to one entry produces a one-line diff, `grep` and
  standard shell tools work, and a corrupted line costs one entry rather than the
  file. This is the same reasoning that made line-oriented formats the norm for
  logs.

**Choice:** JSONL, with a `type` field on every record so new record types can be
added later without breaking old readers.

**Guarantee, enforced by a test:** export → empty database → import → export
produces byte-identical output. Comments and lines the parser does not
understand are preserved through the round trip rather than dropped, so
hand-editing an exported file is safe.

That guarantee is worth nothing unless the rules that make it possible are
pinned down, because it must hold for files a *human* edited, not only for files
`yy` wrote. Three rules:

**Canonical order.** Records are grouped by type in the order `context`,
`issue`, `entry`, and sorted by id within each group. Ids are ULIDs (§4.10), so
this is also chronological, which is the order a reader expects anyway.

**Canonical rendering.** No ambiguity is left to the serializer:

| Rule | Reason |
|---|---|
| Fixed key order, from the struct definition — never a hash map | Map iteration order is not stable across runs |
| No floating-point numbers anywhere, ever | `0.1 + 0.2` does not round-trip; durations are integers of milliseconds |
| Timestamps as RFC 3339 with an explicit offset (`2026-07-30T09:12:03+02:00`) | Preserves the local offset the entry was recorded at (§4.9) |
| Absent optional fields omitted, never emitted as `null` | One representation for "no value", so re-export cannot differ |
| Every line terminated with `\n`, including the last | No "missing trailing newline" diff |
| UTF-8, no BOM, `\n` never `\r\n` | One byte sequence for one document |

**Comment anchoring.** A comment or unparsable line found during import is
stored together with the id of the record it immediately preceded
(`raw_lines.anchor_id`; `NULL` means it was at the end of the file). Export
emits a record's anchored lines immediately before that record, and the
`NULL`-anchored ones last. A comment therefore stays attached to the entry it
describes, even though export re-sorts the file. Without this rule the
guarantee would only hold for files `yy` itself wrote — which is precisely the
case that does not need a guarantee.

### 4.4 Why record every change (and why that means no confirmations)?

**Question:** Should the database hold only the current state, or a history of
how it got there?

Time entries are corrected constantly — a wrong start time, a forgotten stop, a
task assigned to the wrong issue. Two things follow:

- You need to undo mistakes, including mistakes made in a different front-end
  ten seconds ago.
- If you invoice from this data, you need to answer "why does this say 3.5
  hours" months later.

**Choice:** every change is appended to an operation journal. The tables holding
issues and entries are a *projection* of that journal — derived, rebuildable,
never the primary truth. `yy undo` reverts the last operation regardless of
which front-end caused it.

**Consequence, adopted as a rule:** because undo always works, **no command asks
for confirmation and no command destroys data.** Deleting an entry appends a
deletion; it does not remove a row. Confirmation prompts are what you build when
you cannot undo — they trade a real fix for a speed bump. Removing them is a
large part of why keyboard-driven tools feel fast.

**Cost:** the journal grows. At a few dozen operations a day this is measured in
kilobytes per year. Compaction is a problem for a decade from now.

### 4.5 Why store intervals and never a counter?

**Question:** How is elapsed time represented?

The obvious approach — keep a number and add seconds to it while running — has
a failure mode that ruins the tool: if the process dies, the number is wrong,
and if you forget to stop on Friday, it counts all weekend.

**Choice:** store only a start timestamp and an optional end timestamp. Elapsed
time is *computed* on every read: `elapsed = (end or now) − start`.

**Consequences:** a crash loses nothing, because nothing was being accumulated.
Nothing needs to run for time to pass correctly. Correcting a mistake means
editing two timestamps, which is something a human can reason about. This is the
foundational rule of the whole design; every other component assumes it.

A forgotten timer still shows an absurd duration, so an optional automatic close
at end of day is offered — safe precisely because time is derived.

### 4.6 Why gRPC between the host and the front-ends?

**Question:** How do the front-ends talk to the host?

Front-ends will be written in Rust (terminal), and TypeScript (browser). Two
things are needed: one definition of the messages that all languages agree on,
and a way for the host to push updates rather than being polled.

**Choice:** gRPC, defined by `.proto` files.

- The `.proto` files are the contract. Rust and TypeScript types are
  *generated* from them, so a mismatch is a compile error rather than a runtime
  surprise at 6 p.m.
- Pushing updates to a subscribed client is built in (server streaming), which
  is exactly what "the browser stopped the timer, redraw the terminal" needs.

**How the code is generated — and why not `build.rs`.** Generated Rust lives in
`crates/yy-proto/src/gen/` and is **committed**. `yy-proto` has no `build.rs`
at all; it `include!`s those files. Generation is an explicit step,
`cargo xtask proto`, run by whoever changes a `.proto`. It uses
[`protox`](https://crates.io/crates/protox), a protobuf compiler written in
Rust, feeding `tonic-prost-build` — so **`protoc` is not a prerequisite** for
anyone, including the person who edits the protocol. CI runs the same command
and then `git diff --exit-code`, so the committed output cannot drift from the
definitions.

**Compatibility checking.** `buf` lints the definitions and refuses changes that
would break existing clients, so the protocol cannot rot quietly. It runs **in
CI only**, via the published action. Requiring a contributor to install a Go
binary in order to build a Rust project would contradict "a fresh clone builds".
The same applies to `just`: it is the task runner, and every recipe in the
`justfile` is a plain `cargo` invocation you can read and run by hand.

**Browsers:** browsers cannot speak raw gRPC. The host therefore also serves
gRPC-Web, translated inside the host itself by `tonic-web`. **No separate proxy**
is required — this is the single most common complaint about gRPC and it is
avoided by handling both in the same server. What this implies for the listening
socket is a security decision, and it is taken in §6.1 rather than here.

**Cost:** a code generation step in the build, and messages are binary rather
than readable with `curl`.

### 4.7 Why the front-ends do *not* implement any logic

**Question:** When you press a key, should the front-end update the screen
immediately and let the host confirm afterwards?

This is the standard answer for networked applications, and it was in an earlier
version of this plan. On inspection it does not apply here.

The host runs **on the same machine**. A round trip is well under a millisecond
— faster than the terminal redraws. There is nothing to hide. Implementing
immediate local updates would mean writing the rules for what `start` and `stop`
do **twice**, once in Rust and once in TypeScript, keeping the two in agreement
forever, and defining what happens when they disagree.

**Choice for v1:** front-ends are deliberately stupid. They send a request, the
host applies the rules and replies with the new state, the front-end draws it.
All logic lives in one place, in one language.

**When this changes:** when the phone talks to the host over a real network,
latency becomes visible and immediate local updates become worth their cost.
That is a later phase, and the design leaves room for it: because the host
already sends *what changed* rather than only *what is*, the front-end can later
apply that change locally without altering the protocol.

**This is the largest simplification in the plan.** It removes a duplicated
implementation, an agreement problem between two languages, and a conflict
policy — before the first entry is recorded.

#### 4.7.1 …and why the process boundary arrives on day two

There is a second implication, and it is what makes §2 achievable. If the
front-end contains no logic, then *where the logic runs* is an implementation
detail of one trait:

```rust
trait Backend {
    fn start(&self, req: StartRequest) -> Result<State>;
    fn stop(&self, req: StopRequest) -> Result<State>;
    fn status(&self) -> Result<State>;
    fn today(&self, req: TodayRequest) -> Result<DayView>;
    // …one method per request in §6
}
```

Two implementations:

- **`LocalBackend`** — links `yy-store` in-process, no socket, no protocol.
  Ships **day one**.
- **`RemoteBackend`** — a gRPC client. Ships **day two–three**. From then on the
  CLI prefers it whenever a host is running.

Both call the same `yy-core`, so no rule is written twice and §4.7's argument is
untouched. What moves is only the process boundary. The gain is that the
protocol toolchain — `.proto` files, code generation, a server, auto-spawn —
leaves the day-one critical path, and §12 names exactly that toolchain as the
schedule risk while §2 makes day one the binding constraint.

`LocalBackend` does not survive as a shortcut around the host; it survives
because `yy export`, `yy import` and most tests want a database and no server.

**Keeping rule 2 true.** Both `yy-host` and `LocalBackend` take an exclusive
`flock` on `$XDG_RUNTIME_DIR/yy/host.lock` before opening the database for
writing. If a host holds the lock, `LocalBackend` refuses and the CLI uses
`RemoteBackend` instead. So the invariant is **"exactly one process holds a
write connection at a time"**, which is what rule 2 is actually protecting, and
"no front-end opens the database" is the end state from the host's arrival
onward.

### 4.8 Why typed subscriptions rather than address strings

**Question:** How does a front-end say which slice of data it wants to watch?

One common approach is an address string, like `yy-day:/work/2026-07-30`. That
pays off when the set of things to watch is open-ended and unknown in advance —
files, arbitrary resources — because new ones need no protocol change.

`yy` has four kinds of view, and they are known: the day, the issue list, the
review queue, and the list of contexts. Address strings would mean writing a
parser, inventing escaping rules, and turning typos into runtime errors.

**Choice:** subscriptions are typed messages — "day view, context `work`, date
`2026-07-30`". The compiler checks them. Adding a view is a protocol change, as
it should be, since front-ends need to learn to render it anyway.

### 4.9 How time itself is stored

This is the one decision a time tracker cannot get wrong, so it is stated
explicitly rather than left to whichever date library is reached for first.

**Library:** [`jiff`](https://crates.io/crates/jiff), for its separation of
instants from civil dates and its correct handling of time zone transitions.

**Storage:** every timestamp is two integer columns.

| Column | Meaning |
|---|---|
| `*_utc_ms` | Milliseconds since the Unix epoch, UTC. All queries, sorting and elapsed-time arithmetic use only this. |
| `*_offset_min` | The local UTC offset, in minutes, at the moment it was recorded. Display and export only. |

Keeping the offset is what lets export render the original
`2026-07-30T09:12:03+02:00` rather than a UTC instant the reader has to
translate — and it means an entry recorded in another country still reads back
the way you experienced it. Keeping UTC milliseconds separately is what keeps
"which entries overlap" a comparison of two integers.

**"Today" is the local civil day**, resolved through the system time zone at
read time — not a fixed 24-hour window. A day containing a DST transition is 23
or 25 hours long and its total is correct. Day boundaries are computed, never
stored.

**Clocks move backwards.** NTP corrections and manual changes can produce an
entry whose end precedes its start. Such an entry is tagged `need_review` and
kept, never rejected — consistent with §4.5, where storing endpoints rather than
a counter is exactly what makes a wrong endpoint fixable.

### 4.10 Smaller decisions

- **ULID for identifiers** — sortable by creation time, generated without asking
  anyone, and readable enough to type when correcting an entry. Auto-increment
  integers would break as soon as two devices create entries offline.
- **Tags, not a project tree** — see section 3. A hierarchy has to be decided
  before the work is understood; tags can be added afterwards.
- **One repository, one Rust workspace** — the protocol definition, the host,
  and the front-ends change together. Splitting them would mean version
  negotiation between your own components. Dependency versions are declared once
  at the workspace root so they cannot drift.
- **Solid for the browser** — its update model maps directly onto "the host sent
  new state, redraw what changed", the bundle is small, and it requires no
  framework-specific mental model beyond that. The browser client is
  intentionally small, so ecosystem size matters less than simplicity.
- **Bun for the JavaScript toolchain** — one tool for installing, testing,
  and bundling instead of four.
- **mdBook for the documentation** — Rust toolchain, no JavaScript needed to
  build the docs, publishes to GitHub Pages directly.
- **Contexts in the schema from day one** — every entry carries a context
  (`work` by default). The user interface only shows `work` for now. This costs
  one column today and avoids a data migration later.

---

## 5. Data model

One SQLite file (`$YY_DATA/yy.db`, WAL mode), with one writer at a time.

### 5.1 The journal — the truth

```sql
CREATE TABLE operations (
  seq      INTEGER PRIMARY KEY,   -- monotonic, gapless, assigned by SQLite
  at_utc_ms   INTEGER NOT NULL,
  at_offset_min INTEGER NOT NULL,
  actor    TEXT    NOT NULL,      -- 'cli' | 'tui' | 'web' | 'host'
  kind     TEXT    NOT NULL,      -- 'start' | 'stop' | 'edit' | 'delete' | …
  payload  TEXT    NOT NULL       -- JSON
) STRICT;
```

`payload` is **JSON, not protobuf bytes**. Two reasons: the journal stays
inspectable with any SQLite viewer, which matters for a store whose whole
justification is that your data is not locked away; and the payload shapes are
the same shapes §4.3 exports, so there is one set of record definitions rather
than two.

### 5.2 The projections — derived, rebuildable

Nothing here is authoritative. All of it can be dropped and replayed from
`operations`.

| Table | Columns |
|---|---|
| `contexts` | `id`, `slug`, `label`, `daily_target_ms` |
| `issues` | `id`, `context_id`, `external_key`, `title`, `kind`, `created_*` |
| `entries` | `id`, `context_id`, `issue_id?`, `start_utc_ms`, `start_offset_min`, `end_utc_ms?`, `end_offset_min?`, `note?` |
| `entry_tags` | `entry_id`, `tag` — includes `need_review` and `pause` |
| `day_targets` | `context_id`, `date` (civil), `target_ms` |
| `raw_lines` | `anchor_id?`, `ordinal`, `text` — preserved comments (§4.3); `ordinal` orders several lines sharing one anchor |

### 5.3 Undo is an append, not a removal

`yy undo` appends an operation of kind `undo` whose payload carries
`target_seq`. Replay skips any operation that is targeted by a later `undo`
which is not itself undone.

Three things fall out of this, which is why it is worth the small complexity:

- Nothing is ever deleted, so rule 3 holds without exception.
- Undoing an undo is redo, for free, with no extra concept.
- The rebuild test in §13 tests something real. If undo mutated the journal, a
  rebuild would trivially agree with itself and the test would prove nothing.

**Replay is deterministic.** Rebuilding reads no clock and generates no ids —
every timestamp and every identifier a rebuild needs is already in the payload
that recorded it. This is what makes "delete the projections and rebuild" a
safe operation rather than a leap of faith.

### 5.4 Concepts

An *issue* is a unit of work that collects many *entries*. This is the core
modelling decision inherited from the tool's purpose: a day is not a flat list
of activities but several threads you switch between, and a meeting interrupts a
task without ending it.

**In v1 there is at most one open entry.** `start` closes the currently open
entry at the new entry's start time — there is no gap and no overlap. Parallel
entries (a long-running thing that keeps counting while you take a call) are a
later phase; the schema already permits them, since nothing in `entries`
requires the open set to be a single row, so deferring them costs no migration.

An entry may exist with no issue and no description — you can start tracking
first and explain later. This is deliberate: the moment you need to name a task
is the moment you stop tracking it.

---

## 6. The protocol surface

Two kinds of message.

**Requests** — the front-end asks, the host answers. `Initialize` (agree on
version), `Start`, `Stop`, `Resume`, `Edit`, `Delete`, `Undo`, `Assign`,
`Today`, `ListIssues`, `Import`, `Export`, `Ping`.

**Subscriptions** — the front-end asks to watch a view; the host sends the
current state, then sends an update every time it changes. Each update carries
the sequence number of the operation that caused it, so a front-end that
reconnects can say "I last saw 41" and receive only what it missed — or a full
fresh state if it has been away too long.

`Initialize` exchanges version numbers. A version mismatch produces a clear
error naming the versions the host supports, rather than a confusing failure
three requests later.

### 6.1 Where the host listens, and what that exposes

The draft this document supersedes said both "a Unix socket, so nothing is
exposed to the network" and "the host serves the browser application". Those
cannot both be true of one listener: a browser cannot open a Unix socket. There
are two listeners, and they arrive in different phases because they have
different security properties.

**Phase one — Unix socket only.**
`$XDG_RUNTIME_DIR/yy/host.sock`, mode `0600`. Access control is file
permissions; nothing is bound to any network interface. This is the whole of
version one.

**Browser phase — an additional loopback listener.**
The host also binds `127.0.0.1:0` (a kernel-assigned port), serving gRPC-Web via
`tonic-web` plus the static bundle from the same server — §4.6's "no separate
proxy" survives intact. The port and a randomly generated bearer token are
written to `$XDG_RUNTIME_DIR/yy/http.json`, mode `0600`; every request must
carry the token.

The token is not ceremony. **Loopback is not equivalent to the Unix socket:**
any process running as any user on the machine can connect to a loopback port,
whereas the socket is protected by file permissions. The token restores the
property that reading the runtime directory is what grants access. Adding this
listener is a deliberate widening of the trust boundary, taken when the browser
client is worth it — not a detail of the web client's build.

**Remote access** — reaching the host from another machine — is a separate
decision, requiring real transport security, and is out of scope here. The phone
is served over loopback or an existing tunnel.

---

## 7. The front-ends

### Terminal UI — the main one

Fixed panels: issue list, today's timeline, detail/edit, review queue, status
bar. The focused panel is unmistakable. A key-hint bar is always visible and `?`
opens the full list. No confirmation dialogs (§4.4). Every action is a request
to the host, so the terminal stays in sync with everything else for free.

This is where the tool is learned and where a day gets corrected.

### CLI — narrow on purpose

Only what belongs in a shell and needs to be scriptable: `start`, `stop`,
`resume`, `status`, `today`, `undo`, `prompt`, `import`, `export`, `host`. Every
command that prints something supports `--json`. Shell completions for fish,
bash, and zsh.

Anything exploratory or corrective belongs in the terminal UI, not here.

### `yy prompt` — a file read, not a request

`yy prompt` runs on every shell prompt, so it has a 10 ms budget and must never
delay a keystroke. It does **not** speak the protocol.

The host atomically rewrites `$XDG_RUNTIME_DIR/yy/status` — write a temporary
file, `rename` over the target — on every state change. `yy prompt` reads that
one small file, prints, and exits. No socket, no async runtime, no HTTP/2
handshake, and it never spawns a host.

This makes rule 9 hold **by construction** rather than by measurement: no host
means no file, which means no output and no delay. A budget you have to keep
re-measuring as dependencies change is a budget you will eventually miss.

**Cost, stated plainly:** a second derived write path. It is acceptable because
the status file is a projection like any other — delete it and it reappears on
the next state change; it is never read back as truth.

### Browser and phone

The same small Solid application, served by the host over the loopback listener
described in §6.1. Views: today, issues, review, week. Adding a web manifest and
a service worker makes it installable on Android, which covers the actual mobile
need — start and stop a timer away from the desk, see where the day stands.

Accepted limits: no home-screen widget, and no background execution. Neither
matters, because the host does the counting. A native mobile application is out
of scope; it would mean maintaining a third front-end for a feature set of two
buttons.

---

## 8. Repository and open-source practice

### 8.1 Starting point

The repository today is a single package: `Cargo.toml`, `src/main.rs` printing
"Hello, world!", one commit. The first implementation step converts it into a
workspace root with `[workspace.dependencies]`, and moves `src/main.rs` into
`crates/yy-cli/`. Edition 2024 is already in use; the minimum supported Rust
version is declared as **1.94** and tested in CI.

**No git remote is configured.** Everything in §8.4 and §8.5 that assumes GitHub
— Pages, Actions, issue templates, private vulnerability reporting, release
provenance — is blocked until one exists. This is not a design problem, but it
is the reason those items cannot simply be ticked off in order.

### 8.2 Layout

```
yy/
├─ Cargo.toml              # workspace; dependency versions declared once
├─ rust-toolchain.toml     # pinned toolchain
├─ deny.toml               # license / advisory / duplicate checks
├─ README.md  CHANGELOG.md  CONTRIBUTING.md  LICENSE (MIT)
├─ CODE_OF_CONDUCT.md  SECURITY.md  RELEASING.md
├─ justfile                # every recipe is a plain cargo command
├─ .github/                # workflows, issue + PR templates, CODEOWNERS
├─ proto/                  # message definitions — the contract
├─ crates/
│  ├─ yy-proto/            # generated types (committed; no build.rs)
│  ├─ yy-core/             # domain rules and time logic — no I/O
│  ├─ yy-store/            # SQLite, migrations, JSONL import/export
│  ├─ yy-host/             # the process: server, subscriptions, reminders
│  ├─ yy-client/           # Backend trait + Local and Remote impls (§4.7.1)
│  ├─ yy-tui/              # terminal UI
│  ├─ yy-cli/              # command line
│  └─ xtask/               # `cargo xtask proto` — protox codegen
├─ clients/web/            # Solid application (Bun)
└─ docs/                   # this document; mdBook guide + format spec
```

`yy-core` contains the rules and touches nothing external — no files, no
terminal, no network. Its tests need none of those either, which is what keeps
them fast and honest.

### 8.3 Dependencies

Declared once at the workspace root. Verified available at the versions below.

| Crate | Version | Used by |
|---|---|---|
| `rusqlite` (`bundled`) | 0.40 | `yy-store` |
| `rusqlite_migration` | 2.6 | `yy-store` |
| `jiff` | 0.2 | `yy-core` |
| `ulid` | 3.0 | `yy-core` |
| `serde` / `serde_json` | 1 | everywhere |
| `clap` (`derive`) | 4.6 | `yy-cli` |
| `tokio` | 1.53 | `yy-host` |
| `tonic`, `tonic-prost`, `tonic-web` | 0.14 | `yy-host`, `yy-client` |
| `prost` | 0.14 | `yy-proto` |
| `tonic-prost-build`, `protox` | 0.14 / 0.9 | `xtask` only |
| `ratatui` | 0.30 | `yy-tui` |

Day one needs only the first six rows. `cc` is required for `rusqlite`'s bundled
SQLite and is assumed present on any machine with a Rust toolchain.

### 8.4 Documentation

mdBook, published to GitHub Pages. A guide (what it is, how to use it, how to
write a front-end) and a specification (the JSONL format, the messages, the
error codes). The message reference is *generated from the `.proto` comments*,
because hand-written protocol documentation is wrong within a month. This
document is the design of record and lives alongside them.

### 8.5 Process

**Version control** — Git is the contract: the remote, the pull request
workflow, and `CONTRIBUTING.md` all describe Git. Using `jj` locally in a
colocated repository is noted as an option and requires nothing from anyone
else.

**Releases** — Conventional Commits, checked in CI. `CHANGELOG.md` in the Keep a
Changelog format, generated from commits rather than maintained by hand. The
specification, the Rust crates, and the browser client version independently.
Published artifacts carry provenance attestations.

**Community files** — MIT license, contributing guide (commit format, run
`just check` before committing, branch naming), code of conduct, security policy
with private vulnerability reporting enabled, issue and pull request templates.
`CONTRIBUTING.md` states that a clean clone needs only a Rust toolchain and a C
compiler — not `protoc`, not `buf`, not `just`.

**CI** — actions pinned by commit hash with minimal permissions, and: format
check, clippy with warnings denied, tests, coverage floor, `cargo deny`,
`cargo xtask proto` followed by `git diff --exit-code`, `buf lint` and a
breaking-change check against the last released protocol tag, lint and tests for
the browser client, and scheduled dependency updates.

---

## 9. Roadmap

| Step | Content | Done when |
|---|---|---|
| **Day 1** | Workspace; `yy-core` start/stop and time logic; `yy-store` with journal, projections and rebuild; `Backend` trait with `LocalBackend`; CLI `start`/`stop`/`status`/`today` | You track the rest of the project with it |
| **Day 2–3** | `proto/` + `cargo xtask proto`; `yy-host` over the Unix socket; auto-spawn and the lock file; `RemoteBackend`; the status file and `yy prompt`; `undo`, `resume`, `--json` everywhere; JSONL export and import with the byte-identical round-trip test | Your data can leave the tool; the shell prompt shows the timer; two shells agree |
| **Week 1** | Terminal UI: panels, timeline, editing, key hints, `?` overlay | A day can be corrected entirely in the terminal |
| **Week 2** | Subscriptions and live updates, reminders, idle and sleep detection, review queue | Two terminals stay in sync; a laptop suspend produces a reviewable gap, not a lost hour |
| **Week 3–4** | Loopback listener with the token, browser client, then the installable phone version | Start a timer on the phone, the terminal shows it |
| **Ongoing** | Repository hygiene, documentation site, first tagged release | Someone else can build it from a clean clone |
| **Later** | Parallel entries, additional contexts in the UI, weekly and monthly reports, PDF export, remote access, immediate local updates (§4.7) | — |

Repository hygiene is deliberately not step one. A `LICENSE`, a `README`, and CI
that runs the tests are there from the start; the full set of community files
arrives before the project is announced, not before it works.

---

## 10. Rules that must not be broken

1. **Time is computed, never accumulated.** Only start and end are stored.
2. **Exactly one process holds a write connection at a time**, enforced by the
   host lock file. Once the host exists, no front-end opens the database at all.
3. **Nothing is destroyed.** Every change appends to the journal; deletion is an
   entry, not a removal, and so is undo.
4. **Undo always works — therefore nothing asks for confirmation.**
5. **Your data can always leave**, losslessly, as readable JSONL, verified by a
   round-trip test.
6. **The rules live in one place** (`yy-core`), in one language.
7. **`yy-core` touches nothing external** — no files, no terminal, no network.
8. **Protocol changes are additive.** Optional fields, unknown data ignored,
   enforced by the breaking-change check.
9. **`yy prompt` never blocks.** It reads one file and exits; no host means no
   file means no output.
10. **Replay is deterministic.** Rebuilding projections reads no clock and
    generates no identifiers.

---

## 11. Glossary

- **Host** — the small background process that owns the data and enforces the
  rules. Started automatically; not a service you administer.
- **Front-end / client** — anything you interact with: the terminal UI, the
  command line, the browser.
- **Backend** — the trait a front-end calls (§4.7.1). `LocalBackend` links the
  store directly; `RemoteBackend` talks to the host. Not a user-facing concept.
- **Entry** — one tracked span of time: a start, usually an end.
- **Issue** — a unit of work that collects many entries.
- **Context** — which part of life an entry belongs to. Only `work` for now.
- **Journal / operation log** — the append-only record of every change. The
  source of truth, and what makes undo possible.
- **Projection** — a table derived from the journal for fast queries. Can be
  rebuilt and thrown away. The status file (§7) is one too.
- **Subscription** — a front-end asking to be told when a view changes, instead
  of asking repeatedly.
- **JSONL** — one JSON object per line. Readable, appendable, diffable,
  greppable.
- **WAL** — SQLite's write-ahead logging mode; lets one writer and several
  readers work at once and survives power loss.

---

## 12. Risks

| Risk | Response |
|---|---|
| The one-day MVP slips into a week | Cut scope, not the deadline. Day one has no protocol, no host, no streaming, no TUI and no reminders — `LocalBackend` (§4.7.1) is what buys that |
| The host feels heavier than a single binary | Auto-start behind a lock file, Unix socket, and a `yy prompt` that never touches it |
| Code generation complicates the build | `protoc` is never required (`protox` in an xtask), generated code is committed, and CI proves it matches |
| SQLite quietly costs the "readable data" property | Round-trip export test as a hard rule, in CI from day two — with the ordering, rendering and anchoring rules in §4.3 that make it survive hand-edited files |
| Serving the browser quietly exposes the data | The loopback listener is a separate phase with its own token (§6.1), not a side effect of building a web client |
| Four front-ends dilute the effort | The terminal UI is the reference; the CLI is narrow; the browser client is small; no native mobile |
| Documentation drifts from the code | Protocol reference generated from the definitions, never hand-written |

---

## 13. How it is verified

- `just check` — formatting, clippy with warnings denied, tests, workspace build.
- `just coverage` — fails below the agreed floor.
- `just proto` (`cargo xtask proto`) — regenerate and confirm no diff, plus `buf` style and
  breaking-change checks against the last released protocol tag (CI).
- **Round trip** — export to JSONL, import into an empty database, export again:
  byte-identical. Hand-written comments survive, and survive *in place*, which
  the §4.3 anchoring rule is what makes possible.
- **Crash test** — kill the host with a timer running, restart, and confirm the
  entry is intact and still running with no time lost.
- **Rebuild test** — delete the projection tables, rebuild from the journal, and
  confirm the result is identical. Meaningful because undo appends rather than
  mutates (§5.3), so the journal being replayed is the real history.
- **Lock test** — with a host running, `LocalBackend` refuses the database and
  the CLI reaches the host instead; with no host, it opens it directly.
- **Undo test** — an operation from the CLI is undone from the terminal UI.
- **Live test** — two front-ends open; a change in one appears in the other with
  no input.
- **Prompt budget** — `yy prompt` measured under 10 ms with a host running, and
  returning immediately with none.
- **Time tests** — a day containing a DST transition totals 23 or 25 hours
  correctly; an entry recorded at `+02:00` exports as `+02:00` from a machine
  running in UTC; an end before its start is kept and tagged `need_review`.
- **The real test** — the project is tracked in `yy` from day one. Anything that
  annoys you daily gets fixed; anything you never notice was not needed.
