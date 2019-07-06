import unittest


class TestItem10(unittest.TestCase):
    """
    enumerate provides concise syntax for looping over an iterator and getting the index of each
    item from the iterator as you go.
    """
    def test1(self):
        flavor_list = ['vanilla', 'chocolate', 'pecan', 'strawberry']
        mapping = {}
        for i, flavor in enumerate(flavor_list):
            mapping[i+1] = flavor

        expected = {
            1: 'vanilla',
            2: 'chocolate',
            3: 'pecan',
            4: 'strawberry',
        }
        self.assertEqual(expected, mapping)

    def test2(self):
        """
        This is similar to test1. The only difference is that you can pass
        the initial index to enumerate!
        :return:
        :rtype:
        """
        flavor_list = ['vanilla', 'chocolate', 'pecan', 'strawberry']
        mapping = {}
        for i, flavor in enumerate(flavor_list, 1):
            mapping[i] = flavor

        expected = {
            1: 'vanilla',
            2: 'chocolate',
            3: 'pecan',
            4: 'strawberry',
        }
        self.assertEqual(expected, mapping)
