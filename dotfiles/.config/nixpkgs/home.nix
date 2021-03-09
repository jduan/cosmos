{ config, pkgs, ... }:

{
  # Let Home Manager install and manage itself.
  programs.home-manager.enable = true;

  # Home Manager needs a bit of information about you and the
  # paths it should manage.
  home.username = "jingjing_duan";
  home.homeDirectory = "/Users/jingjing_duan";

  # This value determines the Home Manager release that your
  # configuration is compatible with. This helps avoid breakage
  # when a new Home Manager release introduces backwards
  # incompatible changes.
  #
  # You can update Home Manager without changing this value. See
  # the Home Manager release notes for a list of state version
  # changes in each release.
  home.stateVersion = "20.09";

  home.packages = with pkgs; [
    ascii
    bash
    bazelisk
    binutils
    black
    bazel-buildtools
    colordiff
    coreutils
    curlFull
    exa
    fasd
    fd
    fish
    fswatch
    fzf
    git
    gitAndTools.diff-so-fancy
    gitAndTools.hub
    gnupg
    go
    graphviz
    groovy
    htop
    hyperfine
    iterm2
    jdk11
    jq
    ktlint
    kubectl
    maven
    ncdu
    nixfmt
    pmd
    procs
    pstree
    python38
    python38Packages.yamllint
    reattach-to-user-namespace
    redis
    ripgrep
    rlwrap
    ruby_2_7
    rustup
    shellcheck
    terminal-notifier
    tldr
    tmux
    tmuxinator
    tokei
    tree
    universal-ctags
    vim
    watch
    wdiff
    wget
    yq-go
  ];

  programs.bat.enable = true;

  programs.neovim = {
    enable = true;
    extraConfig = "colorscheme gruvbox";
    plugins = with pkgs.vimPlugins; [
      # syntax and language support
      vim-fish
      vim-nix
      vim-toml

      # UI
      gruvbox # colorscheme

      # Editor features
      nerdtree
      supertab
      tcomment_vim
      vim-endwise
      vim-repeat
      vim-surround
    ];
  };
}
