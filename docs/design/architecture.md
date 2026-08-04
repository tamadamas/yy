# Architecture: the host and the front-ends

Part of the [design of record](../DESIGN.md).

Each decision below states the question, the realistic options, the choice, and
what it costs. Nothing here is inherited from another project's vocabulary.

## 4.1 Why a background process at all?

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
([§6.1](protocol.md#61-where-the-host-listens-and-what-that-exposes)). On
`ENOENT` or `ECONNREFUSED` it spawns `yy host serve --detach` and retries with
backoff for up to two seconds. The host takes an exclusive `flock` on
`$XDG_RUNTIME_DIR/yy/host.lock` before binding; if the lock is already held, the
loser exits silently, so two clients racing to spawn a host is harmless. The
lock holder is also responsible for removing a stale socket file it finds on
startup — holding the lock is what proves the previous owner is gone.

**Rejected alternative:** letting each front-end open the SQLite file directly,
permanently. SQLite would handle the locking correctly, but nothing would tell
the terminal UI that the browser just stopped the timer, and the reminder logic
would have no home. The host exists mainly so there is exactly one place where
the rules live.

### 4.1.1 Which platforms this commits to

**Question:** the host listens on a Unix socket, takes an `flock`, and keys its
runtime files off `$XDG_RUNTIME_DIR`. Which operating systems is that?

**Choice: Linux and macOS. Not Windows.**

Windows has had `AF_UNIX` since 2017, so the socket alone would not be the
obstacle. The access control is:
[§6.1](protocol.md#61-where-the-host-listens-and-what-that-exposes) grants
access by file permission — mode `0600`, in a directory only the user can read
— and Windows does not apply that model to `AF_UNIX` sockets. Supporting it
therefore means designing a second access-control story for a platform nobody
has asked about, so it is rejected until somebody does.

macOS costs one rule, stated here once. `$XDG_RUNTIME_DIR` is a
freedesktop.org convention that macOS does not set, so wherever this document
writes `$XDG_RUNTIME_DIR/yy/...`, read *the runtime directory*:
`$XDG_RUNTIME_DIR/yy` where the variable is set, and `$TMPDIR/yy` otherwise,
which is the per-user directory macOS does provide. Created with mode `0700` if
it is not there.

Nothing durable lives in it. The socket, the lock, the status file
([§7](frontends.md)) and the browser token are all recreated by the next host
start, which is why macOS purging its temporary directory costs nothing. The
database is `$YY_DATA/yy.db` ([§5](storage.md)) and is never in the runtime
directory.

**Cost:** a second path rule, and a second release target
([§8.5](repository.md#releases)). Worse, nothing in CI runs on macOS today,
so the macOS path is asserted rather than verified — a real gap, and the reason
it is written down here rather than left implicit in whichever code first calls
`std::env::var`.

## 4.7 Why the front-ends do *not* implement any logic

**Question:** When you press a key, should the front-end update the screen
immediately and let the host confirm afterwards?

This is the standard answer for networked applications, and it was in an earlier
version of this plan. On inspection it does not apply here.

The host runs **on the same machine**. A round trip is well under a millisecond
— faster than the terminal redraws. There is nothing to hide. Implementing
immediate local updates would mean writing the rules for what `start` and `stop`
do **twice**, keeping the two in agreement forever, and defining what happens
when they disagree.

**Choice for v1:** front-ends are deliberately stupid. They send a request, the
host applies the rules and replies with the new state, the front-end draws it.
All logic lives in one place, in one language.

**When this changes:** when the phone talks to the host over a real network,
latency becomes visible and immediate local updates become worth their cost.
That is a later phase, and the design leaves room for it: because the host
already sends *what changed* rather than only *what is*, the front-end can later
apply that change locally without altering the protocol.

**This is the largest simplification in the plan.** It removes a duplicated
implementation, an agreement problem, and a conflict policy — before the first
entry is recorded.

Note that since every front-end is now Rust
([§4.10](smaller-decisions.md#410-smaller-decisions)), "in one language" is no
longer an aspiration held up by discipline. There is no second language in which
the rules *could* be duplicated. This is also what removes the last
justification for a cross-language protocol IDL; see
[§4.6](protocol.md#46-why-json-rpc-20-between-the-host-and-the-front-ends).

### 4.7.1 …and why the process boundary arrives on day two

There is a second implication, and it is what makes [§2](purpose.md#2-the-one-day-mvp)
achievable. If the front-end contains no logic, then *where the logic runs* is
an implementation detail of one trait:

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
- **`RemoteBackend`** — a JSON-RPC client. Ships **day two–three**. From then on
  the CLI prefers it whenever a host is running.

Both call the same `yy-core`, so no rule is written twice and §4.7's argument is
untouched. What moves is only the process boundary. The gain is that the
protocol work — the message types, a server, auto-spawn — leaves the day-one
critical path, and [§12](roadmap.md#12-risks) names exactly that as the schedule
risk while §2 makes day one the binding constraint.

`LocalBackend` does not survive as a shortcut around the host; it survives
because `yy import` and most tests want a database and no server.

**Keeping rule 2 true.** Both `yy-host` and `LocalBackend` take an exclusive
`flock` on `$XDG_RUNTIME_DIR/yy/host.lock` before opening the database for
writing. If a host holds the lock, `LocalBackend` refuses and the CLI uses
`RemoteBackend` instead. So the invariant is **"exactly one process holds a
write connection at a time"**, which is what
[rule 2](rules.md#10-rules-that-must-not-be-broken) is actually protecting, and
"no front-end opens the database" is the end state from the host's arrival
onward.
