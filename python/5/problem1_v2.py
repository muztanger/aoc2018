"""
    --- Day 5: Alchemical Reduction ---
    You've managed to sneak in to the prototype suit manufacturing lab.  The Elves are making decent progress, but are still struggling with the suit's size reduction capabilities.
    

    While the very latest in 1518 alchemical technology might have solved their problem eventually, you can do better.  You scan the chemical composition of the suit's material and discover that it is formed by extremely long 
    polymers
     (one of which is 
    available
     as your puzzle input).
    


    https://adventofcode.com/2018/day/5

    Tested to write the code like the following guy did:
    https://www.youtube.com/watch?v=VBhrueOccZ0
    Suuper fast compared to what I did. The dictionary for mapping characters was really good.

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

M = {}
for c in [chr(x) for x in range(ord('a'), ord('z')+1)]:
    M[c.lower()] = c.upper()
    M[c.upper()] = c.lower()

for line in [x.strip() for x in lines]:
    stack = []
    print("---------------")
    print("{}".format(line))
    print("len(line)={}".format(len(line)))
    for c in line:
        if stack and c == M[stack[-1]]:
            stack.pop()
        else:
            stack.append(c)

    if debug:
        print(stack)
    print("len(line)={}".format(len(stack)))

