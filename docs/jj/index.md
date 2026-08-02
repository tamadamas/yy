# Using Jujutsu with `yy`

**You do not need to read this.** `yy` is a normal Git repository on GitHub.
Clone it with `git`, branch, commit, push, open a pull request. Nothing in the
build, the tests, or CI knows or cares which tool you used. That is
[rule 11](../design/rules.md#10-rules-that-must-not-be-broken), and it is tested.

This directory exists because the maintainer uses
[Jujutsu](https://jj-vcs.dev) (`jj`), and because `jj` is worth learning if you
have only ever used Git. These guides assume Git experience and no `jj`
experience.

## The guides

| Guide | What it covers |
|---|---|
| [Setup](setup.md) | Installing `jj`, configuring it, and turning your clone into a colocated workspace |
| [Tutorial](tutorial.md) | The mental model, then the daily loop, one step at a time |
| [GitHub](github.md) | Fetch, push, pull requests, review rounds, and releases — with the Git equivalent beside each command |

## Why the repository is set up this way

The repository is **colocated**: `.jj/` and `.git/` sit side by side over one
working copy. Every `jj` command imports and exports Git state automatically, so
the Git repository is always current. A collaborator sees ordinary commits on
ordinary branches, pushed to ordinary GitHub.

This is what makes the choice free. It is not a compatibility layer you have to
think about; it is the same repository, described twice.

## Why `jj` at all

Three properties, in the order they will matter to you:

**Nothing you do is dangerous.** Every command is recorded in an operation log,
and `jj undo` reverts the last one — including a botched rebase, a bad merge
resolution, or an `abandon` you regret. In Git the equivalent is knowing that
`git reflog` exists and reading it under pressure. This is the same idea as
`yy`'s own [operation journal](../design/storage.md#44-why-record-every-change-and-why-that-means-no-confirmations),
and for the same reason: undo that always works is what lets you remove
confirmation prompts and hesitation.

**There is no staging area.** The working copy *is* a commit. You edit files;
`jj` snapshots them into the current change on every command. There is no
`git add`, no "did I stage that hunk", no difference between "committed",
"staged", and "in the working tree".

**Conflicts do not block you.** A conflicted merge or rebase produces a commit
that records the conflict, rather than a stopped operation and a dirty tree. You
can leave it, work elsewhere, and resolve it later. `jj rebase` never drops you
into a half-finished state you have to `--abort` out of.

The cost is real and worth stating: `jj` is younger than Git, its documentation
is thinner, and a few things Git does are missing — notably **hooks are not
run** and `.gitattributes` is ignored. Both are accounted for in
[§8.5](../design/repository.md#version-control-jujutsu-and-git): every check
lives in `just check` and CI, never in a hook.

## Getting help

- `jj help <command>` and `jj <command> -h` are good and fast.
- The [official documentation](https://docs.jj-vcs.dev/) is the reference; these
  guides are the `yy`-specific path through it.
- [`jj undo`](tutorial.md#step-6-undo-anything) is the answer to "I broke
  something". Reach for it before searching.
