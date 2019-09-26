#!/usr/bin/env bash
# Usage: remind.sh MINUTES MESSAGE

minutes=$(expr $1 \* 60)
shift
message=$@

(sleep $minutes; terminal-notifier -message "$message") &
