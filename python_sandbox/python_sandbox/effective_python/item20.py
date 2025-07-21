"""
Never use for loop variables after the loop ends
"""

for i in range(3):
    print(f"Inside loop {i=}")
# i still exists after the loop ends
print(f"Outside loop {i=}")

categories = []
for j, name in enumerate(categories):
    if name == "Lithium":
        break
try:
    # j isn't defined because the loop above never started
    print(j)
except NameError:
    pass

# Fortunately, this loop variable leakage behavior is not exhibited by
# list comprehensions or generator expressions
nums = [37, 13, 128, 21]
found = [k for k in nums if k % 2 == 0]
print(found)
try:
    # k doesn't exist
    print(k)
except NameError as ex:
    print(ex)
    pass

# Additionally, exception handlers do not leak exception instance variables.
print(ex)
