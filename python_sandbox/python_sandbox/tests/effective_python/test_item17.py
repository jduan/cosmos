import os
import unittest
from python_sandbox.effective_python.item17 import (
    normalize,
    normalize_copy,
    normalize_func,
    ReadVisit
)


def get_numbers():
    """
    A generator that returns a list of numbers.
    """
    return (x for x in range(0, 5))


class TestItem17(unittest.TestCase):
    def test1(self):
        visits = [15, 25, 35]
        percentages = normalize(visits)
        expected = [15/0.75, 25/0.75, 35/0.75]
        self.assertListEqual(expected, percentages)

    def test2(self):
        numbers = get_numbers()
        percentages = normalize_copy(numbers)
        expected = [0, 10, 20, 30, 40]
        self.assertListEqual(expected, percentages)

    def test3(self):
        percentages = normalize_func(lambda: get_numbers())
        expected = [0, 10, 20, 30, 40]
        self.assertListEqual(expected, percentages)

    def test4(self):
        path = os.path.dirname(os.path.abspath(__file__))
        data_file = os.path.join(path, "visits.txt")
        rv = ReadVisit(data_file)
        percentages = normalize(rv)
        expected = [0, 10, 20, 30, 40]
        self.assertListEqual(expected, percentages)
