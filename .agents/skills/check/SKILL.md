---
name: check
description: Always use this skill to verify a change locally before committing or opening a pull request in the yy repository
---

# Verifying a Change

Keep this file in sync with `.github/workflows/ci.yml` and the `justfile`.

Run these by default:

```
cargo +nightly fmt --all             # nightly is required; stable silently ignores rustfmt.toml
cargo clippy --workspace --all-targets --all-features --locked
cargo test --workspace --all-features --locked
cargo doc --workspace --all-features --no-deps --locked
```

`just check` runs all four.

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
