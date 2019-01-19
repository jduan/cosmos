#!/usr/bin/env python2.7
# This script takes in a file like
# /Users/jduan/workspace/path/to/file/in/git/repo
# and opens it in remote git server.

import os
import sys
import subprocess

# URL_TEMPLATE = "https://source.fitbit.com/projects/%s/repos/%s/browse/%s"
URL_TEMPLATE = "https://git.musta.ch/%s/%s/blob/master/%s"


def locate_git_repo(filepath):
    while True:
        parent_dir = os.path.dirname(filepath)
        if parent_dir is filepath:
            raise Exception("Failed to find a git repo!")
        if os.path.exists(os.path.join(parent_dir, ".git")):
            return parent_dir
        filepath = parent_dir


def find_filepath_from_repo_root(repo_dir, filepath):
    return filepath[len(repo_dir) + 1 :]


def find_group_and_repo_name(dirname):
    os.chdir(dirname)
    output = subprocess.check_output("git config --get remote.origin.url", shell=True)
    # output looks like: ssh://git@source.fitbit.com/group/repo_name.git
    # git@git.musta.ch:airbnb/deployboard.git
    if output.startswith("ssh"):
        parts = output.rstrip().split("/")
        group = parts[-2]
        repo_name = parts[-1]
        # remote the ".git" extension
        repo_name = repo_name.split(".")[0]
    elif output.startswith("git"):
        parts = output.split(":")[1].split("/")
        group = parts[0]
        repo_name = parts[1]
        # remote the ".git" extension
        repo_name = repo_name.split(".")[0]
    else:
        raise "Unknown git remote: %s" % output
    return (group, repo_name)


filepath = os.path.realpath(sys.argv[1])
dirname = locate_git_repo(filepath)
relative_path = find_filepath_from_repo_root(dirname, filepath)
group, repo_name = find_group_and_repo_name(dirname)
url = URL_TEMPLATE % (group, repo_name, relative_path)
subprocess.call("/bin/echo -n '%s' | pbcopy" % url, shell=True)
# print(url)
# subprocess.call("open %s" % url, shell=True)
