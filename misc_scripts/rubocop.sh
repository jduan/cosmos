#!/bin/bash
# Run "rubocop" on the files that have been changed on a user branch
git diff origin/master...HEAD --name-only | grep ".rb$" | xargs bundle exec rubocop
