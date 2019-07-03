from python_sandbox.utils.say_hello import add
import unittest


class TestSayHello(unittest.TestCase):
    def test_add(self):
        result = add(99, 99)
        self.assertEqual(198, result)
