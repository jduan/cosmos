"""
Always surround single-element tuples with parentheses
"""

# there are 4 kinds of tuples in python
first = (1, 2, 3)
second = (
    1,
    2,
    3,
)  # trailing comma makes it easy to add more elements
third = 1, 2, 3
fourth = (
    1,
    2,
    3,
)

assert first == second == third == fourth

# there are 3 special cases when creating tuples
empty = ()
single_with = (1,)
# this isn't a tuple. It's a parenthesized expression instead of a tuple
# fmt: off
single_without = (1)
# fmt: on
print(single_with == single_without)
# fmt: off
single_no_parens = 1,
# fmt: on
assert single_with == single_no_parens


def add(a, b):
    return a + b


# You can add a trailing comma to an expression, including a function call, and
# it creates a tuple!
# fmt: off
refund = add(1, 2),
# fmt: on
print(refund)
print(type(refund))

"""
Summary: It’s all too easy to have an extraneous trailing comma at the end of an
expression, which changes the meaning of the expression into a single-element
tuple that breaks a program.
"""
