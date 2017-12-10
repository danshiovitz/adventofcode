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
    # now say we have a square like
    #   5 4 3
    #   6 . 2
    #   7 8 9
    # we want to map loc to an offset around the square, starting in the lower right
    cur_area = (radius*2+1)**2
    prev_area = (radius*2-1)**2 if radius > 0 else 0
    edge_size = cur_area - prev_area
    offset = (loc - prev_area) % edge_size
    # then given the offset and the edge_size we can figure out the x,y:
    if offset < edge_size // 4:
      x = radius
      mid = radius
      y = offset - mid
    elif offset < edge_size // 2:
      mid = radius * 3
      x = mid - offset
      y = radius
    elif offset < edge_size * 3 // 4:
      x = -radius
      mid = radius * 5
      y = mid - offset
    else:
      mid = radius * 7
      x = offset - mid
      y = -radius
    return (x, y)

  def neighbors(c):
    x, y = c
    return [
      (x+1, y-1), (x+1, y), (x+1, y+1),
      (x, y-1), (x, y+1),
      (x-1, y-1), (x-1, y), (x-1, y+1),
    ]

  cache = {(0, 0): 1}
  def calc_val(c):
    if c not in cache:
      cache[c] = sum(cache.get(n, 0) for n in neighbors(c))
    return cache[c]

  val = int(input[0])
  return [
    sum(abs(c) for c in x_y(val)),
    next(calc_val(x_y(idx)) for idx in count(1) if calc_val(x_y(idx)) > val),
  ]

def solve(day, input, answers):
  func = globals()[f"run_{day}"]
  print(f"Solving {day} ...")
  actuals = [str(a) for a in func(input)]
  for idx, actual in enumerate(actuals):
    print(f"* Part {idx+1} answer is {actual}", end="")
    if len(answers) > idx:
      if actual == answers[idx]:
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
