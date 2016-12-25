#!/usr/bin/env python3
import sys
import re

def parse(lines):
  for line in lines:
    args = []
    if matches(line, args, r'(\d+)'):
      return int(args[0])
    else:
      print("Bad line: {}".format(line))

def matches(line, args, rex):
  m = re.search("^" + rex + "$", line)
  if m:
    args.extend(m.groups())
    return True
  else:
    return False

def steal_presents(num_elves):
  def elves_with_presents(presents):
    i = 0
    yield i
    while True:
      i = (i + 1) % num_elves
      if presents[i] == 0:
        continue
      yield i

  presents = [1] * num_elves
  elves = elves_with_presents(presents)
  while True:
    cur_elf = next(elves)
    next_elf = next(elves)
    if cur_elf == next_elf:
      break
    presents[cur_elf] += presents[next_elf]
    presents[next_elf] = 0

  if presents[cur_elf] != num_elves:
    print("Present mismatch!")
  return cur_elf + 1

def computed_steal_presents(num_elves):
  tot = 0
  top_val = 1

  while tot + top_val < num_elves:
    tot += top_val
    top_val *= 2
  return (num_elves - tot - 1) * 2 + 1

def train(num_elves, steal, compute):
  for i in range(1, num_elves+1):
    last_elf = steal(i)
    computed = compute(i)
    if last_elf != computed:
      print("Compute error! Computed {} instead of {}".
              format(computed, last_elf))
    print("Presents: {}   Last elf: {}".format(i, last_elf))

def steal_presents_across(num_elves):
  presents = {i: 1 for i in range(num_elves)}
  cur_elf = 0

  def pick_target(elf):
    to_skip = len(presents)//2
    idx = cur_elf
    while to_skip > 0:
      idx = (idx + 1) % num_elves
      if idx in presents:
        to_skip -= 1
    return idx

  while True:
    target_elf = pick_target(cur_elf)
    if cur_elf == target_elf:
      break
    presents[cur_elf] += presents[target_elf]
    del presents[target_elf]
    cur_elf = (cur_elf + 1) % num_elves
    while cur_elf not in presents:
      cur_elf = (cur_elf + 1) % num_elves

  if presents[cur_elf] != num_elves or len(presents) > 1:
    print("Present mismatch! Ended up with {}".format(presents))
  return cur_elf + 1

def computed_steal_presents_across(num_elves):
  if num_elves < 2:
    return num_elves

  tot = 2
  top_val = 2

  while tot + top_val <= num_elves:
    tot += top_val
    top_val *= 3    

  diff = num_elves - tot + 1
  if diff <= top_val // 2:
    return diff
  else:
    return (top_val // 2) + 2*(diff - top_val // 2)

def run(input_file):
  with open(input_file) as f:
    num_elves = parse(line.strip() for line in f)
#  train(num_elves, steal_presents_across, computed_steal_presents_across)
  computed = computed_steal_presents_across(num_elves)
  print("When there are {} elves, the last elf is {}".
          format(num_elves, computed))

if __name__ == "__main__":
  input_file = sys.argv[1]
  run(input_file)
