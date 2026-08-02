# Rules that must not be broken

Part of the [design of record](../DESIGN.md).

## 10. Rules that must not be broken

1. **Time is computed, never accumulated.** Only start and end are stored.
   ([§4.5](storage.md#45-why-store-intervals-and-never-a-counter))
2. **Exactly one process holds a write connection at a time**, enforced by the
   host lock file. Once the host exists, no front-end opens the database at all.
   ([§4.7.1](architecture.md#471-and-why-the-process-boundary-arrives-on-day-two))
3. **Nothing is destroyed.** Every change appends to the journal; deletion is an
   entry, not a removal, and so is undo.
   ([§5.3](storage.md#53-undo-is-an-append-not-a-removal))
4. **Undo always works — therefore nothing asks for confirmation.**
   ([§4.4](storage.md#44-why-record-every-change-and-why-that-means-no-confirmations))
5. **Your data can always leave**, losslessly, as readable JSONL, verified by a
   round-trip test. ([§4.3](storage.md#43-why-jsonl-for-export))
6. **The rules live in one place** (`yy-core`), in one language.
   ([§4.7](architecture.md#47-why-the-front-ends-do-not-implement-any-logic))
7. **`yy-core` touches nothing external** — no files, no terminal, no network.
8. **Protocol changes are additive.** Optional fields, unknown data ignored,
   enforced by the schema test in `yy-types` against the frozen `schema/` files.
   This rule lost its off-the-shelf enforcement when the protocol stopped being
   an IDL, so the check is a deliverable, not a formality.
   ([§4.6](protocol.md#46-why-json-rpc-20-between-the-host-and-the-front-ends))
9. **`yy prompt` never blocks.** It reads one file and exits; no host means no
   file means no output. ([§7](frontends.md#yy-prompt--a-file-read-not-a-request))
10. **Replay is deterministic.** Rebuilding projections reads no clock and
    generates no identifiers.
    ([§5.3](storage.md#53-undo-is-an-append-not-a-removal))
11. **Git is never required to be replaced.** The repository is colocated, and
    no `jj`-only step may be required to build, test, or contribute.
    ([§8.5](repository.md#version-control-jujutsu-and-git))
12. **`main` is written only by merging a pull request that passed CI.** No
    direct pushes, no force pushes, from either tool and from anyone.
    ([§8.5](repository.md#version-control-jujutsu-and-git))
