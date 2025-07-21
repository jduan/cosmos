"""
Prefer enumerate over range
"""

import enum
from random import randint

random_bits = 0
for i in range(32):
    if randint(0, 1):
        random_bits |= 1 << i

print(bin(random_bits))

flavor_list = ["vanilla", "chocolate", "pecan", "strawberry"]
for flavor in flavor_list:
    print(f"{flavor} is delicious")

# this works but isn't elegant
for i in range(len(flavor_list)):
    flavor = flavor_list[i]
    print(f"{i + 1}: {flavor}")

# enumerator yields pairs of the loop index and the next value from the given iterator
for i, flavor in enumerate(flavor_list):
    print(f"{i + 1}: {flavor}")

# begin counting at 1 instead of 0
for i, flavor in enumerate(flavor_list, 1):
    print(f"{i}: {flavor}")
