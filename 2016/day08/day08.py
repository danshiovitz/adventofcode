#!/usr/bin/env python3
import sys
import re

def new_grid(width, height):
  return [ ['.'] * width for _ in range(height) ]

def display_grid(grid):
  for row in grid:
    print(''.join(row))
  print()

def operate(command, grid):
  args = []
  def matches(rex):
    m = re.search("^" + rex + "$", command)
    if m:
      args.extend(m.groups())
      return True
    else:
      return False
  if matches(r'rect ([0-9]+)x([0-9]+)'):
    for w in range(int(args[0])):
      for h in range(int(args[1])):
        grid[h][w] = '#'
  elif matches(r'rotate column x=([0-9]+) by (-?[0-9]+)'):
    x = int(args[0])
    amt = int(args[1])
    height = len(grid)
    orig = [ grid[h][x] for h in range(height) ]
    for h in range(height):
      grid[h][x] = orig[(h - amt + height) % height]
  elif matches(r'rotate row y=([0-9]+) by (-?[0-9]+)'):
    y = int(args[0])
    amt = int(args[1])
    width = len(grid[0])
    orig = [ grid[y][w] for w in range(width) ]
    for w in range(width):
      grid[y][w] = orig[(w - amt + width) % width]
  else:
    print("Unknown command: {}".format(command))

def count_lit(grid):
  return sum(sum(1 for c in row if c == '#') for row in grid)

def run(file, width, height, verbose):
  grid = new_grid(width, height)
  with open(file) as f:
    for line in f.readlines():
      if verbose:
        display_grid(grid)

      trimmed = line.strip()
      operate(trimmed, grid)
  display_grid(grid)
  print("Lit pixels: {}".format(count_lit(grid)))

if __name__ == "__main__":
  input_file = sys.argv[1]
  width = int(sys.argv[2]) if len(sys.argv) > 1 else 50
  height = int(sys.argv[3]) if len(sys.argv) > 2 else 6
  verbose = sys.argv[4][0].lower() == "t" if len(sys.argv) > 3 else false
  run(input_file, width, height, verbose)
