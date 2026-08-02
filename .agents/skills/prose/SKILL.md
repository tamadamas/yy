---
name: prose
description: Always use this skill before writing or editing markdown documentation in the yy repository
---

# Prose

## Placement

- **`docs/design/`** -- the design of record. Decisions and their costs, not
  instructions. See the [`design`](../design/SKILL.md) skill before editing.
- **`docs/book/`** -- the user guide and the specification, published to GitHub
  Pages with mdBook. This is where anything a *user* reads belongs.
- **`docs/jj/`** -- the Jujutsu guide, for contributors, optional by design.
- **Doc comments in `yy-types`** -- the source of the published protocol
  reference. Treat them as user-facing text.

Do **not** embed prose guides into rustdoc with `#![doc = include_str!(...)]`.
Topcoat does this correctly for itself because it is a library whose users read
docs.rs; `yy` is an application whose users run a CLI and never open `cargo
doc`. The protocol specification in particular must stay outside rustdoc, since
its audience is explicitly not assumed to be Rust programmers (§4.6).

## Structure

Start with a plain summary of what the document is about, then basic usage,
then advanced topics. When something is already explained in detail elsewhere,
say so briefly and link rather than restating it -- two copies of an explanation
means one of them is wrong within a month.

## Voice

* Simple, concise language. No fancy words.
* State the cost of a thing next to the thing. A guide that only lists benefits
  is not trusted and should not be.
* Describe the current state only. Never write "this used to be A but is now B"
  outside the "What changed" section of `docs/DESIGN.md`, which exists precisely
  so the rest of the documents do not have to.
* Avoid exhaustive lists of implementations or uses that will go stale.
* Ordinary typography, including em dashes, is fine here. This is a deliberate
  difference from Topcoat's ASCII-only rule; match the file you are editing.

## Cross-references

Design section numbers (§4.6, §8.3) are stable across the file split. Link to
the file and anchor, and keep the section number in the text so a reference
survives another reorganisation.

## After editing

If you changed a claim that appears in more than one place -- a rule, a required
tool, a command -- grep for it. `README.md`, `CONTRIBUTING.md`, `AGENTS.md`, the
skills, and `justfile` all restate parts of the design and drift silently.
