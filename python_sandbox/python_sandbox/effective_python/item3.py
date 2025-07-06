"""
Never expect python to detect errors at compile time.
"""


def bad_reference():
    # this is valid python code even if my_var isn't declared
    print(my_var)
    my_var = 123


def sometimes_ok(x):
    if x:
        my_var = 123
    print(my_var)


def bad_math():
    # this is valid python code too
    return 1 / 0


if __name__ == "__main__":
    bad_math()
    sometimes_ok(True)
    sometimes_ok(False)
    bad_reference()
