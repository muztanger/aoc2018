"""
    --- Day 6: Chronal Coordinates ---
    The device on your wrist beeps several times, and once again you feel like you're falling.
    

    "
    Situation critical
    ," the device announces. "Destination indeterminate. Chronal interference detected. Please specify new target coordinates."
    

    The device then produces a list of coordinates (your puzzle input). Are they places it thinks are safe or dangerous? It recommends you check manual page 729. The Elves did not give you a manual.
    


    https://adventofcode.com/2018/day/6
"""

# import aoc
import collections
import itertools
import os
# import re
# import sys
# from operator import add
# from operator import mul
# from itertools import combinations

# from collections import Counter


class Point:

    def __init__(self, number, x, y):
        self.number = int(number)
        self.x = int(x)
        self.y = int(y)

    def dist(self, x, y):
        return abs(self.x - x) + abs(self.y - y)

    # def dist(self, other):
    #     return abs(self.x - other.x) + abs(self.y - other.y)


    def __str__(self):
        return "{}:({},{})".format(self.number, self.x, self.y)

debug = False
if debug:
    N = 10
    lines = ["1, 1",
             "1, 6",
             "8, 3",
             "3, 4",
             "5, 5",
             "8, 9"]
    if os.path.exists('input_debug'):
        with open('input_debug', 'r') as f:
            lines = f.readlines()
else:
    N = 1000
    lines = []
    with open('input', 'r') as f:
        lines = f.readlines()


area = []
for a in range(N):
    area.append([0] * N)

def print_area():
    for y in range(N):
        l = ""
        for x in range(N):
            z = area[y][x]
            if z == 0:
                l += "."
            elif z < 0:
                l += chr(-z + ord('A') - 1)
            else:
                l += chr(z + ord('a') - 1)
        print(l)

points_dict = {}
number = 1
points = []
for line in lines:
    arr = line.strip().split(', ')
    p = Point(number, arr[0], arr[1])
    number += 1
    # print(p)
    points.append(p)
    points_dict[number] = p

# print_area()
for y in range(N):
    for x in range(N):
        # min_dist = None
        # min_dist_i = None
        # tie = False
        total = 0
        for p in points:
            # if not min_dist or p.dist(x, y) <= min_dist:
            #     if p.dist(x, y) == min_dist:
            #         tie = True
            #     else:
            #         tie = False
            #     min_dist = p.dist(x, y)
            #     min_dist_p = p.number
            total += p.dist(x, y)
        if total < 10000:
            area[y][x] = 1
# for p in points:
#     area[p.y][p.x] = -p.number

total = 0
for x in area:
    total += sum(x)
print(total)
# print_area()

# count = collections.Counter([item for sublist in area for item in sublist])
# exclude = set()
# for z in range(N):
#     exclude.add(area[0][z])
#     exclude.add(area[z][0])
#     exclude.add(area[N-1][z])
#     exclude.add(area[z][N-1])
#
# ans = -1
# for c in count:
#     if c not in exclude:
#         ans = max(ans, count[c])
# print(ans+1)
# print_area()
