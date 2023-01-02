#!/usr/bin/env bash
# This script commits all changes to the "notes" repo and pushes it.

set -exuo pipefail

cd "$HOME"/github/jduan/notes
git add .
git commit -m "update notes"
git push

printf "\nSUCCESS\n"
