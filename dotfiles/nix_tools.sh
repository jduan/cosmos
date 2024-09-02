#!/usr/bin/env bash

# List of packages
packages=(
  asdf-vm
  coreutils
  diff-so-fancy
  fasd
  fd
  fish
  git
  iterm2
  python3
  rbenv
  ripgrep
  tmux
  vim
)

# Iterate through the array
for package in "${packages[@]}"
do
  echo ">>> Installing package $package"
  nix-env -iA nixpkgs.$package
done
