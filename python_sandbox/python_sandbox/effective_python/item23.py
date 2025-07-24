"""
Pass iterators to "any" and "all" for efficient short-circuiting logic
"""

import random


def flip_coin():
    if random.randint(0, 1) == 0:
        return "Heads"
    else:
        return "Tails"


def flip_is_heads():
    return flip_coin == "Heads"


def all_heads(times):
    flips = [flip_is_heads() for _ in range(times)]
    return False not in flips


def all_heads2(times):
    """short circuit"""
    all_heads = True
    for _ in range(times):
        if not flip_is_heads():
            all_heads = False
            break
    return all_heads


def all_heads3(times):
    """short circuit using all"""
    return all(flip_is_heads() for _ in range(times))


def repeated_is_heads(times):
    for _ in range(times):
        yield flip_is_heads()  # generator


# this uses a generator function
all_heads4 = all(repeated_is_heads(20))
print(f"all_heads4: {all_heads4}")

print(all_heads(20))
print(all_heads2(20))
print(all_heads3(20))


def flip_is_tails():
    return flip_coin() == "Tails"


def all_heads5(times):
    """any is the opposite of all"""
    return not any(flip_is_tails() for _ in range(times))


"""
If you want to end early with a "True" value, use any.
If you want to end early with a "False" value, use all.
"""
