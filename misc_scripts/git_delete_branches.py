#!/usr/bin/env python2.7
import argparse
import subprocess
import sys

parser = argparse.ArgumentParser(description='Argument Parser')
parser.add_argument('--branch-prefix', '-b', dest='branch_prefix', action='store',
                    default='jduan',
                    help='The prefix for branches that should not be deleted. ' +
                    'The master branch is automatically included.')
parser.add_argument('--confirmation', dest='confirmation', action='store_true',
                    help='Ask the user for confirmation')
parser.add_argument('--no-confirmation', dest='confirmation',
                    action='store_false')
parser.set_defaults(confirmation=True)

args = parser.parse_args()


def list_branches():
    cmd = subprocess.Popen("git branch | grep -v '\*'", shell=True, stdout=subprocess.PIPE)
    return cmd.stdout.read().split()


def filter_branches(branches, prefix):
    return [branch for branch in branches if not branch.startswith(prefix)]


def delete_branches(branches):
    for branch in branches:
        subprocess.Popen("git branch -D %s" % branch, shell=True, stdout=subprocess.PIPE)
        print("Deleted branch: %s" % branch)


def user_confirmation():
    # raw_input returns the empty string for "enter"
    yes = {'yes', 'y', 'ye', ''}
    no = {'no', 'n'}

    choice = raw_input("Are you sure to delete these branches? ").lower()
    if choice in yes:
        return True
    elif choice in no:
        return False
    else:
        sys.stdout.write("Please respond with 'yes' or 'no'")


branches = filter_branches(list_branches(), args.branch_prefix)
branches = filter_branches(branches, 'master')
print("Branches:\n\n%s\n" % "  \n".join(branches))
if branches:
    if args.confirmation:
        answer = user_confirmation()
    else:
        answer = True
    if answer:
        delete_branches(branches)
