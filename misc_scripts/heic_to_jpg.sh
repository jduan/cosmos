#!/bin/bash

# 1
set -eu -o pipefail

# 2
count=$(find . -depth 1 -name "*.HEIC" -or -name "*.heic" | wc -l | sed 's/[[:space:]]*//')
echo "converting $count files .HEIC files to .jpg"

# 3
magick mogrify -monitor -format jpg *.HEIC
magick mogrify -monitor -format jpg *.heic

# 4
echo "Remove .HEIC files? [y/n]"
read remove

# 5
if [[ "$remove" == "y" ]]; then
  find . -depth 1 -name "*.HEIC" -delete
fi
