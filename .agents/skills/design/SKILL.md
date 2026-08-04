---
name: design
description: Always use this skill when a change touches, depends on, or contradicts the design of record in the yy repository
---

# The Design of Record

[`docs/DESIGN.md`](../../../docs/DESIGN.md) and
[`docs/design/`](../../../docs/design) are the design *of record*, not notes.
They state what was decided, what each decision costs, and where a later
decision reversed an earlier one.

## Before writing code

Check the change against
[`docs/design/rules.md`](../../../docs/design/rules.md). Twelve invariants, and
several are violated by code that looks entirely normal. The
[`style`](../style/SKILL.md) skill lists the ones that bite most often.

Then read the section that governs the area you are touching. The index is in
[`AGENTS.md`](../../../AGENTS.md).

## If the change contradicts the design

Do not work around it silently, and do not implement it and mention the conflict
afterwards. Say so before writing the code, name the section, and state whether
you believe the design or the requirement should give way.

If the design should change, **update it in the same pull request as the code**.
A decision that exists only in a diff is one nobody can find in six months.

## Section numbers are stable

The document was split across files, but the numbering (§4.6, §8.3, ...) was
kept. A reference to a section number means the same thing it always did; only
the file changed. When adding a section, do not renumber existing ones.

## How to write a decision

Match the existing form. Every substantive decision has four parts:

1. **The question**, stated plainly.
2. **The realistic options**, including the ones rejected and why.
3. **The choice.**
4. **What it costs.** This part is not optional and is the reason the document
   is worth reading. A decision recorded without its cost reads as advocacy.

Where a cost is mitigated, say by what, and where it is simply accepted, say
that. See §4.6 for an example carrying an explicit "this made something weaker"
admission.

## Do not duplicate a file that exists

A design document states a decision, the options rejected, the choice, and its
cost. It does not carry a pasted copy of the file that implements it. A fenced
block is fine for something not yet built, or for a command; once the file
exists, the block goes and a path reference stays. The reason is not tidiness:
a pasted copy is a second source of truth, and it goes stale exactly because
nobody edits both places at once. `reorder_imports` is the case that happened
here: `rustfmt.toml` and §8.6 both filed it under "nightly-only", and it is a
stable option whose default is already the configured value — a line that said
nothing, called something it was not, in two files at once.

## Keep the derived documents true

Several files restate the design and drift silently:

- `README.md` -- the user-facing summary of the guarantees.
- `CONTRIBUTING.md` -- setup and workflow claims.
- `AGENTS.md` and these skills.
- `.github/workflows/ci.yml` and the `justfile` -- §13 lists what must be
  verified; if you add a check there, add it to §13, and the reverse.

If you change a rule, grep for it before you finish.
