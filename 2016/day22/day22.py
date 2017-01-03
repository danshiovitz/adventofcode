#!/usr/bin/env python3
import sys
import re

def parse(lines):
  nodes = {}
  for line in lines:
    args = []
    if matches(line, args, r'/dev/grid/node-x(\d+)-y(\d+)\s+(\d+)T\s+(\d+)T\s+(\d+)T\s+\d+%'):
      x, y = int(args[0]), int(args[1])
      size, used, avail = int(args[2]), int(args[3]), int(args[4])
      nodes[(x, y)] = {"size": size, "used": used, "avail": avail}
    elif matches(line, args, r'root@ebhq-gridcenter# df -h') or \
      matches(line, args, r'Filesystem\s+Size\s+Used\s+Avail\s+Use%'):
      pass
    else:
      print("Bad line: {}".format(line))
  for x, y in nodes.keys():
    neighbors = [(x+xm, y+ym) for xm, ym in
                 ((0, 1), (0, -1), (1, 0), (-1, 0))
                 if (x+xm, y+ym) in nodes]
    nodes[(x, y)]["neighbors"] = neighbors
  return nodes

def matches(line, args, rex):
  m = re.search("^" + rex + "$", line)
  if m:
    args.extend(m.groups())
    return True
  else:
    return False

def calc_viable_pairs(nodes):
  pairs = []
  
  for n1 in nodes.keys():
    n1_used = nodes[n1]["used"]
    if n1_used == 0:
      continue
    for n2 in nodes.keys():
      if n1 == n2: continue
      if nodes[n2]["avail"] >= n1_used:
        pairs.append((n1, n2))
  return pairs

# rather than write a solver for pt2, you can just examine the data -
# at least for me, the "full" nodes are just a line at y=12 from x=9 to x=31,
# and the empty node is at x=24, y=22, so basically the steps are:
# - move the empty space to x=8, y=22 (16 moves)
# - move the empty space to x=8, y=0 (22 moves)
# - move the empty space to x=30, y=0 (22 moves)
# - move the goal data to the now-empty space at x=30, y=0 (1 move)
# - now you're in a state like this:
#     .G_
#     ...
# - so repeatedly, move the empty space to the left of the goal data and
#   then move the goal data into it (5 moves per loop, and you need to move
#   the goal data from x=30 to x=0 (150 moves)

def run(input_file):
  with open(input_file) as f:
    nodes = parse(line.strip() for line in f)
  viable_pairs = calc_viable_pairs(nodes)
  print("Number of viable pairs: {}".format(len(viable_pairs)))

if __name__ == "__main__":
  input_file = sys.argv[1]
  run(input_file)
