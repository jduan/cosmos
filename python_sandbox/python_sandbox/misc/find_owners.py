# This script takes a list of project directories and find the owner info from "_infra/project.yml"
import os
import sys
import yaml


def find_owner(file):
    fullpath = "/Users/jingjing_duan/repos2/treehouse_worktree3/%s/_infra/project.yml" % file
    if os.path.exists(fullpath):
        with open(fullpath) as fd:
            data = yaml.load(fd, Loader=yaml.FullLoader)
            email = data.get("email")
            print("%s,%s,%s" % (file, email, data["teams"][0]))
    else:
        print("%s,," % file)


def main():
    for arg in sys.argv[1:]:
        if find_owner(arg):
            print(arg)


if __name__ == '__main__':
    main()
