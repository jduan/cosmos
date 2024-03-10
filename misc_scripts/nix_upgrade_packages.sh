#!/usr/bin/env bash

# BLUE="\033[34m"
ORANGE="\033[0;33m"
RESET="\033[0m"

packages=(
  bat
  fasd-unstable
  fd
  fish
  fzf
  git
  htop
  jq
  ncdu
  tldr
  tmux
  tokei
  vim
  yq
  yamllint
)

for pkg in "${packages[@]}"; do
  echo -e "${ORANGE}================ Upgrading $pkg ===================${RESET}"
  nix-env --upgrade "$pkg"
done
