# Consider generators instead of returning lists.
# Generators are functions that use yield expressions. When called, generator functions do not
# actually run but instead immediately return an iterator. With each call to the next built-in
# function, the iterator will advance the generator to its next yield expression. Each value
# passed to yield by the generator will be returned by the iterator to the caller.


# Things to Remember
#
# Using generators can be clearer than the alternative of returning lists of accumulated
# results.
#
# The iterator returned by a generator produces the set of values passed to yield expressions
# within the generator function’s body.
#
# Generators can produce a sequence of outputs for arbitrarily large inputs because their
# working memory doesn't include all inputs and outputs.


def index_words(text):
    """
    Return the index of every word in a string.
    """
    result = []
    if text:
        result.append(0)
    for index, letter in enumerate(text):
        if letter == " ":
            result.append(index + 1)
    return result


def index_words2(text):
    """
    Similar to index_words but returns a generator.
    """
    if text:
        yield 0
    for index, letter in enumerate(text):
        if letter == " ":
            yield index + 1
