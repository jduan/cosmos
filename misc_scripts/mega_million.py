#!/usr/bin/env python3
# print a random number for mega million

from random import randint

def play():
    print("white balls")
    for i in range(0, 5):
        print(randint(1, 70))
    print("mega ball")
    print(randint(1, 25))

play()
