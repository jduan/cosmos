"""
Understand the difference between repr and str when printing objects
"""

# The "repr" built-in function returns the "printable representation" of an object
# which should be its most clearly understandable string serialization.

a = "\x07"
print(repr(a))
# Passing the value returned by "repr" to the "eval" built-in function often
# results in the same python object that you started with:
b = eval(repr(a))
assert a == b

# when you're debugging with "print", you should call "repr" on a value
# to ensure that any difference in types is clear:
int_var = 3
str_var = "3"
print(repr(int_var))
print(repr(str_var))
print("Is %r == %r?" % (int_var, str_var))
print(f"Is {int_var!r} == {str_var!r}")

# special methods
# when the "str" build-in function is called on a user-defined object, it first
# tries to call the "__str__" special method. If that's not defined, it falls back
# to call the "__repr__" special method. If that's not defined either, the it goes
# through method resolution, eventually calling the default implementation from the
# "object" parent class. Unfortunately, the default implementations aren't helpful.


class OpaqueClass:
    def __init__(self, x, y):
        self.x = x
        self.y = y

    def __str__(self):
        return f"({self.x}, {self.y})"

    def __repr__(self):
        return f"BetterClass({self.x!r}, {self.y!r})"


obj = OpaqueClass(1, "foo")
print(obj)
print(repr(obj))
