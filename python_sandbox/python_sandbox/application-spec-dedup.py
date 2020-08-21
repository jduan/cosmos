import sys
import yaml


def find_duplicate(filename):
    # print("Processing file %s" % filename)
    with open(filename) as f:
        # use safe_load instead load
        data_map = yaml.safe_load(f)
        service_idl = data_map.get("service_idl", list())
        service_dependency = data_map.get("service_dependency", list())
        intersection = set(service_idl or list()) & set(service_dependency or list())
        if intersection:
            # print("found circular dependencies in %s" % filename)
            print(filename)


def main():
    for filename in sys.argv[1:]:
        find_duplicate(filename)


if __name__ == '__main__':
    main()
