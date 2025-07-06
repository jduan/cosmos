# # Things to Remember
#
# Functions can accept a variable number of positional arguments by using *args in the def
# statement.
#
# You can use the items from a sequence as the positional arguments for a function with the *
# operator.
#
# Using the * operator with a generator may cause your program to run out of memory and crash.
#
# Adding new positional parameters to functions that accept *args can introduce hard-to-find
# bugs.


def log(message, *values):
    # The type of values is a tuple.
    # print(type(values))
    if values:
        values_str = ", ".join(str(x) for x in values)
        print("%s: %s" % (message, values_str))
    else:
        print(message)


# One problem with *args is that you can’t add new positional arguments to your function in the
# future without migrating every caller. If you try to add a positional argument in the front of the
# argument list, existing callers will subtly break if they aren’t updated.
#
# To avoid this possibility entirely, you should use keyword-only arguments when you want to extend
# functions that accept *args
def log2(sequence, message, *values):
    if values:
        values_str = ", ".join(str(x) for x in values)
        print("%s: %s: %s" % (sequence, message, values_str))
    else:
        print("%s: %s" % (sequence, message))


# arguments after varargs are keyword-only arguments!
def log3(message, *values, sequence):
    if values:
        values_str = ", ".join(str(x) for x in values)
        print("%s: %s: %s" % (sequence, message, values_str))
    else:
        print("%s: %s" % (sequence, message))


def main():
    log("My numbers are", 1, 2)
    log("Hi there")
    favorites = [7, 33, 99]
    # This is called the * or splat operator which unpacks a list and passes them
    # to the log function as var args.
    log("Favorite colors", *favorites)

    # this works
    log2(1, "Favorites", 7, 33)
    # old usage breaks silently because the sequence will be 'Favorite numbers'
    log2("Favorite numbers", 7, 33)

    # leverage keyword-only arguments
    log3("Favorites", 7, 33, sequence=1)
    log3("Favorite numbers", 7, 33, sequence=2)


if __name__ == "__main__":
    main()
