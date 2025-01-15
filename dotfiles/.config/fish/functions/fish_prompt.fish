function fish_prompt --description 'Write out the prompt'
    if not set -q __fish_prompt_normal
        set -g __fish_prompt_normal (set_color normal)
    end

    if not set -q __fish_prompt_cwd
        set -g __fish_prompt_cwd (set_color $fish_color_cwd)
    end

    # original:
    # echo -n -s "$USER" @ (prompt_hostname) ' ' "$__fish_prompt_cwd" (prompt_pwd) (__fish_vcs_prompt) " $__fish_prompt_normal" '> '
    echo -n -s ' ' "$__fish_prompt_cwd" (prompt_pwd)
    # just show the "git branch name" without showing "git status"
    if git rev-parse --is-inside-work-tree >/dev/null 2>&1
        set_color magenta
        echo -n ' ('(git rev-parse --abbrev-ref HEAD 2>/dev/null)')'
    end
    echo "$__fish_prompt_normal" '> '
end

set normal (set_color normal)
set magenta (set_color magenta)
set yellow (set_color yellow)
set green (set_color green)
set red (set_color red)
set gray (set_color -o black)

# Fish git prompt
# set __fish_git_prompt_showdirtystate 'yes'
# set __fish_git_prompt_showstagedstate 'yes'
# set __fish_git_prompt_showstashstate 'yes'
# set __fish_git_prompt_showuntrackedfiles 'yes'
# set __fish_git_prompt_showupstream 'yes'
# set __fish_git_prompt_color_branch yellow
# set __fish_git_prompt_color_upstream_ahead green
# set __fish_git_prompt_color_upstream_behind red

# Status Chars
# set __fish_git_prompt_char_dirtystate '✘'
# set __fish_git_prompt_char_stagedstate '→'
# set __fish_git_prompt_char_stashstate '↩'
# set __fish_git_prompt_char_untrackedfiles '☡'
# set __fish_git_prompt_char_upstream_ahead ' ⬆ '
# set __fish_git_prompt_char_upstream_behind ' ⬇ '


# function fish_prompt
#   set last_status $status
#
#   set_color $fish_color_cwd
#   printf '%s' (prompt_pwd)
#   set_color normal
#
#   # Don't show "git prompt" for large repos!
#   # set BIG_REPOS treehouse fullhouse airbnb chef data apps
#   set is_big_repo false
#   for repo in $BIG_REPOS
#     if string match --regex --quiet $repo $PWD
#       set is_big_repo true
#       break
#     end
#   end
#   if test $is_big_repo = true
#     set git_repo_root (git rev-parse --show-toplevel)
#     set git_repo_name (basename $git_repo_root)
#     # Show repo name for large repos
#     printf ' (%s) ' $git_repo_name
#   else
#     printf '%s ' (__fish_git_prompt)
#   end
#
#   set_color normal
# end
