#!/usr/bin/env bash
# run this script from the "repos" dir which has git repos
# it will iterate through each repo and create a worktree under "repos/worktrees"

set -euo pipefail

function add_worktrees() {
  local repos_dir=$1

  for dir in "$repos_dir"/*; do
    if [[ -d "$dir" && -d "$dir/.git" ]]; then
      cd "$dir"
      repo_name=$(basename "$dir")
      echo "adding worktree for $repo_name"
      if [ ! -d "../worktrees/$repo_name" ]; then
        git worktree add "../worktrees/$repo_name" origin/main
      fi
    fi
  done
}

repos_dir="$HOME/repos"
mkdir -p "$repos_dir/worktrees"
add_worktrees "$repos_dir"
