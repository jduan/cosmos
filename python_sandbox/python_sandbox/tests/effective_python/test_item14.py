import unittest
from python_sandbox.effective_python.item14 import (divide, divide2)


class TestItem14(unittest.TestCase):
    def test1(self):
        success, result = divide(1, 3)
        self.assertTrue(success)
        self.assertEqual(1 / 3, result)

    def test2(self):
        success, result = divide(1, 0)
        self.assertFalse(success)
        self.assertIsNone(result)

    def test3(self):
        try:
            result = divide2(5, 2)
        except ValueError:
            raise RuntimeError('This should not happen')
        else:
            self.assertEqual(2.5, result)
