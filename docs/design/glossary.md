# Glossary

Part of the [design of record](../DESIGN.md).

## 11. Glossary

- **Host** — the small background process that owns the data and enforces the
  rules. Started automatically; not a service you administer. It also serves the
  web front-end.
- **Front-end / client** — anything you interact with: the terminal UI, the
  command line, the browser.
- **Backend** — the trait a front-end calls
  ([§4.7.1](architecture.md#471-and-why-the-process-boundary-arrives-on-day-two)).
  `LocalBackend` links the store directly; `RemoteBackend` talks to the host.
  Not a user-facing concept.
- **Entry** — one tracked span of time: a start, usually an end.
- **Issue** — a unit of work that collects many entries.
- **Context** — which part of life an entry belongs to. Only `work` for now.
- **Journal / operation log** — the append-only record of every change. The
  source of truth, and what makes undo possible.
- **Projection** — a table derived from the journal for fast queries. Can be
  rebuilt and thrown away. The status file
  ([§7](frontends.md#yy-prompt--a-file-read-not-a-request)) is one too.
- **Subscription** — a front-end asking to be told when a view changes, instead
  of asking repeatedly.
- **JSON-RPC 2.0** — the request/response format spoken over the Unix socket.
  Requests carry an `id` and expect a response; **notifications** carry no `id`
  and expect none, which is how the host pushes updates.
- **Resync** — the host telling a subscriber "start again from sequence `n`"
  instead of buffering updates it cannot keep up with
  ([§6](protocol.md#6-the-protocol-surface)).
- **JSONL** — one JSON object per line. Readable, appendable, diffable,
  greppable.
- **WAL** — SQLite's write-ahead logging mode; lets one writer and several
  readers work at once and survives power loss.
- **Topcoat** — the Rust web framework the browser front-end is written in.
  Components are async functions that run on the server, so the browser needs no
  protocol client ([§7](frontends.md#browser-and-phone--topcoat-served-by-the-host)).
- **Colocated workspace** — a directory that is both a `jj` workspace and a Git
  repository, sharing one working copy. How this project is developed without
  requiring anyone else to use `jj`
  ([§8.5](repository.md#version-control-jujutsu-and-git)).
