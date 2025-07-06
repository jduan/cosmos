def log_missing():
    print("Key added")
    return 0


class CountMissing(object):
    """
    A stateful object that tracks the number of missing keys.
    """

    def __init__(self):
        self.missing_keys = 0

    def missing(self):
        self.missing_keys += 1
        return 0


class BetterCountMissing(object):
    """
    This is even cleaner. The __call__ method indicates that a class’s instances will be used
    somewhere a function argument would also be suitable (like API hooks). It directs new readers of
    the code to the entry point that’s responsible for the class’s primary behavior. It provides a
    strong hint that the goal of the class is to act as a stateful closure.
    """

    def __init__(self):
        self.missing_keys = 0

    def __call__(self, *args, **kwargs):
        self.missing_keys += 1
        return 0
