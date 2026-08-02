# Jujutsu, step by step

Part of the [jj guide](index.md). This assumes you know Git and have finished
[Setup](setup.md). Work through it in the `yy` repository; nothing here touches
the remote, and [step 6](#step-6-undo-anything) undoes anything you regret.

## Step 0: three ideas that are not Git

Read this once. Everything afterwards follows from it.

**1. The working copy is a commit.** Not "changes that will become a commit" —
an actual commit, called `@`. When you edit a file, `jj` folds the edit into `@`
the next time you run any command. There is no staging area, no `git add`, and
no distinction between modified, staged, and committed. "Committing" in `jj`
means giving `@` a description and starting a *new* empty `@` on top.

**2. Changes have two identities.** `jj log` shows a **change ID** on the left
and a **commit ID** on the right:

```
@  kntqzsrp  you@example.com  2026-08-01 14:22:31  8f4c1a2e
│  (no description set)
○  wqnwkozp  you@example.com  2026-08-01 14:09:02  main  3b7e9d01
│  feat(core): compute elapsed time from endpoints
```

The commit ID is Git's hash and changes whenever you rewrite the commit. The
**change ID is stable across rewrites** — amend a change ten times and its
change ID is the same. That is what makes "rebase this change" and "the commit I
was working on" say the same thing, and it is why `jj` needs no `ORIG_HEAD` or
reflog-archaeology. Use change IDs when you refer to work; you only need the
first few characters.

**3. Nothing is destructive.** Every command appends to an operation log. `jj
undo` reverts the last operation, whatever it was. There is no command in this
tutorial you cannot take back.

Two pieces of notation you will use constantly: `@` is the working-copy commit,
and a trailing `-` means "parent". So `@-` is the parent of the working copy,
and `@--` its grandparent.

## Step 1: look around

```sh
jj st      # what has changed in @, and what @ is
jj log     # the graph
jj diff    # the changes in @
```

`jj st` replaces `git status`, and there is no "staged" section because there is
no index. `jj log` replaces `git log --graph --oneline`; by default it hides
ancestors that are already on the trunk, which is why it is short.

To see a specific change, use `jj show`:

```sh
jj show @-           # the parent of the working copy
jj show wqnw         # by change ID prefix
```

## Step 2: make a change

Edit a file. Any file.

```sh
$EDITOR README.md
jj st
```

Your edit is already in `@`. You did not add it, and you cannot forget to. This
is the single biggest day-one difference: **there is no command between editing
a file and it being part of your change.**

## Step 3: describe it

`@` currently has no description. Give it one:

```sh
jj describe -m "docs: fix the install command"
```

`jj describe` sets the message of the working-copy commit. It is the closest
thing to `git commit --amend -m`, except that it is not an amend — the commit
was always there, it just had no message.

Check `jj log`: `@` now carries your description, and still contains your edit.

## Step 4: start the next change

You are done with that piece of work and want to begin the next one:

```sh
jj new
```

`@` is now a new, empty commit on top of the one you just described. Your
previous work is at `@-`.

There is a shortcut for "describe the current change and start a new one", which
is what `git commit` does:

```sh
jj commit -m "docs: fix the install command"
```

That is exactly `jj describe -m "..."` followed by `jj new`. Use whichever fits
your rhythm. Describing as you go (`jj describe`) and only running `jj new` when
you genuinely switch tasks tends to feel better than Git's habit, because there
is no penalty for leaving `@` undescribed while you think.

## Step 5: fix something you already "committed"

This is where `jj` earns its keep. Three tools, in increasing order of cleverness.

### Go back and edit it directly

```sh
jj edit @-       # or: jj edit wqnw
```

`@` is now that older change. Edit files; the edits go into it. Every descendant
is rebased automatically, immediately, with no `--continue` and no stopping.
When you are done, move back to the tip:

```sh
jj new           # start fresh work on top again
```

There is no `git rebase -i`, no `edit` marker, no `git rebase --continue`.

### Push the current change down into its parent

You are working in `@`, and you realise this belongs in the previous change:

```sh
jj squash                    # move all of @'s changes into @-
jj squash -i                 # choose hunks interactively
jj squash --into wqnw        # into a specific change, not just the parent
```

This is `git commit --amend` when the target is the parent, and something Git
has no clean equivalent for otherwise.

### Let `jj` work out where each fix belongs

```sh
jj absorb
```

`jj absorb` takes the changes in `@` and distributes each one into whichever
ancestor commit last touched those lines. If you fixed a typo in the storage
layer and a comment in the CLI, both land in the right commits without you
naming either. Anything it cannot place unambiguously stays in `@`.

This is the command to reach for after a review round with scattered comments.

## Step 6: undo anything

```sh
jj op log        # every operation, newest first
jj undo          # revert the last operation
jj redo          # and put it back
```

`jj op log` shows what each command did, with a timestamp and the arguments. To
jump to a specific point:

```sh
jj op restore <operation-id>
```

This is not "revert the commit". It restores the whole repository — every
change, every bookmark — to how it looked at that moment. A rebase that went
wrong, an `abandon` of the wrong change, a bad conflict resolution: one command,
and it never fails.

`git reflog` plus `git reset --hard` is the nearest Git equivalent, and it only
covers branch tips.

## Step 7: split a change that grew too big

```sh
jj split
```

An editor opens; select the hunks that belong in the *first* of the two
resulting changes. What you leave behind stays in the second. To split a change
that is not `@`:

```sh
jj split -r wqnw
```

Use this when you notice a pull request is doing two things. `yy` prefers small,
focused pull requests, so this is a normal part of preparing one.

## Step 8: rebase, and conflicts that do not stop you

```sh
jj rebase -d main            # move @ and its descendants onto main
jj rebase -s wqnw -d main    # move that change and everything after it
jj rebase -b @ -d main       # move the whole "branch" containing @
```

If there is a conflict, **the rebase still completes**. The conflicted commit
now records the conflict, and `jj log` marks it. You can keep working, switch to
something else, or resolve it now:

```sh
jj st          # names the conflicted files
jj resolve     # open the merge tool
```

Resolve markers in the file directly if you prefer; `jj` picks the resolution up
on the next command. When every conflict is gone, nothing needs to be
"continued" — there is no in-progress operation to finish.

This is the behaviour that makes `jj rebase` safe to run casually. In Git you
think before rebasing; here the worst case is a commit with a conflict in it,
and `jj undo` removes even that.

## Step 9: bookmarks, which are Git branches

`jj` calls them **bookmarks**, and the one thing to internalise is: **a bookmark
does not follow you.** In Git, committing on `main` moves `main`. In `jj`, a
bookmark points at a change and stays there until you move it.

That sounds like a regression and is the point: most of the time you do not need
a name at all. You work on a stack of changes, and you only create a bookmark
when you are ready to push.

```sh
jj bookmark list                          # what exists
jj bookmark create fix-export -r @-       # name a change
jj bookmark move fix-export --to @-       # point it somewhere else
jj bookmark delete fix-export
```

The `-r @-` is not a typo. After `jj commit`, `@` is the new empty change and
your work is at `@-`. Bookmark the work, not the empty commit on top of it.

Pushing is covered in [GitHub](github.md).

## The translation table

| Git | Jujutsu |
|---|---|
| `git add` | nothing; edits are already in `@` |
| `git status` | `jj st` |
| `git log --graph --oneline` | `jj log` |
| `git show HEAD` | `jj show @-` |
| `git diff` | `jj diff` |
| `git diff --cached` | nothing; there is no index |
| `git commit -a -m X` | `jj commit -m X` |
| `git commit --amend` | `jj describe` (message), or just edit the files (content) |
| `git commit --fixup` + autosquash | `jj absorb` |
| `git switch -c foo` | `jj new`, then `jj bookmark create foo -r @-` when you push |
| `git checkout <commit>` | `jj edit <change>` to modify it, `jj new <change>` to build on it |
| `git rebase main` | `jj rebase -d main` |
| `git rebase -i` | `jj squash`, `jj split`, `jj edit`, `jj absorb` |
| `git rebase --continue` | nothing; rebases never stop |
| `git stash` | nothing; `jj new <trunk>` and come back with `jj edit` |
| `git reset --hard` | `jj restore`, or `jj abandon` for a whole change |
| `git cherry-pick` | `jj duplicate -d <dest>` |
| `git revert` | `jj revert` |
| `git reflog` + `git reset` | `jj op log` + `jj undo` / `jj op restore` |
| `git fetch` | `jj git fetch` |
| `git push` | `jj git push` |
| `git pull` | `jj git fetch` then `jj rebase -d main` |

## What to practise first

In order, because each one removes a Git habit:

1. Stop typing anything resembling `git add`. Edit, then `jj st`.
2. Use `jj edit` instead of an interactive rebase, once.
3. Break something on purpose and fix it with `jj undo`. Do this early, while
   the stakes are zero — it is the command that makes the rest feel safe.
4. Use `jj absorb` after a review round.

---

Next: [GitHub](github.md) for pushing, pull requests, and releases.
