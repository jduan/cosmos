"""
Prevent repetition with assignment expressions, ie the walrus operator.

In general, when you find yourself repeating the same expression or assignment
multiple times within a grouping of lines, it’s time to consider using
assignment expressions in order to improve readability.
"""

fresh_fruit = {
    "apple": 10,
    "banana": 8,
    "lemon": 5,
}


def make_lemonade(count):
    print("making lemonade")
    pass


def make_cider(count):
    print("making cider")
    pass


def out_of_stock():
    print("out of stock")
    pass


if count := fresh_fruit.get("lemon", 0):
    make_lemonade(count)
else:
    print(f"count is {count}")
    out_of_stock()

# Note that you need to surround the assignment expression with parentheses
# because the walrus operator has a lower precedence than >=, ie count will be a boolean.
if (count := fresh_fruit.get("apple", 0)) >= 4:
    make_cider(count)
else:
    print(f"You need at least 4 apples to make cider")
    out_of_stock()


def pick_fruit():
    pass


def make_juice(fruit, count):
    pass


bottles = []
# without the walrus operator, this loop would be less elegant.
while fresh_fruit := pick_fruit():
    for fruit, count in fresh_fruit.items():
        batch = make_juice(fruit, count)
        bottles.append(batch)
