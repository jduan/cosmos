#!/usr/bin/env bash
# Restore tmux sessions directly from the command line without starting a tmux
# session first.
# See https://github.com/tmux-plugins/tmux-resurrect/issues/270
tmux send-keys -t "$session" cd "$HOME"/.tmux/plugins/tmux-resurrect/scripts/ && \\
  "$HOME"/.tmux/plugins/tmux-resurrect/scripts/restore.sh enter
