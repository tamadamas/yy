#!/usr/bin/env bash
# Wires a git worktree to share the main repo's cargo target dir and .local/.
# Run with cwd = worktree root, or pass the worktree path as $1.
set -euo pipefail

worktree_root="${1:-$(pwd)}"
cd "$worktree_root"

common_git_dir="$(git rev-parse --path-format=absolute --git-common-dir)"
main_root="$(cd "$(dirname "$common_git_dir")" && pwd)"
worktree_root="$(git rev-parse --show-toplevel)"

if [ "$main_root" = "$worktree_root" ]; then
  echo "setup-worktree: already at main repo root, nothing to do"
  exit 0
fi

mkdir -p "$worktree_root/.cargo"
cat > "$worktree_root/.cargo/config.toml" <<EOF
[build]
target-dir = "$main_root/target"
EOF

if [ ! -e "$worktree_root/.local" ]; then
  ln -s "$main_root/.local" "$worktree_root/.local"
fi

echo "setup-worktree: target-dir -> $main_root/target"
echo "setup-worktree: .local -> $main_root/.local"
