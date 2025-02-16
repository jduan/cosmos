#!/usr/bin/env bash
# Return the first 12 chars of the input

cat | cut -c1-12 | tr -d '\n' | pbcopy
