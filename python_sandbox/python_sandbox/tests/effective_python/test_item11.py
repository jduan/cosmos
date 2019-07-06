import unittest
from itertools import zip_longest


class TestItem11(unittest.TestCase):
    def test1(self):
        names = ['Cecilia', 'Lise', 'Marie']
        letters = [len(n) for n in names]
        max_letters = 0
        longest_name = None
        for name, count in zip(names, letters):
            if count > max_letters:
                max_letters = count
                longest_name = name

        self.assertEqual('Cecilia', longest_name)

    def test2(self):
        """
        zip truncates its output silently if you supply it with iterators of different lengths.

        Use zip_longest from itertools if you don't like the truncating behavior.
        :return:
        :rtype:
        """
        names = ['Cecilia', 'Lise', 'Marie']
        ages = [30, 50, 20, 99]
        names_and_ages = list(zip(names, ages))
        self.assertEqual([('Cecilia', 30), ('Lise', 50), ('Marie', 20)], names_and_ages)

        # the default fillvalue is None
        names_and_ages2 = list(zip_longest(names, ages, fillvalue=''))
        self.assertEqual(
            [('Cecilia', 30), ('Lise', 50), ('Marie', 20), ('', 99)],
            names_and_ages2
        )
