"""
Know how to slice sequences
"""

# You can slice built-in types like: list, tuple, str, bytes

a = ["a", "b", "c", "d", "e", "f", "g", "h"]
assert a[3:5] == ["d", "e"]
assert a[1:7] == ["b", "c", "d", "e", "f", "g"]

# slice from the start
assert a[:5] == a[0:5]

# slice from the end
assert a[5:] == a[5 : len(a)]

print(a[:])  # ["a", "b", "c", "d", "e", "f", "g", "h"]
print(a[:5])  # ["a", "b", "c", "d", "e"]
print(a[:-1])  # ["a", "b", "c", "d", "e", "f", "g"]
print(a[4:])  # ["e", "f", "g", "h"]
print(a[-3:])  # ["f", "g", "h"]
print(a[2:5])  # ["c", "d", "e"]
print(a[2:-1])  # ["c", "d", "e", "f", "g"]
print(a[-3:-1])  # ["f", "g"]

# slicing deals properly with start and end indices that are beyond the boundaries
# of a list by silently omitting missing items.
assert a[5:] == a[5:20]
assert a[:5] == a[-20:5]

# but directly accessing the same missing index causes an exception
try:
    print(a[20])
    raise RuntimeError("Expected an exception but it didn't happen")
except IndexError:
    pass

list_of_lists = [[1, 2], [3, 4]]
list_of_lists2 = list_of_lists[1:]
list_of_lists2[0].append(5)
# The result of slicing is a whole new list. However, each of the elements in the new
# list will refer to the corresponding objects from the original list. Modifying the
# existing objects change the original list too!
assert list_of_lists == [[1, 2], [3, 4, 5]]

list_of_lists2.append([5, 6])
assert list_of_lists2 == [[3, 4, 5], [5, 6]]
# appending a new item to list_of_lists doesn't change list_of_lists
assert list_of_lists == [[1, 2], [3, 4, 5]]

# slicing can be used in assignments
a = ["a", "b", "c", "d", "e", "f", "g", "h"]
print(a)
a[2:7] = ["H", "E"]
print(a)
assert a == ["a", "b", "H", "E", "h"]
