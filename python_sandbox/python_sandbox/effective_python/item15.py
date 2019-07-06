# Things to Remember
#
# Image Closure functions can refer to variables from any of the scopes in which they were defined.
#
# Image By default, closures can’t affect enclosing scopes by assigning variables.
#
# Image In Python 3, use the nonlocal statement to indicate when a closure can modify a variable in
# its enclosing scopes.
#
# Image In Python 2, use a mutable value (like a single-item list) to work around the lack of the
# nonlocal statement.
#
# Image Avoid using nonlocal statements for anything beyond simple functions.


def sort_priority(values, group):
    """
    Given a list of values, sort them by checking if a number is in the priority group.
    """
    def helper(x):
        # Python support closures: function helper can access variables from its scope.
        if x in group:
            # When comparing tuples, python checks the first item, the second item, and so on.
            return (0, x)
        else:
            return (1, x)
    # Functions are first-class objects. That's why you can assign helper to key.
    values.sort(key=helper)


def sort_priority2(values, group):
    """
    Similar to set_priority but it returns a boolean indicating if any higher-priority items
    were seen at all.
    """
    found = False

    def helper(x):
        # The nonlocal statement makes it clear when data is being assigned out of a closure into
        # another scope. It’s complementary to the global statement, which indicates that a
        # variable’s assignment should go directly into the module scope.
        nonlocal found
        if x in group:
            found = True
            return (0, x)
        else:
            return (1, x)
    values.sort(key=helper)
    return found


# When your usage of nonlocal starts getting complicated, it’s better to wrap your state in a
# helper class.
class Sorter(object):
    def __init__(self, group):
        self.group = group
        self.found = False

    def __call__(self, x):
        if x in self.group:
            self.found = True
            return (0, x)
        else:
            return (1, x)


def sort_priority3(values, group):
    sorter = Sorter(group)
    values.sort(key=sorter)
    return sorter.found
