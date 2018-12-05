"""
    --- Day 5: Alchemical Reduction ---
    You've managed to sneak in to the prototype suit manufacturing lab.  The Elves are making decent progress, but are still struggling with the suit's size reduction capabilities.
    

    While the very latest in 1518 alchemical technology might have solved their problem eventually, you can do better.  You scan the chemical composition of the suit's material and discover that it is formed by extremely long 
    polymers
     (one of which is 
    available
     as your puzzle input).
    


    https://adventofcode.com/2018/day/5
"""

# import aoc
import os
# import re
# import sys
# from operator import add
# from operator import mul
# from itertools import combinations

# from collections import Counter

debug = False
if debug:
    lines = ["aA", "abBA", "abAB", "aabAAB", "CaAbBcDd"]
    if os.path.exists('input_debug'):
        with open('input_debug', 'r') as f:
            lines = f.readlines()
else:
    lines = []
    with open('input', 'r') as f:
        lines = f.readlines()

D = abs(ord('a') - ord('A'))
# print("D={}".format(D))

for line in lines:
    line = line.strip()
    print("---------------")
    print("{}".format(line))
    print("len(line)={}".format(len(line)))
    origLine = line
    for removeChar in range(ord('a'), ord('z') + 1):
        print("Remove char {}".format(chr(removeChar)))
        # print("len(origLine)={}".format(origLine))
        line = list(filter(lambda x: x.lower() != chr(removeChar), origLine))
        if debug:
            print("filtered line: {}".format(len(line)))
        N = len(line) + 1
        while N > len(line) >= 2:
            N = len(line)

            new_line = ""
            skip = False
            for i, c in enumerate(line):
                if not skip:
                    if i < N - 1:
                        if abs(ord(line[i + 1]) - ord(c)) != D:
                            new_line += c
                        else:
                            skip = True
                    else:
                        if abs(ord(line[i - 1]) - ord(c)) != D:
                            new_line += c

                else:
                    skip = False
                # print("new_line={}".format(new_line))
            line = new_line
            if debug:
                print(line)
                print("len(line)={}".format(len(line)))

        # 9299 too high
        # print(line)
        if not debug:
            print(len(line))
    # print("line={}\nlen(line)={}".format(line, len(line)))

# answer
# Remove char o
# 5534

