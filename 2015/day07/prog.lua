require "bit32"
require "string"

function make_and(vals)
  return function(defs)
    return bit32.band(0xFFFF,
       bit32.band(eval_wire(vals[1], defs), eval_wire(vals[2], defs)))
  end
end

function make_or(vals)
  return function(defs)
    return bit32.band(0xFFFF,
       bit32.bor(eval_wire(vals[1], defs), eval_wire(vals[2], defs)))
  end
end

function make_not(vals)
  return function(defs)
    return bit32.band(0xFFFF, 
       bit32.bnot(eval_wire(vals[1], defs)))
  end
end

function make_lshift(vals)
  return function(defs)
    return bit32.band(0xFFFF, 
       bit32.lshift(eval_wire(vals[1], defs), eval_wire(vals[2], defs)))
  end
end

function make_rshift(vals)
  return function(defs)
    return bit32.band(0xFFFF, 
       bit32.rshift(eval_wire(vals[1], defs), eval_wire(vals[2], defs)))
  end
end

function make_assign(vals)
  return function(defs)
    return bit32.band(0xFFFF, 
       eval_wire(vals[1], defs))
  end
end

ops = {
  ["^(%S+) AND (%S+)$"] = make_and,
  ["^(%S+) OR (%S+)$"] = make_or,
  ["^NOT (%S+)$"] = make_not,
  ["^(%S+) LSHIFT (%S+)$"] = make_lshift,
  ["^(%S+) RSHIFT (%S+)$"] = make_rshift,
  ["^(%S+)$"] = make_assign
}

function parse(line)
  for pat, func in pairs(ops) do
    vals = {line:match(pat)}
    if vals[1] ~= nil then
      return func(vals)
    end
  end

  error(string.format("unexpected line: %s", line))
end

function eval_wire(wire, defs)
  numval = string.match(wire, "^(%d+)$")
  if numval ~= nil then
    return numval
  end

  d = defs[wire]
  if type(d) == "function" then
    d = d(defs)
    defs[wire] = d
  end
  return d
end

function run(file, wires)
  defs = {}
  for line in io.lines(file) do
    if line:len() > 0 then
      i, j = line:find("(%s*)%->(%s*)")
      func = parse(line:sub(1, i):gsub("(%s*)$", ""))
      dest = line:sub(j):gsub("^(%s*)", "")
      defs[dest] = func
    end
  end

  for i, wire in ipairs(wires) do
    print(string.format("%s: %s", wire, eval_wire(wire, defs)))
  end
end

file = table.remove(arg, 1)
run(file, arg)
