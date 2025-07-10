"""
Consider "match" for destructuring in flow control;
avoid when if statements are sufficient
"""

import enum


def take_action(light):
    if light == "red":
        print("stop")
    elif light == "yellow":
        print("slow down")
    elif light == "green":
        print("go!")
    else:
        raise RuntimeError(f"unknown light: {light}")


take_action("red")
take_action("yellow")
take_action("green")


def take_match_action(light):
    match light:
        case "red":
            print("stop")
        case "yellow":
            print("slow down")
        case "green":
            print("go!")
        case _:
            raise RuntimeError(f"unknown light: {light}")


take_match_action("red")
take_match_action("yellow")
take_match_action("green")

RED = "red"
YELLOW = "yellow"
GREEN = "green"


# this works!
class ColorEnum(enum.Enum):
    RED = "red"
    YELLOW = "yellow"
    GREEN = "green"


def take_enum_action(light):
    match light:
        case ColorEnum.RED:
            print("stop")
        case ColorEnum.YELLOW:
            print("slow down")
        case ColorEnum.GREEN:
            print("go!")
        case _:
            raise RuntimeError(f"unknown light: {light}")


take_enum_action(ColorEnum.RED)
take_enum_action(ColorEnum.YELLOW)
take_enum_action(ColorEnum.GREEN)

# this doesn't work as expected because:
# SyntaxError: name capture 'RED' makes remaining patterns unreachable
# def take_constant_action(light):
#     match light:
#         case RED:
#             print("stop")
#         case YELLOW:
#             print("slow down")
#         case GREEN:
#             print("go!")
#         case _:
#             raise RuntimeError(f"unknown light: {light}")


"""
Match is for destructuring in flow control
"""

# this represents a binary search tree
# (value, left child, right child)
my_tree = (10, (7, None, 9), (13, 11, None))


def contains(tree, value):
    if not isinstance(tree, tuple):
        return tree == value
    pivot, left, right = tree
    if value < pivot:
        return contains(left, value)
    elif value > pivot:
        return contains(right, value)
    else:
        return value == pivot


print(f"tree contains 10? {contains(my_tree, 10)}")
print(f"tree contains 7? {contains(my_tree, 7)}")
print(f"tree contains 9? {contains(my_tree, 9)}")
print(f"tree contains 13? {contains(my_tree, 13)}")
print(f"tree contains 11? {contains(my_tree, 11)}")
print(f"tree contains 20? {contains(my_tree, 20)}")
print(f"tree contains 6? {contains(my_tree, 6)}")


def contains_match(tree, value):
    match tree:
        # the "if" here is called the "guard expression"
        case pivot, left, _ if value < pivot:
            return contains_match(left, value)
        case pivot, _, right if value > pivot:
            return contains_match(right, value)
        # | means it matches either pattern
        case (pivot, _, _) | pivot:
            return pivot == value


# my version
def contains_match2(tree, value):
    match tree:
        case None:
            return False
        case pivot, left, right:
            if pivot == value:
                return True
            elif value < pivot:
                return contains_match2(left, value)
            elif value > pivot:
                return contains_match2(right, value)
            else:
                return False
        case pivot:
            return pivot == value


assert contains_match(my_tree, 10)
assert contains_match(my_tree, 7)
assert contains_match(my_tree, 9)
assert contains_match(my_tree, 13)
assert contains_match(my_tree, 11)
assert not contains_match(my_tree, 20)
assert not contains_match(my_tree, 6)

assert contains_match2(my_tree, 10)
assert contains_match2(my_tree, 7)
assert contains_match2(my_tree, 9)
assert contains_match2(my_tree, 13)
assert contains_match2(my_tree, 11)
assert not contains_match2(my_tree, 20)
assert not contains_match2(my_tree, 6)


class Node:
    def __init__(self, value, left=None, right=None):
        self.value = value
        self.left = left
        self.right = right


obj_tree = Node(
    value=10,
    left=Node(value=7, right=9),
    right=Node(value=13, left=11),
)


def contains_class(tree, value):
    if not isinstance(tree, Node):
        return tree == value

    pivot = tree.value
    if value < pivot:
        return contains_class(tree.left, value)
    elif value > pivot:
        return contains_class(tree.right, value)
    else:
        return value == pivot


assert contains_class(obj_tree, 10)
assert contains_class(obj_tree, 7)
assert contains_class(obj_tree, 9)
assert contains_class(obj_tree, 13)
assert contains_class(obj_tree, 11)
assert not contains_class(obj_tree, 20)
assert not contains_class(obj_tree, 6)


def contains_class_match(tree, value):
    match tree:
        # this is how you extract the object's attributes (using name matches)
        # you can't do:
        # case Node(pivot, left, _) if value < pivot:
        case Node(value=pivot, left=left) if value < pivot:
            return contains_class_match(left, value)
        case Node(value=pivot, right=right) if value > pivot:
            return contains_class_match(right, value)
        case Node(value=pivot) | pivot:
            return pivot == value


assert contains_class_match(obj_tree, 10)
assert contains_class_match(obj_tree, 7)
assert contains_class_match(obj_tree, 9)
assert contains_class_match(obj_tree, 13)
assert contains_class_match(obj_tree, 11)
assert not contains_class_match(obj_tree, 20)
assert not contains_class_match(obj_tree, 6)
