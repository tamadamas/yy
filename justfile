run:
    cargo run

# create a git worktree wired to share this repo's cargo target-dir and .local
worktree name:
    git worktree add .claude/worktrees/{{name}} -b worktree-{{name}}
    ./scripts/setup-worktree.sh .claude/worktrees/{{name}}
