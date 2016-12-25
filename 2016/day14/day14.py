#!/usr/bin/env python3
from hashlib import md5
import os
import re
import sys

def parse(lines):
  for line in lines:
    args = []
    if matches(line, args, r'(\w+) ([0-9]+) ([0-9]+)( \S+)?'):
      if args[3]:
        f = analyzed_hash_stream(cached_hash_stream(args[3].strip()))
      else:
        f = analyzed_hash_stream(computed_hash_stream(args[0]))
      return f, int(args[1]), int(args[2])
    else:
      print("Bad line: {}".format(line))

def matches(line, args, rex):
  m = re.search("^" + rex + "$", line)
  if m:
    args.extend(m.groups())
    return True
  else:
    return False

def computed_hash_stream(salt, inc_size=2):
  base_md5 = md5(bytes(salt, 'utf-8'))

  inc_idx = 0
  suffixes = [ bytes("{}".format(i), 'ascii') for i in
                   range(0, 10**inc_size) ]
  inc_md5 = None
  
  while True:
    inc_md5 = base_md5.copy()
    if inc_idx > 0:
      inc_md5.update(bytes(str(inc_idx), 'ascii'))
    
    for i in range(0, 10**inc_size):
      cur_md5 = inc_md5.copy()
      cur_md5.update(suffixes[i])
      hexed = cur_md5.hexdigest()
      yield hexed

    if inc_idx == 0:
      fmt = "{{:0{}}}".format(inc_size)
      suffixes = [ bytes(fmt.format(i), 'ascii') for i in
                     range(0, 10**inc_size) ]
    inc_idx += 1

def cached_hash_stream(hash_file):
  if hash_file:
    with open(hash_file) as f:
      for line in f:
        yield line.strip()

def analyzed_hash_stream(source):
  idx = 0
  for hexed in source:
    run3s = get_runs(hexed, width=3)
    if run3s:
      run5s = get_runs(hexed, width=5)
      yield {"hash": hexed, "idx": idx, "run3": run3s[0], "run5s": run5s}
    idx += 1

def find_keys(buf, keys):
  existing_keys = set(k["key_idx"] for k in keys)

  for run5 in set(buf[-1]["run5s"]):
    for candidate in buf[:-1]:
      # the same key might be made live by two confirms
      if candidate["idx"] in existing_keys:
        continue
      if candidate["run3"] == run5:
        keys.append({"key_idx": candidate["idx"], "key": candidate["hash"],
                     "confirm_idx": buf[-1]["idx"], "confirm": buf[-1]["hash"]
                       })
#        print("found key: {}".format(keys[-1]))

def get_runs(txt, width):
  runs = []
  for i in range(len(txt) - width + 1):
    if all(txt[i] == txt[i+j+1] for j in range(width - 1)):
      runs.append(txt[i])
  return runs

def run(input_file):
  with open(input_file) as f:
    hashes, window, num_keys = parse(line.strip() for line in f)

  window += 1 # we say "the next 1000", so we mean it has 1001 entries
  keys = []
  buf = []

  stop_idx = None
  for cur in hashes:
    if stop_idx is not None and cur["idx"] > stop_idx:
      break
    buf.append(cur)
    while buf[0]["idx"] + window <= cur["idx"]:
      buf.pop(0)

    find_keys(buf, keys)
    if len(keys) >= num_keys and stop_idx is None:
      stop_idx = cur["idx"] + window

  # now clean up a bit:
  keys = sorted(keys, key=lambda k: k["key_idx"])
  keys = keys[:num_keys]
  print("Last key: {}".format(keys[-1]))
  print("Stop index: {}".format(stop_idx))

if __name__ == "__main__":
  input_file = sys.argv[1]
  run(input_file)
