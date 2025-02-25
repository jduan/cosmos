#!/usr/bin/env bash
# This uses "gh pr merge" underneath but it allows you to check things in the diff and block the
# merge!
set -euo pipefail

PR_NUMBER=$(gh pr view --json number --jq .number)

# Fetch the diff
DIFF=$(gh pr diff "$PR_NUMBER")

# Check for forbidden patterns
if echo "$DIFF" | grep -q "TODO"; then
  echo "❌ Merge blocked: TODOs found."
  exit 1
fi

# If no issues, proceed with the merge
gh pr merge "$PR_NUMBER" --merge
