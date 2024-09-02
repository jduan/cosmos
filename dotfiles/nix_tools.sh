#!/usr/bin/env bash
set -euo pipefail

# List of packages
packages=(
  actionlint
  asdf-vm
  bat
  bazelisk
  colordiff
  coreutils
  diff-so-fancy
  fasd
  fd
  fish
  fswatch
  fzf
  gh
  git
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
  yamllint
  yq-go
)

# Iterate through the array
for package in "${packages[@]}"
do
  echo ">>> Installing package $package"
  nix-env -iA nixpkgs.$package
done
