"""
Be defensive when iterating over arguments.

Beware of functions and methods that iterate over input arguments multiple
times. If these arguments are iterators, you might see strange behavior and
missing values.
"""


def normalize(numbers):
    total = sum(numbers)
    percentages = []
    for val in numbers:
        percentages.append(100 * val / total)
    return percentages


visits = [15, 35, 80]
percentages = normalize(visits)
print(percentages)


def read_visits(data_path):
    with open(data_path) as f:
        for line in f:
            yield int(line)


it = read_visits("my_numbers.txt")
percentages2 = normalize(it)
print(percentages2)


def normalize_copy(numbers):
    """same as normalize but it copies the argument so it can be used twice"""
    numbers_copy = list(numbers)
    total = sum(numbers_copy)
    percentages = []
    for val in numbers_copy:
        percentages.append(100 * val / total)
    return percentages


it = read_visits("my_numbers.txt")
percentages3 = normalize_copy(it)
print(percentages3)


def normalize_func(get_iter):
    """This addresses a problem with normalize_copy: copying the iterator could cause
    the program to run out of memory and it defeats the purpose of writing "read_visits"
    as a generator in the first place.

    This function takes a lambda instead and we can call it each time."""
    total = sum(get_iter())
    percentages = []
    for val in get_iter():
        percentages.append(100 * val / total)
    return percentages


percentages4 = normalize_func(lambda: read_visits("my_numbers.txt"))
print(percentages4)


class ReadVisits:
    def __init__(self, data_path) -> None:
        self.data_path = data_path

    def __iter__(self):
        with open(self.data_path) as f:
            for line in f:
                yield (int(line))


visits = ReadVisits("my_numbers.txt")
"""
This works because the sum method in normalize calls ReadVisits.__iter__ to
allocate a new iterator object. The for loop to normalize the numbers also calls
__iter__ to allocate a second iterator object. Each of those iterators will be
advanced and exhausted independently, ensuring that each unique iteration sees
all of the input data values. The only downside of this approach is that it
reads the input data multiple times.  """
percentages5 = normalize(visits)
print(percentages5)
