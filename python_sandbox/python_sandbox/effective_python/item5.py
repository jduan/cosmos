from urllib.parse import parse_qs


def other():
    """
    Show how to slice sequences.
    somelist[start:end]
    * start is inclusive and end is exclusive
    * negative numbers are offsets relative to the end of a list
    * slicing deals properly with start and end indexes that are beyond the boundaries
    """
    a = ["a", "b", "c", "d", "e", "f", "g", "h"]
    print("First four: ", a[:4])
    print("Last four: ", a[-4:])
    print("Middle two", a[3:-3])
    print("Copy list", a[:])
    print("Last five", a[-5:])
    print("Exclude last one", a[:-1])
    print("5th and the rest", a[4:])
    print("Last 3", a[-3:])
    print("3 items", a[2:5])
    print("3rd to 2nd to the last", a[2:-1])
    print("2 items", a[-3:-1])
    # beyond boundaries
    print("first 20 items", a[:20])
    print("last 20 items", a[-20:])


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
