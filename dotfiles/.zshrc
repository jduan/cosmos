# enable fasd
# https://github.com/clvv/fasd
# built-in aliases
# alias a='fasd -a'            # any
# alias s='fasd -si'           # show / search / select
# alias d='fasd -d'            # directory
# alias f='fasd -f'            # file
# alias sd='fasd -sid'         # interactive directory selection
# alias sf='fasd -sif'         # interactive file selection
# alias z='fasd_cd -d'         # cd, same functionality as j in autojump
# alias zz='fasd_cd -d -i'     # cd with interactive selection
eval "$(fasd --init auto)"
# customized aliases
alias v='f -e vim'           # quick opening files with vim

export EDITOR=vim
export HISTSIZE=50000
export SAVEHIST=50000
# setopt SHARE_HISTORY
export HISTFILE=~/.zsh_history
# export PS1='%m %d %# '
# Just show the last component of the directory
# export PS1="%1d$(git_super_status) %# "
source $HOME/.zsh/zshrc.sh
PROMPT='%B%~%b$(git_super_status) %# '
source $HOME/.sh_aliases
# source $HOME/.sh_exports
bindkey -v # vi mode
bindkey "^[[A" history-search-backward
bindkey "^[[B" history-search-forward
setopt NO_NOMATCH # make HEAD^ work in zsh

# The following lines were added by compinstall
zstyle ':completion:*' completer _complete _ignored
zstyle ':completion:*' max-errors 1 not-numeric
zstyle :compinstall filename '/Users/jingjing.duan/.zshrc'

autoload -Uz compinit
compinit
# End of lines added by compinstall

# case-insensitive completion
zstyle ':completion:*' matcher-list 'm:{a-zA-Z}={A-Za-z}' \
        'r:|[._-]=* r:|=*' 'l:|=* r:|=*'

# make tagbar work in vim
export NODE_PATH=/usr/local/lib/jsctags/

# zsh resource file loading sequence:
# .zprofile
# .zshrc
# .zlogin
export PATH="$HOME/.local/bin:$HOME/bin:$HOME/scripts:$PATH"

# Hit C-z from the shell to trigger 'fg'
fancy-ctrl-z () {
  if [[ $#BUFFER -eq 0 ]]; then
    BUFFER="fg"
    zle accept-line
  else
    zle push-input
    zle clear-screen
  fi
}
zle -N fancy-ctrl-z
bindkey '^Z' fancy-ctrl-z

#THIS MUST BE AT THE END OF THE FILE FOR SDKMAN TO WORK!!!
export SDKMAN_DIR="/Users/jduan/.sdkman"
[[ -s "/Users/jduan/.sdkman/bin/sdkman-init.sh" ]] && source "/Users/jduan/.sdkman/bin/sdkman-init.sh"

[ -f ~/.fzf.zsh ] && source ~/.fzf.zsh
