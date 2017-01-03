#!/usr/bin/env python3
from collections import defaultdict
from itertools import chain, combinations, product
import sys
import re

# folks on the mud commented that one big optimization is to "anonymize"
# the elements - a state with the plutonium microchip on floor 1 is
# basically the same as one with the cobalt microchip on floor 1 (all other
# items being equal)
#
# I think it's also a mistake to have tried the retrying depth-first search
# on this originally - that worked well on a problem last year when every
# search terminated eventually (and the hard part was finding the win states);
# here there is no failure state so the only termination is at a win state
# and it's easy to get lost
#
# So going to try switching that up (and then being a little tighter about
# memory use since we're now doing breadth-first and it makes more of a
# difference)
GENERATOR = "generator"
MICROCHIP = "microchip"

def parse(lines):
  # assumptions: the floors are given in order; the last floor is the goal
  floor_names = []
  floor_inv = []
  start_floor = 0
  
  for line in lines:
    if line.startswith("*"):
      start_floor = len(floor_names)
      line = line[1:]
    args = []
    if not matches(line, args, r'The (.*?) contains (.*)\.'):
      print("Bad line: {}".format(line))
    floor_names.append(args[0])
    floor_inv.append(parse_inventory(args[1]))

  goal_inv = [frozenset()] * len(floor_inv)
  goal_inv[-1] = frozenset(chain.from_iterable(floor_inv))

  return {
    "names": floor_names,
    "start_state": (start_floor, tuple(floor_inv)),
    "goal_state": (len(goal_inv) - 1, tuple(goal_inv)),
  }

def parse_inventory(text):
  inventory = set()
  for phrase in re.split(r'(?:, and |, | and )', text):
    args = []
    if matches(phrase, args, r'an? (.*?) generator'):
      inventory.add((GENERATOR, args[0]))
    elif matches(phrase, args, r'an? (.*?)-compatible microchip'):
      inventory.add((MICROCHIP, args[0]))
    elif matches(phrase, args, r'nothing relevant'):
      pass
    else:
      print("Bad phrase: {}".format(phrase))
  return frozenset(inventory)

def matches(line, args, rex):
  m = re.search("^" + rex + "$", line)
  if m:
    args.extend(m.groups())
    return True
  else:
    return False

def solve(floor_data):
  start_state = floor_data["start_state"]
  goal_state = floor_data["goal_state"]

  seen_statekeys = set()
  active = [(start_state, make_statekey(start_state), [])]
  while active:
    cur_state, cur_statekey, moves = active.pop(0)
    if cur_statekey in seen_statekeys:
      continue
    if cur_state == goal_state:
      return moves
    seen_statekeys.add(cur_statekey)

    next_state_moves = make_next_state_moves(cur_state)
    for next_state, move in next_state_moves:
      next_statekey = make_statekey(next_state)
      if next_statekey in seen_statekeys:
        continue
      active.append((next_state, next_statekey, moves + [move]))
  print("Couldn't find a solution :C")

def make_statekey(state):
  floor, inv = state
  mapping = {}
  def map_one(item):
    if item[1] not in mapping:
      mapping[item[1]] = len(mapping)
    val = mapping[item[1]]
    return (item[0], val)
  key_inv = tuple(tuple(map_one(i) for i in floor_inv) for floor_inv in inv)
  return (floor, key_inv)

def make_next_state_moves(cur_state):
  cur_floor, cur_inv = cur_state
  ret = []

  def do_single(to_floor):
    removals = calc_possible_removals(cur_inv[cur_floor])
    for removal, remaining in removals:
      gained = cur_inv[to_floor] | removal
      if is_illegal_inv(gained):
        continue
      next_inv = list(cur_inv)
      next_inv[cur_floor] = remaining
      next_inv[to_floor] = gained
      next_state = (to_floor, tuple(next_inv))
      move = (removal, to_floor)
      ret.append((next_state, move))
  
  if cur_floor > 0:
    do_single(cur_floor - 1)
  if cur_floor < len(cur_inv) - 1:
    do_single(cur_floor + 1)

  return ret

def calc_possible_removals(inv):
  removals = [set([i]) for i in inv] + [set(c) for c in combinations(inv, 2)]
  pairs = [(r, inv - r) for r in removals]
  return [p for p in pairs if not is_illegal_inv(p[1])]

def is_illegal_inv(inv, cache={}):
  if inv not in cache:
    cache[inv] = False    
    microchips = set(i[1] for i in inv if i[0] == MICROCHIP)
    generators = set(i[1] for i in inv if i[0] == GENERATOR)
    for m in microchips:
      if m not in generators and len(generators) > 0:
        cache[inv] = True
        break
  return cache[inv]

def display_moves(floor_data, moves):
  state = floor_data["start_state"]
  floor_names = floor_data["names"]
  print("Initial state:")
  display_state(state, floor_names)
  print()
  for move in moves:
    state = perform_move(move, state, floor_names)
    print()
  print("Final state:")
  display_state(state, floor_names)
  print()
  print("Total moves: {}".format(len(moves)))

def display_state(state, floor_names):
  floor, inv = state
  for i in range(len(floor_names)):
    fl_text = "*" if floor == i else " "
    inv_text = ", ".join(sorted(format_item(i) for i in inv[i]))
    print("{}The {}: {}".format(fl_text, floor_names[i], inv_text))

def perform_move(move, old_state, floor_names):
  old_floor, old_inv = old_state
  moved_items, new_floor = move
  
  old_items = old_inv[old_floor]
  new_items = old_inv[new_floor]  

  if len(old_items & moved_items) != len(moved_items):
    print("Simulator error: couldn't find all items of {} in {}".
            format(moved_items, old_items))

  new_inv = list(old_inv[:])
  new_inv[old_floor] = old_items - moved_items
  new_inv[new_floor] = new_items | moved_items

  moved_item_text = ", ".join(sorted(format_item(i) for i in moved_items))
  print("Move {} from the {} to the {}".format(
    moved_item_text, floor_names[old_floor], floor_names[new_floor]))
  return (new_floor, tuple(new_inv))

def format_item(item):
  return "a {} {}".format(item[1], item[0])

def run(input_file):
  with open(input_file) as f:
    floor_data = parse(line.strip() for line in f.readlines())
    moves = solve(floor_data)
    if moves is None:
      print("No solution found?")
    else:
      display_moves(floor_data, moves)

if __name__ == "__main__":
  input_file = sys.argv[1]
  run(input_file)
