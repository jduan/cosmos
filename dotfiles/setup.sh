#!/usr/bin/env bash
set -euxo pipefail

# Link to ~/github/jduan/notes
ln -s ~/github/jduan/notes ~/notes
sudo ln -s $HOME/.nix-profile/bin/reattach-to-user-namespace /usr/local/bin/hmmm
