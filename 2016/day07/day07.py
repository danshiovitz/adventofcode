#!/usr/bin/env python3
import sys
import re

def has_abba(s):
  for i in range(0, len(s) - 3):
    if s[i] == s[i+3] and s[i+1] == s[i+2] and s[i] != s[i+1]:
      return True
  return False

def supports_tls(s):
  has_anywhere = False
  for piece in re.split("(\[\w*\])", s):
    if len(piece) == 0: continue
    if piece[0] == '[':
      if has_abba(piece[1:-1]):
        return False
    else:
      if has_abba(piece):
        has_anywhere = True
  return has_anywhere

def get_abas(s):
  ret = set()
  for i in range(0, len(s) - 2):
    if s[i] == s[i+2] and s[i] != s[i+1]:
      ret.add(s[i:i+3])
  return ret
  
def supports_ssl(s):
  abas = set()
  babs = set()
  for piece in re.split("(\[\w*\])", s):
    if len(piece) == 0: continue
    if piece[0] == '[':
      babs |= get_abas(piece[1:-1])
    else:
      abas |= get_abas(piece)

  for aba in abas:
    bab = aba[1] + aba[0] + aba[1]
    if bab in babs:
      return True
  return False

def run(file):
  tls_count = 0
  ssl_count = 0  
  with open(file) as f:
    for line in f.readlines():
      trimmed = line.strip()
      if supports_tls(trimmed):
        tls_count += 1
      if supports_ssl(trimmed):
        ssl_count += 1
  print("tls_count is {}".format(tls_count))
  print("ssl_count is {}".format(ssl_count))

if __name__ == "__main__":
  run(sys.argv[1])
