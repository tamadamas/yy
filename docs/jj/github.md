# Jujutsu and GitHub

Part of the [jj guide](index.md). Everything here has a Git equivalent, shown
beside it, because the repository is colocated and both work.

## The one rule

**`main` is never written to directly.** Not with `git push`, not with `jj git
push`, not by anyone, including the maintainer. Every change reaches `main` by
being merged from a pull request, and every pull request must pass CI first.
GitHub branch protection enforces this server-side, so a mistake is rejected
rather than absorbed.

That means three things you will never do in this repository:

```sh
jj bookmark move main --to @      # no
jj git push --bookmark main       # no
git push origin main              # no, and GitHub will refuse it
```

Your local `main` is a read-only mirror of the remote. It moves when you run
`jj git fetch`, and at no other time.

**`jj` already enforces this locally.** Its default `immutable_heads()` revset
is `trunk() | tags() | untracked_remote_bookmarks()`, so `jj` refuses to rewrite
any commit on `main` or on a tag and tells you so. You get the protection rule
on your own machine, before you push, without configuring anything.

Because of this, the guides below rebase onto `trunk()` rather than a local
bookmark. `trunk()` resolves to `main@origin`, which is the actual authority.

## Sync with the remote

There is no `jj pull`. Fetching and rebasing are separate, deliberately:

```sh
jj git fetch                 # git fetch --all
jj rebase -d trunk()         # move your work onto the updated main
```

`jj git fetch` updates `main@origin`, and your tracked local `main` follows.
Your own changes are untouched until you rebase them.

If you have several independent stacks in flight, rebase each one:

```sh
jj rebase -b <change> -d trunk()
```

To see what you have that the remote does not:

```sh
jj log -r 'trunk()..@'                     # your unpushed ancestors
jj log -r 'mine() & ~::remote_bookmarks()' # everything of yours not pushed
```

## Push a change and open a pull request

Do the work as a stack of changes ([tutorial](tutorial.md)), then name it and
push.

```sh
# 1. make sure your work is described and sitting under an empty @
jj log

# 2. name the tip of your work -- any name except main
jj bookmark create fix-export-path -r @-

# 3. push it, creating the remote bookmark and tracking it
jj git push --allow-new

# 4. open the pull request against main
gh pr create --fill --base main
```

`--allow-new` is required the first time a bookmark is pushed; afterwards plain
`jj git push` is enough. `gh` works because the workspace is colocated.

**Git equivalent:** `git switch -c fix-export-path && git push -u origin
fix-export-path && gh pr create --base main`.

### Without inventing a name

If the branch name does not matter, let `jj` generate one:

```sh
jj git push -c @-
```

This creates and pushes a bookmark called `push-<change-id>`. Because pull
requests are squash-merged, the branch name never appears in the history, so a
generated name costs nothing. The pull request *title* is what matters: it
becomes the commit on `main`, and it must be a
[Conventional Commit](https://www.conventionalcommits.org/). See
[CONTRIBUTING](../../CONTRIBUTING.md).

## Wait for CI

Every pull request runs the checks in
[`.github/workflows/ci.yml`](../../.github/workflows/ci.yml): formatting, clippy
with warnings denied, tests, the MSRV build, and `cargo deny`. They are required
checks, so the merge button stays disabled until they pass.

Save yourself the round trip by running the same thing locally first:

```sh
just check
```

Remember that **`jj` does not run Git hooks**, so nothing will remind you. This
is exactly why the checks live in `just check` and CI rather than in a
`pre-commit` hook
([§8.5](../design/repository.md#version-control-jujutsu-and-git)).

Watch a run without leaving the terminal:

```sh
gh pr checks --watch
```

## Respond to review comments

Pull requests are squash-merged, so your branch history is not preserved and you
do not need to keep it tidy. Both approaches below are fine; the second produces
a cleaner diff for the reviewer to re-read.

### Add a change on top

```sh
jj new fix-export-path
$EDITOR ...
jj describe -m "address review comments"
jj bookmark move fix-export-path --to @
jj git push
```

### Fix the original change in place

This is where `jj` is noticeably better than Git:

```sh
jj edit fix-export-path      # or any change in the stack
$EDITOR ...
jj git push                  # force-push happens automatically and safely
```

No `git rebase -i`, no `--force-with-lease`. `jj` knows the remote bookmark
moved because it rewrote the change itself, and refuses to push if the remote
moved underneath you for any other reason.

If the comments are scattered across several commits in a stack, make all the
edits in `@` and then:

```sh
jj absorb
jj git push
```

Each fix lands in the commit it belongs to.

## Main moved while you were in review

```sh
jj git fetch
jj rebase -b fix-export-path -d trunk()
jj git push
```

If that produces a conflict, the rebase still succeeds and the conflict is
recorded in the commit. Resolve it whenever you like:

```sh
jj st
jj resolve
jj git push
```

You never end up in a "you are currently rebasing" state.

## After your pull request is merged

GitHub squash-merged it, so the commit on `main` is a *different* commit from
yours. Clean up:

```sh
jj git fetch                      # main@origin now contains your work
jj bookmark delete fix-export-path
jj git push --deleted             # remove the remote branch too
jj abandon <your old change>      # optional; it is superseded
jj new trunk()                    # start the next piece of work
```

`jj log` will show your original change as an unrelated head until you abandon
it. That is expected with squash-merging and is not a sign anything went wrong.

## Work with someone else's pull request

`jj git fetch` does not create local bookmarks for other people's branches, so
address them by remote:

```sh
jj git fetch
jj new their-branch@origin     # build on it
jj log -r their-branch@origin  # just look
```

**Git equivalent:** `gh pr checkout <number>` also works here, but it leaves
Git's HEAD attached and the next `jj` command detaches it again. Prefer the `jj`
form.

## Releases

Releases go through a pull request like everything else; `main` is not special-cased
for them. The full procedure is in [RELEASING.md](../../RELEASING.md). In short:

1. Open a release pull request that bumps versions and updates `CHANGELOG.md`.
2. CI passes, it is reviewed, it is merged.
3. **Then** tag the commit that landed on `main`:

```sh
jj git fetch
jj tag set v0.2.0 -r trunk()
jj git push --tag v0.2.0
```

Note the command is `jj tag set`, not `create`.

**One limitation:** `jj` creates lightweight tags but **not annotated tags**. If
a release must carry an annotated, signed tag, do that one step with Git in the
colocated repository:

```sh
git tag -a -s v0.2.0 -m "yy 0.2.0" <commit-on-main>
git push origin v0.2.0
```

Pushing a *tag* is allowed; pushing the `main` *branch* is not. Signing commits
works natively (`jj sign`, or automatically via `signing.behavior`), so only
annotated tags need Git.

## A note on running Git commands

In a colocated workspace you may run any Git command. Two habits keep it
uneventful:

- **Read freely, write with `jj`.** `git log`, `git show`, `git diff`, and `gh`
  are all fine. Prefer `jj` for anything that changes the repository.
- **If you do run a mutating Git command,** the next `jj` command imports the
  result, `jj op log` shows it as an import operation, and `jj undo` can revert
  it like anything else.

## Revsets worth remembering

```sh
# everything of yours not yet on any remote
jj log -r 'mine() & ~::remote_bookmarks()'

# what is on your bookmark but not on main
jj log -r 'trunk()..fix-export-path'

# all local bookmarks not merged into main
jj log -r 'bookmarks() & ~::trunk()'

# what did I do an hour ago
jj op log
```

The full syntax is in the [revset reference](https://docs.jj-vcs.dev/latest/revsets/).
