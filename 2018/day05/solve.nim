import algorithm
import os
import re
import math
import tables
import sequtils
import sets
import strutils
import system

proc readInput(f: string): string =
  if not fileExists(f):
    return f
  return readFile(f).strip()

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

proc doReactions(str: string): string =
  var ret = str
  var i = 0
  while i < len(ret) - 1:
    if toLowerAscii(ret[i]) == toLowerAscii(ret[i+1]) and ret[i] != ret[i+1]:
      ret[i..i+1] = ""
      if i > 0:
        i -= 1
    else:
      i += 1
  return ret

proc bestRemoved(str: string): (string, string, int) =
  var types = initSet[string]()
  for c in str:
    types.incl(toLowerAscii($c))

  var best_c = ""
  var best_str = ""
  var best_len = len(str) + 1
  for c in types:
    let no_c = str.multiReplace((c, ""), (toUpperAscii(c), ""))
    let reacted = doReactions(no_c)
    let cur_len = len(reacted)
    if cur_len < best_len:
      best_c = c
      best_str = reacted
      best_len = cur_len
  return (best_c, best_str, best_len)

proc main(): void =
  let line = readInput(paramStr(1))
  let reacted = doReactions(line)
  echo "Final: ", line[0..min(99, len(line) - 1)]
  echo "Final size: ", len(reacted)
  let (best_c, best_str, best_len) = bestRemoved(line)
  echo "Best to remove ", best_c, ": ", line[0..min(99, best_len - 1)], " (", best_len, ")"

main()
