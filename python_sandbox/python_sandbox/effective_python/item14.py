# Functions that return None to indicate special meaning are error prone because None and other
# values (e.g., zero, the empty string) all evaluate to False in conditional expressions.
# You can return a tuple (success, result) and the caller can check if success is True
def divide(a, b):
    try:
        return True, a / b
    except ZeroDivisionError:
        return False, None


# Raise exceptions to indicate special situations instead of returning None. Expect the calling
# code to handle exceptions properly when they’re documented.
def divide2(a, b):
    try:
        return a / b
    except ZeroDivisionError as e:
        raise ValueError("Invalid inputs") from e
