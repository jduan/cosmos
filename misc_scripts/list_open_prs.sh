#!/usr/bin/env bash
# This script shows all of your open PRs.

GITHUB_ORG=bay1inc

gh search prs --author @me --state open --owner "$GITHUB_ORG" --json title,url,isDraft,repository,updatedAt | \
jq -r 'map(select(.isDraft == false)) | sort_by(.updatedAt) | reverse[] | "\(.repository.name): \(.title)\n\(.url)\nLast updated: \(.updatedAt)\n"'
