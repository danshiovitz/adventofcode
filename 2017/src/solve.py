#!/usr/bin/env python3
import argparse
from itertools import count, permutations
from pathlib import Path
import re
import sys

def run_day01(input):
  digits = input[0]
  def ck(i, offset):
    return digits[i] == digits[(i + offset + len(digits)) % len(digits)]
  return [str(sum(int(digits[i]) for i in range(len(digits)) if ck(i, o)))
          for o in (1, len(digits) // 2)]

def run_day02(input):
  def split_digits(line):
    return [int(n) for n in re.split(r'\s+', line)]
  def minmax(nums):
    return max(nums) - min(nums)
  def divr(nums):
    return next((n // m) for n, m in permutations(nums, 2) if n % m == 0)
  
  return [
    sum(minmax(split_digits(line)) for line in input),
    sum(divr(split_digits(line)) for line in input),    
  ]

def run_day03(input):
  def x_y(loc):
    radius = next(r for r in count() if loc <= (r*2+1)**2)
    # if you have, eg,
    #  5 4 3
    #  6 . 2
    #  7 8 9
    # then you want to map that so 2 -> 1, 3 -> 2, etc, but 9 -> 0, thus:
    cur_max = (radius*2+1)**2
    prev_max = (radius*2-1)**2
    size = cur_max - prev_max
    offset = loc - prev_max
    offset %= size
    # now we can use the offset to calculate x/y
    if offset < (size / 4):
      x = radius
      mid = size // 4
      y = 
    elif offset < (size / 2):
      mid = size * 3 // 8
      x = mid - offset
    elif offset < (size * 3 // 4):
      x = -radius
    else:
      mid = size * 5 // 8
      x = mid - offset
  loc = int(input[0])
  return [
    sum(x_y(loc)),
  ]

def solve(day, input, answers):
  func = globals()[f"run_{day}"]
  print(f"Solving {day} ...")
  actuals = [str(a) for a in func(input)]
  for idx, actual in enumerate(actuals):
    print(f"* Part {idx+1} answer is {actuals[idx]}", end="")
    if len(answers) > idx:
      if actuals[idx] == answers[idx]:
        print(" (correct)")
      else:
        print(f" (INCORRECT; should be {answers[idx]})") 
    else:
      print("")

def load_file(fname):
  fpath = Path(fname)
  if not fpath.exists():
    return []
  with open(fpath) as f:
    return [line.strip() for line in f.readlines()]

def parse_args():
  parser = argparse.ArgumentParser(description='Solve advent of code problems')
  parser.add_argument('days', metavar='day', nargs='+', help='days to solve')
  parser.add_argument('--input', default='', help='Custom input to use (a string)')
  parser.add_argument('--answer', default='', help='Custom answer to use (a string)')
  args = parser.parse_args()
  
  if len(args.days) == 1:
    day = args.days[0]
    inp = [args.input] if args.input else load_file(f"inputs/{day}")
    ans = [args.answer] if args.answer else load_file(f"answers/{day}")
    return [(args.days[0], inp, ans)]
  else:
    raise Exception("Can only handle one day right now")
  
if __name__ == "__main__":
  days = parse_args()
  for (day, input, answers) in days:
    solve(day, input, answers)
