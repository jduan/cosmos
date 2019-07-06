import unittest


class TestItem8(unittest.TestCase):
    def test1(self):
        matrix = [
            [1, 2, 3],
            [4, 5, 6],
            [7, 8, 9],
        ]
        # flat a matrix
        flat = [x for row in matrix for x in row]
        self.assertEqual([1, 2, 3, 4, 5, 6, 7, 8, 9], flat)

    def test2(self):
        matrix = [
            [1, 2, 3],
            [4, 5, 6],
            [7, 8, 9],
        ]
        # flat a matrix
        squared = [[x**2 for x in row] for row in matrix]
        expected = [
            [1, 4, 9],
            [16, 25, 36],
            [49, 64, 81],
        ]
        self.assertEqual(expected, squared)

    def test3(self):
        my_lists = [
            [[1, 2, 3], [4, 5, 6]],
            [[7, 8, 9], [10, 11, 12]]
        ]
        # pretty hard to read
        flat = [x for sublist1 in my_lists
                for sublist2 in sublist1
                for x in sublist2]
        expected = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
        self.assertEqual(expected, flat)

        # easier to read
        flat2 = []
        for sublist1 in my_lists:
            for sublist2 in sublist1:
                flat2.extend(sublist2)
        self.assertEqual(expected, flat2)

    def test4(self):
        a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
        # filters
        b = [x for x in a if x > 4 and x % 2 == 0]
        self.assertEqual([6, 8, 10], b)

        matrix = [
            [1, 2, 3],
            [4, 5, 6],
            [7, 8, 9],
        ]
        # You can filter at each level but it can get complicated fast!
        filtered = [[x for x in row if x % 3 == 0]
                    for row in matrix if sum(row) >= 10]
        expected = [
            [6],
            [9]
        ]
        self.assertEqual(expected, filtered)
