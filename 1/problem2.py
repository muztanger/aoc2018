"""
    --- Day 1: Chronal Calibration ---
    "We've detected some temporal anomalies," one of Santa's Elves at the 
    Temporal Anomaly Research and Detection Instrument Station
     tells you. She sounded pretty worried when she called you down here. "At 500-year intervals into the past, someone has been changing Santa's history!"
    

    "The good news is that the changes won't propagate to our time stream for another 25 days, and we have a device" - she attaches something to your wrist - "that will let you fix the changes with no such propagation delay. It's configured to send you 500 years further into the past every few days; that was the best we could do on such short notice."
    

    "The bad news is that we are detecting roughly 
    fifty

    https://adventofcode.com/2018/day/1
"""

# import aoc
import os
# import re
# import sys
# from operator import add
# from operator import mul
# from itertools import combinations

debug = False
if debug:
    lines = ['2', '-1', '+1', '3']
    if os.path.exists('input_debug'):
        with open('input_debug', 'r') as f:
            lines = f.readlines()
else:
    lines = []
    with open('input', 'r') as f:
        lines = f.readlines()
s = 0
freq = set()
run = True
while run:
    for line in lines:
        freq.add(s)
        s += int(line.strip())
        if s in freq:
            print(s)
            run = False
            break
#print(s)

