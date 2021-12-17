use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;

extern crate common;

use common::framework::{parse_grid, run_day, BaseDay, InputReader};
use common::grid::{four_neighbors, Coord, Grid};

struct Day15 {
    vals: Grid<i32>,
}

fn eff_risk(grid: &Grid<i32>, coord: Coord, mult: i32) -> Option<i32> {
    if coord.x >= (grid.max.x + 1) * mult
        || coord.x < 0
        || coord.y >= (grid.max.y + 1) * mult
        || coord.y < 0
    {
        return None;
    }
    let ex = coord.x % (grid.max.x + 1);
    let ey = coord.y % (grid.max.y + 1);
    let rb = coord.x / (grid.max.x + 1) + coord.y / (grid.max.y + 1);
    return grid
        .coords
        .get(&Coord { x: ex, y: ey })
        .map(|v| ((*v + rb - 1) % 9) + 1);
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    estimate: i32,
    coord: Coord,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .estimate
            .cmp(&self.estimate)
            .then_with(|| self.coord.cmp(&other.coord))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn compute_costs(grid: &Grid<i32>, start: Coord, end: Coord, mult: i32) -> i32 {
    let mut costs: HashMap<Coord, i32> = HashMap::new();

    let heuristic = |c: &Coord| -> i32 { (end.x - c.x) + (end.y - c.y) };

    costs.insert(start, 0);
    let mut working = BinaryHeap::new();
    working.push(State {
        estimate: heuristic(&start),
        coord: start.clone(),
    });
    while let Some(cur) = working.pop() {
        let cur_cost = *costs.get(&cur.coord).unwrap_or(&i32::MAX);
        for ngh in four_neighbors(&cur.coord) {
            if let Some(ngh_risk) = eff_risk(&grid, ngh, mult) {
                let existing_cost = *costs.get(&ngh).unwrap_or(&i32::MAX);
                let via_cur_cost = cur_cost + ngh_risk;
                if via_cur_cost < existing_cost {
                    // heuristic never over-estimates, so we can return as soon as we
                    // find the end
                    if ngh == end {
                        return via_cur_cost;
                    }
                    // found a cheaper way to get to ngh, so have to recalc it
                    costs.insert(ngh, via_cur_cost);
                    working.push(State {
                        estimate: via_cur_cost + heuristic(&ngh),
                        coord: ngh,
                    });
                }
            }
        }
    }
    panic!("Never found end?");
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
        let eff_max = Coord {
            x: (self.vals.max.x + 1) * 5 - 1,
            y: (self.vals.max.y + 1) * 5 - 1,
        };
        let cost = compute_costs(&self.vals, self.vals.min, eff_max, 5);
        return cost.to_string();
    }
}

fn main() {
    let mut day = Day15 { vals: Grid::new() };
    run_day(&mut day);
}
