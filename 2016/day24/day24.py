#!/usr/bin/env python3
from itertools import permutations
import sys
import re

EMPTY = "."
WALL = "#"

def parse(lines):
  grid = {}
  max_y = 0
  for line in lines:
    for x in range(len(line)):
      grid[(x, max_y)] = line[x]
    max_y += 1
    max_x = len(line)

  poi = {}
  for y in range(max_y):
    for x in range(max_x):
      if grid[(x, y)] not in (EMPTY, WALL):
        poi[grid[(x, y)]] = (x, y)
        grid[(x, y)] = EMPTY
  return grid, poi

def calc_poi_distances(poi, grid):
  coi = {v: k for k, v in poi.items()}

  def bfs(source_poi):
    active = [(poi[source_poi], 0)]
    seen = set()
    distances = {}
    while active:
      coord, steps = active.pop(0)
      if coord in seen:
        continue
      if coord in coi:
        distances[coi[coord]] = steps
        if len(distances) == len(coi):
          return distances
      seen.add(coord)
      for mod in ((-1, 0), (1, 0), (0, -1), (0, 1)):
        next_coord = (coord[0] + mod[0], coord[1] + mod[1])
        if grid[next_coord] == EMPTY:
          active.append((next_coord, steps + 1))
    print("Couldn't find all points :C")

  return {p: bfs(p) for p in poi}

def calc_shortest_path(poi_distances, grid, with_zero):
  points = set(p for p in poi_distances.keys() if p != "0")

  best_path = None
  best_dist = 0
  for path in permutations(points, len(points)):
    full_path = ["0"] + list(path) + (["0"] if with_zero else [])
    path_dist = sum(poi_distances[full_path[i]][full_path[i+1]]
                      for i in range(len(full_path) - 1))
    if best_path is None or path_dist < best_dist:
      best_path = full_path
      best_dist = path_dist
  print("Best path: {}".format(full_path))
  return best_dist

def run(input_file):
  with open(input_file) as f:
    grid, poi = parse(line.strip() for line in f)
  poi_distances = calc_poi_distances(poi, grid)
  shortest_path_size = calc_shortest_path(poi_distances, grid, False) 
  print("Shortest path: {}".format(shortest_path_size))
  shortest_path_size = calc_shortest_path(poi_distances, grid, True) 
  print("Shortest cycle: {}".format(shortest_path_size))

if __name__ == "__main__":
  input_file = sys.argv[1]
  run(input_file)
