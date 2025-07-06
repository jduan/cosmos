from urllib.parse import parse_qs


def get_first_int(values, key, default=0):
    found = values.get(key, [""])
    if found[0]:
        found = int(found[0])
    else:
        found = default
    return found


def main():
    my_values = parse_qs("red=5&blue=0&green=", keep_blank_values=True)
    print("my_values: %s" % my_values)
    red = get_first_int(my_values, "red")
    green = get_first_int(my_values, "green")
    opacity = get_first_int(my_values, "opacity")

    print("Red: %r" % red)
    print("Green: %r" % green)
    print("Opacity: %r" % opacity)


if __name__ == "__main__":
    main()
