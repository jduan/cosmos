from pprint import pprint


def for_loop():
    symbols = "hello"
    codes = []
    # A for loop may be used to do lots of different things
    # In contrast, a listcomp's goal is always to build a new list.
    for symbol in symbols:
        codes.append(ord(symbol))
    pprint(codes)


def listcomp():
    symbols = "hello"
    # list comprehension is more readable
    codes = [ord(symbol) for symbol in symbols]
    pprint(codes)

    # In python 3, list comprehensions, generator expressions, and their
    # siblings "set" and "dict" comprehensions, have a local scope to hold
    # the variables assigned in the for clause
    # warning: you can't access "symbol" outside of the listcomp
    # print(symbol)

    # However, variables assigned with the "walrus operator" := remain accessible
    x = "ABC"
    codes = [last := ord(c) for c in x]
    # print(f"c: {c}")
    print(f"last: {last}")
    print(f"codes: {codes}")


def cartesian_product():
    """Cartesian product using a list comprehension"""
    colors = ["black", "white"]
    sizes = ["S", "M", "L"]
    # The resulting list is arranged as if the for loops were nested in the same order
    # as they appear in the listcomp
    tshirts = [(color, size) for color in colors for size in sizes]
    pprint(tshirts)


def main():
    for_loop()
    listcomp()
    cartesian_product()


if __name__ == "__main__":
    main()
