#!/usr/bin/env python3
import sys
import re
from itertools import combinations

GENERATOR = 'g'
CHIP = 'c'

def parse(lines):
  # assumptions: the floors are given in order; the last floor is the goal
  floor_names = []
  floor_inv = []
  all_inv = set()
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
    all_inv |= floor_inv[-1]

  goal_inv = [frozenset() for _ in range(len(floor_inv))]
  goal_inv[-1] = frozenset(all_inv)

  return {
    "names": floor_names,
    "start_state": (start_floor, tuple(floor_inv)),
    "goal_state": (len(goal_inv) - 1, tuple(goal_inv)),
    }

def parse_inventory(text):
  ret = set()
  for phrase in re.split(r'(?:, and |, | and )', text):
    args = []
    if matches(phrase, args, r'an? (.*?) generator'):
      ret.add((GENERATOR, args[0]))
    elif matches(phrase, args, r'an? (.*?)-compatible microchip'):
      ret.add((CHIP, args[0]))
    elif matches(phrase, args, r'nothing relevant'):
      pass
    else:
      print("Bad phrase: {}".format(phrase))
  return frozenset(ret)

def matches(line, args, rex):
  m = re.search("^" + rex + "$", line)
  if m:
    args.extend(m.groups())
    return True
  else:
    return False

def solve(floor_data):
  best_moves = None
  start_state = floor_data["start_state"]
  goal_state = floor_data["goal_state"]
  
  def solve_dfs(cur_state, moves_left, seen_states):
    if cur_state == goal_state:
      return []
    elif moves_left == 0:
      return None
    else:
      seen_states[cur_state] = moves_left
      next_states = make_next_states(cur_state)
      next_states.sort(key=score_state)
      for next_state, move in next_states:
        if next_state in seen_states and moves_left <= seen_states[next_state]:
          continue
        result = solve_dfs(next_state, moves_left - 1, seen_states)
        if result is not None:
          return [move] + result
      return None

  while True:
    max_moves = len(best_moves) - 1 if best_moves is not None else 20
    result = solve_dfs(start_state, max_moves, {})
    if result is None:
      return best_moves
    elif best_moves is None or len(result) < len(best_moves):
      print("New best, length {}".format(len(result)))
      best_moves = result
    else:
      print("Max moves wasn't respected somehow?")

def make_next_states(cur_state):
  cur_floor, cur_inv = cur_state
  ret = []

  def consider_state(items, floor):
    from_inv = cur_inv[cur_floor] - frozenset(items)
    if not is_safe(from_inv):
      return
    to_inv = cur_inv[floor] | frozenset(items)
    if not is_safe(to_inv):
      return

    next_inv = list(cur_inv[:])
    next_inv[cur_floor] = from_inv
    next_inv[floor] = to_inv
    
    next_state = (floor, tuple(next_inv))
    ret.append((next_state, (items, floor)))
  
  def moves_to_floor(floor):
    for item in cur_inv[cur_floor]:
      consider_state((item,), floor)
    for item1, item2 in combinations(cur_inv[cur_floor], 2):
      consider_state((item1, item2,), floor)

  if cur_floor > 0:
    moves_to_floor(cur_floor - 1)
  if cur_floor < len(cur_inv) - 1:
    moves_to_floor(cur_floor + 1)
  return ret

def is_safe(items):
  chips = set(c[1] for c in items if c[0] == CHIP)
  generators = set(c[1] for c in items if c[0] == GENERATOR)
  for c in chips:
    if c not in generators and len(generators) > 0:
      return False
  return True

def score_state(state):
  floor, inv = state
  score = 0
  for i in range(len(inv) - 1):
    floor_penalty = 2 ** (len(inv) - i)
    score -= floor_penalty * len(inv[i])
  return score

def ditem(item):
  return "{}-{}".format(*item)

def display_state(names, floor, inv):
  for i in range(len(names)):
    fl_text = "*" if floor == i else " "
    inv_text = ", ".join(sorted((ditem(t) for t in inv[i])))
    print("{}The {}: {}".format(fl_text, names[i], inv_text))

def display_moves(floor_data, moves):
  print("Initial state:")
  display_state(floor_data["names"], *floor_data["start_state"])
  print()
  names = floor_data["names"]
  cur_inv = list(floor_data["start_state"][1][:])
  cur_floor = floor_data["start_state"][0]
  for items, to_floor in moves:
    print("Move {} from the {} to the {}".format(
      ",".join(ditem(it) for it in items), names[cur_floor], names[to_floor]))
    cur_inv[cur_floor] = cur_inv[cur_floor] - frozenset(items)
    cur_inv[to_floor] = cur_inv[to_floor] | frozenset(items)
    cur_floor = to_floor   
    print()
  print("Final state:")
  display_state(floor_data["names"], cur_floor, cur_inv)
  print("Total moves: {}".format(len(moves)))

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
