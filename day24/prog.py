#!/usr/bin/env python3
import sys
from operator import mul
from functools import reduce
from itertools import combinations
from itertools import chain

def run(file, num_groups):
  weights = load_weights(file)
  best_arrangement = min(make_arrangements(weights, num_groups),
                         key=lambda a: by_size_then_qe(a[0]))
  print("Best arrangement (QE={}): {}".format(reduce(mul, best_arrangement[0], 1),
                                              best_arrangement))

def load_weights(file):
  with open(file) as f:
    return [int(line.strip()) for line in f.readlines()]

def make_arrangements(weights, num_groups):
  total_weight = sum(weights)
  if total_weight % num_groups != 0:
    raise Exception("Total weight must be divisible by %d".format(num_groups))
  target = total_weight // num_groups

  weight_set = set(weights)
  if len(weight_set) != len(weights):
    raise Exception("For the current implementation, weights must be unique")

  # the goal is to find the smallest group, so we start by generating all
  # size-1 groups, then size-2 groups, etc, and validate that they're a
  # valid solution
  for size in range(1, (len(weights) // num_groups) + 1):
    print("trying size {}....".format(size))
    found_any = False
    for group in combinations(weights, size):
      if sum(group) != target:
        continue

      arrangement = [list(group)]
      remaining = weight_set.difference(set(group))

      # now once we've established the first group, we have to attempt to build all the
      # remaining groups out of what's left over
      failed = False
      for i in range(1, num_groups):
        try:
          next_group = next(groups_of_weight(list(remaining), target))
        except StopIteration:
          failed = True # this means we couldn't make a group from the remainder
          break
        arrangement.append(next_group)
        remaining = remaining.difference(set(next_group))

      if not failed:
        if len(remaining) > 0:
          raise Exception("Something's wrong, not all weights were used: {} (used {})".format(remaining, arrangement))
        found_any = True
        yield arrangement

    # if we found any with this size, one of them will be the best one,
    # so we don't have to go onto the next size
    if found_any:
      break

def groups_of_weight(weights, target):
  if len(weights) == 0:
    if target == 0:
      yield []
    return
  if weights[0] <= target:
    for g in groups_of_weight(weights[1:], target - weights[0]):
      yield g + [weights[0]]
  for g in groups_of_weight(weights[1:], target):
    yield g
  
def by_size_then_qe(group):
  return (len(group), reduce(mul, group, 1))
          
if __name__ == "__main__":
  run(sys.argv[1], int(sys.argv[2]) if len(sys.argv) > 2 else 3)

