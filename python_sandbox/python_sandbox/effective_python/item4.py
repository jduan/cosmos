"""
Write helper functions instead of complex expressions.
"""

from urllib.parse import parse_qs


def query_params():
    my_values = parse_qs("red=5&blue=0&green=", keep_blank_values=True)
    print(repr(my_values))
    print("Red: ", get_first_int(my_values, "red"))
    print("Green: ", get_first_int(my_values, "green"))
    print("Opacity: ", get_first_int(my_values, "opacity"))


# move complex expressions into helper functions, esp. if you need to use
# the same logic repeatedly.
def get_first_int(values, key, default=0):
    found = values.get(key, [""])
    if found[0]:
        return int(found[0])
    return default


if __name__ == "__main__":
    query_params()
