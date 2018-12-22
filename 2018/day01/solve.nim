import re
import sets
import strutils

var foundDup = false
var foundTot = false

var firstTot = 0
var dup = 0
var seen = initSet[int]()

var tot = 0
while not foundDup or not foundTot:
  for line in lines "input.txt":
    tot += parseInt(line)
    if seen.contains(tot) and not foundDup:
      dup = tot
      foundDup = true
    else:
      seen.incl(tot)
  if not foundTot:
    firstTot = tot
    foundTot = true

echo "Tot is ", firstTot
echo "Dupe is ", dup
