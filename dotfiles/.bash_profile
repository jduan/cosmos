# .bash_profile

# Get the aliases and functions
if [ -f ~/.bashrc ]; then
	. ~/.bashrc
fi

export PATH="$HOME/.cargo/bin:$PATH"
if [ -e /Users/jingjing_duan/.nix-profile/etc/profile.d/nix.sh ]; then . /Users/jingjing_duan/.nix-profile/etc/profile.d/nix.sh; fi # added by Nix installer
