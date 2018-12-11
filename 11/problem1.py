"""
    --- Day 11: Chronal Charge ---
    You watch the Elves and their sleigh fade into the distance as they head toward
    the North Pole.
    Actually, you're the one fading. The
    falling sensation
     returns.
    The low fuel warning light is illuminated on your wrist-mounted device. Tapping
    it once causes it to project a hologram of the situation: a
    300x300

    https://adventofcode.com/2018/day/11
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
    # lines = [[3,5,8],[122,79,57], [217,196,39], [101,153,71]]
    lines = [[33, 45, 18], [21, 61, 42]]
    if os.path.exists('input_debug'):
        with open('input_debug', 'r') as f:
            lines = f.readlines()
else:
    lines = []
    with open('input', 'r') as f:
        lines = f.readlines()
# print(lines)
N = 300


def power_level(x, y, serial):
    # print("power_level({},{},{})".format(x, y, serial))
    rack_id = x + 10
    # print("  {}".format(rack_id))
    ans = rack_id * y
    # print("  {}".format(ans))
    ans += serial
    # print("  {}".format(ans))
    ans *= rack_id
    # print("  {}".format(ans))
    ans %= 1000
    # print("  {}".format(ans))
    ans //= 100
    # print("  {}".format(ans))
    ans -= 5
    # print("  {}".format(ans))
    return ans


grid = []


# for x, y, serial in lines:
#     print(power_level(x, y, serial))
def print_grid():
    for j in range(N):
        line = ""
        for i in range(N):
            line += "{:3d} ".format(grid[j][i])
        print(line)


def print_pos(x, y):
    for dj in range(5):
        j = y - 2 + dj
        line = ""
        for di in range(5):
            i = x - 2 + di
            line += "{:3d}".format(grid[j][i])
        print(line)


def sum_3x3(x, y):
    s = 0
    for j in range(3):
        # print(grid[y-1+j][x-1:x+2])
        s += sum(grid[y-1+j][x-1:x+2])
    return s


# for x, y, serial in lines:
#     grid = []
#     for j in range(N):
#         y2 = j + 1
#         line = []
#         for i in range(N):
#             x2 = i + 1
#             line.append(power_level(x2, y2, serial))
#         grid.append(line)
#     print(sum_3x3(x, y))
    # print_pos(x, y)
    # print()

serial = 7315
# serial = 18
grid = []
ans = (-10, -1, -1)
for j in range(N):
    y2 = j + 1
    line = []
    for i in range(N):
        x2 = i + 1
        line.append(power_level(x2, y2, serial))
    grid.append(line)

for dy in range(N-2):
    y = dy + 1
    for dx in range(N-2):
        x = dx + 1
        s = sum_3x3(x, y)
        if s > ans[0]:
            ans = (s, x, y)
print(ans)