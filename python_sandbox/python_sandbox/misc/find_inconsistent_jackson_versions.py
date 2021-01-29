# This script reads a list of files. For each file, it finds the versions of jackson packages and
# prints out files that have inconsistent jackson versions.
import sys


def inconsistent_jackson_version(file):
    with open(file) as fd:
        for line in fd.readlines():
            if line.startswith("#"):
                continue
            group, artifact, version = line.split(":")
            if ((group.startswith("com.fasterxml.jackson.core") or
                    group.startswith("com.fasterxml.jackson.datatype") or
                    group.startswith("com.fasterxml.jackson.module")) and
                    not version.startswith("2.9.10")):
                return True

    return False


def main():
    for arg in sys.argv[1:]:
        if inconsistent_jackson_version(arg):
            print(arg)


if __name__ == '__main__':
    main()
