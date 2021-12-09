extern crate common;

use common::framework::{parse_grid, run_day, BaseDay, InputReader};
use common::grid::{Coord, Grid, color_grid, four_neighbors};

struct Day09 {
    vals: Grid<i32>,
}

fn is_lower(a: &Coord, b: &Coord, grid: &Grid<i32>) -> bool {
    return grid.coords.get(a).unwrap_or(&9) < grid.coords.get(b).unwrap_or(&9);
}

impl BaseDay for Day09 {
    fn parse(&mut self, input: &mut InputReader) {
        fn parse_coord(c: char, _coord: &Coord) -> Option<i32> {
            let val = c.to_digit(10).unwrap() as i32;
            if val < 9 {
                Some(val)
            } else {
                None
            }
        }

        self.vals = parse_grid(input, &mut parse_coord);
    }

    fn pt1(&mut self) -> String {
        let mut risk = 0;
        for (coord, val) in self.vals.coords.iter() {
            if four_neighbors(coord).iter().all(|n| is_lower(coord, n, &self.vals)) {
                risk += 1 + val;
            }
        }
        return risk.to_string();
    }

    fn pt2(&mut self) -> String {
        let mut basins = color_grid(&self.vals, &mut four_neighbors);
        basins = basins.into_iter().filter(|b| b.len() > 0).collect();
        basins.sort_by(|a, b| b.len().cmp(&a.len()));
        for b in &basins {
            println!("Found basin of size {}: {:?}", b.len(), b);
        }
        let amt: i32 = basins.iter().take(3).fold(1, |tot, val| tot * val.len() as i32);
        return amt.to_string();
    }
}

fn main() {
    let mut day = Day09 { vals: Grid::new() };
    run_day(&mut day);
}
