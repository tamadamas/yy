run:
    RUSTFLAGS="-Awarnings" rtk cargo run

build:
    RUSTFLAGS="-Awarnings" rtk cargo build

git_list:
    git worktree list

# create a git worktree wired to share this repo's cargo target-dir and .local
worktree name:
    git worktree add .claude/worktrees/{{ name }} -b worktree-{{ name }}
    ./scripts/setup-worktree.sh .claude/worktrees/{{ name }}

coverage:
    rtk cargo llvm-cov --fail-under-lines 80

format:
    rtk cargo fmt --all

format-check:
    rtk cargo fmt --all --check

check: format-check clippy test
    RUSTFLAGS="-Awarnings" rtk cargo check

clippy:
    rtk cargo clippy --all-targets

test:
    rtk cargo test
