#!/usr/bin/ruby

def run(filename, size, is_digital)
  grid = Array.new(size) { Array.new(size, 0) }
  actions = action_map(is_digital)
  File.foreach(filename).with_index do |line, line_num|
    line.chop!
    next if line.length == 0
    exec_line(line, grid, actions)
  end
  if is_digital
    puts "Total brightness: %s" % [grid.flatten.inject(0, :+)]
  else
    puts "Lights on: %s" % [grid.flatten.count(1)]
  end
end

def exec_line(line, grid, actions)
  m = /^(.*?) ([0-9]+),([0-9]+) through ([0-9]+),([0-9]+)/.match(line)
  raise "Bad line: #{line}" unless m
  raise "Bad line action: #{line}" unless actions.has_key?(m[1])
  apply(m[2].to_i, m[3].to_i, m[4].to_i, m[5].to_i, grid, actions[m[1]])
end

def action_map(is_digital)
  if is_digital
    return {
      "turn on" => lambda {|v| v+1},
      "turn off" => lambda {|v| [v-1, 0].max},
      "toggle" => lambda {|v| v+2},
    }
  else
    return {
      "turn on" => lambda {|v| 1},
      "turn off" => lambda {|v| 0},
      "toggle" => lambda {|v| v == 1 ? 0 : 1},
    }
  end
end

def apply(from_x, from_y, to_x, to_y, grid, action)
  for x in from_x..to_x
    for y in from_y..to_y
      grid[y][x] = action.call(grid[y][x])
    end
  end
end

is_digital = false
if ARGV[0] == "--digital"
  is_digital = true
  ARGV.shift
end
if ARGV[0] == "--analog"
  is_digital = false
  ARGV.shift
end
run(ARGV[0], ARGV.length > 1 ? ARGV[1].to_i : 1000, is_digital)
