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


if __name__ == "__main__":
    other()
