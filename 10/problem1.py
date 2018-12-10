"""
    --- Day 10: The Stars Align ---
    It's no use; your navigation system simply isn't capable of providing
    walking directions
     in the arctic circle, and certainly not in 1018.
    The Elves suggest an alternative. In times like these, North Pole rescue
    operations will arrange points of light in the sky to guide missing Elves back
    to base. Unfortunately, the message is easy to miss: the points move slowly
    enough that it takes hours to align them, but have so much momentum that they
    only stay aligned for a second. If you blink at the wrong time, it might be
    hours before another message appears.
    You can see these points of light floating in the distance, and record their
    position in the sky and their velocity, the relative change in position per
    second (your puzzle input). The coordinates are all given from your perspective;
    given enough time, those positions and velocities will move the points into a
    cohesive message!

    https://adventofcode.com/2018/day/10
"""

# import aoc
import os
# import re
# import sys
# from operator import add
# from operator import mul
# from itertools import combinations

# from collections import Counter
import re
import sys

debug = False
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


class Position:

    def __init__(self, x, y):
        self.x = int(x)
        self.y = int(y)

    def move(self, v):
        self.x += v.vx
        self.y += v.vy

    def min(self, other):
        return Position(min(self.x, other.x), min(self.y, other.y))

    def max(self, other):
        return Position(max(self.x, other.x), max(self.y, other.y))

    def __str__(self):
        return "({},{})".format(self.x, self.y)


class Velocity:

    def __init__(self, vx, vy):
        self.vx = int(vx)
        self.vy = int(vy)

    def __str__(self):
        return "({},{})".format(self.vx, self.vy)


min_x = 1000
min_y = 1000

max_x = -1000
max_y = -1000

positions = []
velocities = []
for line in lines:
    line = line.strip()
    m = re.match(".*< *(-?\d+), *(-?\d+)>[^<]+< *(-?\d+), *(-?\d+)>.*", line)
    pos = Position(m.group(1), m.group(2))
    max_x = max(max_x, pos.x)
    max_y = max(max_y, pos.y)
    min_x = min(min_x, pos.x)
    min_y = min(min_y, pos.y)
    vel = Velocity(m.group(3), m.group(4))
    # print("pos={} vel={}".format(pos, vel))
    positions.append(pos)
    velocities.append(vel)

print("{} {} {} {}".format(min_x, min_y, max_x, max_y))

N = 300
# assert(N > (max_x - min_x))
# assert(N > (max_y - min_y))


min_y = -N//2
min_x = -N//2


class Print:

    printed = 0


def print_area():
    p_min = find_p_min()
    p_max = find_p_max()
    y_n = min(N, p_max.y - p_min.y)
    x_n = min(N, p_max.x - p_min.x)
    for dy in range(y_n + 1):
        line = ""
        y = p_min.y + dy
        for dx in range(x_n + 1):
            x = p_min.x + dx
            found = False
            for pos in positions:
                if pos.x == x and pos.y == y:
                    line += "#"
                    found = True
                    break
            if not found:
                line += "."
        print(line)
    print()


def find_p_min():
    p_min = Position(60000, 60000)
    for pos in positions:
        p_min = p_min.min(pos)
    return p_min


def find_p_max():
    p_max = Position(-60000, -60000)
    for pos in positions:
        p_max = p_max.max(pos)
    return p_max


def area_size():
    p_min = find_p_min()
    p_max = find_p_max()

    # print("{} {}".format(p_min, p_max))
    return (p_max.x - p_min.x) * (p_max.y - p_min.y)


def tick():
    for i in range(len(positions)):
        positions[i].move(velocities[i])


print(area_size())
ticks = 0
last = area_size() + 1
while area_size() > 300 * 300:
    # last = area_size()
    ticks += 1
    if ticks % 1000 == 0:
        print("ticks={}".format(ticks))
    tick()

for i in range(25):
    ticks += 1
    tick()

for i in range(2):
    print_area()  # problem 1
    print(ticks)  # problem 2
    ticks += 1
    tick()


# print("ticks={}".format(ticks))
# print(area_size())
# Print.printed += 1
# # print_area()
