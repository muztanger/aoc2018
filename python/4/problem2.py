"""
    --- Day 4: Repose Record ---
    You've 
    sneaked
     into another supply closet - this time, it's across from the prototype suit manufacturing lab. You need to sneak inside and fix the issues with the suit, but there's a guard stationed outside the lab, so this is as close as you can safely get.
    

    As you search the closet for anything that might help, you discover that you're not the first person to want to sneak in.  Covering the walls, someone has spent an hour starting every midnight for the past few months secretly observing this guard post!  They've been writing down the ID of 
    the one guard on duty that night
     - the Elves seem to have decided that one guard was enough for the overnight shift - as well as when they fall asleep or wake up while at their post (your puzzle input).
    

    For example, consider the following records, which have already been organized into chronological order:
    

    [1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up

    

    Timestamps are written using 
    year-month-day hour:minute
     format. The guard falling asleep or waking up is always the one whose shift most recently started. Because all asleep/awake times are during the midnight hour (
    00:00
     - 
    00:59
    ), only the minute portion (
    00
     - 
    59
    ) is relevant for those events.
    

    Visually, these records show that the guards are asleep at these times:
    

    Date   ID   Minute
            000000000011111111112222222222333333333344444444445555555555
            012345678901234567890123456789012345678901234567890123456789
11-01  #10  .....####################.....#########################.....
11-02  #99  ........................................##########..........
11-03  #10  ........................#####...............................
11-04  #99  ....................................##########..............
11-05  #99  .............................................##########.....

    

    The columns are Date, which shows the month-day portion of the relevant day; ID, which shows the guard on duty that day; and Minute, which shows the minutes during which the guard was asleep within the midnight hour.  (The Minute column's header shows the minute's ten's digit in the first row and the one's digit in the second row.) Awake is shown as 
    .
    , and asleep is shown as 
    #
    .
    

    Note that guards count as asleep on the minute they fall asleep, and they count as awake on the minute they wake up. For example, because Guard #10 wakes up at 00:25 on 1518-11-01, minute 25 is marked as awake.
    

    If you can figure out the guard most likely to be asleep at a specific time, you might be able to trick that guard into working tonight so you can have the best chance of sneaking in.  You have two strategies for choosing the best guard/minute combination.
    

    Strategy 1:
     Find the guard that has the most minutes asleep. What minute does that guard spend asleep the most?
    

    In the example above, Guard #10 spent the most minutes asleep, a total of 50 minutes (20+25+5), while Guard #99 only slept for a total of 30 minutes (10+10+10). Guard #
    10
     was asleep most during minute 
    24
     (on two days, whereas any other minute the guard was asleep was only seen on one day).
    

    While this example listed the entries in chronological order, your entries are in the order you found them. You'll need to organize them before they can be analyzed.
    

    What is the ID of the guard you chose multiplied by the minute you chose?
     (In the above example, the answer would be 
    10 * 24 = 240
    .)
    


    https://adventofcode.com/2018/day/4
"""

# import aoc
import os
import re
import sys
# from operator import add
# from operator import mul
# from itertools import combinations

# from collections import Counter
import datetime
import time
from operator import itemgetter

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


class Event:
    shift_start = "shift_start"
    start_sleep = "start_sleep"
    stop_sleep = "stop_sleep"

    def __init__(self, epoch, date, description):
        self.epoch = epoch
        self.date = date
        self.minute = int(re.match(".*:(\d+)", date).group(1))
        self.description = description
        if "#" in description:
            self.guard = int(re.match(".*#(\d+) ", self.description).group(1))
        else:
            self.guard = None
        if "Guard" in self.description:
            self.event = Event.shift_start
        elif "falls asleep" in self.description:
            self.event = Event.start_sleep
        elif "wakes up" in self.description:
            self.event = Event.stop_sleep

    def __str__(self):
        return "event({} {} {} {})".format(self.epoch, self.minute, self.guard, self.event)

events = {}
for line in lines:
    # print(line)
    m = re.match("\[([^\]]+)\] (.*)$", line.strip())
    date = m.group(1)

    t = time.strptime(re.sub("1518", "1970", date), "%Y-%m-%d %H:%M")
    epoch = int(time.mktime(t))
    description = m.group(2)
    e = Event(epoch, date, description)
    if epoch in events:
        print("FAIL: epoch in events")
        sys.exit(1)
    events[epoch] = e

total_sleep = [0] * 60
guard_sleep = {}
guard = -1
for epoch in sorted(events):
    e = events[epoch]
    if e.event == Event.shift_start:
        guard = e.guard
    elif e.event == Event.start_sleep:
        sleep_start = e.minute
        e.guard = guard
    elif e.event == Event.stop_sleep:
        sleep_end = e.minute
        # print("sleep_start={}".format(sleep_start))
        # print("sleep_end={}".format(sleep_end))
        e.guard = guard
        for i in range(sleep_start, sleep_end):
            # print(i)
            total_sleep[i] += 1
            if guard not in guard_sleep:
                guard_sleep[guard] = [0] * 60
            guard_sleep[guard][i] += 1

total_min_minute = max(enumerate(total_sleep), key=itemgetter(1))[0]
best_time = -1
for guard in guard_sleep:
    (sleep_time_minute, sleep_time) = max(enumerate(guard_sleep[guard]), key=itemgetter(1))
    if sleep_time > best_time:
        best_time = sleep_time
        best_time_minute = sleep_time_minute
        max_guard = guard

print("{} * {} = {}".format(max_guard, best_time_minute, max_guard * best_time_minute))
