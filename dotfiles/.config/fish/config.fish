export EDITOR=vim

if [ -f '/usr/libexec/java_home' ];
    export JAVA_HOME=(/usr/libexec/java_home -v 1.11)
end
# export JAVA_HOME=/nix/store/84kssxbjlfbjkah4hd8hpylkjnq5isb3-zulu8.54.0.21-ca-jdk-8.0.292/

export CARGO_HOME=$HOME/.cargo
export RUSTUP_HOME=$HOME/.rustup

#########################################################
# Why do we check if a path has been added to $PATH?
#
# Typically, I start a terminal which loads your default shell.
# That's when this file gets loaded the first time and $PATH is set.
# Then I start "tmux" inside the shell. tmux inherits all the
# environment variables, including $PATH, from the parent shell.
# When tmux launches a fish shell inside it, the fish shell inherits
# the $PATH env var too. Then the fish shell loads this file again.
# That's why we don't want to add things to $PATH again.
#########################################################
# define a list of paths
set paths \
    $CARGO_HOME/bin \
    $JAVA_HOME \
    $HOME/github/jduan/cosmos/misc_scripts \
    $HOME/repos2/sysops/optica_tools \
    $HOME/airlab/runtime_gems/tools/bin \
    # haskell stack
    $HOME/.local/bin \
    # pip installed binaries
    $HOME/Library/Python/3.7/bin/ \
    # executables installed by rubygems
    $HOME/.gem/ruby/2.7.0/bin
for path in $paths
    if test -d "$path"
        if not contains "$path" $PATH
            set -gx PATH "$path" $PATH
        end
    end
end
# Linuxbrew
switch (uname)
case Linux
    if [ -d '/home/linuxbrew' ];
        eval (/home/linuxbrew/.linuxbrew/bin/brew shellenv)
    end
end

# homebrew
if [ -d '/opt/homebrew' ];
    eval (/opt/homebrew/bin/brew shellenv)
end

source $HOME/.fish_aliases

fish_vi_key_bindings
source $HOME/.config/fish/functions/fish_user_key_bindings.fish
# set -g fish_user_paths "/usr/local/opt/node@6/bin" $fish_user_paths

# The next line updates PATH for the Google Cloud SDK.
if [ -f '/Users/jingjing_duan/google-cloud-sdk/path.fish.inc' ];
    if type source > /dev/null;
        source '/Users/jingjing_duan/google-cloud-sdk/path.fish.inc';
    else;
        . '/Users/jingjing_duan/google-cloud-sdk/path.fish.inc';
    end;
end

# airbnb
set -gx DATA_DIR $HOME/repos2/data
set -gx AFDEV_HOST "i-0b911c61132abd2e1.inst.aws.airbnb.com"
set -gx ONETOUCHGEN_ACCEPT_EULA yep
# the port you'd like to use to run the local airflow webserver. This should be
# a number between 49152–65535. Do not use 61903, you should choose a different
# port
set -gx AFDEV_PORT 61803
set -gx arborist $HOME/repos2/treehouse/projects/arborist
set -gx treehouse $HOME/repos2/treehouse
set -gx fullhouse $HOME/repos/fullhouse

# ripgrep
set -gx RIPGREP_CONFIG_PATH $HOME/.ripgreprc

# fzf
# Setting fd as the default source for fzf
export FZF_DEFAULT_COMMAND='fd --type f'

# To apply the command to CTRL-T as well
export FZF_CTRL_T_COMMAND="$FZF_DEFAULT_COMMAND"

# OneTouch
export K2=1

# Don't generate .pyc files!
export PYTHONDONTWRITEBYTECODE=1

# set a global variable to the current epoch time in seconds
# before executing any comment
function get_start --on-event fish_preexec
    set -g cmdstart ($HOME/.nix-profile/bin/date '+%s%N')
end

# show the time consumed and the exit status
# after the command is run
function postcmd --on-event fish_postexec
    set old_status $status
    switch $old_status
    case 0
        set_color magenta
    case '*'
        set_color red
    end

    set now (date)
    set cmdend ($HOME/.nix-profile/bin/date '+%s%N')
    set duration (math "($cmdend - $cmdstart) / 1000000")
    if test "$duration" -lt 1000
        # print in milliseconds
        printf "(Exit: %s, Time: %.0fms, %s)\n" $old_status $duration $now
    else
        # print in human-readable format: hh:mm:ss
        set duration_seconds (math "$duration / 1000")
        set duration_seconds_rounded (math "round($duration_seconds)")
        set human_format ($HOME/.nix-profile/bin/date -d@$duration_seconds_rounded -u +%H:%M:%S)
        printf "(Exit: %s, Time: %s, %s)\n" $old_status $human_format $now
    end
    set_color normal
end

# The next line updates PATH for the Google Cloud SDK.
if [ -f '/Users/jduan/google-cloud-sdk/path.fish.inc' ]; . '/Users/jduan/google-cloud-sdk/path.fish.inc'; end
