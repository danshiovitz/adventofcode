#!/usr/bin/env python3
import sys
import re

EMPTY = "."
WALL = "#"
STEPPED = "O"

def parse(lines):
  instructions = []
  for line in lines:
    args = []
    if matches(line, args, r'([0-9]+) ([0-9]+) ([0-9]+) ([0-9]+)'):
      fav_number = int(args[0])
      goal = (int(args[1]), int(args[2]))
      max_steps = int(args[3])
      return lambda c: calc_contents(c, fav_number), \
        lambda c: c == goal, \
        max_steps
    else:
      print("Bad line: {}".format(line))

def matches(line, args, rex):
  m = re.search("^" + rex + "$", line)
  if m:
    args.extend(m.groups())
    return True
  else:
    return False

def calc_contents(coord, fav_number):
  x, y = coord
  if x < 0 or y < 0:
    return WALL
  tot = x*x + 3*x + 2*x*y + y + y*y + fav_number
  ones = bin(tot).count("1")
  if ones % 2 == 0:
    return EMPTY
  else:
    return WALL

def seek(calc_contents, is_goal):
  cache = {}
  def get_contents(c):
    if c not in cache:
      cache[c] = calc_contents(c)
    return cache[c]
  def set_contents(c, val):
    cache[c] = val

  active = [((1, 1), 0)]
  while active:
    coord, steps = active.pop(0)
    if get_contents(coord) != EMPTY:
      continue
    set_contents(coord, STEPPED)
    if is_goal(coord):
      return steps
    else:
      for mod in ((-1, 0), (1, 0), (0, -1), (0, 1)):
        next_coord = (coord[0] + mod[0], coord[1] + mod[1])
        active.append((next_coord, steps + 1))

def spread(calc_contents, max_steps):
  cache = {}
  def get_contents(c):
    if c not in cache:
      cache[c] = calc_contents(c)
    return cache[c]
  def set_contents(c, val):
    cache[c] = val

  active = [((1, 1), 0)]
  spread_count = 0
  while active:
    coord, steps = active.pop(0)
    if get_contents(coord) != EMPTY:
      continue
    set_contents(coord, STEPPED)
    spread_count += 1
    if steps < max_steps:
      for mod in ((-1, 0), (1, 0), (0, -1), (0, 1)):
        next_coord = (coord[0] + mod[0], coord[1] + mod[1])
        active.append((next_coord, steps + 1))
  display(get_contents, 10, 10)
  return spread_count

def display(get_contents, max_x, max_y):
  for y in range(max_y):
    print("".join(get_contents((x, y)) for x in range(max_x)))

def run(input_file):
  with open(input_file) as f:
    custom_calc, is_goal, max_steps = parse(line.strip() for line in f)
    steps = seek(custom_calc, is_goal)
    print("Finished in {} steps".format(steps))
    spread_count = spread(custom_calc, max_steps)
    print("Spread to {} spots".format(spread_count))

if __name__ == "__main__":
  input_file = sys.argv[1]
  run(input_file)
