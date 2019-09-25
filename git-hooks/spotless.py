#!/usr/bin/env python3

# This script can serve as a git "pre-push" script. It figures out what Gradle projects
# have changed and run "spotlessCheck" for each project.

import os
import subprocess
from pathlib import Path


def find_build_gradle(dir):
    """
    Recursively go up and find the nearest build.gradle file. Return the directory name
    of that build.gradle file.
    """
    build_gradle = dir.joinpath("build.gradle")
    if build_gradle.exists():
        return dir
    else:
        return find_build_gradle(dir.parent)


git_repo_dir = subprocess.getoutput("git rev-parse --show-toplevel")
gradlew = "%s/gradlew" % git_repo_dir
root_build_gradle = "%s/build.gradle" % git_repo_dir

output = subprocess.getoutput("git diff origin/master...HEAD --name-only")
files_changed = output.split("\n")
should_run = any(file_changed.endswith(".java") or file_changed.endswith(".kt") for
                 file_changed in files_changed)
if not should_run:
    print("No source files have been changed. Not going to run spotlessCheck.")
    exit(0)

project_dirs = set()
for file_changed in files_changed:
    dir = Path(git_repo_dir).joinpath(file_changed).absolute().parent
    project_dir = find_build_gradle(dir)
    # Never run spotless
    if project_dir != Path(git_repo_dir):
        project_dirs.add(find_build_gradle(dir))

for project_dir in project_dirs:
    os.chdir(project_dir)
    try:
        print("Running spotlessCheck for %s" % project_dir)
        subprocess.check_call("%s spotlessCheck" % gradlew, shell=True, stdout=subprocess.DEVNULL,
                              stderr=subprocess.DEVNULL)
    except subprocess.CalledProcessError:
        print("spotlessCheck failed in %s" % project_dir)
        exit(-1)
