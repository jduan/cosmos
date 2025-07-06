"""
Prefer multiple-assignment unpacking over indexing.
Using unpacking wisely will enable you to avoid indexing when possible, resulting in
clearer and more pythonic code.
"""


def unpacking():
    favorite_snacks = {
        "salty": ("pretzels", 100),
        "sweet": ("cookies", 180),
        "veggie": ("carrots", 20),
    }

    ((type1, (name1, cals1)), (type2, (name2, cals2)), (type3, (name3, cals3))) = (
        favorite_snacks.items()
    )

    print(f"Favorite {type1} is {name1} with {cals1} calories")
    print(f"Favorite {type2} is {name1} with {cals2} calories")
    print(f"Favorite {type3} is {name3} with {cals3} calories")

    snacks = [("bacon", 350), ("donut", 240), ("muffin", 190)]
    for rank, (name, cals) in enumerate(snacks, 1):
        print(f"#{rank}: {name} has {cals} calories")


def bubble_sort(a):
    for _ in range(len(a)):
        for i in range(1, len(a)):
            if a[i] < a[i - 1]:
                # unpacking can be used to swap variables!
                a[i - 1], a[i] = a[i], a[i - 1]
            # print(f"after {i} iteration, a: {a}")


if __name__ == "__main__":
    unpacking()
    names = ["pretzels", "carrots", "arugula", "bacon"]
    bubble_sort(names)
    print(names)
