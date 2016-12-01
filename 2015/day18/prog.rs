use std::collections::hash_map::HashMap;
use std::env;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::path::Path;

type Point = (i32, i32);
type Grid = HashMap<Point, char>;

fn run(filename: &String, steps: i32) {
  let init_grid = load_grid(filename);
  println!("Initial:");
//  display_grid(&init_grid);
  let fixed_points = &get_fixed_points(&init_grid);
  let final_grid = run_steps(&init_grid, steps, fixed_points);
  let num_lights = count_lights(&final_grid);
  println!("Number of lights on after {} steps: {}", steps, num_lights);
}

fn load_grid(filename: &String) -> Grid {
  let mut grid = HashMap::new();

  let path = Path::new(filename);
  let file = BufReader::new(File::open(&path).unwrap());
  for (i, line) in file.lines().filter_map(|result| result.ok()).enumerate() {
    for (j, c) in line.chars().enumerate() {
      grid.insert((i as i32, j as i32), c);
    }
  }

  return grid;
}

fn get_fixed_points(grid: &Grid) -> Vec<Point> {
  let max_i = grid.keys().map(|&(i, _)| i).max().unwrap();
  let max_j = grid.keys().map(|&(_, j)| j).max().unwrap();
  return vec![(0, 0), (max_i, 0), (0, max_j), (max_i, max_j)];
}

fn run_steps(grid: &Grid, steps: i32, fixed_points: &Vec<Point>) -> Grid {
  if steps <= 0 {
    return grid.clone();
  } else {
    return run_steps(&single_step(grid, fixed_points), steps - 1, fixed_points);
  }
}

fn single_step(grid: &Grid, fixed_points: &Vec<Point>) -> Grid {
  let mut new_grid = HashMap::new();
  for point in grid.keys() {
    if fixed_points.contains(point) {
      new_grid.insert(*point, '#');
      continue;
    }

    let neighbors = get_neighbors(point, grid);
    let neighbor_count = neighbors.iter().map(|p| if grid[p] == '#' { 1 } else { 0 }).fold(0, |sum, i| sum + i);
    if (grid[point] == '#' && neighbor_count == 2) || neighbor_count == 3 {
      new_grid.insert(*point, '#');
    } else {
      new_grid.insert(*point, '.');
    }
  }
  return new_grid;
}

fn get_neighbors(point: &Point, grid: &Grid) -> Vec<Point> {
  let &(i, j) = point;
  let neighbor_offsets = vec![(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
  return neighbor_offsets.iter().map(|&(a,b)| (i+a,j+b)).filter(|n| grid.contains_key(n)).collect();
}

fn count_lights(grid: &Grid) -> i32 {
  return grid.values().map(|c| if *c == '#' { 1 } else { 0 }).fold(0, |sum, i| sum + i);
}

fn display_grid(grid: &Grid) {
  let max_i = grid.keys().map(|&(i, _)| i).max().unwrap();
  let max_j = grid.keys().map(|&(_, j)| j).max().unwrap();
  for i in 0..max_i+1 {
    for j in 0..max_j+1 {
      print!("{}", grid[&(i, j)]);
    }
    println!("");
  }
  println!("");
}

fn main() {
  let args: Vec<_> = env::args().collect();
  let filename = args.get(1).unwrap();
  let steps = args.get(2).unwrap().parse::<i32>().ok().unwrap();
  run(filename, steps);
}
