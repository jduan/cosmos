def tuples_as_records():
    lax_coordinates = (33.9425, -118.408056)
    city, year, pop, chg, area = ("Tokyo", 2003, 32_450, 0.66, 8014)
    traver_ids = [("USA", "31195855"), ("BRA", "CE342567"), ("ESP", "XDA205856")]
    for passport in sorted(traver_ids):
        # The % formatting operator understands tuples and treats each item as a separate field
        print("%s/%s" % passport)

    # The for loop knows how to "unpack" tuples
    for country, _ in traver_ids:
        print(country)


def tuples_as_immutable_lists():
    """Tuples have two key benefits:
    1. clarity: when you see tuples, you know their lengths will never change
    2. performance: a tuple uses less memory than a list of the same length

    Note that references in a tuple can't be deleted or replaced. But if the
    references point to mutable objects, those objects can change.

    Tuples with mutable items can be a source of bugs. For example, an object
    is only hashable if its value can't ever change.
    """
    a = (10, 'alpha', [1, 2])
    b = (10, 'alpha', [1, 2])
    assert a == b

    b[-1].append(99)
    assert a != b
    assert a[-1] == [1, 2]
    assert b[-1] == [1, 2, 99]


def main():
    tuples_as_records()
    tuples_as_immutable_lists()


if __name__ == "__main__":
    main()
