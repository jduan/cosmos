"""
Avoid else blocks after for and while loops

The expressivity you gain from the else block doesn't outweigh the burden you
put on people (including yourself) who want to understand your code in the
future. Simple constructs like loops should be self-evident in Python. You
should avoid using else blocks after loops entirely.
"""

for i in range(3):
    print("loop", i)
else:
    print("Else block!")

# You may assume the else part means "do this if the loop wasn't complete".
# But it does exactly the opposite. Using a break statement in a loop actually skips the else block.
for i in range(3):
    print("loop", i)
    if i == 1:
        break
else:
    print("Else block!")

# Another surprise is that the else block runs immediately if you loop over an empty sequence.
for x in []:
    print("Never runs")
else:
    print("For else block!")

# Else blocks after loops are useful when you're searching for something.
# Else blocks are skipped if you break out of loops.
a = 4
b = 9
for i in range(2, min(a, b) + 1):
    print("Testing", i)
    if a % i == 0 and b % i == 0:
        print("Not coprime")
        break
else:
    print("Coprime")


# this is better code
def coprime(a, b):
    for i in range(2, min(a, b) + 1):
        if a % i == 0 and b % i == 0:
            return False
    return True


# another way
def coprime2(a, b):
    is_coprime = True
    for i in range(2, min(a, b) + 1):
        if a % i == 0 and b % i == 0:
            is_coprime = False
            break
    return is_coprime
