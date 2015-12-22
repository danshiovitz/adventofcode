#!/usr/bin/ruby

def run(filename, size)
  grid = Array.new(size) { Array.new(size, false) }
  File.foreach(filename).with_index do |line, line_num|
    line.chop!
    next if line.length == 0
    exec_line(line, grid)
  end
  puts "Lights on: %s" % [grid.flatten.count(true)]
end

def exec_line(line, grid)
  m = /^(.*?) ([0-9]+),([0-9]+) through ([0-9]+),([0-9]+)/.match(line)
  raise "Bad line: #{line}" unless m
  if m[1] == "turn on"
    apply(m[2].to_i, m[3].to_i, m[4].to_i, m[5].to_i, grid) {|v| true}
  elsif m[1] == "turn off"
    apply(m[2].to_i, m[3].to_i, m[4].to_i, m[5].to_i, grid) {|v| false}
  elsif m[1] == "toggle"
    apply(m[2].to_i, m[3].to_i, m[4].to_i, m[5].to_i, grid) {|v| !v}
  else
    raise "Bad line action: #{line}"
  end
end

def apply(from_x, from_y, to_x, to_y, grid)
  for x in from_x..to_x
    for y in from_y..to_y
      grid[y][x] = yield(grid[y][x])
    end
  end
end

run(ARGV[0], ARGV.length > 1 ? ARGV[1].to_i : 1000)
