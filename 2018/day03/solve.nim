import os
import re
import tables
import sequtils
import sets
import strutils
import system

proc readInput(f: string): seq[string] =
  var ret = newSeq[string]()
  for line in lines f:
    ret.add(line)
  return ret

type Claim = tuple[id: int, left: int, top: int, width: int, height: int]
type Pos = tuple[x: int, y: int]

proc fail(msg: string): void =
  var e: ref Exception
  new(e)
  e.msg = msg
  raise e

proc parseClaim(str: string): Claim =
  var matches: array[5, string]
  if match(str, re"#([0-9]+) @ ([0-9]+),([0-9]+): ([0-9]+)x([0-9]+)", matches):
    return (parseInt(matches[0]), parseInt(matches[1]), parseInt(matches[2]),
            parseInt(matches[3]), parseInt(matches[4]))
  else:
    fail("Bad claim: " & str)

proc overlapsPos(claim: Claim, pos: Pos): bool =
  return (
    pos.x >= claim.left and pos.x < claim.left + claim.width and
    pos.y >= claim.top and pos.y < claim.top + claim.height
  )

proc overlappingPosSeq(claim1: Claim, claim2: Claim): seq[Pos] =
  var overlaps = newSeq[Pos]()
  # echo "Checking ", claim1, " and ", claim2
  for x in max(claim1.left, claim2.left) .. min(claim1.left + claim1.width, claim2.left + claim2.width) - 1:
    for y in max(claim1.top, claim2.top) .. min(claim1.top + claim1.height, claim2.top + claim2.height) - 1:
      overlaps.add((x, y))
  # echo "Overlaps of ", claim1.id, " and ", claim2.id, ": ", overlaps
  return overlaps

proc allOverlaps(claims: seq[Claim]): (int, int) =
  var tot = 0
  var olaps = initSet[Pos]()
  var rejected = initSet[int]()
  var best = 0
  for i in 0..len(claims)-1:
    var claim1 = claims[i]
    var anyO = false
    for j in i+1..len(claims)-1:
      var claim2 = claims[j]
      var thislap = toSet(overlappingPosSeq(claim1, claim2))
      if len(thislap) > 0:
        anyO = true
        olaps = union(olaps, thislap)
        rejected.incl(claim1.id)
        rejected.incl(claim2.id)
    if not anyO and not rejected.contains(claim1.id):
      best = claim1.id
  return (len(olaps), best)

proc allOverlaps2(claims: seq[Claim]): int =
  let max_x = max(map(claims, proc(c: Claim): int = c.left + c.width))
  let max_y = max(map(claims, proc(c: Claim): int = c.top + c.height))
  var tot = 0
  for x in 0..max_x - 1:
    for y in 0..max_y - 1:
      var oc = 0
      for claim in claims:
        if overlapsPos(claim, (x, y)):
          oc += 1
        if oc > 1:
          tot += 1
          break
  return tot

proc toString(str: seq[char]): string =
  result = newStringOfCap(len(str))
  for ch in str:
    add(result, ch)
  return result

proc main(): void =
  var lines = readInput(paramStr(1))
  var claims = map(lines, parseClaim)
  var (osq, best) = allOverlaps(claims)
  echo "Overlapping squares: ", osq
  echo "Best: ", best

main()
