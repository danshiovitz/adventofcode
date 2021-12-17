use std::collections::HashSet;

extern crate common;

use common::framework::{parse_grid, run_day, BaseDay, InputReader};
use common::grid::{eight_neighbors, print_grid, Coord, Grid};

struct Day11 {
    vals: Grid<i32>,
}

fn print_one(_c: &Coord, maybe_val: Option<&i32>) -> String {
    let val = *maybe_val.unwrap();
    if val > 9 {
        return "*".to_owned();
    }
    return val.to_string().to_owned();
}

fn run_flashes(grid: &Grid<i32>, pt1: bool, verbose: bool) -> i32 {
    let mut cur_grid = grid.coords.clone();
    let mut flash_count = 0;

    if verbose {
        println!("Initial");
        print_grid(&grid, &mut print_one);
        println!();
    }

    let num_steps = if pt1 { 100 } else { 10000 };
    for step in 0..num_steps {
        let mut should_flash: Vec<Coord> = Vec::new();
        let mut has_flashed = HashSet::new();
        for (k, v) in cur_grid.iter_mut() {
            *v += 1;
            if *v > 9 {
                should_flash.push(*k);
            }
        }

        while let Some(cur) = should_flash.pop() {
            if has_flashed.contains(&cur) {
                continue;
            }
            flash_count += 1;
            has_flashed.insert(cur);
            for ngh in eight_neighbors(&cur) {
                if let Some(nv) = cur_grid.get_mut(&ngh) {
                    *nv += 1;
                    if *nv > 9 {
                        should_flash.push(ngh);
                    }
                }
            }
        }

        for cur in has_flashed.iter() {
            cur_grid.insert(*cur, 0);
        }

        if verbose {
            let grid_copy = Grid {
                min: grid.min,
                max: grid.max,
                coords: cur_grid.clone(),
            };
            println!("Step {}", step + 1);
            print_grid(&grid_copy, &mut print_one);
            println!();
        }

        if !pt1 && has_flashed.len() == 100 {
            return step + 1;
        }
    }

    return flash_count;
}

impl BaseDay for Day11 {
    fn parse(&mut self, input: &mut InputReader) {
        fn parse_coord(c: char, _coord: &Coord) -> Option<i32> {
            let val = c.to_digit(10).unwrap() as i32;
            Some(val)
        }

        self.vals = parse_grid(input, &mut parse_coord);
    }

    fn pt1(&mut self) -> String {
        let flash_count = run_flashes(&self.vals, true, false);
        return flash_count.to_string();
    }

    fn pt2(&mut self) -> String {
        let sync_step = run_flashes(&self.vals, false, false);
        return sync_step.to_string();
    }
}

fn main() {
    let mut day = Day11 { vals: Grid::new() };
    run_day(&mut day);
}
