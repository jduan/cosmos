#!/usr/bin/env bash
# This script fetches all the repos under $HOME/repos

set -exuo pipefail

function fetch_repos() {
  local repos_dir=$1

  for dir in "$repos_dir"/*; do
    if [[ -d "$dir" && -d "$dir/.git" ]]; then
      cd "$dir"
      date
      git fetch origin
    fi
  done
}

fetch_repos "$HOME/repos"
fetch_repos "$HOME/repos/worktrees"

printf "\nSUCCESS\n"
date
