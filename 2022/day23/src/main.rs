use std::collections::{HashMap, HashSet};

extern crate common;

use common::framework::{parse_grid, run_day, BaseDay, InputReader};
use common::grid::{add_direction, eight_neighbors, print_grid, Coord, Direction, Grid};

struct Day23 {
    grid: Grid<char>,
}

fn compute_score(coords: &HashSet<Coord>) -> i32 {
    let grid = Grid::from_set(coords, '#');
    return (grid.max.x - grid.min.x + 1) * (grid.max.y - grid.min.y + 1)
        - grid.coords.len() as i32;
}

fn print_coords(coords: &HashSet<Coord>) {
    let grid = Grid::from_set(coords, '#');
    print_grid(&grid, &mut |_c, ch: Option<&char>| {
        ch.unwrap_or(&'.').to_string()
    });
    println!();
}

fn walk_grid(
    grid: &Grid<char>,
    rounds: i32,
    check_dirs: &Vec<Vec<Direction>>,
) -> (HashSet<Coord>, i32) {
    fn propose(
        elf: &Coord,
        elves: &HashSet<Coord>,
        offset: usize,
        check_dirs: &Vec<Vec<Direction>>,
    ) -> Option<Coord> {
        if eight_neighbors(&elf).iter().all(|c| !elves.contains(c)) {
            return None;
        }
        for i in 0..4 {
            if check_dirs[(i + offset) % 4]
                .iter()
                .all(|d| !elves.contains(&add_direction(elf, d)))
            {
                return Some(add_direction(elf, &check_dirs[(i + offset) % 4][0]));
            }
        }
        return None;
    }

    let mut elves = grid.coords.keys().map(|k| *k).collect::<HashSet<Coord>>();
    for round in 0..rounds {
        // print_coords(&elves);
        let mut nxt: HashSet<Coord> = HashSet::new();
        let mut proposals: HashMap<Coord, Vec<Coord>> = HashMap::new();
        for elf in &elves {
            let offset = round as usize % check_dirs.len();
            if let Some(proposal) = propose(elf, &elves, offset, check_dirs) {
                proposals
                    .entry(proposal)
                    .or_insert_with(|| vec![])
                    .push(*elf);
            } else {
                nxt.insert(*elf);
            }
        }
        for (proposed, old_lst) in proposals.iter() {
            if old_lst.len() == 1 {
                nxt.insert(*proposed);
            } else {
                nxt.extend(old_lst);
            }
        }
        if elves == nxt {
            return (elves, round + 1);
        }
        elves = nxt;
    }
    return (elves, rounds + 1);
}

fn get_check_dirs() -> Vec<Vec<Direction>> {
    return vec![
        vec![
            Direction { dx: 0, dy: -1 },
            Direction { dx: -1, dy: -1 },
            Direction { dx: 1, dy: -1 },
        ],
        vec![
            Direction { dx: 0, dy: 1 },
            Direction { dx: -1, dy: 1 },
            Direction { dx: 1, dy: 1 },
        ],
        vec![
            Direction { dx: -1, dy: 0 },
            Direction { dx: -1, dy: -1 },
            Direction { dx: -1, dy: 1 },
        ],
        vec![
            Direction { dx: 1, dy: 0 },
            Direction { dx: 1, dy: -1 },
            Direction { dx: 1, dy: 1 },
        ],
    ];
}

impl BaseDay for Day23 {
    fn parse(&mut self, input: &mut InputReader) {
        self.grid = parse_grid(input, &mut |c: char, _c: &Coord| {
            if c == '#' {
                Some(c)
            } else {
                None
            }
        });
    }

    fn pt1(&mut self) -> String {
        let (elves, _) = walk_grid(&self.grid, 10, &get_check_dirs());
        return compute_score(&elves).to_string();
    }

    fn pt2(&mut self) -> String {
        let (_, rounds) = walk_grid(&self.grid, i32::MAX, &get_check_dirs());
        return rounds.to_string();
    }
}

fn main() {
    let mut day = Day23 { grid: Grid::new() };
    run_day(&mut day);
}
