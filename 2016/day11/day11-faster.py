#!/usr/bin/env python3
from bitarray import bitarray
from collections import defaultdict
from itertools import chain, combinations, product
import sys
import re

# same as day11-fast, but that wasn't fast enough to do part 2, so trying
# the other optimization from the mud - representing inventory as bitstrings
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

  index_map = make_index_map(floor_inv)

  return {
    "names": floor_names,
    "start_state": (start_floor, inv_to_bits(floor_inv, index_map)),
    "goal_state": (len(goal_inv) - 1, inv_to_bits(goal_inv, index_map)),
    "index_map": index_map
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

def make_index_map(inv):
  index_map = {}
  for floor_inv in inv:
    for item in floor_inv:
      if item[1] not in index_map:
        index_map[item[1]] = len(index_map) * 2
  return index_map

def inv_to_bits(inv, index_map):
  def convert_floor(floor_inv):
    b = bitarray(max(index_map.values()) + 2)
    b.setall(False)
    for item in floor_inv:
      idx = index_map[item[1]] + (1 if item[0] == GENERATOR else 0)
      b[idx] = True
    return b
  return [convert_floor(floor_inv) for floor_inv in inv]

def solve(floor_data):
  start_state = floor_data["start_state"]
  goal_state = floor_data["goal_state"]

  seen_statekeys = set()
  start_statekey = make_statekey(start_state, floor_data["index_map"])
  active = [(start_state, start_statekey, [])]
  while active:
    cur_state, cur_statekey, moves = active.pop(0)
    if cur_statekey in seen_statekeys:
      continue
    if cur_state == goal_state:
      return moves
    seen_statekeys.add(cur_statekey)

    next_state_moves = make_next_state_moves(cur_state)
    for next_state, move in next_state_moves:
      next_statekey = make_statekey(next_state, floor_data["index_map"])
      if next_statekey in seen_statekeys:
        continue
      active.append((next_state, next_statekey, moves + [move]))
  print("Couldn't find a solution :C")

def make_statekey(state, index_map):
  floor, bits = state

  remaining_idxs = set(i for i in range(0, bits[0].length(), 2))
  mapping = {}

  for floor_bits in bits:
    for i in range(0, bits[0].length(), 2):
      if floor_bits[i] and i in remaining_idxs:
        mapping[i] = len(mapping) * 2
        remaining_idxs.remove(i)
  
  def map_floor(floor_bits):
    new_bits = bitarray(floor_bits.length())
    for k, v in mapping.items():
      new_bits[v] = floor_bits[k]
      new_bits[v+1] = floor_bits[k+1]
    return tuple(new_bits.tobytes())

  return (floor, tuple(map_floor(floor_bits) for floor_bits in bits))

def make_next_state_moves(cur_state):
  cur_floor, cur_bits = cur_state
  ret = []

  def do_single(to_floor):
    removals = calc_possible_removals(cur_bits[cur_floor])
    for removal, remaining in removals:
      gained = cur_bits[to_floor] | removal
      if is_illegal(gained):
        continue
      next_bits = cur_bits[:]
      next_bits[cur_floor] = remaining
      next_bits[to_floor] = gained
      next_state = (to_floor, next_bits)
      move = (removal, to_floor)
      ret.append((next_state, move))
  
  if cur_floor > 0:
    do_single(cur_floor - 1)
  if cur_floor < len(cur_bits) - 1:
    do_single(cur_floor + 1)

  return ret

def calc_possible_removals(floor_bits):
  on_bits = [idx for idx, b in enumerate(floor_bits) if b]
  bit_groups = [set([i]) for i in on_bits] + \
    [set(c) for c in combinations(on_bits, 2)]
  removals = [bitarray([(i in group) for i in range(floor_bits.length())])
    for group in bit_groups]
  pairs = [(r, floor_bits & ~r) for r in removals]
  return [p for p in pairs if not is_illegal(p[1])]

def is_illegal(floor_bits, cached={}):
  if "generators_bitmask" not in cached:
    b = bitarray(len(floor_bits))
    b.setall(False)
    b[1::2] = True
    cached["generators_bitmask"] = b
  # illegality in this variant means the microchip bit is on, the corresponding
  # generator bit is off, but at least one generator bit is on
  if not (floor_bits & cached["generators_bitmask"]).any():
    return False
  
  for i in range(0, floor_bits.length(), 2):
    if floor_bits[i] and not floor_bits[i+1]:
      return True
  return False

def display_moves(floor_data, moves):
  state = floor_data["start_state"]
  floor_names = floor_data["names"]
  print("Initial state:")
  display_state(state, floor_names, floor_data["index_map"])
  print()
  for move in moves:
    state = perform_move(move, state, floor_names, floor_data["index_map"])
    print()
  print("Final state:")
  display_state(state, floor_names, floor_data["index_map"])
  print()
  print("Total moves: {}".format(len(moves)))

def display_state(state, floor_names, index_map):
  floor, bits = state
  for i in range(len(floor_names)):
    fl_text = "*" if floor == i else " "
    print("{}The {}: {}".format(
      fl_text, floor_names[i], format_items(bits[i], index_map)))

def perform_move(move, old_state, floor_names, index_map):
  old_floor, old_bits = old_state
  moved_bitmask, new_floor = move

  old_items = old_bits[old_floor]
  new_items = old_bits[new_floor]  

  if (old_items & moved_bitmask) != moved_bitmask:
    print("Simulator error: couldn't find all items of {} in {}".
            format(moved_bitmask, old_items))

  new_bits = old_bits[:]
  new_bits[old_floor] = old_items & ~moved_bitmask
  new_bits[new_floor] = new_items | moved_bitmask

  print("Move {} from the {} to the {}".format(
    format_items(moved_bitmask, index_map),
    floor_names[old_floor], floor_names[new_floor]))
  return (new_floor, new_bits)

def format_items(bits, index_map):
  rev_index_map = {v: k for k, v in index_map.items()}  
  item_strings = []
  for idx, bit in enumerate(bits):
    if not bit: continue
    t = MICROCHIP
    if idx % 2 == 1:
      idx -= 1
      t = GENERATOR
    item_strings.append("a {} {}".format(rev_index_map[idx], t))
  return ", ".join(sorted(item_strings))

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
