"""
Prefer catch-all unpacking over slicing

1. Unpacking assignments may include a starred expression to store all values
that weren’t assigned to the other parts of the unpacking pattern in a list.

2. Starred expressions may appear in any position of the unpacking pattern.
They will always become a list instance containing zero or more values.

3. When dividing a list into non-overlapping pieces, catch-all unpacking is
much less error prone than using separate statements that do slicing and
indexing.

"""

car_ages = [0, 9, 4, 8, 7, 20, 19, 1, 6, 15]
car_ages_descending = sorted(car_ages, reverse=True)
# ValueError: too many values to unpack
# oldest, second_oldest = car_ages_descending

# solution 1
oldest = car_ages_descending[0]
second_oldest = car_ages_descending[1]
others = car_ages_descending[2:]
print(oldest, second_oldest, others)

# solution 2: catch-all unpacking
oldest, second_oldest, *others = car_ages_descending
print(oldest, second_oldest, others)
# "star expressions" may appear in any position: start, middle, or end
oldest, *others, youngest = car_ages_descending
print(oldest, youngest, others)

*others, second_youngest, youngest = car_ages_descending
print(youngest, second_youngest, others)

# unpacking can also be used in nested structures
car_inventory = {
    "Downtown": ("Silver Shadow", "Pinto", "DMC"),
    "Airport": ("Skyline", "Viper", "Gremlin", "Nova"),
}
((loc1, (best1, *rest1)), (loc2, (best2, *rest2))) = car_inventory.items()
print(f"Best at {loc1} is {best1}, {len(rest1)} others")
print(f"Best at {loc2} is {best2}, {len(rest2)} others")

# you can unpack an iterator
it = iter(range(1, 3))
first, second = it
print(f"first: {first}, second: {second}")


def generate_csv():
    """Read a CSV file and generate its header and rows"""
    yield ("Date", "Make", "Model", "Year", "Price")
    yield ("2025", "Honda", "Civic", "2025", 35_000)
    yield ("2025", "Honda", "Civic", "2025", 35_000)
    yield ("2025", "Honda", "Civic", "2025", 35_000)


# you can unpack a generator
# warning: a star expression is always turned into a list, unpacking an iterator
# risks using up all the memory on your computer!
it = generate_csv()
header, *rows = it
print(f"CSV Header: {header}")
print(f"Row count: {len(rows)}")
