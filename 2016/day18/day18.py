#!/usr/bin/env python3
import sys
import re

SAFE = "."
TRAP = "^"

def parse(lines):
  for line in lines:
    args = []
    if matches(line, args, r'(\d+) (\S+)'):
      return args[1], int(args[0])
    else:
      print("Bad line: {}".format(line))

def matches(line, args, rex):
  m = re.search("^" + rex + "$", line)
  if m:
    args.extend(m.groups())
    return True
  else:
    return False

def build_map(init_row, num_rows):
  yield init_row
  prev_row = init_row
  for i in range(num_rows - 1):
    next_row = [SAFE] * len(init_row)
    for i in range(len(next_row)):
      left = SAFE if i == 0 else prev_row[i-1]
      right = SAFE if i == len(next_row)-1 else prev_row[i+1]
      if left != right:
        next_row[i] = TRAP
    yield next_row
    prev_row = next_row

def run(input_file):
  with open(input_file) as f:
    init_row, num_rows = parse(line.strip() for line in f)
  rows = build_map(init_row, num_rows)
  safe_spots = sum(sum(1 for c in row if c == SAFE) for row in rows)
  print("Safe spots: {}".format(safe_spots))

if __name__ == "__main__":
  input_file = sys.argv[1]
  run(input_file)
