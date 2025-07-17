"""
Avoid striding and slicing in a single expression
"""
# some_list[start:end:stride]

x = ["red", "orange", "yellow", "green", "blue", "purple"]
assert x[::2] == ["red", "yellow", "blue"]
assert x[1::2] == ["orange", "green", "purple"]

# reverse a byte string
x = b"mongoose"
assert x[::-1] == b"esoognom"
# this works well for unicode strings
x = "寿司"
assert x[::-1] == "司寿"

# but it breaks when unicode data is encoded as a UTF-8 byte string
w = "寿司"
x = w.encode("utf-8")
y = x[::-1]
try:
    # this breaks
    z = y.decode("utf-8")
except UnicodeDecodeError:
    pass

# the point is that the stride part of the slicing syntax can be extremely confusing
x = ["a", "b", "c", "d", "e", "f", "g", "h"]
x[2::2]  # ["c", "e", "g"]
x[-2::-2]  # ["g", "e", "c", "a"]
x[-2:2:-2]  # ["g", "e"]
x[2:2:-2]  # []
