# Things to Remember
#
# Python’s standard method resolution order (MRO) solves the problems of superclass
# initialization order and diamond inheritance.
#
# Always use the "super" built-in function to initialize parent classes.


class MyBaseClass(object):
    def __init__(self, value):
        self.value = value


class TimesFive(MyBaseClass):
    def __init__(self, value):
        # this is equivalent to python's syntax:
        # super(TimesFive, self).__init__(value)
        # the python3's way works because python3 lets you reliabily reference the current
        # class in methods using the __class__ variable.
        print("__class__", __class__)
        super().__init__(value)
        self.value *= 5


class PlusTwo(MyBaseClass):
    def __init__(self, value):
        super().__init__(value)
        self.value += 2


class GoodWay(TimesFive, PlusTwo):
    def __init__(self, value):
        super().__init__(value)
