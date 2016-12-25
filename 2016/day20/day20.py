#!/usr/bin/env python3
import sys
import re

def parse(lines):
  blocks = []
  for line in lines:
    args = []
    if matches(line, args, r'(\d+)-(\d+)'):
      blocks.append((int(args[0]), int(args[1])))
    else:
      print("Bad line: {}".format(line))
  return blocks

def matches(line, args, rex):
  m = re.search("^" + rex + "$", line)
  if m:
    args.extend(m.groups())
    return True
  else:
    return False

def standardize_blocks(blocks):
  ret = []
  sorted_blocks = sorted(blocks, key=lambda b: b[0])
  for i in range(len(sorted_blocks)):
    if sorted_blocks[i] is None:
      continue
    block_start, block_end = sorted_blocks[i]
    for j in range(i+1, len(sorted_blocks)):
      if sorted_blocks[j] is None:
        continue
      if sorted_blocks[j][0] <= block_end <= sorted_blocks[j][1]:
        block_end = max(block_end, sorted_blocks[j][1])
        sorted_blocks[j] = None
      elif block_end >= sorted_blocks[j][1]:
        sorted_blocks[j] = None
    ret.append((block_start, block_end))
  return ret

def find_lowest_unblocked(blocks):
  val = 0
  for block in blocks:
    if block[0] <= val <= block[1]:
      val = block[1] + 1
    else:
      break
  return val

def count_unblocked(blocks, max_val=4294967295):
  val = 0
  unblocked = 0
  for block in blocks:
    if val < block[0]:
      unblocked += block[0] - val
    val = block[1] + 1
  unblocked += max_val + 1 - val

  tot = max_val + 1
  for block in blocks:
    tot -= (block[1] - block[0] + 1)
  print("tot {}, unblocked {}".format(tot, unblocked))
  return unblocked

def run(input_file):
  with open(input_file) as f:
    blocks = parse(line.strip() for line in f)
  standardized_blocks = standardize_blocks(blocks)
  lowest = find_lowest_unblocked(standardized_blocks)
  count = count_unblocked(standardized_blocks)
  print("Lowest unblocked value is {}".format(lowest))
  print("Number unblocked is {}".format(count))

if __name__ == "__main__":
  input_file = sys.argv[1]
  run(input_file)
