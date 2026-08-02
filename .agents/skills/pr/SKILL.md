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
- **Disclaimer** -- if you are an AI agent, state which model and what it did.

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
