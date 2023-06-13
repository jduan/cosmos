#!/usr/bin/env bash
# This script fetches all the repos under $HOME/repos

set -exuo pipefail

repos_dir=$HOME/repos
for dir in "$repos_dir"/*; do
  if [[ -d "$dir" && -d "$dir/.git" ]]; then
    cd "$dir"
    date
    git fetch origin
  fi
done

printf "\nSUCCESS\n"
date
