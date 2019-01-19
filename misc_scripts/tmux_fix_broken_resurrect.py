#!/usr/bin/env python3
# When a computer crashes, the "last" symlink of tmux resurrect can be broken.
# This script points the "last" symlink to the most recent session.

import os
from os.path import expanduser
from subprocess import (call, run, PIPE)

home = expanduser("~")

os.chdir("%s/.tmux/resurrect/" % home)
last_session = run("ls -ltr tmux_resurrect* | tail -1 | awk '{ print $NF }'",
                   stdout=PIPE, shell=True).stdout.decode('utf-8').strip()
call("rm last || true", shell=True)
call("ln -s %s last" % last_session, shell=True)
print("The 'last' symlink has been restored.")
