# The front-ends

Part of the [design of record](../DESIGN.md).

## 7. Four surfaces, one set of rules

Every front-end here is Rust, and none of them contains any logic
([§4.7](architecture.md#47-why-the-front-ends-do-not-implement-any-logic)).

### Terminal UI — the main one

Fixed panels: issue list, today's timeline, detail/edit, review queue, status
bar. The focused panel is unmistakable. A key-hint bar is always visible and `?`
opens the full list. No confirmation dialogs
([§4.4](storage.md#44-why-record-every-change-and-why-that-means-no-confirmations)).
Every action is a request to the host, so the terminal stays in sync with
everything else for free.

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
one small file, prints, and exits. No socket, no async runtime, no protocol
handshake, and it never spawns a host.

This makes [rule 9](rules.md#10-rules-that-must-not-be-broken) hold **by
construction** rather than by measurement: no host means no file, which means no
output and no delay. A budget you have to keep re-measuring as dependencies
change is a budget you will eventually miss.

It is also why protocol performance never enters the design: the only latency
budget in the project belongs to the one component that does not use the
protocol.

**Cost, stated plainly:** a second derived write path. It is acceptable because
the status file is a projection like any other — delete it and it reappears on
the next state change; it is never read back as truth.

### Browser and phone — Topcoat, served by the host

A [Topcoat](https://github.com/tokio-rs/topcoat) application, compiled into the
host binary and served over the loopback listener described in
[§6.1](protocol.md#61-where-the-host-listens-and-what-that-exposes). Views:
today, issues, review, week.

The important structural consequence: Topcoat components are async Rust
functions that run **on the server** and can call `yy-core` and `yy-store`
directly, so the web front-end needs no API layer, no client bundle of message
types, and no protocol client. It is the same process reading the same tables
the TUI's requests go through.

Client-side reactivity comes from Topcoat's own runtime — signals, `@` event
handlers, and `#[shard]` components that re-render on the server when their
inputs change, streamed over server-sent events. There is no WebAssembly, and
there is no Node.js: Tailwind runs through Topcoat's build-script wrapper around
the standalone Tailwind CLI.

**htmx is not used.** Topcoat ships a `topcoat-htmx` integration, but it is a
request/response header helper for people who already run htmx; Topcoat's own
shard-and-signal model covers the same ground. Running both would mean two
reactivity models in one small application. If a case appears that shards handle
badly, adding `topcoat-htmx` later is a dependency line, not a redesign.

Adding a web manifest and a service worker makes the application installable on
Android, which covers the actual mobile need — start and stop a timer away from
the desk, see where the day stands.

Accepted limits: no home-screen widget, and no background execution. Neither
matters, because the host does the counting. A native mobile application is out
of scope; it would mean maintaining a third front-end for a feature set of two
buttons.

**Cost, stated plainly.** Topcoat is at 0.5.0, published four months ago, and
its README says "early-stage and experimental, expect breaking changes". This is
accepted deliberately and bounded three ways: the browser front-end is week
three or four on the [roadmap](roadmap.md#9-roadmap), not day one; it is the
only component that depends on Topcoat; and nothing behind
[rule 6](rules.md#10-rules-that-must-not-be-broken) changes if it has to be
replaced, because the rules live in `yy-core` and the web front-end has none.
See [§8.3](repository.md#83-dependencies) for how the dependency is pinned.
