import unittest


class TestItem9(unittest.TestCase):
    """
    The problem with list comprehensions (see Item 7: “Use List Comprehensions Instead of map and
    filter”) is that they may create a whole new list containing one item for each value in the
    input sequence. This is fine for small inputs, but for large inputs this could consume
    significant amounts of memory and cause your program to crash.

    To solve this, Python provides generator expressions, a generalization of list comprehensions
    and generators. Generator expressions don’t materialize the whole output sequence when they’re
    run. Instead, generator expressions evaluate to an iterator that yields one item at a time from
    the expression.
    """
    def test1(self):
        a = [1, 2, 3, 4]
        b = (x**2 for x in a)
        # b is a generator
        self.assertEqual(1, next(b))
        self.assertEqual(4, next(b))
        self.assertEqual(9, next(b))
        self.assertEqual(16, next(b))

    def test2(self):
        """
        Generators can be composed together!
        Chaining generators like this executes very quickly in Python. When you’re looking for a way
        to compose functionality that’s operating on a large stream of input, generator expressions
        are the best tool for the job.
        """
        a = [1, 2, 3, 4]
        b = (x**2 for x in a)
        c = (x + 1 for x in b)
        self.assertEqual([2, 5, 10, 17], list(c))
