#!/usr/bin/env python3
import sys
import re

def decompress(line):
  ret = ""
  while True:
    m = re.search(r'\(([0-9]+)x([0-9]+)\)', line)
    if m:
      ret += line[:m.start()]
      line = line[m.end():]
      amt = int(m.group(1))
      times = int(m.group(2))
      if len(line) < amt:
        print("Can't grab {} chars at {}".format(amt, line))
        continue
      ret += line[:amt] * times
      line = line[amt:]
    else:
      ret += line
      return ret

def decompress_len(line, v2):
  ret = 0
  while True:
    m = re.search(r'\(([0-9]+)x([0-9]+)\)', line)
    if m:
      ret += m.start()
      line = line[m.end():]
      amt = int(m.group(1))
      times = int(m.group(2))
      if len(line) < amt:
        print("Can't grab {} chars at {}".format(amt, line))
        continue
      ret += (decompress_len(line[:amt], v2) if v2 else amt) * times
      line = line[amt:]
    else:
      ret += len(line)
      return ret
    
def run(input_file):
  with open(input_file) as f:
    for line in f.readlines():
      trimmed = line.strip()
      d = decompress(trimmed)
      dl = decompress_len(trimmed, False)
      dl2 = decompress_len(trimmed, True)      
      print("decompressed length is {} (v1) / {} (v2)".format(dl, dl2))

if __name__ == "__main__":
  input_file = sys.argv[1]
  run(input_file)
