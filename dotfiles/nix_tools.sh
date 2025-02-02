#!/usr/bin/env bash
set -euo pipefail

# List of packages
packages=(
  actionlint
  asdf-vm
  bat
  bazelisk
  buildifier
  colordiff
  coreutils
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
  nodejs
  pkgdiff
  pstree
  python3
  rbenv
  reattach-to-user-namespace
  ripgrep
  shellcheck
  temporal
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
