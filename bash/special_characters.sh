#!/usr/bin/env bash

# Special characters:
#  # means comments
#  ; command separator, permitting two or more commands on the same line
#  ;; terminator in a case option
#  .  equivalent to "source" which is a bash builtin

echo hello; echo there

filename=/tmp/hello
if [ -f "$filename" ]; then
  echo "File $filename exists."; cp $filename $filename.bak
else
  echo "File $filename not found."; touch $filename
fi


name=abc
case "$name" in
  abc) echo "\$name = abc" ;;
  xyz) echo "\$name = xyz" ;;
esac
