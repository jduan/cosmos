import unittest
from python_sandbox.effective_python.item15 import (
    sort_priority,
    sort_priority2,
    sort_priority3,
)


class TestItem15(unittest.TestCase):
    def test1(self):
        numbers = [8, 3, 1, 2, 5, 4, 7, 6]
        group = {2, 3, 5, 7}
        sort_priority(numbers, group)
        expected = [2, 3, 5, 7, 1, 4, 6, 8]
        self.assertEqual(expected, numbers)

    def test2(self):
        numbers = [8, 3, 1, 2, 5, 4, 7, 6]
        group = {2, 3, 5, 7}
        found = sort_priority2(numbers, group)
        expected = [2, 3, 5, 7, 1, 4, 6, 8]
        self.assertEqual(expected, numbers)
        self.assertTrue(found)

    def test3(self):
        numbers = [8, 3, 1, 2, 5, 4, 7, 6]
        group = {2, 3, 5, 7}
        found = sort_priority3(numbers, group)
        expected = [2, 3, 5, 7, 1, 4, 6, 8]
        self.assertEqual(expected, numbers)
        self.assertTrue(found)
