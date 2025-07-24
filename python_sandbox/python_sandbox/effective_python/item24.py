"""
Consider "itertools" for working with iterators and generators
"""

import itertools

# 1. linking iterators together
it = itertools.chain([1, 2, 3], [4, 5, 6])
print(f"chain: {list(it)}")

it1 = [i * 3 for i in ("a", "b", "c")]
it2 = [i * 3 for i in ("x", "y", "z")]
# nested_it is an iterator of iterators
nested_it = [it1, it2]
output_it = itertools.chain.from_iterable(nested_it)
print(f"output_it: {list(output_it)}")

it = itertools.repeat("hello", 3)
print(list(it))

it = itertools.cycle([1, 2])
result = [next(it) for _ in range(10)]
print(result)

# tee splits a single iterator into N parallel iterators
it1, it2, it3 = itertools.tee(["first", "second"], 3)
print(list(it1))
print(list(it2))
print(list(it3))

# zip_longest
keys = ["one", "two", "three"]
values = [1, 2]
normal = list(zip(keys, values))
print(f"zip: {normal}")
longest = list(itertools.zip_longest(keys, values, fillvalue="nope"))
print(f"zip_longest: {longest}")

# 2. filtering items from an iterator

# islice slices an iterator by numerical indexes without copying, similar to standard slicing
values = list(i + 1 for i in range(10))
first_five = itertools.islice(values, 5)
print(f"first_five: {list(first_five)}")
middle_odds = itertools.islice(values, 2, 8, 2)
print(f"middle_odds: {list(middle_odds)}")

values = list(i + 1 for i in range(10))
less_than_seven = itertools.takewhile(lambda x: x < 7, values)
print(f"less than seven: {list(less_than_seven)}")

values = list(i + 1 for i in range(10))
more_than_seven = itertools.dropwhile(lambda x: x < 7, values)
print(f"more than seven: {list(more_than_seven)}")

odd_nums = itertools.filterfalse(lambda x: x % 2 == 0, values)
print(f"odd_nums: {list(odd_nums)}")

# 3. produce combinations of items from iterators

it = itertools.batched([1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 3)
print(f"batched: {list(it)}")

route = ["Los Angeles", "Bakersfield", "Modesto", "Sacramento"]
it = itertools.pairwise(route)
print(f"pairwise: {list(it)}")

values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
sum_reduce = itertools.accumulate(values)
print(f"sum_reduce: {list(sum_reduce)}")


def sum_modulo_20(first, second):
    output = first + second
    return output % 20


module_reduce = itertools.accumulate(values, sum_modulo_20)
print(f"modulo_reduce: {list(module_reduce)}")

single = itertools.product([1, 2], repeat=2)
print(f"single: f{single}")
multiple = itertools.product([1, 2], ["a", "b"])
print(f"multiple: {multiple}")

it = itertools.permutations([1, 2, 3, 4], 2)
print(f"permutations: {list(it)}")

it = itertools.combinations([1, 2, 3, 4], 2)
print(f"combinations: {list(it)}")

it = itertools.combinations_with_replacement([1, 2, 3, 4], 2)
print(f"combinations_with_replacement: {list(it)}")
