---
name: check
description: Always use this skill to verify a change locally before committing or opening a pull request in the yy repository
---

# Verifying a Change

Run this by default:

```
just check
```

It is the format check, clippy, the tests and rustdoc, in that order, and it is
what CI runs — `ci.yml` installs `just` and calls this same recipe, so there is
no second list to keep in sync. The recipes are plain cargo commands; read the
`justfile` if you want to run one by hand.

Formatting needs the nightly rustfmt pinned on the `nightly :=` line of the
`justfile`, installed with `just fmt-toolchain`. That line is the only copy of
the version in the repository. Never `cargo fmt`: stable ignores the
nightly-only options in `rustfmt.toml` in silence and produces a file CI
rejects.

## What is deliberately not here

**No `-D warnings`.** `.cargo/config.toml` sets `build.warnings = "deny"`, so
warnings already fail the build. Adding `RUSTFLAGS` back would invalidate the
build cache and make every job recompile the workspace.

**No toolchain version.** `rust-toolchain.toml` pins it; rustup applies it.

**No MSRV job.** Everyone is on the pinned version, so there is nothing to
discover.

## After a protocol change

Touching the types in `yy-types` fails the schema test until you regenerate the
snapshot:

```
just schema        # UPDATE_SCHEMA=1 cargo test -p yy-types schema
```

Commit the `schema/current.json` diff. It is meant to be visible in review: rule
8 says protocol changes are additive, and that diff is the evidence.

## Only on request

```
cargo deny check                     # licences, advisories, duplicates
mdbook build docs/book               # the guide and specification
```

## Do not rely on hooks

There is no pre-commit hook and there never will be one: the maintainer uses
Jujutsu, which does not execute Git hooks, so a hook would protect some people
and not others. Run the commands.
