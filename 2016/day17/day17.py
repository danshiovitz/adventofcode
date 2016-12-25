#!/usr/bin/env python3
from hashlib import md5
import sys
import re

UP = "U"
DOWN = "D"
LEFT = "L"
RIGHT = "R"

def parse(lines):
  for line in lines:
    args = []
    if matches(line, args, r'(\d+) (\d+) (\S+)'):
      return int(args[0]), int(args[1]), args[2]
    else:
      print("Bad line: {}".format(line))

def matches(line, args, rex):
  m = re.search("^" + rex + "$", line)
  if m:
    args.extend(m.groups())
    return True
  else:
    return False

def seek(width, height, passcode):
  seen = {}
  for i in range(width):
    seen[(i, 0, UP)] = 0
    seen[(i, height - 1, DOWN)] = 0
  for i in range(height):
    seen[(0, i, LEFT)] = 0
    seen[(width - 1, i, RIGHT)] = 0

  base_md5 = md5(bytes(passcode, 'utf-8'))
  
  def exits(path):
    cur_md5 = base_md5.copy()
    cur_md5.update(bytes(path, 'ascii'))
    hexed = cur_md5.hexdigest()
    return [d for idx, d in enumerate((UP, DOWN, LEFT, RIGHT))
              if hexed[idx] in 'bcdef']

  def moved(from_x, from_y, dr):
    to_x = from_x
    to_y = from_y
    if dr == UP:
      to_y -= 1
    elif dr == DOWN:
      to_y += 1
    elif dr == LEFT:
      to_x -= 1
    else:
      to_x += 1
    return to_x, to_y

  active = []
  for e in exits(""):
    active.append(((0, 0, e), ""))

  paths = []
  while active:
    move, path = active.pop(0)
    if move in seen:
#      print("skipping move {}".format(move))
      continue

    from_x, from_y, dr = move
    to_x, to_y = moved(from_x, from_y, dr)
    path += dr

#    print("trying {} from {} to {}".format(dr, (from_x, from_y), (to_x, to_y)))
    
    if to_x == width - 1 and to_y == height - 1:
      paths.append(path)
    elif len(path) == 1000:
      print("Too deep, giving up")
    else:
      for e in exits(path):
#        print("Exit {} from {}".format(e, (to_x, to_y)))
        active.append(((to_x, to_y, e), path))
  return paths
  
def run(input_file):
  with open(input_file) as f:
    width, height, passcode = parse(line.strip() for line in f)
  paths = seek(width, height, passcode)
  print("Best paths:")
  for p in paths:
    print("[{}] {}".format(len(p), p))

if __name__ == "__main__":
  input_file = sys.argv[1]
  run(input_file)
