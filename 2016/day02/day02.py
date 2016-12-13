#!/usr/bin/env python3
import sys

def run(file):
  keypad1 = {c: {d: c for d in "UDLR"} for c in "123456789"}
  for c in "124578": keypad1[c]["R"] = str(int(c) + 1)
  for c in "235689": keypad1[c]["L"] = str(int(c) - 1)
  for c in "123456": keypad1[c]["D"] = str(int(c) + 3)
  for c in "456789": keypad1[c]["U"] = str(int(c) - 3)    

  keypad2 = {c: {d: c for d in "UDLR"} for c in "123456789ABCD"}
  for c in "235678AB": keypad2[c]["R"] = "{:X}".format(int(c, 16) + 1)
  for c in "346789BC": keypad2[c]["L"] = "{:X}".format(int(c, 16) - 1)
  for c in "1B": keypad2[c]["D"] = "{:X}".format(int(c, 16) + 2)      
  for c in "234678": keypad2[c]["D"] = "{:X}".format(int(c, 16) + 4)
  for c in "3D": keypad2[c]["U"] = "{:X}".format(int(c, 16) - 2)      
  for c in "678ABC": keypad2[c]["U"] = "{:X}".format(int(c, 16) - 4)
    
  cur1 = "5"
  cur2 = "5"  
  with open(file) as f:
    combo1 = ""
    combo2 = ""
    for line in f.readlines():
      for c in line[:-1]:
        cur1 = keypad1[cur1][c]
        cur2 = keypad2[cur2][c]        
      combo1 += cur1
      combo2 += cur2
    print("combo (pt1): {}".format(combo1))
    print("combo (pt2): {}".format(combo2))
          
if __name__ == "__main__":
  run(sys.argv[1])
