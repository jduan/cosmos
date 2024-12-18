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
  fasd
  fd
  fish
  fswatch
  fzf
  gh
  git
  go
  google-cloud-sdk
  htop
  iterm2
  jq
  kubectl
  kubectx
  kubernetes-helm
  ncdu
  pkgdiff
  pstree
  python3
  rbenv
  reattach-to-user-namespace
  ripgrep
  shellcheck
  terraform
  tldr
  tmux
  tmuxinator
  tokei
  tree
  vim
  wget
  xmlstarlet
  yamllint
  yq-go
)

# Iterate through the array
for package in "${packages[@]}"
do
  echo ">>> Installing package $package"
  nix-env -iA nixpkgs.$package
done
