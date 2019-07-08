from pprint import pprint
import unittest
from python_sandbox.effective_python.item25 import GoodWay


class TestItem25(unittest.TestCase):
    def test1(self):
        foo = GoodWay(5)
        print("Method Resolution Order (MRO)")
        pprint(GoodWay.mro())
        self.assertEqual(35, foo.value)
