#!/bin/env ruby

def run(num_presents)
  target = num_presents / 10

  max_house = target
  tots = Hash.new(0)
  (1..max_house).each do |i|
    puts "Up to elf %d..." % [i] if (i+1) % 1000 == 0
    (i..max_house).step(i) {|j| tots[j] += i}
  end

  possibles = tots.find_all {|k,v| v >= target}.sort {|a,b| a[0] <=> b[0]}
#  puts "Possibles: %s" % [possibles]
  puts "Lowest is %s, with %s presents" % [possibles[0][0], possibles[0][1]*10]
end

run(ARGV[0].to_i)

