# Things to Remember
#
# Image Keyword arguments make the intention of a function call more clear.
#
# Image Use keyword-only arguments to force callers to supply keyword arguments for potentially
# confusing functions, especially those that accept multiple Boolean flags.
#
# Image Python 3 supports explicit syntax for keyword-only arguments in functions.
#
# Image Python 2 can emulate keyword-only arguments for functions by using **kwargs and manually
# raising TypeError exceptions.


def safe_division(number, divisor, ignore_overflow=False, ignore_zero_division=False):
    try:
        return number / divisor
    except OverflowError:
        if ignore_overflow:
            return 0
        else:
            raise
    except ZeroDivisionError:
        if ignore_zero_division:
            return float('inf')
        else:
            raise


# In python 3, you can define keyword-only arguments. These arguments can only be
# supplied by keyword, never by position.
# The * symbol indicates the end of positional arguments and the beginning of
# keyword-only arguments.
def safe_division2(number, divisor, *, ignore_overflow=False, ignore_zero_division=False):
    try:
        return number / divisor
    except OverflowError:
        if ignore_overflow:
            return 0
        else:
            raise
    except ZeroDivisionError:
        if ignore_zero_division:
            return float('inf')
        else:
            raise


def main():
    print("ignore overflow: %s" % safe_division(10**500, 1, ignore_overflow=True))
    print("ignore overflow: %s" % safe_division(10**500, 1, True))
    print("ignore overflow: %s" % safe_division(10**500, 1, True, False))
    print("ignore zero division: %s" % safe_division(1, 0, ignore_zero_division=True))

    # these won't work
    # print("ignore overflow: %s" % safe_division2(10**500, 1, True))
    # print("ignore overflow: %s" % safe_division2(10**500, 1, True, False))
    print("ignore overflow: %s" % safe_division2(10**500, 1, ignore_overflow=True))
    print("ignore zero division: %s" % safe_division2(1, 0, ignore_zero_division=True))



if __name__ == '__main__':
    main()
