# Every recipe here is a plain cargo command you can read and run by hand, and
# that is the promise this file keeps: contributing needs cargo, never `just`.
#
# CI does run it, though. This file used to say that nothing in CI needed it,
# and the reason that changed is that the alternative was a copy -- the workflow
# repeating these command lines and the pinned rustfmt version beside them, free
# to drift. `just check` is now literally what a pull request runs rather than
# something that resembles it. See docs/design/repository.md.

# The nightly rustfmt this project formats with, pinned by date. Formatting is
# the only thing here that is not the toolchain rust-toolchain.toml pins (§8.6),
# and a rolling `nightly` would reformat the workspace whenever rustfmt changed
# its mind.
#
# This line is the only place the version is written. CI reads it out of this
# file with sed rather than repeating it, and .vscode/settings.json reaches it
# by calling `fmt-stdin` below, so a bump is this one line. Keep the shape
# `nightly := "..."`: the workflow matches on it and fails if it stops matching.
nightly := "nightly-2026-08-01"

# List the recipes.
default:
    @just --list

# Everything CI runs on a pull request. Run this before you push.
check: fmt-check clippy test doc

# Format the workspace. Nightly is required: rustfmt.toml uses nightly-only
# options (§8.6). It is the only nightly in the project.
fmt:
    cargo +{{ nightly }} fmt --all

# Fail if anything is unformatted.
fmt-check:
    cargo +{{ nightly }} fmt --all --check

# Install the pinned nightly rustfmt. Needed once, and again after a bump.
fmt-toolchain:
    rustup toolchain install {{ nightly }} --profile minimal --component rustfmt

# This exists for editors -- .vscode/settings.json points rust-analyzer at it --
# so that the pinned nightly above is the only copy of that version anywhere in
# the repository.
#
# `--edition` must be explicit: with the source arriving on stdin there is no
# file path, so rustfmt cannot find Cargo.toml and falls back to edition 2015.
# It still reads rustfmt.toml, which it finds from the working directory.
#
# Format one buffer from stdin to stdout, for an editor.
fmt-stdin:
    @rustup run {{ nightly }} rustfmt --edition 2024 --

# Lints. Warnings are denied by build.warnings in .cargo/config.toml, so there
# is no `-D warnings` here and the build cache stays shared with `test`.
clippy:
    cargo clippy --workspace --all-targets --all-features --locked

# Tests, including the protocol schema check that enforces rule 8.
test:
    cargo test --workspace --all-features --locked

# rustdoc as a lint: the doc comments on yy-types are the specification's
# source, so a broken intra-doc link is a broken spec later.
doc:
    cargo doc --workspace --all-features --no-deps --locked

# Regenerate schema/current.json after an intentional protocol change. The diff
# belongs in the pull request, where a reviewer sees it.
schema:
    UPDATE_SCHEMA=1 cargo test -p yy-types schema

# Licence, advisory, and duplicate checks. Needs `cargo install cargo-deny`.
deny:
    cargo deny check

# Build the guide and specification.
book:
    mdbook build docs/book

# Serve the book with live reload while writing.
book-serve:
    mdbook serve docs/book --open

# Silence warnings temporarily while working. Never commit code that needs this.
check-noisy:
    CARGO_BUILD_WARNINGS=allow cargo check --workspace --all-targets
