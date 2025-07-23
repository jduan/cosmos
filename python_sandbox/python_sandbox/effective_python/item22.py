"""
Never modify containers while iterating over them; use copies or caches instead.
"""

# if you add a new item to a dict while iterating over it, python will raise a runtime exception
search_key = "red"
my_dict = {"red": 1, "blue": 2, "green": 3}
# for key in my_dict:
#     if key == "blue":
#         my_dict["yellow"] = 4

# same thing happens if you delete an item while iterating over it
# for key in my_dict:
#     if key == "blue":
#         del my_dict["green"]

# but it's ok to update an item while iterating it
for key in my_dict:
    if key == "blue":
        my_dict["green"] = 4
print(my_dict)

# set is slightly different; if you add an item that already exists in the set, it won't
# cause any problems while iterating over it
my_set = {"red", "blue", "green"}
for color in my_set:
    if color == "blue":
        my_set.add("green")

# list is similar to dict; if update an existing index, it's ok
my_list = [1, 2, 3]
for number in my_list:
    print(number)
    if number == 2:
        my_list[0] = -1
print(f"my_list: {my_list}")

# if you try to insert an item before the current iterator position, your code will
# get stuck in an infinite loop
my_list = [1, 2, 3]
for number in my_list:
    print(number)
    # if number == 2:
    #     my_list.insert(0, 4)

# but appending to the list or inserting an item after the current iterator position is ok!
my_list = [1, 2, 3]
for number in my_list:
    print(number)
    if number == 2:
        # both of these are fine
        # my_list.append(4)
        my_list.insert(2, 5)
print(f"my_list: {my_list}")

"""
Looking at each of the examples above, it can be hard to guess whether the code
will work in all cases. Modifying containers during iteration can be especially
error prone in situations where the modification point changes based on input to
the algorithm. In some cases it'll work, and in others there will be an error.
Thus, my advice is to never modify containers while you iterate over them.
"""

# if you really want to make modifications during iteration due to the nature
# of your algorithm, you can make a copy of the container and iterate the copy.
# the same works well for lists and sets!
my_dict = {"red": 1, "blue": 2, "green": 3}
keys_copy = list(my_dict.keys())
for key in keys_copy:
    if key == "blue":
        my_dict["green"] = 4
print(f"my_dict: {my_dict}")

# for some extremely large containers, copying might be too slow. One way to deal
# with poor performance is to stage modifications in a separate container and merge
# the changes back to the main data structure after iteration.
my_dict = {"red": 1, "blue": 2, "green": 3}
modifications = {}
for key in my_dict:
    if key == "blue":
        modifications["green"] = 4
my_dict.update(modifications)
print(f"my_dict: {my_dict}")
