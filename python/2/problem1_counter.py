"""
    --- Day 2: Inventory Management System ---
    You stop falling through time, catch your breath, and check the screen on the device. "Destination reached. Current Year: 1518. Current Location: North Pole Utility Closet 83N10." You made it! Now, to find those anomalies.
    

    Outside the utility closet, you hear footsteps and a voice. "...I'm not sure either. But now that 
    so many people have chimneys
    , maybe he could sneak in that way?" Another voice responds, "Actually, we've been working on a new kind of 
    suit
     that would let him fit through tight spaces like that. But, I heard that a few days ago, they lost the prototype fabric, the design plans, everything! Nobody on the team can even seem to remember important details of the project!"
    


    https://adventofcode.com/2018/day/2
"""

# import aoc
import os
# import re
# import sys
# from operator import add
# from operator import mul
# from itertools import combinations
from collections import Counter

debug = False
if debug:
    lines = ["abcdef","bababc","abbcde","abcccd","aabcdd","abcdee","ababab"]
    if os.path.exists('input_debug'):
        with open('input_debug', 'r') as f:
            lines = f.readlines()
else:
    lines = []
    with open('input', 'r') as f:
        lines = f.readlines()

count2 = 0
count3 = 0
for line in lines:
    counter = Counter(line.strip())
    check = [counter[x] for x in counter if counter[x] == 2 or counter[x] == 3]
    if 2 in check:
        count2 += 1
    if 3 in check:
        count3 += 1
print("{} * {} = {}".format(count2, count3, count2 * count3))
