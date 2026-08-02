# Every recipe here is a plain cargo command you can read and run by hand.
# `just` is a convenience, never a requirement: nothing in the build, the tests,
# or CI needs it. See docs/design/repository.md.

# List the recipes.
default:
    @just --list

# Everything CI runs on a pull request. Run this before you push.
check: fmt-check clippy test doc

# Format the workspace. Nightly is required: rustfmt.toml uses nightly-only
# options (§8.6). It is the only nightly in the project.
fmt:
    cargo +nightly fmt --all

# Fail if anything is unformatted.
fmt-check:
    cargo +nightly fmt --all --check

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
