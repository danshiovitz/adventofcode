import re
import tables
import strutils

proc toString(str: seq[char]): string =
  result = newStringOfCap(len(str))
  for ch in str:
    add(result, ch)
  return result

var boxes = newSeq[string]()
for line in lines "input.txt":
  boxes.add(line)

var has2 = 0
var has3 = 0
for line in boxes:
  var cnts = initTable[char, int]()
  for c in line:
    discard cnts.hasKeyOrPut(c, 0)
    cnts[c] += 1
  var any2 = false
  var any3 = false
  for k, v in cnts.pairs():
    if v == 2:
      any2 = true
    if v == 3:
      any3 = true
  if any2:
    has2 += 1
  if any3:
    has3 += 1

echo "Checksum ", (has2 * has3)

block loop:
  for box1 in boxes:
    for box2 in boxes:
      var common = newSeq[char]()
      for i in 0..box1.len() - 1:
        if box1[i] == box2[i]:
          common.add(box1[i])
      if common.len() == box1.len() - 1:
        echo "box1=", box1, " box2=", box2
        echo "Common: ", toString(common)
        break loop
      
