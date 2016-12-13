#!/usr/bin/env python3
from collections import defaultdict
import sys
import re

def parse(lines):
  actions = []
  rules = {}

  for line in lines:
    args = []
    def matches(rex):
      m = re.search("^" + rex + "$", line)
      if m:
        args.extend(m.groups())
        return True
      else:
        return False

    if matches(r'value (-?[0-9]+) goes to ((?:bot|output) [0-9]+)'):
      actions.append((args[1], int(args[0])))
      continue
    elif matches(r'(bot [0-9]+) gives low to ((?:bot|output) [0-9]+) and high to ((?:bot|output) [0-9]+)'):
      rules[args[0]] = (args[1], args[2])
    else:
      print("Unknown line: {}".format(line))

  return {"actions": actions, "rules": rules}

def exec_actions(state, verbose):
  bot_state = defaultdict(list)
  output_state = {}
  def send_value(dest, value):
    if verbose:
      print("Sending value {} to {}".format(value, dest))
    if dest.startswith("output"):
      output_state[dest] = value
    else:
      if len(bot_state[dest]) == 0 or bot_state[dest][0] < value:
        bot_state[dest].append(value)
      else:
        bot_state[dest].insert(0, value)
      if len(bot_state[dest]) > 2:
        print("Overflow at action {}".format(action))
      elif len(bot_state[dest]) == 2:
        if verbose:
          print("Triggered {}".format(dest))
        if set(bot_state[dest]) == {61, 17}:
          print("Target microchips processed by {}".format(dest))
        if not dest in state["rules"]:
          print("No rule for bot {}".format(dest))
        else:
          low_dest, high_dest = state["rules"][dest]
          send_value(low_dest, bot_state[dest].pop(0))
          send_value(high_dest, bot_state[dest].pop(0))

  for action in state["actions"]:
    send_value(*action)

  prod = output_state["output 0"] * output_state["output 1"] * \
    output_state["output 2"]
  print("Product of outputs 0, 1, 2: {}".format(prod))

def run(input_file):
  with open(input_file) as f:
    parsed = parse(line.strip() for line in f.readlines())
    exec_actions(parsed, True)

if __name__ == "__main__":
  input_file = sys.argv[1]
  run(input_file)
