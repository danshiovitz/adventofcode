import algorithm
import os
import re
import math
import tables
import sequtils
import sets
import strutils
import system

proc readInput(f: string): seq[string] =
  var ret = newSeq[string]()
  for line in lines f:
    ret.add(line)
  sort(ret, system.cmp[string])
  return ret

proc fail(msg: string): void =
  var e: ref Exception
  new(e)
  e.msg = msg
  raise e

proc toString(str: seq[char]): string =
  result = newStringOfCap(len(str))
  for ch in str:
    add(result, ch)
  return result

type Guard = tuple[id: int, sleepCounts: array[60, int]]

proc parseGuards(lines: seq[string]): seq[Guard] =
  var guards = initTable[int, Guard]()
  var i = 0
  while i < len(lines):
    var matches: array[2, string]
    if match(lines[i], re"\[[0-9]+-[0-9]+-[0-9]+ [0-9]+:([0-9]+)\] Guard #([0-9]+) begins shift", matches):
      var id = parseInt(matches[1])
      var sc: array[60, int]
      discard guards.hasKeyOrPut(id, (id, sc))
      i += 1
      var startMin = 0
      while i < len(lines):
        if match(lines[i], re"\[[0-9]+-[0-9]+-[0-9]+ [0-9]+:([0-9]+)\] falls asleep", matches):
          startMin = parseInt(matches[0])
          i += 1
        elif match(lines[i], re"\[[0-9]+-[0-9]+-[0-9]+ [0-9]+:([0-9]+)\] wakes up", matches):
          var endMin = parseInt(matches[0])
          if startMin > endMin:
            fail("Something is wrong around " & lines[i])
          for m in startMin..endMin-1:
            guards[id].sleepCounts[m] += 1
          i += 1
          # echo "Guard ", id, " sleeps from ", startMin, " to ", endMin
        else:
          break
    else:
      fail("Bad line: " & lines[i])
  return toSeq(guards.values())

proc sleepiestGuard1(guards: seq[Guard]): (int, int) =
  var best_tot = 0
  var ret = (0, 0)
  for guard in guards:
    var mmax = 0
    var tot = 0
    for m in 0..59:
      if guard.sleepCounts[m] > guard.sleepCounts[mmax]:
        mmax = m
      tot += guard.sleepCounts[m]
    if tot > best_tot:
      best_tot = tot
      ret = (guard.id, mmax)
  return ret

proc sleepiestGuard2(guards: seq[Guard]): (int, int) =
  var best_tot = 0
  var ret = (0, 0)
  for guard in guards:
    for m in 0..59:
      if guard.sleepCounts[m] > best_tot:
        best_tot = guard.sleepCounts[m]
        ret = (guard.id, m)
  return ret

proc main(): void =
  var lines = readInput(paramStr(1))
  var guards = parseGuards(lines)
  var (id, m) = sleepiestGuard1(guards)
  echo "Sleepy 1 is ", id, " * ", m, " = ", (id * m)
  (id, m) = sleepiestGuard2(guards)
  echo "Sleepy 2 is ", id, " * ", m, " = ", (id * m)

main()
