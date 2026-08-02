---
name: jj
description: Always use this skill before running any Jujutsu or Git command in the yy repository
---

# Version Control

The repository is a **colocated Jujutsu/Git workspace**: `.jj/` and `.git/` over
one working copy.

## ALWAYS USE `jj`. NEVER USE `git` TO COMMIT.

Git is reached only through `jj git ...` (`jj git fetch`, `jj git push`). These
are forbidden:

```
git add        git commit     git rebase     git merge
git switch     git checkout   git worktree   git push       git reset
```

Read-only inspection is fine: `git log`, `git show`, `git diff`, `gh`.

There is deliberately no `git` skill. Human contributors may use Git and the
project guarantees they can (rule 11); an agent has no reason to. `jj` does the
same jobs, its operation log makes a mistake one `jj undo` away rather than an
investigation, and mixing the two produces states that are hard to reason about
because every `jj` command imports and exports Git state underneath you.

If you are reaching for a mutating Git command, stop and find the `jj`
equivalent in [`docs/jj/tutorial.md`](../../../docs/jj/tutorial.md), which has a
full translation table.

The single exception is annotated, signed tags, which `jj` cannot create. That
is a maintainer step in [`RELEASING.md`](../../../RELEASING.md), not an agent
one.

`git status` correctly reports a detached HEAD. That is permanent and must not
be "fixed".

## The guides are in the repository

This skill carries the rules. The commands and the reasoning live in
[`docs/jj/`](../../../docs/jj/index.md), written for someone who knows Git and
has never used `jj`. **Read the relevant page rather than guessing at a
command:**

| Page | Covers |
|---|---|
| [`docs/jj/index.md`](../../../docs/jj/index.md) | Why the repository is set up this way, and the two limitations |
| [`docs/jj/setup.md`](../../../docs/jj/setup.md) | Installing and configuring `jj`, colocating a clone |
| [`docs/jj/tutorial.md`](../../../docs/jj/tutorial.md) | The mental model, the daily loop, and a full Git-to-`jj` translation table |
| [`docs/jj/github.md`](../../../docs/jj/github.md) | Fetch, push, pull requests, review rounds, tags, releases |

If you are about to run a `jj` command you have not run before in this
repository, open [`tutorial.md`](../../../docs/jj/tutorial.md) or
[`github.md`](../../../docs/jj/github.md) first. Both give the Git equivalent
beside each command.

## Rules that override anything in the guides

**`main` is written only by merging a pull request that passed CI** (rule 12).
Branch protection rejects the following server-side; never attempt them:

```
jj bookmark move main --to @      # no
jj git push --bookmark main       # no
git push origin main              # no
git push --force                  # no, on any shared branch
```

`jj` also refuses locally: its default `immutable_heads()` is
`trunk() | tags() | untracked_remote_bookmarks()`, so it will not rewrite a
commit on `main` or on a tag.

**Rebase onto `trunk()`, not onto a local bookmark.** `trunk()` resolves to
`main@origin`, which is the actual authority.

**Git hooks do not run.** `jj` does not execute them, so nothing checks
formatting for you. Run the [`check`](../check/SKILL.md) skill before pushing.
This is also why no change may ever depend on a hook existing.

**Never `git switch` or `git checkout` to attach HEAD.** The next `jj` command
detaches it again, and detached is the correct state here.

**Never `git worktree add`.** If you need an isolated checkout -- working on two
things at once, or several agents in parallel -- use `jj workspace add`:

```
jj workspace add ../yy-<topic>       # new working copy, same repo
jj workspace list
jj workspace forget <name>           # when done
```

A `git worktree` directory contains `.git` but no `.jj`, so no `jj` command
works inside it, which is exactly the wrong way round for an agent. A `jj`
workspace is the reverse: it has `.jj` but no `.git`, because `jj workspace add`
has no `--colocate`. If you need `gh` inside one, export
`GIT_DIR=$(jj git root)` first. See
[`docs/jj/setup.md`](../../../docs/jj/setup.md#parallel-work-jj-workspace-not-git-worktree).

## When something goes wrong

```
jj op log            # every operation, newest first
jj undo              # revert the last one
jj op restore <id>   # restore the whole repo to that moment
```

Reach for `jj undo` before investigating. Nothing here is destructive, and a
conflicted rebase is a recorded conflict rather than a halted operation you have
to `--abort` out of. See
[`tutorial.md`](../../../docs/jj/tutorial.md#step-6-undo-anything).
