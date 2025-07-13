"""
Prefer interpolated F-strings over C-style format strings and str.format.

The combination of expressiveness, terseness, and clarity provided by f-strings
makes them the best built-in option for Python programmers. Any time you find
yourself needing to format values into strings, choose f-strings over the
alternatives.
"""

# C-style format strings
a = 0b10111011
b = 0xC5F
print("binary is %d, hex is %d" % (a, b))

key = "my_var"
value = 1.234
formatted = "%-10s = %.2f" % (key, value)
print(formatted)
# this throws a TypeError at runtime
# formatted = "%-10s = %.2f" % (value, key)

# this throws a TypeError at runtime as well
# reordered_string = "%.2f = %-10s" % (key, value)

pantry = [
    ("avocados", 1.25),
    ("bananas", 2.5),
    ("cherries", 15),
]

for i, (item, count) in enumerate(pantry):
    print("#%d: %-10s = %d" % (i + 1, item.title(), round(count)))

# this throws a TypeError at runtime
# print("{} = {}".format(key, value))

old_way = "%-10s = %.2f" % (key, value)
# using dictionaries in formatting expressions allows you to reorder the arguments
new_way = "%(key)-10s = %(value).2f" % {"value": value, "key": key}
assert old_way == new_way


# The "format" build-in function and "str.format"

key = "my_var"
value = 1.234
formatted = "{} = {}".format(key, value)
print(formatted)

formatted = "{:<10} = {:.2f}".format(key, value)
print(formatted)

for i, (item, count) in enumerate(pantry):
    old_style = "#%d: %-10s = %d" % (
        i + 1,
        item.title(),
        round(count),
    )

    # this isn't much better than the above
    new_style = "#{}: {:<10s} = {}".format(
        i + 1,
        item.title(),
        round(count),
    )

    assert old_style == new_style


# f-strings are the best!
key = "my_var"
value = 1.234

formatted = f"{key} = {value}"
print(formatted)

formatted = f"{key!r:<10} = {value:.2f}"
print(formatted)

for i, (item, count) in enumerate(pantry):
    old_style = "#%d: %-10s = %d" % (
        i + 1,
        item.title(),
        round(count),
    )

    # this isn't much better than the above
    new_style = "#{}: {:<10s} = {}".format(
        i + 1,
        item.title(),
        round(count),
    )

    # much shorter than the two ways above
    f_string = f"#{i + 1}: {item.title():<10s} = {round(count)}"

    assert old_style == new_style
    assert new_style == f_string
