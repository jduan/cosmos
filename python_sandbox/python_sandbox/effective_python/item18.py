"""
Use "zip" to process iterators in parallel.
"""

from itertools import zip_longest
from textwrap import fill

names = ["Cecilia", "Lise", "Marie"]
counts = [len(n) for n in names]
print(counts)


def find_longest_name(names):
    counts = [len(n) for n in names]
    longest_name = None
    max_count = 0
    for i in range(len(names)):
        count = counts[i]
        if count > max_count:
            longest_name = names[i]
            max_count = count
    return longest_name


print(f"Longest name: {find_longest_name(names)}")


def find_longest_name2(names):
    counts = [len(n) for n in names]
    longest_name = None
    max_count = 0
    for name, count in zip(names, counts):
        if count > max_count:
            longest_name = name
            max_count = count
    return longest_name


print(f"Longest name: {find_longest_name2(names)}")

names.append("Rosalind")
# this doesn't show "Rosalind" because zip keeps yielding tuples until any one of the
# wrapped iteators is exhausted.
for name, count in zip(names, counts):
    print(f"name: {name}, count: {count}")

# You can pass "strict=True" to make zip raise an exception if any of the inputs is
# exhausted before the others
try:
    for name, count in zip(names, counts, strict=True):
        print(f"name: {name}, count: {count}")
except ValueError:
    pass

# zip_longest fills in missing values for shorter iterators using the "fillvalue"
for name, count in zip_longest(names, counts, fillvalue=0):
    print(f"name: {name}, count: {count}")
