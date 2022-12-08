use std::collections::{HashMap, HashSet};

extern crate common;

use common::framework::{parse_grid, run_day, BaseDay, InputReader};
use common::grid::{add_direction, print_grid, Coord, Direction, Grid};

struct Day08 {
    vals: Grid<i32>,
}

impl BaseDay for Day08 {
    fn parse(&mut self, input: &mut InputReader) {
        fn parse_coord(c: char, _coord: &Coord) -> Option<i32> {
            let val = c.to_digit(10).unwrap() as i32;
            Some(val)
        }

        self.vals = parse_grid(input, &mut parse_coord);
    }

    fn pt1(&mut self) -> String {
        fn add_visible(
            start: Coord,
            dir: Direction,
            grid: &Grid<i32>,
            visible: &mut HashSet<Coord>,
        ) {
            let mut cur = start;
            let mut cur_max = -1;
            loop {
                if let Some(v) = grid.coords.get(&cur) {
                    if *v > cur_max {
                        visible.insert(cur);
                        cur_max = *v;
                    }
                    cur = add_direction(&cur, &dir);
                } else {
                    return;
                }
            }
        }

        let mut visible = HashSet::new();
        for x in self.vals.min.x..=self.vals.max.x {
            add_visible(
                Coord { x: x, y: self.vals.min.y },
                Direction { dx: 0, dy: 1 },
                &self.vals,
                &mut visible,
            );
            add_visible(
                Coord { x: x, y: self.vals.max.y },
                Direction { dx: 0, dy: -1 },
                &self.vals,
                &mut visible,
            );
        }
        for y in self.vals.min.y..=self.vals.max.y {
            add_visible(
                Coord { x: self.vals.min.x, y: y },
                Direction { dx: 1, dy: 0 },
                &self.vals,
                &mut visible,
            );
            add_visible(
                Coord { x: self.vals.max.x, y: y },
                Direction { dx: -1, dy: 0 },
                &self.vals,
                &mut visible,
            );
        }

        // print_grid(&self.vals, &mut |c: &Coord, _v: Option<&i32>| if visible.contains(c) { "*".to_string() } else { " ".to_string() });
        return visible.len().to_string();
    }

    fn pt2(&mut self) -> String {
        fn count_visible(start: Coord, dir: Direction, grid: &Grid<i32>) -> i32 {
            let limit = *grid.coords.get(&start).unwrap();
            let mut cur = add_direction(&start, &dir);
            let mut cnt = 0;
            loop {
                if let Some(v) = grid.coords.get(&cur) {
                    if *v < limit {
                        cnt += 1;
                        cur = add_direction(&cur, &dir);
                    } else {
                        cnt += 1;
                        return cnt;
                    }
                } else {
                    return cnt;
                }
            }
        }

        let mut scores: HashMap<Coord, i32> = HashMap::new();
        for &coord in self.vals.coords.keys() {
            let score = count_visible(coord, Direction { dx: 0, dy: 1 }, &self.vals)
                * count_visible(coord, Direction { dx: 1, dy: 0 }, &self.vals)
                * count_visible(coord, Direction { dx: 0, dy: -1 }, &self.vals)
                * count_visible(coord, Direction { dx: -1, dy: 0 }, &self.vals);
            scores.insert(coord, score);
        }

        // print_grid(&self.vals, &mut |c: &Coord, _v: Option<&i32>| if let Some(val) = scores.get(c) { (val/1).to_string() } else { " ".to_string() });

        let best = scores.values().max().unwrap();
        return best.to_string();
    }
}

fn main() {
    let mut day = Day08 { vals: Grid::new() };
    run_day(&mut day);
}
