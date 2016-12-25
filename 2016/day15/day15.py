#!/usr/bin/env python3
import sys
import re

def parse(lines):
  discs = []
  for line in lines:
    args = []
    if matches(line, args, r'Disc #\d+ has (\d+) positions?; at time=0, it is at position (\d+)\.'):
      discs.append((int(args[0]), int(args[1])))
    else:
      print("Bad line: {}".format(line))
  return discs

def matches(line, args, rex):
  m = re.search("^" + rex + "$", line)
  if m:
    args.extend(m.groups())
    return True
  else:
    return False

def find_passthrough(discs):
  def disc_values(size, position):
    t = size - position
    if t < 0:
      t += size
    while True:
      yield t
      t += size

  # adjust position so we can assume the ball drops in zero time
  values = [disc_values(discs[i][0], (discs[i][1] + i + 1) % discs[i][0])
              for i in range(len(discs))]

  cur_values = [next(v) for v in values]
  while True:
    min_idx = 0
    all_same = True
    for i in range(len(cur_values)):
      if cur_values[i] < cur_values[min_idx]:
        min_idx = i
      if cur_values[i] != cur_values[0]:
        all_same = False
    if all_same:
      return cur_values[0]
    else:
      cur_values[min_idx] = next(values[min_idx])

def simulate_drop(discs, start_time, drop_time):
  print("Discs:")
  for d in discs: print(d)
  print()
  print("Capsule dropped at t={}".format(start_time))
  t = 0
  for d in discs:
    t += drop_time
    dpos = (d[1] + t + start_time) % d[0]
    print("At t={}, {}-disc is at {}".format(start_time + t, d[0], dpos))
    if dpos != 0:
      print("BONK!")
      return

def run(input_file):
  with open(input_file) as f:
    discs = parse(line.strip() for line in f)
    t = find_passthrough(discs)
    print("Passthrough time is {}".format(t))

if __name__ == "__main__":
  input_file = sys.argv[1]
  run(input_file)
