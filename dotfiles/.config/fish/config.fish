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
if [ -f '/usr/libexec/java_home' ];
    export JAVA_HOME=(/usr/libexec/java_home -v 1.8)
end
export CARGO_HOME=$HOME/.cargo
export RUSTUP_HOME=$HOME/.rustup
set paths \
    $CARGO_HOME/bin \
    $JAVA_HOME \
    $HOME/github/jduan/cosmos/misc_scripts \
    $HOME/repos2/sysops/optica_tools \
    $HOME/airlab/runtime_gems/tools/bin \
    # haskell stack
    $HOME/.local/bin \
    # pip installed binaries
    $HOME/Library/Python/3.7/bin/
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
set -gx AFDEV_HOST "i-0a55744d91bc4533a.inst.aws.airbnb.com"
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

# set up rbenv
status --is-interactive; and . (rbenv init -|psub)
# set up pyenv
status --is-interactive; and . (pyenv init -|psub)

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
    set -g cmdstart (/usr/local/bin/gdate '+%s%N')
end

# show the time consumed and the exit status
# after the command is run
function postcmd --on-event fish_postexec
    set old_status $status
    set cmdend (/usr/local/bin/gdate '+%s%N')
    set taken (math "($cmdend - $cmdstart) / 1000000")
    switch $old_status
    case 0
        set_color magenta
    case '*'
        set_color red
    end
    set now (date)
    printf "(Exit: %s, Time: %'dms, %s)\n" $old_status $taken $now
    set_color normal
end
