# Things to Remember
#
# Function arguments can be specified by position or by keyword.
#
# Keywords make it clear what the purpose of each argument is when it would be confusing with
# only positional arguments.
#
# Keyword arguments with default values make it easy to add new behaviors to a function,
# especially when the function has existing callers.
#
# Optional keyword arguments should always be passed by keyword instead of by position.


# All positional arguments to Python functions can also be passed by keyword, where the name of the
# argument is used in an assignment within the parentheses of a function call. The keyword arguments
# can be passed in any order as long as all of the required positional arguments are specified. You
# can mix and match keyword and positional arguments.


def remainder(number, divisor):
    return number % divisor


# Keyword arguments provide a powerful way to extend a function’s parameters while remaining
# backwards compatible with existing callers. This lets you provide additional functionality without
# having to migrate a lot of code, reducing the chance of introducing bugs.
# For example, when you add a 4th argument "units_per_ke" with a default value, you don't need to
# migrate existing callers.
def flow_rate(weight_diff, time_diff, period=1, units_per_kg=1):
    return ((weight_diff / units_per_kg) / time_diff) * period


def main():
    print(remainder(20, 7))
    print(remainder(20, divisor=7))
    print(remainder(number=20, divisor=7))
    print(remainder(divisor=7, number=20))

    # Positional arguments must be specified before keyword arguments.
    # You can't do this:
    # print(remainder(number=20, 7))

    print("Flow rate per second: %s" % flow_rate(1000, 100))
    print("Flow rate per hour: %s" % flow_rate(1000, 100, period=3600))
    print("Flow rate per hour different units: %s" %
          flow_rate(1000, 100, period=3600, units_per_kg=2.2))


if __name__ == '__main__':
    main()
