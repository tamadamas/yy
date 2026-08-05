---
name: pr
description: Always use this skill before opening a pull request in the yy repository
---

# Opening Pull Requests

## Describe the diff, not the latest commit

A branch usually holds several changes: initial work, fixups, review responses,
rebases. The title and body describe the net change landing on `main`. Read the
full diff first:

```
jj diff -r 'trunk()..@'      # or: git diff main...HEAD
```

## Title

Same Conventional Commits format as a commit message (see the
[`commit`](../commit/SKILL.md) skill). `.github/workflows/semantic-pr.yml`
enforces the type and that the subject is lowercase with no trailing period.
Pull requests are squash-merged, so the title becomes the commit on `main` and
the changelog entry.

## Body

No template. Keep it short and high-signal:

- **Summary** -- what the change does and why, drawn from the diff.
- **Design** -- if it touches or contradicts `docs/design/`, name the section
  and say what changed. If it contradicts a rule in
  `docs/design/rules.md`, say so prominently; that is the thing a reviewer most
  needs to know and the thing least visible in a diff.
- **Testing** -- which checks you ran (see the [`check`](../check/SKILL.md)
  skill), and any new test and what it guarantees.
- **Schema** -- if `schema/current.json` changed, confirm the change is additive
  and say what was added. Rule 8 depends on someone actually looking.
- **Disclaimer** -- name the tool and the model, then say which parts of the
  change they produced. See below; "what it did" is the load-bearing half.

## The disclaimer

A pull request is rarely all human or all machine, so a line that only says
"written by AI" describes almost nothing:

```
Specification written by Claude Opus 5 via Claude Code; the implementation and
tests are the author's.

Implementation by the author. Tests and this description written by GPT-5.3 via
Codex.

Written by Claude Opus 5 via Claude Code, in full: code, tests, and this
description.
```

Two rules decide the cases:

- **Attribute what landed, not what assisted.** Text that ends up in the
  repository or in this description gets a line. A model used to search, read,
  or explain something does not, or the line appears under every pull request
  and stops being read. Writing only the commit message or only this body does
  count -- both end up in the history.
- **Several tools, several lines.** And prefer nothing to something inaccurate:
  a blanket "written by AI" over a change a human wrote teaches the reviewer to
  discount the disclaimers that are true.

The `Co-Authored-By` trailer on the commit says the same thing
machine-readably, but not *which part*, which is why the prose stays here and
is not a duplicate of it.

## Before opening

```
just check
```

Nothing runs it for you: Git hooks do not execute in this repository because the
maintainer uses Jujutsu.

## Base branch

Always `main`, always a pull request. Direct pushes to `main` are rejected by
branch protection for everyone (rule 12).

## VCS commands

Not here. The [`jj`](../jj/SKILL.md) skill holds the rules and points at
[`docs/jj/github.md`](../../../docs/jj/github.md), which covers pushing,
review rounds, and cleaning up after a squash-merge -- with the Git equivalent
beside each command.

## After opening it

```
gh pr checks <number> --watch --fail-fast
```

**Exit code 8 means checks are still pending, not that anything failed** -- 0 is
green and 1 is a real failure. Do not report a pull request as broken on a
non-zero exit without reading which code it was. `Analyze (rust)` and `Analyze
(actions)` are CodeQL from the repository settings rather than from a workflow
file, and they usually finish last.

No check is required to merge; the ruleset on `main` only requires that a change
arrives by pull request. Read the results rather than trusting the merge button
([`docs/jj/github.md`](../../../docs/jj/github.md)).
