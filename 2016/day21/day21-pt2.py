#!/usr/bin/env python3
import sys
import re

def parse(lines):
  instructions = []
  for line in lines:
    args = []
    if matches(line, args, r'swap position (\d+) with position (\d+)'):
      x, y = int(args[0]), int(args[1])
      instructions.append(wrap(rev_swap_positions, x, y))
    elif matches(line, args, r'swap letter (\w) with letter (\w)'):
      x, y = args[0], args[1]
      instructions.append(wrap(rev_swap_letters, x, y))
    elif matches(line, args, r'rotate left (\d+) steps?'):
      x = int(args[0])
      instructions.append(wrap(rev_rotate_left, x))
    elif matches(line, args, r'rotate right (\d+) steps?'):
      x = int(args[0])
      instructions.append(wrap(rev_rotate_right, x))
    elif matches(line, args, r'rotate based on position of letter (\w)'):
      x = args[0]
      instructions.append(wrap(rev_rotate_position, x))
    elif matches(line, args, r'reverse positions (\d+) through (\d+)'):
      x, y = int(args[0]), int(args[1])
      instructions.append(wrap(rev_reverse, x, y))
    elif matches(line, args, r'move position (\d+) to position (\d+)'):
      x, y = int(args[0]), int(args[1])
      instructions.append(wrap(rev_move, x, y))
    else:
      print("Bad line: {}".format(line))
  return instructions

def matches(line, args, rex):
  m = re.search("^" + rex + "$", line)
  if m:
    args.extend(m.groups())
    return True
  else:
    return False

def wrap(f, *args):
  return lambda s: f(s, *args)

def rev_swap_positions(s, x, y):
  if x > y:
    x, y = y, x
  if x >= len(s) or y >= len(s):
    raise ValueError("{}/{} out of range in {}".format(x, y, s))
  else:
    return s[0:x] + s[y] + s[x+1:y] + s[x] + s[y+1:]

def rev_swap_letters(s, x, y):
  t = str.maketrans(x + y, y + x)
  return s.translate(t)

def rev_rotate_right(s, x):
  if len(s) < 1:
    raise ValueError("empty string (rot left)")
  for _ in range(x):
    s = s[1:] + s[0]
  return s

def rev_rotate_left(s, x):
  if len(s) < 1:
    raise ValueError("empty string (rot right)")
  for _ in range(x):
    s = s[-1] + s[:-1]
  return s

def rev_rotate_position(s, x):
  idx = s.find(x)
  if x is None:
    raise ValueError("Can't find {} in {}".format(x, s))
  else:
    if idx % 2 == 0:
      val = idx
      while val <= len(s):
        val += len(s)
      val = (val + 2) // 2
    else:
      val = (idx + 1) // 2
    return rev_rotate_right(s, val)

def rev_reverse(s, x, y):
  if x > y:
    x, y = y, x
  if x >= len(s) or y >= len(s):
    raise ValueError("{}/{} out of range in {} (rev)".format(x, y, s))
  else:
    return s[0:x] + s[x:y+1][::-1] + s[y+1:]

def rev_move(s, x, y):
  x, y = y, x
  if x >= len(s) or y >= len(s):
    raise ValueError("{}/{} out of range in {} (move)".format(x, y, s))
  else:
    c = s[x]
    s = s[0:x] + s[x+1:]
    return s[0:y] + c + s[y:]

def exec_instructions(instructions, val):
  for inst in instructions[::-1]:
    print("val = {}".format(val))
    val = inst(val)
  return val

def run(input_file, init_val):
  with open(input_file) as f:
    instructions = parse(line.strip() for line in f)
  final_val = exec_instructions(instructions, init_val)
  print("Final val: {}".format(final_val))

if __name__ == "__main__":
  input_file = sys.argv[1]
  init_val = sys.argv[2]
  run(input_file, init_val)
