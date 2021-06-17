#!/usr/bin/env bash
# This script finds all the bazel files from either the working directory or a
# given directory passed on the command line.

if [ -z "$1" ]
then
  dir=.
else
  dir=$*
fi

fd '(.bazel$|.bzl$)' $dir
