#!/usr/bin/env python3
from collections import Counter
import sys
  
def run(file):
  counters = None
  with open(file) as f:
    for line in f.readlines():
      trimmed = line.strip()
      if counters is None:
        counters = [ Counter() for i in range(0, len(trimmed)) ]
      for i in range(0, len(trimmed)):
        counters[i][trimmed[i]] += 1
  msg1 = "".join(c.most_common(1)[0][0] for c in counters)
  msg2 = "".join(c.most_common()[-1][0] for c in counters)  
  print("corrected message 1 is {}".format(msg1))
  print("corrected message 2 is {}".format(msg2))  

if __name__ == "__main__":
  run(sys.argv[1])
