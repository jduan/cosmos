import unittest


class TestItem6(unittest.TestCase):
    def test_slicing(self):
        a = ['red', 'orange', 'yellow', 'green', 'blue', 'purple']
        odds = a[::2]
        evens = a[1::2]
        self.assertEqual(['red', 'yellow', 'blue'], odds)
        self.assertEqual(['orange', 'green', 'purple'], evens)

        # reverse a list
        b = [1, 2, 3]
        b2 = b[::-1]
        self.assertEqual([3, 2, 1], b2)

        # select every 2nd item starting from the end and move backwards
        c = [1, 2, 3, 4, 5]
        c2 = c[::-2]
        self.assertEqual([5, 3, 1], c2)

        # stride can become pretty confusing!
        # Avoid using start, end, and stride together in a single slice!
