#!/usr/bin/env bash
# Return the first 12 chars of the input

echo $1 | cut -c1-12 | pbcopy
