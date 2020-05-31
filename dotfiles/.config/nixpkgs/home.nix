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

  home.packages = [
    pkgs.ascii
    pkgs.binutils
    pkgs.black
    pkgs.colordiff
    pkgs.exa
    pkgs.fasd
    pkgs.fd
    pkgs.fswatch
    pkgs.fzf
    pkgs.git
    pkgs.gitAndTools.diff-so-fancy
    pkgs.gitAndTools.hub
    pkgs.gnupg
    pkgs.go
    pkgs.groovy
    pkgs.htop
    pkgs.hyperfine
    pkgs.jq
    pkgs.ktlint
    pkgs.kubectl
    pkgs.maven
    pkgs.procs
    pkgs.pstree
    pkgs.python38
    pkgs.python38Packages.yamllint
    pkgs.reattach-to-user-namespace
    pkgs.redis
    pkgs.ripgrep
    pkgs.rlwrap
    pkgs.rustup
    pkgs.shellcheck
    pkgs.socat
    pkgs.terminal-notifier
    pkgs.tldr
    pkgs.tmux
    pkgs.tokei
    pkgs.tree
    pkgs.universal-ctags
    pkgs.vim
    pkgs.watch
    pkgs.wdiff
    pkgs.wget
    pkgs.yq-go
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
