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

**One bookmark, one subject.** A bookmark carries a single feature, bug fix,
documentation fix, or CI fix. The moment a change is about something other than
what the open pull request is about, it belongs on a new bookmark and a new
pull request — not appended to the branch because the working copy happened to
be there:

```sh
jj new 'trunk()'                     # start from main, not from the other branch
jj bookmark create fix-ci-wildcard -r @
jj git push --bookmark fix-ci-wildcard
gh pr create --fill --base main
```

**Quote every revset.** The shell here is `fish`, where bare parentheses are
command substitution: `jj new trunk()` does not start a change on `main`, and it
can fail without saying much. This applies to every revset argument, and
`trunk()` is the one short enough to look safe.

**There is no `--allow-new`.** `--bookmark` creates and tracks the remote
bookmark by itself; passing the flag other projects document fails here with a
suggestion to use `--all`, which pushes everything.

If the work is already sitting in the wrong change when you notice, move the
files rather than starting over — this does not touch what was pushed:

```sh
jj new 'trunk()' -m "<conventional commit title>"
jj squash --from <change-id> --into @ <paths>...
```

A pull request title becomes the commit on `main`, and a Conventional Commit
title can only describe one subject honestly. Two subjects on one branch also
mean the reviewer cannot approve half of it. Fixing the design document that a
code change contradicts is *the same* subject and stays together
([the `design` skill](../design/SKILL.md) requires it); a `jj` rule and a
`deny.toml` fix are not.

Use [`jj workspace add`](../../../docs/jj/setup.md#5-parallel-work-jj-workspace-not-git-worktree)
when two subjects are genuinely in flight at once, or when subagents work in
parallel.

**Never rewrite a commit that has been pushed. Add a new one.** Every fix,
review response, and CI repair is its own commit with its own message naming
what it fixed. These are forbidden on anything already on a pull request:

```
jj squash                    # no
jj absorb                    # no
jj describe <pushed change>  # no
jj edit <pushed change>      # no, then pushing the rewrite
git commit --amend           # no, and git is forbidden anyway
```

`jj git push` force-pushes silently when the change was rewritten, which is
exactly the problem: the branch the reviewer read is gone and there is no record
of what the second attempt changed. A branch of small honest commits costs
nothing, because **the maintainer squashes at merge** and the pull request title
is what lands on `main`. Tidying history is their call, not an agent's.

`jj new @` (or `jj new <bookmark>`), describe, move the bookmark, push. If a
push is rejected as non-fast-forward, stop and say so — do not reach for
`--force`.

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
[`docs/jj/setup.md`](../../../docs/jj/setup.md#5-parallel-work-jj-workspace-not-git-worktree).

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
