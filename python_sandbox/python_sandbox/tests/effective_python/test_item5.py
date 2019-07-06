import unittest


class TestItem5(unittest.TestCase):
    def test_slicing(self):
        a = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h']
        self.assertEqual(['a', 'b', 'c', 'd'], a[:4])
        self.assertEqual(['e', 'f', 'g', 'h'], a[-4:])
        self.assertEqual(['d', 'e'], a[3:-3])
        self.assertEqual(['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'], a[:])
        self.assertEqual(['d', 'e', 'f', 'g', 'h'], a[-5:])
        self.assertEqual(['a', 'b', 'c', 'd', 'e', 'f', 'g'], a[:-1])
        self.assertEqual(['e', 'f', 'g', 'h'], a[4:])
        self.assertEqual(['f', 'g', 'h'], a[-3:])
        self.assertEqual(['c', 'd', 'e'], a[2:5])
        self.assertEqual(['c', 'd', 'e', 'f', 'g'], a[2:-1])
        self.assertEqual(['f', 'g'], a[-3:-1])

    def test_slicing2(self):
        a = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h']
        # beyond boundaries
        self.assertEqual(['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'], a[:20])
        self.assertEqual(['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'], a[-20:])
        # in contrast, accessing indexes that are out of boundaries would cause an exception
        try:
            a[20]
            raise RuntimeError("Should've got an IndexError")
        except IndexError as ex:
            pass

    def test_slicing3(self):
        """
        When you slice a list, the returned list maintains the references to the objects
        from the original list. If those objects are immutable, changes to them will
        reflect in both lists!
        """
        names = ["John", "Joe", "Jack", "Jake"]
        names2 = names[2:]
        names2[0] = "Tom"
        self.assertEqual(names2[0], "Tom")
        # Changing names2 doesn't affect names
        self.assertEqual(names, ["John", "Joe", "Jack", "Jake"])

        numbers = [[1, 2], [3, 4]]
        numbers2 = numbers[1:]
        # Changing numbers2 this way does affect numbers!
        numbers2[0].append(5)
        self.assertEqual(numbers, [[1, 2], [3, 4, 5]])

    def test_slicing4(self):
        """
        When you assign slices, the list will grow and shrink to accommodate the new values.
        :return:
        :rtype:
        """
        a = [1, 2, 3, 4]
        # expand
        a[3:] = [4, 5, 6]
        self.assertEqual([1, 2, 3, 4, 5, 6], a)
        # shrink
        a[3:] = [4]
        self.assertEqual([1, 2, 3, 4], a)

    def test_slicing5(self):
        # [:] returns a copy of the original list
        a = [1, 2, 3, 4]
        b = a[:]
        self.assertEqual([1, 2, 3, 4], b)

        # assigning [:] would replace its entire contents
        a[:] = [5, 6, 7, 8]
        self.assertEqual([5, 6, 7, 8], a)
