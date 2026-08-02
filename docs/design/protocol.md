# The protocol between the host and the front-ends

Part of the [design of record](../DESIGN.md).

## 4.6 Why JSON-RPC 2.0 between the host and the front-ends?

**Question:** How do the front-ends talk to the host?

An earlier version of this plan chose gRPC with `.proto` files, and gave two
reasons: one message definition that several languages agree on, and a way for
the host to push updates rather than being polled. The first reason assumed a
TypeScript browser client. It no longer exists — the browser front-end is
Topcoat, which is Rust
([§4.10](smaller-decisions.md#410-smaller-decisions)) — so half of the
justification is gone, and the remaining half does not require gRPC.

**Choice:** JSON-RPC 2.0 over the Unix socket
([§6.1](#61-where-the-host-listens-and-what-that-exposes)), framed as
newline-delimited JSON. Request and response types are ordinary serde structs in
`yy-types`, shared by host and clients as a normal Rust dependency.

**Why not gRPC.** gRPC is built for a situation `yy` does not have. It is worth
being explicit about the mismatch, because "the Tokio team also makes Tonic" is
not by itself a reason:

| gRPC's design centre | `yy` |
|---|---|
| Many services, deployed independently, owned by different teams | One workspace, one release unit (§4.10) |
| Several languages, so a language-neutral IDL is mandatory | Rust only |
| A network between machines, so deadlines, retries, TLS and load balancing matter | A Unix socket on one machine; access control is file permissions (§6.1) |
| High throughput, so binary encoding and HTTP/2 multiplexing pay off | A few dozen operations a day ([§4.4](storage.md#44-why-record-every-change-and-why-that-means-no-confirmations)) |

Four properties, none of which hold. Meanwhile the *costs* were all real: a
`proto/` directory, a generated-code crate, an `xtask` for codegen, six
dependencies, and two CI jobs.

**The decisive argument is the type layer, not the build.**
[§5.1](storage.md#51-the-journal--the-truth) already requires the journal payload
to be JSON with the same shapes §4.3 exports. With protobuf on the wire that
promise becomes false: prost-generated types cannot satisfy §4.3's canonical
rendering rules (fixed key order, omit-versus-`null`, no floats, RFC 3339 with
offset), so a second serde representation and a conversion layer between them
are unavoidable. With JSON-RPC there is one set of serde types that is
simultaneously the wire format, the journal payload, and the export format.
That removes a layer and an entire class of "the schema says optional, the
domain says required" bugs.

**Inspectability is on-brand, not cosmetic.** This project's whole claim is that
your data is not locked away — [rule 5](rules.md#10-rules-that-must-not-be-broken),
§4.3, and §5.1 all say so. A protocol you can watch with
`socat - UNIX-CONNECT:$XDG_RUNTIME_DIR/yy/host.sock` and read with your eyes is
continuous with every other decision here. Choosing protobuf on the wire while
insisting on JSON in the journal would have been an internal inconsistency.

**Third-party front-ends get easier, not harder.** A `yy` client is now a socket,
`json`, and about fifty lines in any language. With gRPC it would have needed a
protobuf runtime, generated stubs, and HTTP/2.

**Cost, stated plainly.** Three things get worse, and each has an answer that
must actually be built rather than assumed:

1. **No `buf breaking`.** This is the real loss. The breaking-change check was
   never a standalone feature; it was a *by-product* of having an IDL, and the
   IDL existed for the polyglot problem. Remove the polyglot problem and the
   by-product goes with it. [Rule 8](rules.md#10-rules-that-must-not-be-broken)
   therefore needs a deliberate replacement: a test in `yy-types` serialises
   every message type to a JSON Schema with `schemars` and asserts two things —
   that it matches the committed `schema/current.json`, and that it is an
   additive extension of every frozen `schema/vN.json` from a released protocol
   version. Without this, rule 8 degrades from an enforced invariant to a good
   intention. It is a test rather than a build step precisely so that it cannot
   be skipped: it fails on the contributor's machine, in the run they were going
   to do anyway.
2. **Subscription lifecycle is hand-written.** JSON-RPC 2.0 has no subscription
   concept; §6 specifies the one `yy` uses.
3. **No transport-level backpressure.** Answered in §6 by the resync rule.

## 4.8 Why typed subscriptions rather than address strings

**Question:** How does a front-end say which slice of data it wants to watch?

One common approach is an address string, like `yy-day:/work/2026-07-30`. That
pays off when the set of things to watch is open-ended and unknown in advance —
files, arbitrary resources — because new ones need no protocol change.

`yy` has four kinds of view, and they are known: the day, the issue list, the
review queue, and the list of contexts. Address strings would mean writing a
parser, inventing escaping rules, and turning typos into runtime errors.

**Choice:** subscriptions are typed messages — "day view, context `work`, date
`2026-07-30`". They are an internally tagged serde enum, so the compiler checks
them on both sides; this argument survives the move away from protobuf intact,
precisely because every client is Rust. Adding a view is a protocol change, as
it should be, since front-ends need to learn to render it anyway.

---

## 6. The protocol surface

JSON-RPC 2.0, newline-delimited, bidirectional over one socket. Three message
kinds, all standard: **requests** (with an `id`, expecting a response),
**responses**, and **notifications** (no `id`, no response). Errors use the
standard code space, with `yy`'s own codes in the implementation-defined range
below `-32000` and documented in the specification
([§8.4](repository.md#84-documentation)).

**Requests** — the front-end asks, the host answers: `initialize`, `start`,
`stop`, `resume`, `edit`, `delete`, `undo`, `assign`, `today`, `list_issues`,
`import`, `export`, `ping`, plus `subscribe` and `unsubscribe`.

`initialize` exchanges version numbers and must be the first request on a
connection. A version mismatch produces a clear error naming the versions the
host supports, rather than a confusing failure three requests later. With the
`buf` check gone this handshake is no longer decorative: it is the mechanism
that turns a stale client into a readable error instead of a wrong parse.

**Subscriptions** — `subscribe` takes a typed view descriptor (§4.8) and returns
a subscription id. The host then sends the current state and, from then on, a
`view_changed` **notification** every time it changes. Each update carries the
sequence number of the operation that caused it, so a front-end that reconnects
can say "I last saw 41" and receive only what it missed — or a full fresh state
if it has been away too long. `unsubscribe` ends it.

Three lifecycle rules, written down because nothing enforces them for us:

- **Disconnect cancels everything.** When a connection closes, every
  subscription on it is dropped. There is no server-side state that outlives a
  socket.
- **The host may drop, the client resyncs.** If a subscriber is not keeping up,
  the host is allowed to discard queued updates and send a single "resync from
  `seq`" notification instead. This is what replaces transport-level
  backpressure, and it costs nothing because the sequence-number design above
  already makes a full resync a normal, cheap operation. Unbounded buffering is
  forbidden.
- **Notifications are never acknowledged.** A front-end that needs to know the
  host is alive uses `ping`.

**`export` and `import` do not stream through the socket.** A full export is
unbounded in size, and a single newline-delimited stream would both block the
socket while it is written and force both sides to buffer it. Instead the host
writes the file itself and the response carries the path; `import` takes a path
and the host reads it. This also resolves a tension that existed independently
of the protocol choice: [§4.7.1](architecture.md#471-and-why-the-process-boundary-arrives-on-day-two)
wanted `LocalBackend` for `export`, but
[rule 2](rules.md#10-rules-that-must-not-be-broken) forbids a second writer
while the host holds the lock. Handing the work to the lock holder satisfies
both. For `export`, `-` as the path means the host streams to a file descriptor
the client passes over the socket, so `yy export - | gzip` still works.

### 6.1 Where the host listens, and what that exposes

The draft this document supersedes said both "a Unix socket, so nothing is
exposed to the network" and "the host serves the browser application". Those
cannot both be true of one listener: a browser cannot open a Unix socket. There
are two listeners, and they arrive in different phases because they have
different security properties.

**Phase one — Unix socket only.**
`$XDG_RUNTIME_DIR/yy/host.sock`, mode `0600`. Access control is file
permissions; nothing is bound to any network interface. This is the whole of
version one, and it is the only listener that speaks JSON-RPC.

**Browser phase — an additional loopback listener.**
The host also binds `127.0.0.1:0` (a kernel-assigned port) and serves the
Topcoat application there: ordinary HTML over HTTP, rendered server-side in the
host process. Because Topcoat components run on the server and call `yy-core`
directly, **the browser never speaks the protocol at all** — there is no
gRPC-Web translation, no proxy, and no client bundle that has to be kept in
sync with the message types. The port and a randomly generated bearer token are
written to `$XDG_RUNTIME_DIR/yy/http.json`, mode `0600`; the token is accepted
as a query parameter on first load and then held in a session cookie.

The token is not ceremony. **Loopback is not equivalent to the Unix socket:**
any process running as any user on the machine can connect to a loopback port,
whereas the socket is protected by file permissions. The token restores the
property that reading the runtime directory is what grants access. Adding this
listener is a deliberate widening of the trust boundary, taken when the browser
client is worth it — not a detail of the web client's build.

**Remote access** — reaching the host from another machine — is a separate
decision, requiring real transport security, and is out of scope here. The phone
is served over loopback or an existing tunnel.
