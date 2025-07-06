# Things to Remember
#
# Default arguments are only evaluated once: during function definition at module load time.
# This can cause odd behaviors for mutable values (like {} or []).
#
# Use None as the default value for keyword arguments that have a mutable value. Document the
# actual default behavior in the function’s docstring.
from datetime import datetime
from time import sleep
import json


# The timestamps are the same because datetime.now is only executed a single time: when the function
# is defined. Default argument values are evaluated only once per module load, which usually happens
# when a program starts up. After the module containing this code is loaded, the datetime.now
# default argument will never be evaluated again.
def log(message, when=datetime.now()):
    print("%s: %s" % (when, message))


def log2(message, when=None):
    """
    Log a message with a timestamp. 'when' defaults to the present time.
    """
    when = datetime.now() if when is None else when
    print("%s: %s" % (when, message))


# Using None for default argument values is especially important when the arguments are mutable.
# This function doesn't work because the dictionary specified for default will be shared by all
# calls to decode because default argument values are only evaluated once (at module load time).
def decode(data, default={}):
    try:
        return json.loads(data)
    except ValueError:
        return default


def decode2(data, default=None):
    """
    Load json data from a string. Return a default value of {} if the data is invalid.
    """
    if default is None:
        default = {}
    try:
        return json.loads(data)
    except ValueError:
        return default


def main():
    log("Hi there!")
    sleep(1)
    log("Hi again!")

    log2("Hi there!")
    sleep(1)
    log2("Hi again!")

    foo = decode("bad data")
    foo["stuff"] = 5
    bar = decode("also bad")
    bar["meep"] = 1
    print("Foo: ", foo)
    print("Bar: ", bar)

    foo2 = decode2("bad data")
    foo2["stuff"] = 5
    bar2 = decode2("also bad")
    bar2["meep"] = 1
    print("Foo: ", foo2)
    print("Bar: ", bar2)


if __name__ == "__main__":
    main()
