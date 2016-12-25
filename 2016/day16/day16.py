#!/usr/bin/env python3
import sys
import re

def parse(lines):
  for line in lines:
    args = []
    if matches(line, args, r'(\S+) (\d+)'):
      return args[0], int(args[1])
    else:
      print("Bad line: {}".format(line))

def matches(line, args, rex):
  m = re.search("^" + rex + "$", line)
  if m:
    args.extend(m.groups())
    return True
  else:
    return False

def generative_answer(seed, target_size):
  invert_trans = str.maketrans('01', '10')
  def grow(d):
    return d + '0' + d[::-1].translate(invert_trans)

  cs_map = {"00": "1", "11": "1", "10": "0", "01": "0"}
  def calc_checksum(d):
    if len(d) % 2 == 1:
      return d
    else:
      chunks = (d[i:i+2] for i in range(0, len(d), 2))
      return calc_checksum("".join(cs_map[c] for c in chunks))
      
  data = seed
  while len(data) < target_size:
    data = grow(data)
  data = data[:target_size]
  checksum = calc_checksum(data)
  return checksum

def analytic_answer(seed, target_size):
  # Observations about the checksum:
  # - A bitstring of length N*(2**X) will eventually produce a checksum of
  #   length N (assuming N isn't divisible by 2)
  # - If N=1, then the final checksum is 1 bit, and that bit is 1 if the
  #   number of 1s in the input is even, and 0 if the number of 1s is odd
  # - If N>1, this generalizes to N chunks, each of which come from the i-th
  #   chunk of the input (which is of size 2**X) and can be reduced to
  #   a single bit by counting 1s as described above
  # So given the puzzle's setup where (at least for me) the target size is
  # an N*(2**X) situation, then if you can generate the chunks bit-by-bit,
  # you can calculate the checksum bit-by-bit directly.
  #
  # ...Unfortunately, I can't figure out how to do that. Like, you can see
  # the final result is a series of repetitions of the original string
  # followed by the original string reversed and flipped, interspersed
  # with 1s and 0s, but I don't see any obvious pattern to the 1s and 0s
  # such that you could generate them for a particular size. For the first
  # four, it's
  #   orig 0 ORIG
  #   orig 0 ORIG 0 orig 1 ORIG
  #   orig 0 ORIG 0 orig 1 ORIG 0 orig 0 ORIG 1 orig 1 ORIG
  #   orig 0 ORIG 0 orig 1 ORIG 0 orig 0 ORIG 1 orig 1 ORIG 0 orig 0 ORIG 0 orig 1 ORIG 1 orig 0 ORIG 1 orig 1 ORIG
  # or just looking at the 0s/1s, it's
  #   0
  #   001
  #   0010011
  #   001001100011011
  # which has no particularly obvious pattern. You could grow the thing from
  # 0 and then intersperse the original string as appropriate, but that 
  # seems against the spirit of the analytic approach
  return ""

def run(input_file):
  with open(input_file) as f:
    seed, target_size = parse(line.strip() for line in f)
    gan = generative_answer(seed, target_size)
    aan = analytic_answer(seed, target_size)    
    print("Generative answer is {}".format(gan))
    print("Analytic answer is {}".format(aan))

if __name__ == "__main__":
  input_file = sys.argv[1]
  run(input_file)
