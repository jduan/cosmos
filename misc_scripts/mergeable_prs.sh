#!/usr/bin/env bash
# This script needs an env var GITHUB_TOKEN
set -euxo pipefail

# Replace <owner> with the repository owner and <repo> with the repository name
owner=bay1inc
repo=bayone

# Get a list of all open pull requests in the repository
pull_requests=$(curl -H "Authorization: token $GITHUB_TOKEN" -s "https://api.github.com/repos/${owner}/${repo}/pulls?state=open")


# Filter the list of pull requests based on certain criteria
# For example, you can filter based on the 'mergeable_state' attribute to find pull requests that are mergeable
# Other criteria might include the 'labels' or 'reviews' attributes
# ready_to_merge=$(echo "$pull_requests" | jq -r 'map(select(.mergeable_state == "clean")) | length')
reviews_approved=$(echo "$pull_requests" | jq -r '.reviews[] | select(.state == "APPROVED") | .user.login')
checks_passed=$(echo "$pull_requests" | jq -r '.statuses[].state')


echo "Number of pull requests ready to be merged: $ready_to_merge"
