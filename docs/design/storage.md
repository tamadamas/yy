# Storage, time, and the data model

Part of the [design of record](../DESIGN.md).

## 4.2 Why SQLite instead of a text file?

**Question:** The plain-text trackers store a text file, and
[§3](prior-art.md#3-prior-art) says the readable format is why people choose
them. So why a database?

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

## 4.3 Why JSONL for export?

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
`issue`, `entry`, and sorted by id within each group. Ids are ULIDs
([§4.10](smaller-decisions.md#410-smaller-decisions)), so this is also
chronological, which is the order a reader expects anyway.

**Canonical rendering.** No ambiguity is left to the serializer:

| Rule | Reason |
|---|---|
| Fixed key order, from the struct definition — never a hash map | Map iteration order is not stable across runs |
| No floating-point numbers anywhere, ever | `0.1 + 0.2` does not round-trip; durations are integers of milliseconds |
| Timestamps as RFC 3339 with an explicit offset (`2026-07-30T09:12:03+02:00`) | Preserves the local offset the entry was recorded at (§4.9) |
| Absent optional fields omitted, never emitted as `null` | One representation for "no value", so re-export cannot differ |
| Every line terminated with `\n`, including the last | No "missing trailing newline" diff |
| UTF-8, no BOM, `\n` never `\r\n` | One byte sequence for one document |

These rules are not export-only. Because the wire format is JSON as well
([§4.6](protocol.md#46-why-json-rpc-20-between-the-host-and-the-front-ends)) and
the journal payload is JSON
([§5.1](#51-the-journal--the-truth)), one set of serde types
serves all three, and this table is what those types guarantee everywhere.

**Comment anchoring.** A comment or unparsable line found during import is
stored together with the id of the record it immediately preceded
(`raw_lines.anchor_id`; `NULL` means it was at the end of the file). Export
emits a record's anchored lines immediately before that record, and the
`NULL`-anchored ones last. A comment therefore stays attached to the entry it
describes, even though export re-sorts the file. Without this rule the
guarantee would only hold for files `yy` itself wrote — which is precisely the
case that does not need a guarantee.

## 4.4 Why record every change (and why that means no confirmations)?

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

## 4.5 Why store intervals and never a counter?

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

## 4.9 How time itself is stored

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

**Milliseconds survive JSON.** Since both the export format and the wire format
are JSON, and JSON has exactly one number type, an `i64` millisecond timestamp
has to be representable exactly as an IEEE 754 double. It is: doubles are exact
up to 2^53 ≈ 9.0 × 10^15 ms, and current timestamps are around 1.8 × 10^12 ms,
leaving roughly 285,000 years of headroom. This is checked rather than assumed
because it is the standard way JSON protocols lose `i64` precision. Durations
are milliseconds for the same reason, and the "no floats, ever" rule in §4.3 is
what keeps the property from eroding.

**"Today" is the local civil day**, resolved through the system time zone at
read time — not a fixed 24-hour window. A day containing a DST transition is 23
or 25 hours long and its total is correct. Day boundaries are computed, never
stored.

**Clocks move backwards.** NTP corrections and manual changes can produce an
entry whose end precedes its start. Such an entry is tagged `need_review` and
kept, never rejected — consistent with §4.5, where storing endpoints rather than
a counter is exactly what makes a wrong endpoint fixable.

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

`payload` is **JSON**. Two reasons: the journal stays inspectable with any
SQLite viewer, which matters for a store whose whole justification is that your
data is not locked away; and the payload shapes are the same shapes §4.3
exports and §4.6 puts on the wire, so there is **one** set of record definitions
rather than three.

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
- The rebuild test in [§13](verification.md#13-how-it-is-verified) tests
  something real. If undo mutated the journal, a rebuild would trivially agree
  with itself and the test would prove nothing.

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
