#!/usr/bin/env python3
from hashlib import md5
import sys

INC_SIZE = 2

def interesting_hashes(door):
  base_md5 = md5(bytes(door, 'utf-8'))

  inc_idx = 0
  suffixes = [ bytes("{}".format(i), 'utf-8') for i in
                   range(0, 10**INC_SIZE) ]
  inc_md5 = None
  
  while True:
    inc_md5 = base_md5.copy()
    if inc_idx > 0:
      inc_md5.update(bytes(str(inc_idx), 'utf-8'))
    
    for i in range(0, 10**INC_SIZE):
      cur_md5 = inc_md5.copy()
      cur_md5.update(suffixes[i])
      hexed = cur_md5.hexdigest()
      if hexed.startswith("00000"):
        yield hexed

    if inc_idx == 0:
      fmt = "{{:0{}}}".format(INC_SIZE)
      suffixes = [ bytes(fmt.format(i), 'utf-8') for i in
                     range(0, 10**INC_SIZE) ]
    inc_idx += 1

def compute_password(door):
  size = 8
  password1 = ""
  password2 = [None] * size

  for h in interesting_hashes(door):
    password1 += h[5]
    if h[5] >= "0" and h[5] <= "9":
      h_idx = int(h[5])
      if h_idx < size and password2[h_idx] is None:
        password2[h_idx] = h[6]
      if all([p is not None for p in password2]):
        return password1[0:size], "".join(password2)
  
def run(file):
  with open(file) as f:
    for line in f.readlines():
      door = line[:-1]
      print("password for {} is {} / {}".format(door, *compute_password(door)))

if __name__ == "__main__":
  run(sys.argv[1])
