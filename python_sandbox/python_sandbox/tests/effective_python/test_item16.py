import unittest
from python_sandbox.effective_python.item16 import index_words, index_words2


class TestItem16(unittest.TestCase):
    def test1(self):
        address = "Four score and seven years ago..."
        result = index_words(address)
        self.assertEqual([0, 5, 11, 15, 21, 27], result)

    def test2(self):
        address = "Four score and seven years ago..."
        result = index_words2(address)
        self.assertEqual([0, 5, 11, 15, 21, 27], list(result))
