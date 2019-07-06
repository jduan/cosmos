import unittest


class TestItem7(unittest.TestCase):
    def test_list_comprehension(self):
        a = [1, 2, 3, 4, 5]
        squares = [x ** 2 for x in a]
        self.assertEqual([1, 4, 9, 16, 25], squares)
        # this is not as elegant
        squares2 = list(map(lambda x: x**2, a))
        self.assertEqual([1, 4, 9, 16, 25], squares2)

        # filters
        even_squares = [x**2 for x in a if x % 2 == 0]
        self.assertEqual([4, 16], even_squares)
        # this is less elegant
        even_squares2 = list(map(lambda x: x**2, filter(lambda x: x % 2 == 0, a)))
        self.assertEqual([4, 16], even_squares2)

    def test_dict_comprehension(self):
        chile_ranks = {
            'ghost': 1,
            'habanero': 2,
            'cayenne': 3
        }
        rank_dict = {rank: name for name, rank in chile_ranks.items()}
        self.assertEqual({
            1: 'ghost',
            2: 'habanero',
            3: 'cayenne'
        }, rank_dict)

    def test_set_comprehension(self):
        chile_ranks = {
            'ghost': 1,
            'habanero': 2,
            'cayenne': 3
        }
        len_set = {len(name) for name in chile_ranks.keys()}
        self.assertEqual({8, 5, 7}, len_set)
