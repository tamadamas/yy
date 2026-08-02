# Releasing yy

For maintainers. Every step obeys the same rules as any other change: `main` is
written only by merging a pull request that passed CI
([rule 12](docs/design/rules.md#10-rules-that-must-not-be-broken)), and a
release is not an exception to that.

## What versions independently

Three things move on their own schedules, so "the version" is ambiguous unless
you say which:

- **The binaries** (`yy`), tagged `vX.Y.Z`. This is what people install.
- **The protocol**, versioned by the frozen files in `schema/`. A new protocol
  version freezes `schema/current.json` as `schema/vN.json`; the schema test
  then asserts every future change stays an additive extension of it
  ([rule 8](docs/design/rules.md#10-rules-that-must-not-be-broken)).
- **The JSONL format**, versioned by the `type` field on each record. New record
  types are additive by construction, which is why the field exists.

Most releases move only the first.

## Preparing

### 1. Check the dependency situation

Topcoat is tracked from `main` rather than a release
([§8.3](docs/design/repository.md#83-dependencies)), so the lockfile is the pin
and a release freezes whatever it currently points at.

```sh
gh run list --workflow=dependencies.yml --limit 5
```

If the scheduled build is red, fix that before releasing. Shipping a lockfile
that is known to be behind a broken upstream is how you end up unable to
reproduce the build later.

### 2. Open the release pull request

It contains exactly three kinds of change and nothing else:

- version bumps in `Cargo.toml` and the resulting `Cargo.lock`,
- the `CHANGELOG.md` entry,
- if the protocol changed, `schema/current.json` copied to `schema/vN.json`.

```sh
# with jj
jj new trunk()
$EDITOR Cargo.toml CHANGELOG.md
cargo check                       # refresh Cargo.lock
jj describe -m "chore(release): v0.2.0"
jj bookmark create release-v0.2.0 -r @
jj git push --allow-new

# with git
git switch -c release-v0.2.0
$EDITOR Cargo.toml CHANGELOG.md
cargo check
git commit -am "chore(release): v0.2.0"
git push -u origin release-v0.2.0
```

```sh
gh pr create --base main --title "chore(release): v0.2.0" --fill
```

### 3. The changelog

`CHANGELOG.md` follows [Keep a Changelog](https://keepachangelog.com/), and its
content is derived from the pull request titles since the last tag rather than
written from memory. Because titles are Conventional Commits, this is
mechanical:

```sh
git log --pretty="%s" v0.1.0..main | grep -E '^(feat|fix)'
```

Only `feat` and `fix` are user-facing. Everything else stays out of the
changelog; the commit history is where it lives.

Version bumps follow from the same types: `feat` is a minor bump, `fix` a patch,
and a `!` or a `BREAKING CHANGE:` footer a major one. While the project is
pre-1.0, a breaking change is a minor bump.

## Releasing

### 4. Merge

CI must be green. Squash-merge the release pull request like any other.

### 5. Tag the commit that landed

Only now, and only on the merged commit. Pushing a *tag* is allowed; pushing the
`main` *branch* is not.

```sh
# with jj -- note the command is `tag set`, not `tag create`
jj git fetch
jj tag set v0.2.0 -r trunk()
jj git push --tag v0.2.0

# with git
git fetch origin
git tag -a -s v0.2.0 origin/main -m "yy 0.2.0"
git push origin v0.2.0
```

**If the tag must be annotated or signed, use Git.** `jj` creates lightweight
tags only, and cannot create annotated ones. This is the one place in the
project where the Git command is not optional, and it is safe because the
workspace is colocated. Signing *commits* works natively in `jj`
(`jj sign`, or `signing.behavior` in the config); only annotated tags need Git.

### 6. Publish

The tag triggers the release workflow, which builds the binaries, attaches
provenance attestations, and creates the GitHub release from the changelog
entry.

```sh
gh run watch
gh release view v0.2.0
```

### 7. Verify the promise, not just the build

Before announcing, confirm the thing the project actually claims. Install the
released binary and run the round trip against real data:

```sh
yy export /tmp/before.jsonl
YY_DATA=$(mktemp -d) yy import /tmp/before.jsonl
YY_DATA=$(mktemp -d) yy export /tmp/after.jsonl
diff /tmp/before.jsonl /tmp/after.jsonl && echo "rule 5 holds"
```

The test suite checks this on synthetic data every run. Doing it once per
release on *your* data is what makes it a guarantee rather than a green tick.

## What is not published to crates.io

`yy-host` depends on Topcoat by git URL, and crates.io rejects git dependencies,
so it cannot be published while that holds
([§8.3](docs/design/repository.md#83-dependencies)). This is fine: it is a
binary. If anything is ever published as a library it is `yy-types`, `yy-core`
and `yy-store`, none of which depend on Topcoat.

Releases therefore ship **binaries**, and the installation instructions say
`cargo install --git` or "download the binary", not `cargo install yy`.
