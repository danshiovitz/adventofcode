extern crate common;

use common::framework::{parse_grid, run_day, BaseDay, InputReader};
use common::grid::{four_neighbors, Coord, Grid};
use common::solver::{cost_minimizing_bfs, SolverBase};

struct Day12 {
    vals: Grid<i32>,
    start: Coord,
    end: Coord,
}

impl SolverBase<Coord> for Day12 {
    fn is_finished(&self, state: &Coord) -> bool {
        return *state == self.end;
    }

    fn print_state(&self, state: &Coord) -> () {
        println!("{:?}", state);
    }

    // returns a list of (cost, new state) pairs
    fn gen_possible_moves(&self, state: &Coord) -> Vec<(i32, Coord)> {
        let cur = self.vals.coords.get(state).unwrap();
        return four_neighbors(state)
            .into_iter()
            .filter(|c| *self.vals.coords.get(c).unwrap_or(&999) <= cur + 1)
            .map(|c| (1, c))
            .collect::<Vec<(i32, Coord)>>();
    }

    fn is_verbose(&self) -> bool {
        return false;
    }

    fn cant_solve(&self) -> i32 {
        return self.vals.max.x * self.vals.max.y + 1;
    }
}

impl BaseDay for Day12 {
    fn parse(&mut self, input: &mut InputReader) {
        let mut parse_coord = |c: char, coord: &Coord| -> Option<i32> {
            if c == 'S' {
                self.start = *coord;
                return Some(0);
            } else if c == 'E' {
                self.end = *coord;
                return Some(25);
            } else {
                return Some(c as i32 - 'a' as i32);
            }
        };
        self.vals = parse_grid(input, &mut parse_coord);
    }

    fn pt1(&mut self) -> String {
        return cost_minimizing_bfs(self, &self.start).to_string();
    }

    fn pt2(&mut self) -> String {
        let starts = self
            .vals
            .coords
            .iter()
            .filter_map(|(c, h)| if *h == 0 { Some(*c) } else { None })
            .collect::<Vec<Coord>>();
        return starts
            .into_iter()
            .map(|s| cost_minimizing_bfs(self, &s))
            .min()
            .unwrap()
            .to_string();
    }
}

fn main() {
    let mut day = Day12 {
        vals: Grid::new(),
        start: Coord { x: 0, y: 0 },
        end: Coord { x: 0, y: 0 },
    };
    run_day(&mut day);
}
