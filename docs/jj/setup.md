# Setting up Jujutsu

Part of the [jj guide](index.md). These instructions target `jj` 0.43 or newer.

## 1. Install

`jj` is not packaged for Fedora, so install it from crates.io. You already have
a Rust toolchain because you build `yy`:

```sh
cargo install --locked --bin jj jj-cli
```

That puts `jj` in `~/.cargo/bin`. Check it:

```sh
jj --version
```

Other options, if you prefer them:

```sh
# prebuilt binary from the latest GitHub release
cargo binstall --strategies crate-meta-data jj-cli

# the development version, if you want to follow main
cargo install --git https://github.com/jj-vcs/jj.git --locked --bin jj jj-cli

# Homebrew, which you already have on this machine
brew install jj
```

Then install shell completion for fish, which is worth doing immediately because
`jj`'s subcommands are not guessable:

```sh
jj util completion fish > ~/.config/fish/completions/jj.fish
```

## 2. Configure

`jj` refuses to create commits until it knows who you are:

```sh
jj config set --user user.name "Your Name"
jj config set --user user.email "you@example.com"
```

Two more that make the first week much easier:

```sh
# use your usual editor for commit messages
jj config set --user ui.editor "nvim"

# show a diff summary in `jj status` and after commands
jj config set --user ui.diff-formatter :git
```

`jj config edit --user` opens the whole file if you would rather see it. The
full list of options is in `jj config list --include-defaults` and in the
[configuration reference](https://docs.jj-vcs.dev/latest/config/).

### Optional: a shorter log

The default `jj log` shows every visible head, which is a lot once you have a
few changes in flight. This alias narrows it to your own work plus the trunk:

```sh
jj config set --user 'aliases.l' '["log", "-r", "trunk() | (trunk()..mine())"]'
```

## 3. Get the repository

You have two situations.

### You already have a Git clone

Turn it into a colocated workspace in place. Nothing is moved or rewritten; a
`.jj/` directory appears next to the existing `.git/`:

```sh
cd yy
jj git init --colocate
```

### You are starting fresh

`jj git clone` colocates by default in 0.43 (`git.colocate` defaults to `true`),
so this is all you need:

```sh
jj git clone https://github.com/<owner>/yy
cd yy
```

Either way, verify:

```sh
ls -d .jj .git    # both must exist
jj log            # should show the repository history
git status        # should work, and say "HEAD detached"
```

**"HEAD detached" is correct and permanent.** `jj` has no concept of a currently
checked-out branch, so it leaves Git in a detached HEAD state. Do not "fix" it.
Git commands that only read (`git log`, `git show`, `git diff`) work normally.

## 4. Make `gh` work

The GitHub CLI works out of the box in a colocated workspace, because `.git/` is
right there. Verify:

```sh
gh pr list
```

If you ever move to a non-colocated workspace, `gh` cannot find the Git
directory and needs `GIT_DIR` set. You have `direnv` installed, so the clean fix
is an `.envrc`:

```sh
echo 'export GIT_DIR=$(jj git root)' >> .envrc
direnv allow
```

For the colocated setup this project uses, you do not need it.

## 5. Parallel work: `jj workspace`, not `git worktree`

If you want two checkouts at once — two features in flight, or several agents
working in parallel — `jj` has the equivalent of a Git worktree, and you should
use it rather than mixing the two:

```sh
jj workspace add ../yy-review      # a second working copy over the same repo
jj workspace list
jj workspace forget yy-review      # when you are done with it
```

**Do not run `git worktree add` in this repository.** The two are not
interchangeable, and the failure mode is confusing:

| | The directory contains | What works inside it |
|---|---|---|
| `git worktree add` | `.git`, no `.jj` | Git only. Every `jj` command fails |
| `jj workspace add` | `.jj`, no `.git` | `jj` only. `gh` and `git` need `GIT_DIR` |

Only the top-level clone is colocated. `jj workspace add` has no `--colocate`
option, so an added workspace is `jj`-only. If you need `gh` inside one:

```sh
export GIT_DIR=$(jj git root)
```

The same `.envrc` trick from step 4 works if you want that automatically.

One thing worth knowing: workspaces share one repository, so a change you make
in one is immediately visible from the other with `jj log`. They isolate the
*working copy*, not the history. If another workspace's working copy has moved
underneath you, `jj workspace update-stale` reconciles it.

## 6. Know the two limitations before they bite you

**Hooks do not run.** `jj` does not execute Git hooks
([jj-vcs/jj#405](https://github.com/jj-vcs/jj/issues/405)). If you are used to a
`pre-commit` hook catching formatting, it will not fire. This is why `yy` puts
every check in `just check` and in CI, and why nothing in the project may depend
on a hook. Run this yourself before pushing:

```sh
just check
```

**`.gitignore` is read, `.gitattributes` is not.** Ignore patterns work
normally. Attributes, including line-ending normalisation, are ignored
([jj-vcs/jj#53](https://github.com/jj-vcs/jj/issues/53)). `yy` does not use
them; [§4.3](../design/storage.md#43-why-jsonl-for-export) requires `\n` and no
BOM, and the round-trip test enforces it rather than the VCS.

One consequence worth knowing: because almost every `jj` command snapshots the
working copy, a file that was already tracked and *then* added to `.gitignore`
stays tracked. `jj file untrack <path>` removes it. Set up ignore patterns
before creating the files.

---

Next: the [tutorial](tutorial.md).
