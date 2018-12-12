"""
    --- Day 12: Subterranean Sustainability ---
    The year 518 is significantly more underground than your history books implied.
    Either that, or you've arrived in a
    vast cavern network
     under the North Pole.
    After exploring a little, you discover a long tunnel that contains a row of
    small pots as far as you can see to your left and right.  A few of them contain
    plants - someone is trying to grow things in these geothermally-heated caves.
    The pots are numbered, with
    0

    https://adventofcode.com/2018/day/12
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
from pprint import pprint

debug = False
if debug:
    lines = [
        "initial state: #..#.#..##......###...###",
        "",
        "...## => #",
        "..#.. => #",
        ".#... => #",
        ".#.#. => #",
        ".#.## => #",
        ".##.. => #",
        ".#### => #",
        "#.#.# => #",
        "#.### => #",
        "##.#. => #",
        "##.## => #",
        "###.. => #",
        "###.# => #",
        "####. => #",
    ]
    if os.path.exists('input_debug'):
        with open('input_debug', 'r') as f:
            lines = f.readlines()
else:
    lines = []
    with open('input', 'r') as f:
        lines = f.readlines()
# print(len(lines[0]))
N = 180
state = "." * (N * 2)
init = lines[0][len("initial state: "):].strip()
state = state[:N] + init + state[len(init) + N:]
print("    " + "-" * N + "0" + "-" * (N - 1))
# print(state)

lines = lines[2:]
patterns = {}
for line in lines:
    arr = re.split(" => ", line)

    pattern = arr[0]
    result = arr[1][0]
    patterns[pattern] = result


print("{:2d}: {}".format(0, state))
for gen in range(20):
    next = ["."] * (N * 2)
    for i in range(N * 2 - 5):
        key = "".join(state[i:i+5])
        try:
            if key in patterns:
                # print(type(next[i + 1]))
                # print(type(patterns[key]))
                next[i + 2] = patterns[key]
        except TypeError as e:
            print(e)
            raise
    state = next
    print("{:2d}: {}".format(gen + 1, "".join(state)))

s = 0
for i, x in enumerate(state):
    if x == "#":
        s += i - N
print(s)
# 2612 is too low