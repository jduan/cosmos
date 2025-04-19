#!/usr/bin/env bash
set -euo pipefail

# List of packages
packages=(
  actionlint
  bash
  asdf-vm
  bat
  bazelisk
  buildifier
  colordiff
  coreutils
  dialog
  diff-so-fancy
  difftastic
  dive
  fasd
  fd
  fish
  fswatch
  fzf
  gh
  ghostscript
  git
  ghz
  gnupg
  go
  google-cloud-sdk
  htop
  imagemagick
  iterm2
  jq
  kubectl
  kubectx
  kubernetes-helm
  meld
  ncdu
  pkgdiff
  pstree
  python3
  rbenv
  reattach-to-user-namespace
  ripgrep
  shellcheck
  temporal
  temporal-cli
  terraform
  tldr
  tmux
  tmuxinator
  tokei
  tree
  vim
  watch
  wget
  xmlstarlet
  yamllint
  yarn
  yq-go
)

# Iterate through the array
for package in "${packages[@]}"
do
  echo ">>> Installing package $package"
  nix-env -iA "nixpkgs.$package"
done
