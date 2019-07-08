# Things to Remember
#
# Instead of defining and instantiating classes, functions are often all you need for simple
# interfaces between components in Python.
#
# References to functions and methods in Python are first class, meaning they can be used in
# expressions like any other type.
#
# The __call__ special method enables instances of a class to be called like plain Python functions.
#
# When you need a function to maintain state, consider defining a class that provides the __call__
# method instead of defining a stateful closure (see Item 15: “Know How Closures Interact with
# Variable Scope”).
from collections import defaultdict
import unittest
from python_sandbox.effective_python.item23 import (
    log_missing,
    CountMissing,
    BetterCountMissing
)

current = {
    'green': 12,
    'blue': 3,
}
increments = [
    ('red', 5),
    ('blue', 17),
    ('orange', 9),
]


class TestItem23(unittest.TestCase):
    def test1(self):
        result = defaultdict(log_missing, current)
        print('Before:', dict(result))
        for key, amount in increments:
            result[key] += amount
        print('After:', dict(result))

    def test2(self):
        counter = CountMissing()
        result = defaultdict(counter.missing, current)
        print('Before:', dict(result))
        for key, amount in increments:
            result[key] += amount
        print('After:', dict(result))
        assert counter.missing_keys == 2

    def test3(self):
        counter = BetterCountMissing()
        result = defaultdict(counter, current)
        print('Before:', dict(result))
        for key, amount in increments:
            result[key] += amount
        print('After:', dict(result))
        assert counter.missing_keys == 2
