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
import sys
# from operator import add
# from operator import mul
# from itertools import combinations

debug = False
if debug:
    lines = ["abcde","fghij","klmno","pqrst","fguij","axcye","wvxyz"]
    if os.path.exists('input_debug'):
        with open('input_debug', 'r') as f:
            lines = f.readlines()
else:
    lines = []
    with open('input', 'r') as f:
        lines = f.readlines()

lines = [line.strip() for line in lines]

for i, line1 in enumerate(lines[:-1]):
    for line2 in lines[(i + 1):]:
        # print("line1={} line2={}".format(line1, line2))
        if len(line1) != len(line2):
            print("diff line length")
            sys.exit(1)

        diff_count = 0
        equal_string = ""
        for j in range(len(line1)):
            if line1[j] == line2[j]:
                equal_string += line1[j]
            else:
                diff_count += 1
        # print("{}".format(equal_string))
        if diff_count == 1:
            print("{}".format(equal_string))


