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

alpha = [chr(x) for x in range(ord('a'), ord('z')+1)]
M = {}
for c in alpha:
    M[c.lower()] = c.upper()
    M[c.upper()] = c.lower()

for line in [x.strip() for x in lines]:
    ans = 1e5
    for rem in alpha:
        s2 = [c for c in line if c.upper() != rem and c.lower() != rem]
        stack = []
        print("---------------")
        print("{}".format(line))
        print("len(line)={}".format(len(line)))
        for c in s2:
            if stack and c == M[stack[-1]]:
                stack.pop()
            else:
                stack.append(c)
        ans = min(ans, len(stack))
        if debug:
            print(stack)
        print("len(line)={}".format(len(stack)))
print(ans)
