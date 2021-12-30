use std::collections::{HashMap, HashSet};

extern crate common;

use common::framework::{parse_grid, run_day, BaseDay, InputReader};
use common::grid::{print_grid, Coord, Grid};

struct Day25 {
    vals: Grid<char>,
}

fn print_one(_c: &Coord, maybe_val: Option<&char>) -> String {
    if let Some(val) = maybe_val {
        return val.to_string();
    } else {
        return ".".to_owned();
    }
}

fn print_all(easts: &HashSet<Coord>, souths: &HashSet<Coord>, grid: &Grid<char>) {
    let mut gg = Grid { min: grid.min, max: grid.max, coords: HashMap::new() };
    gg.coords.extend(easts.iter().map(|c| (*c, '>')));
    gg.coords.extend(souths.iter().map(|c| (*c, 'v')));
    print_grid(&gg, &mut print_one);
    println!();
}

fn move_cukes(grid: &Grid<char>, verbose: bool) -> i32 {
    let mut cur_easts: HashSet<Coord> = grid
        .coords
        .iter()
        .filter_map(|(c, v)| if *v == '>' { Some(*c) } else { None })
        .collect();
    let mut cur_souths: HashSet<Coord> = grid
        .coords
        .iter()
        .filter_map(|(c, v)| if *v == 'v' { Some(*c) } else { None })
        .collect();

    if verbose {
        println!("Initial");
        print_all(&cur_easts, &cur_souths, grid);
        println!();
    }

    let mut steps = 0;
    loop {
        let move_east = |east: &Coord| -> Coord {
            let new_c = Coord { x: (east.x + 1) % (grid.max.x + 1), y: east.y };
            if cur_easts.contains(&new_c) || cur_souths.contains(&new_c) {
                return *east;
            } else {
                return new_c;
            }
        };

        let next_easts: HashSet<Coord> = cur_easts.iter().map(|c| move_east(c)).collect();

        let move_south = |south: &Coord| -> Coord {
            let new_c = Coord { x: south.x, y: (south.y + 1) % (grid.max.y + 1) };
            if next_easts.contains(&new_c) || cur_souths.contains(&new_c) {
                return *south;
            } else {
                return new_c;
            }
        };

        let next_souths: HashSet<Coord> = cur_souths.iter().map(|c| move_south(c)).collect();

        steps += 1;
        if verbose {
            println!("Step {}", steps);
            print_all(&cur_easts, &cur_souths, grid);
            println!();
        }

        if cur_easts == next_easts && cur_souths == next_souths {
            return steps;
        }
        cur_easts = next_easts;
        cur_souths = next_souths;
    }
}

impl BaseDay for Day25 {
    fn parse(&mut self, input: &mut InputReader) {
        fn parse_coord(c: char, _coord: &Coord) -> Option<char> {
            if c == '>' || c == 'v' {
                Some(c)
            } else if c == '.' {
                None
            } else {
                panic!("bad char: {}", c);
            }
        }

        self.vals = parse_grid(input, &mut parse_coord);
    }

    fn pt1(&mut self) -> String {
        let steps = move_cukes(&self.vals, false);
        return steps.to_string();
    }

    fn pt2(&mut self) -> String {
        return "".to_string();
    }
}

fn main() {
    let mut day = Day25 { vals: Grid::new() };
    run_day(&mut day);
}
