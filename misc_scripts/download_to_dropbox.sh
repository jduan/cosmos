#!/usr/bin/env bash
# This script moves the most recently downloaded item to the "_knowledge_base"
# folder in dropbox.

set -euxo pipefail

recent_file=$(ls -Art ~/Downloads | tail -n 1)
mv "$HOME/Downloads/$recent_file" ~/Dropbox/_knowledge_base
