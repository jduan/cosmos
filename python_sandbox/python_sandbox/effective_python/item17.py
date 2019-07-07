def normalize(numbers):
    """
    This function doesn't work for generators because the 2nd loop of the generator wouldn't
    do anything because it has been exhausted by the first loop!
    """
    total = sum(numbers)
    return [100 * value / total for value in numbers]


def normalize_copy(numbers):
    """
    This function works for generators by converting the generator to a list first.
    """
    numbers = list(numbers)
    total = sum(numbers)
    return [100 * value / total for value in numbers]


def normalize_func(get_iter):
    """
    get_iter is a function that returns an iterator so it doesn't have the problem of generators
    getting exhausted.
    """
    total = sum(get_iter())
    return [100 * value / total for value in get_iter()]


# The iterator protocol is how Python for loops and related expressions traverse the contents of a
# container type. When Python sees a statement like for x in foo it will actually call iter(foo).
# The iter built-in function calls the foo.__iter__ special method in turn. The __iter__ method must
# return an iterator object (which itself implements the __next__ special method). Then the for loop
# repeatedly calls the next built-in function on the iterator object until it’s exhausted (and
# raises a StopIteration exception).
#
# It sounds complicated, but practically speaking you can achieve all of this behavior for your
# classes by implementing the __iter__ method as a generator.


class ReadVisit(object):
    """
    A class that implements the "iterator protocol". You can use it in a for loop. The important
    thing is you can iterate it many times. The iterators returned are advanced and exhausted
    independently. For example, you can use it in the "normalize" function.

    The only downsize of this approach is that it reads the input data multiple times!
    """
    def __init__(self, data_path):
        self.data_path = data_path

    def __iter__(self):
        with open(self.data_path) as fd:
            for line in fd:
                yield int(line)
