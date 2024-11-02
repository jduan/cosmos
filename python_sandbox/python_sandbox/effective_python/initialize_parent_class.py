class MyBaseClass:
    def __init__(self, value):
        self.value = value


class MyChildClass(MyBaseClass):
    def __init__(self):
        MyBaseClass.__init__(self, 5)


class TimesTwo:
    def __init__(self):
        self.value *= 2


class PlusFive:
    def __init__(self):
        self.value += 5


class OneWay(MyBaseClass, TimesTwo, PlusFive):
    def __init__(self, value):
        MyBaseClass.__init__(self, value)
        TimesTwo.__init__(self)
        PlusFive.__init__(self)


foo = OneWay(5)
print(f"foo.value: {foo.value}")


class AnotherWay(MyBaseClass, TimesTwo, PlusFive):
    def __init__(self, value):
        MyBaseClass.__init__(self, value)
        PlusFive.__init__(self)
        TimesTwo.__init__(self)


bar = AnotherWay(5)
print(f"bar.value: {bar.value}")


class TimesSeven(MyBaseClass):
    def __init__(self, value):
        super().__init__(value)
        self.value *= 7


class PlusNine(MyBaseClass):
    def __init__(self, value):
        super().__init__(value)
        self.value += 9


class ThisWay(TimesSeven, PlusNine):
    def __init__(self, value):
        super().__init__(value)


baz = ThisWay(5)
print(f"baz.value: {baz.value}")
