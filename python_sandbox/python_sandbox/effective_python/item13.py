"""
Prefer explicit string concatenation over implicit, especially in lists
"""

s1 = "helloworld"
s2 = "hello" + "world"
assert s1 == s2

x = 1
s3 = (
    r"first \ part is here with escapes\n, "
    f"string interpolation {x} in here, "
    'this has "double quotes" inside'
)
print(s3)

# implicit concatenation on a single line can be hard to read
y = 2
s4 = r"fir\st" f"{y}" '"third"'
print(s4)
# the extra comma is hard to see and it makes this a tuple
s5 = r"fir\st", f'{y}"third"'
print(s5)
assert s4 != s5
