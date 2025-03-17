#!/usr/bin/env bash
# Given a name, it will create a folder and a program under the folder.

name=$1
mkdir "$name"
program="$name/$name.go"
touch "$program"

cat <<EOF > "$program"
package main

import (
	"fmt"
	"net/http"
)

func main() {
}
EOF
