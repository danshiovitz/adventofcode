#!/usr/bin/env python3
import sys
import re

def parse(lines):
  instructions = []
  for line in lines:
    args = []
    if matches(line, args, r'cpy (-?[0-9]+|[a-z]) ([a-z])'):
      instructions.append(('cpy', *args))
    elif matches(line, args, r'(inc|dec) ([a-z])'):
      instructions.append(tuple(args))
    elif matches(line, args, r'jnz (-?[0-9]+|[a-z]) (-?[0-9]+|[a-z])'):
      instructions.append(('jnz', *args))
    else:
      print("Bad line: {}".format(line))
  return instructions

def matches(line, args, rex):
  m = re.search("^" + rex + "$", line)
  if m:
    args.extend(m.groups())
    return True
  else:
    return False

def parse_init_vals(txt):
  ret = {k: 0 for k in ("a", "b", "c", "d")}
  for assign in re.split(r',\s*', txt):
    if len(assign) > 0:
      pieces = re.split(r'=', assign)
      ret[pieces[0]] = int(pieces[1])
  return ret

def exec_instructions(instructions, init_vals):
  state = init_vals.copy()
  state["pc"] = 0
  
  def read_arg(arg):
    if arg.isalpha():
      return state[arg]
    else:
      return int(arg)
  def write_arg(arg, value):
    if arg.isalpha():
      state[arg] = value
    else:
      print("Can't write {}".format(arg))

  while state["pc"] < len(instructions):
    if state["pc"] < 0:
      print("Segementation fault. Core dumped. -more-")
      return
    func_name, *args = instructions[state["pc"]]
    globals()[func_name + "_func"](*args, read_arg, write_arg)
  print("Final state: {}".format(state))

def cpy_func(arg1, arg2, read_arg, write_arg):
  write_arg(arg2, read_arg(arg1))
  write_arg("pc", read_arg("pc") + 1)

def inc_func(arg1, read_arg, write_arg):
  write_arg(arg1, read_arg(arg1) + 1)
  write_arg("pc", read_arg("pc") + 1)

def dec_func(arg1, read_arg, write_arg):
  write_arg(arg1, read_arg(arg1) - 1)
  write_arg("pc", read_arg("pc") + 1)

def jnz_func(arg1, arg2, read_arg, write_arg):
  if read_arg(arg1) == 0:
    write_arg("pc", read_arg("pc") + 1)
  else:
    write_arg("pc", read_arg("pc") + read_arg(arg2))

def run(input_file, init_vals_str):
  with open(input_file) as f:
    instructions = parse(line.strip() for line in f)
    init_vals = parse_init_vals(init_vals_str)
    exec_instructions(instructions, init_vals)

if __name__ == "__main__":
  input_file = sys.argv[1]
  init_vals = sys.argv[2] if len(sys.argv) > 2 else ""
  run(input_file, init_vals)
