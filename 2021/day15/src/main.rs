use itertools::iproduct;
use std::collections::HashMap;

extern crate common;

use common::framework::{parse_grid, run_day, BaseDay, InputReader};
use common::grid::{Coord, Grid, four_neighbors};

struct Day15 {
    vals: Grid<i32>,
}

fn eff_risk(grid: &Grid<i32>, coord: Coord, mult: i32) -> Option<i32> {
    if coord.x >= (grid.max.x + 1) * mult || coord.x < 0 || coord.y >= (grid.max.y + 1) * mult || coord.y < 0 {
        return None;
    }
    let ex = coord.x % (grid.max.x + 1);
    let ey = coord.y % (grid.max.y + 1);
    let rb = coord.x / (grid.max.x + 1) + coord.y / (grid.max.y + 1);
    return grid.coords.get(&Coord {x: ex, y: ey}).map(|v| ((*v + rb - 1) % 9) + 1);
}

fn compute_costs(grid: &Grid<i32>, start: Coord, end: Coord, mult: i32) -> i32 {
    let mut costs: HashMap<Coord, i32> = iproduct!(0..(grid.max.x + 1) * mult, 0..(grid.max.y + 1) * mult).map(|(x, y)| (Coord {x: x, y: y}, i32::MAX)).collect();
    costs.insert(start, 0);
    let mut working = vec![start];
    while !working.is_empty() {
        let cur = working.remove(0);
        let cur_cost = *costs.get(&cur).unwrap();
        for ngh in four_neighbors(&cur) {
            if let Some(ngh_risk) = eff_risk(&grid, ngh, mult) {
                let existing_cost = *costs.get(&ngh).unwrap();
                let via_cur_cost = cur_cost + ngh_risk;
                if via_cur_cost < existing_cost {
                    // found a cheaper way to get to ngh, so have to recalc it
                    costs.insert(ngh, via_cur_cost);
                    working.push(ngh);
                }
            }
        }
    }

    return *costs.get(&end).unwrap();
}

impl BaseDay for Day15 {
    fn parse(&mut self, input: &mut InputReader) {
        fn parse_coord(c: char, _coord: &Coord) -> Option<i32> {
            let val = c.to_digit(10).unwrap() as i32;
            Some(val)
        }

        self.vals = parse_grid(input, &mut parse_coord);
    }

    fn pt1(&mut self) -> String {
        let cost = compute_costs(&self.vals, self.vals.min, self.vals.max, 1);
        return cost.to_string();
    }

    fn pt2(&mut self) -> String {
        let eff_max = Coord { x: (self.vals.max.x + 1) * 5 - 1, y: (self.vals.max.y + 1) * 5 - 1};
        let cost = compute_costs(&self.vals, self.vals.min, eff_max, 5);
        return cost.to_string();
    }
}

fn main() {
    let mut day = Day15 { vals: Grid::new() };
    run_day(&mut day);
}
