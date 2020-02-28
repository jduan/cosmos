#!/usr/bin/env bash

tmpdir=$(mktemp -d "${TMPDIR:-/tmp/}$(basename "$0").XXXXXXXXXXXX")
stdout=${tmpdir}/stdout
stderr=${tmpdir}/stderr

cargo test > "$stdout" 2> "$stderr"

if [[ $? -ne 0 ]]
then
  cat "$stdout"
  cat "$stderr"
  echo -e "\033[31m Tests failed\033[0m"
else
  echo -e "\033[32m Tests passed!\033[0m"
fi

if cat "$stderr" | grep -q "warning:"; then
  cat "$stderr"
  echo -e "\033[31m Clippy errors\033[0m"
else
  echo -e "\033[32m Clippy passed!\033[0m"
fi
