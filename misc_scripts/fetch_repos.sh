#!/usr/bin/env bash
# This script fetches all the repos under $HOME/repos

set -exuo pipefail

repos_dir=$HOME/repos
for dir in "$repos_dir"/*; do
  cd "$dir"
  $HOME/.nix-profile/bin/git fetch origin
done

printf "\nSUCCESS\n"
