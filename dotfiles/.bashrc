[ -f ~/.fzf.bash ] && source ~/.fzf.bash

# added by Nix installer
if [ -e /Users/jingjing_duan/.nix-profile/etc/profile.d/nix.sh ]; then
	. /Users/jingjing_duan/.nix-profile/etc/profile.d/nix.sh;
fi

#THIS MUST BE AT THE END OF THE FILE FOR SDKMAN TO WORK!!!
export SDKMAN_DIR="/Users/jingjing_duan/.sdkman"
[[ -s "/Users/jingjing_duan/.sdkman/bin/sdkman-init.sh" ]] && source "/Users/jingjing_duan/.sdkman/bin/sdkman-init.sh"
