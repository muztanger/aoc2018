"""
    --- Day 8: Memory Maneuver ---
    The sleigh is much easier to pull than you'd expect for something its weight.
    Unfortunately, neither you nor the Elves know
    which way
     the North Pole is from here.
    You check your wrist device for anything that might help.  It seems to have some
    kind of navigation system!  Activating the navigation system produces more bad
    news: "Failed to start navigation system. Could not read software license file."
    The navigation system's license file consists of a list of numbers (your puzzle
    input).  The numbers define a data structure which, when processed, produces
    some kind of
    tree

    https://adventofcode.com/2018/day/8
"""

# import aoc
import os
# import re
# import sys
# from operator import add
# from operator import mul
# from itertools import combinations

# from collections import Counter
from pprint import pprint

debug = False
if debug:
    lines = ["2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2"]
    if os.path.exists('input_debug'):
        with open('input_debug', 'r') as f:
            lines = f.readlines()
else:
    lines = []
    with open('input', 'r') as f:
        lines = f.readlines()

s = lines[0].strip().split()
print("len(s)={}".format(len(s)))
nodes = []
ident = ord('A')


class Node:
    metadata_count = 0

    def __init__(self, name, children, metadata):
        self.name = name
        self.children = children
        self.metadata = metadata

    def __str__(self):
        return "Node(name='{}', children='{}', metadata='{}')".format(
            self.name, self.children, self.metadata
        )

    def __repr__(self):
        return self.__str__()

    def count_metadata(self):
        Node.metadata_count += sum(map(int, self.metadata))
        for c in self.children:
            c.count_metadata()
        return Node.metadata_count

    def value(self):
        if len(self.children) == 0:
            return sum(map(int, self.metadata))
        v = 0
        for m in self.metadata:
            m = int(m)
            # print(m)
            if 0 < m < len(self.children) + 1:
                # print('add')
                v += self.children[m - 1].value()
        # print("v={}".format(v))
        return v


class Tree:
    s = ""
    i = 0
    name = 0

    def __init__(self):
        pass

    @staticmethod
    def parse():
        name = Tree.get_name()
        n_childs = int(Tree.get_next())
        n_metadata = int(Tree.get_next())

        children = []
        # Need recursive parse child function

        for x in range(n_childs):
            children.append(Tree.parse_child())
        metadata = []
        for m in range(n_metadata):
            metadata.append(Tree.get_next())
        return Node(name, children, metadata)

    @staticmethod
    def get_next():
        x = Tree.s[Tree.i]
        Tree.i += 1
        return x

    @staticmethod
    def get_name():
        x = "S{:02d}".format(Tree.name)
        Tree.name += 1
        return x

    @staticmethod
    def parse_child():
        name = Tree.get_name()
        n_childs = int(Tree.get_next())
        n_metadata = int(Tree.get_next())
        children = []
        for x in range(n_childs):
            children.append(Tree.parse_child())
        metadata = []
        for m in range(n_metadata):
            metadata.append(Tree.get_next())
        return Node(name, children, metadata)


Tree.s = s
root = Tree.parse()
# pprint(root)
print(root.value())



