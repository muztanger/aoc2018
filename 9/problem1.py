"""
    --- Day 9: Marble Mania ---
    You talk to the Elves while you wait for your navigation system to
    initialize
    . To pass the time, they introduce you to their favorite
    marble
     game.
    The Elves play this game by taking turns arranging the marbles in a
    circle

    https://adventofcode.com/2018/day/9
"""

# import aoc
import os
# import re
# import sys
# from operator import add
# from operator import mul
# from itertools import combinations

# from collections import Counter

debug = True
if debug:
    lines = []
    if os.path.exists('input_debug'):
        with open('input_debug', 'r') as f:
            lines = f.readlines()
else:
    lines = []
    with open('input', 'r') as f:
        lines = f.readlines()
# print(lines)


class Node:

    def __init__(self, name, left, right):
        self.name = name
        self.left = left
        self.right = right

    def __str__(self):
        return str(self.name)

    def __repr__(self):
        return self.__str__()

    def __cmp__(self, other):
        return self.name.__cmp__(other.name)

    def __eq__(self, other):
        return self.name.__eq__(other.name)


class Circle:

    def __init__(self):
        n = Node(0, None, None)
        n.left = n
        n.right = n
        self.current = n
        self.zero = self.current  # Keeping this for print
        self.size = 1

    def __str__(self):
        result = ""
        n = self.zero
        for i in range(self.size):
            if i != 0:
                result += " "
            if n == self.current:
                result += "("
            else:
                result += " "
            result += str(n)
            if n == self.current:
                result += ")"
            elif i != 0:
                result += " "

            n = n.right
        return result

    def step_counter_clockwise(self):
        self.current = self.current.left

    def step_clockwise(self):
        self.current = self.current.right

    def insert(self, name):
        # print("insert({})".format(name))
        for i in range(2):
            self.step_clockwise()
        n = Node(name, self.current.left, self.current)
        # print("{} {} {}".format(self.current.left, self.current, self.current.right))
        self.current.left.right = n
        self.current.left = n
        self.current = n
        # print("{} {} {}".format(self.current.left, self.current, self.current.right))
        self.size += 1

    def remove(self):
        for i in range(7):
            self.step_counter_clockwise()
        score = int(self.current.name)
        self.current.left.right = self.current.right
        self.current.right.left = self.current.left
        self.current = self.current.right
        self.size -= 1
        return score


def main():
    N = 471
    last_marble = 72026
    last_remove_score = -1
    c = Circle()
    elf_score = [0] * N
    elf = -1
    # for i in range(25):
    i = 0
    while c.size > 0 and i < last_marble:
        # print("[{}] {}".format(elf + 1, c))
        elf = (elf + 1) % N
        if (i + 1) % 23 == 0:
            elf_score[elf] += (i + 1)
            remove_score = c.remove()
            last_remove_score = remove_score
            elf_score[elf] += remove_score
            # print("remove_score={}".format(remove_score))
        else:
            c.insert(i + 1)
        i += 1
    print("[{}] {}".format(elf + 1, c))
    # print(elf_score)
    print("{} {}".format(last_remove_score, max(elf_score)))


if __name__ == '__main__':
    main()
