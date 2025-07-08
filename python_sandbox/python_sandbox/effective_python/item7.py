"""
Consider conditional expressions for simple inline logic
"""

i = 3
x = "even" if i % 2 == 0 else "odd"
print(x)


def fail():
    raise Exception("Oops")


# fail() will not get evaluated because the conditional is false
x = fail() if False else 20
print(x)

# filtering in listing comprehensions
result = [x / 4 for x in range(10) if x % 2 == 0]
print(result)


def my_very_long_function_call(a, b, c):
    return a + b + c


# You should avoid conditional expressions when they must be split over multiple lines.
# This is quite hard to read.
x = (
    my_very_long_function_call(1, 2, 3)
    if i % 2 == 0
    else my_very_long_function_call(4, 5, 6)
)
