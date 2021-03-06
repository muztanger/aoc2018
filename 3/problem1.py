"""
    --- Day 3: No Matter How You Slice It ---
    The Elves managed to locate the chimney-squeeze prototype fabric for Santa's suit (thanks to 
    someone
     who helpfully wrote its box IDs on the wall of the warehouse in the middle of the night).  Unfortunately, anomalies are still affecting them - nobody can even agree on how to 
    cut
     the fabric.
    

    The whole piece of fabric they're working on is a very large square - at least 
    1000 inches on each side.

Each Elf has made a claim about which area of fabric would be ideal for Santa's suit. All claims have an ID and consist of a single rectangle with edges parallel to the edges of the fabric. Each claim's rectangle is defined as follows:

The number of inches between the left edge of the fabric and the left edge of the rectangle.
The number of inches between the top edge of the fabric and the top edge of the rectangle.
The width of the rectangle in inches.
The height of the rectangle in inches.

    https://adventofcode.com/2018/day/3
"""

# import aoc
import os
import re
# import sys
# from operator import add
# from operator import mul
# from itertools import combinations

# from collections import Counter


class Claim(object):

    def __init__(self, identity, x, y, w, h):
        self.identity = int(identity)
        self.x = int(x)
        self.y = int(y)
        self.w = int(w)
        self.h = int(h)

    def __str__(self):
        return "#{} @ {},{} {}x{}".format(self.identity, self.x, self.y, self.w, self.h)


class Fabric(object):
    width = 1000
    height = 1000

    def __init__(self):
        self.fabric = []
        for y in range(Fabric.height):
            self.fabric.append([0] * Fabric.width)

    def print(self):
        for y in range(10):
            for x in range(10):
                print(self.fabric[y][x], end=' ')
            print()

    def add_claim(self, claim):
        for dy in range(claim.h):
            for dx in range(claim.w):
                # print("{} {}".format(claim.y + dy, claim.x + dx))
                self.fabric[claim.y + dy][claim.x + dx] += 1
                # self.print()

    def count_two_or_more(self):
        count = 0
        for y in range(Fabric.height):
            for x in range(Fabric.width):
                if self.fabric[y][x] >= 2:
                    count += 1
        return count


debug = False
if debug:
    lines = ["#1 @ 1,3: 4x4",
             "#2 @ 3,1: 4x4",
             "#3 @ 5,5: 2x2"]
    if os.path.exists('input_debug'):
        with open('input_debug', 'r') as f:
            lines = f.readlines()
else:
    lines = []
    with open('input', 'r') as f:
        lines = f.readlines()

f = Fabric()

for line in lines:
    m = re.match("#(\d+) @ (\d+),(\d+): (\d+)x(\d+)", line.strip())
    c = Claim(m.group(1), m.group(2), m.group(3), m.group(4), m.group(5))
    print(c)
    f.add_claim(c)
    # f.print()

print(f.count_two_or_more())
# 6000 is too low