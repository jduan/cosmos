import json
import sys


def main():
    for json_file in sys.argv[1:]:
        with open(json_file) as fd:
            content = json.load(fd)
            for artifact in content["artifacts"]:
                print(f"{artifact['name']}, {artifact['size_in_bytes']}, {artifact['created_at']}")


if __name__ == '__main__':
    main()
