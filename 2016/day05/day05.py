#!/usr/bin/env python3
from hashlib import md5
import sys
  
def compute_password(door):
  password = ""
  idx = 0

  db = bytes(door, 'utf-8')
  while len(password) < 8:
    hexed = md5(db + bytes(str(idx), 'utf-8')).hexdigest()
    if hexed.startswith("00000"):
        password += hexed[5]
    idx += 1
#    if idx % 100000 == 0:
#      print("{}: {}".format(idx, len(password)))
  return password
  
def run(file):
  with open(file) as f:
    for line in f.readlines():
      door = line[:-1]
      print("password for {} is {}".format(door, compute_password(door)))

if __name__ == "__main__":
  run(sys.argv[1])
