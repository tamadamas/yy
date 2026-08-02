# `yy` — a time tracker you can trust

> This is the design of record. It states what `yy` is, what was decided, and
> why. Where a decision has a cost, the cost is written down next to it. When a
> decision changes, this document changes with it.

The document is split across [`docs/design/`](design/). Section numbers are
stable and survive the split, so a reference to §4.6 means the same thing it
always did — only the file it lives in changed.

## The documents

| Document | Sections | What it settles |
|---|---|---|
| [Purpose and the one-day MVP](design/purpose.md) | §1, §2 | What `yy` is for, and the constraint that decides everything else |
| [What existing tools figured out](design/prior-art.md) | §3 | Lessons copied deliberately from klog, bartib, lazygit, jj, atuin, starship |
| [Architecture](design/architecture.md) | §4.1, §4.7, §4.7.1 | Why there is a host, why front-ends hold no logic, and why the process boundary can wait until day two |
| [Storage, time, and the data model](design/storage.md) | §4.2–§4.5, §4.9, §5 | SQLite, JSONL export, the operation journal, intervals instead of counters, how timestamps are stored |
| [The protocol](design/protocol.md) | §4.6, §4.8, §6, §6.1 | JSON-RPC 2.0 over a Unix socket, typed subscriptions, the method surface, and where the host listens |
| [The front-ends](design/frontends.md) | §7 | Terminal UI, CLI, `yy prompt`, and the Topcoat web application |
| [Smaller decisions](design/smaller-decisions.md) | §4.10 | ULIDs, tags, one workspace, Topcoat, Jujutsu, the licence |
| [Repository and open-source practice](design/repository.md) | §8 | Layout, dependencies, documentation, version control, releases, CI, formatting |
| [Roadmap and risks](design/roadmap.md) | §9, §12 | What ships when, and what could go wrong |
| [Rules that must not be broken](design/rules.md) | §10 | The twelve invariants everything else is checked against |
| [Glossary](design/glossary.md) | §11 | Host, journal, projection, resync, colocated workspace |
| [How it is verified](design/verification.md) | §13 | The tests that turn the rules above into something mechanical |

Start with [Purpose](design/purpose.md) and
[Rules](design/rules.md). Between them they carry most of the design; the rest
is the reasoning that produced them.

## What changed, and when

This document supersedes earlier drafts. The changes worth knowing about, so
that a reader who saw an older version is not confused by a decision that looks
reversed:

**The browser front-end is Rust, not TypeScript.** It was Solid, bundled with
Bun. It is now [Topcoat](https://github.com/tokio-rs/topcoat), server-rendered
inside the host process. This removes the JavaScript toolchain entirely, and its
knock-on effects reach further than the front-end
([§4.10](design/smaller-decisions.md#410-smaller-decisions),
[§7](design/frontends.md#browser-and-phone--topcoat-served-by-the-host)).

**The protocol is JSON-RPC 2.0, not gRPC.** gRPC was chosen so that Rust and
TypeScript could agree on one message definition. With no TypeScript, that
reason is gone, and the remaining reason (server push) does not require it. What
this costs — the `buf` breaking-change check, which was a by-product of having
an IDL — is now an explicit deliverable rather than something inherited
([§4.6](design/protocol.md#46-why-json-rpc-20-between-the-host-and-the-front-ends)).

**`yy-proto` is now `yy-types`, and `xtask` is gone.** The old crate was named
after protobuf and holds no protobuf; the old `xtask` existed only to run the
code generator. Rule 8's enforcement is a test in `yy-types` rather than a
binary plus a CI job
([§8.2](design/repository.md#82-layout),
[§13](design/verification.md#13-how-it-is-verified)).

**`export` and `import` no longer go through the socket.** The host writes and
reads the file itself. This resolves a contradiction between §4.7.1 and rule 2
that predates the protocol change
([§6](design/protocol.md#6-the-protocol-surface)).

**The repository is developed with Jujutsu, in a colocated workspace.** Git
remains the contract, and rule 11 exists to keep it that way
([§8.5](design/repository.md#version-control-jujutsu-and-git)). If you have only
used Git, [`docs/jj/`](jj/index.md) is written for you.

**The licence is MIT, held by "the yy authors".** A copyright notice does not
have to name a person
([§4.10](design/smaller-decisions.md#410-smaller-decisions)).

**The minimum supported Rust version is 1.97, not 1.94, and the toolchain is
pinned to 1.97.1 exactly.** `yy` is an application, not a library, so a
conservative MSRV costs features and buys nothing. 1.97 is what cargo's
`build.warnings` needs, which replaces `RUSTFLAGS: -D warnings` without
invalidating the build cache
([§8.1](design/repository.md#81-starting-point),
[§8.7](design/repository.md#87-warnings-are-errors-without-the-usual-cost)).

**`main` is protected.** Direct pushes are rejected for everyone; every change
arrives by a squash-merged pull request that passed CI. This is
[rule 12](design/rules.md#10-rules-that-must-not-be-broken).
