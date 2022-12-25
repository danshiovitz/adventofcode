use std::collections::{HashMap, HashSet};

extern crate common;

use common::framework::{parse_grid, run_day, BaseDay, InputReader};
use common::grid::{four_neighbors, print_grid, Coord, Grid};
use common::solver::{cost_minimizing_bfs, SolverBase, SolverState};
use common::utils::mod_one_through_range;

struct Day24 {
    vals: Grid<char>,
}

struct Map {
    start: Coord,
    end: Coord,
    main_width: usize,
    main_height: usize, // this excludes start/end
    coords: HashSet<Coord>,
    n_blizzards: HashMap<i32, Vec<i32>>,
    s_blizzards: HashMap<i32, Vec<i32>>,
    e_blizzards: HashMap<i32, Vec<i32>>,
    w_blizzards: HashMap<i32, Vec<i32>>,
}

fn make_map(grid: &Grid<char>) -> Map {
    let start = *grid
        .coords
        .keys()
        .filter(|c| c.y == grid.min.y)
        .next()
        .unwrap();
    let end = *grid
        .coords
        .keys()
        .filter(|c| c.y == grid.max.y)
        .next()
        .unwrap();
    let mut n_blizzards = HashMap::new();
    let mut s_blizzards = HashMap::new();
    let mut e_blizzards = HashMap::new();
    let mut w_blizzards = HashMap::new();

    for (&coord, &ch) in &grid.coords {
        if ch == '^' {
            n_blizzards
                .entry(coord.x)
                .or_insert_with(|| vec![])
                .push(coord.y);
        } else if ch == 'v' {
            s_blizzards
                .entry(coord.x)
                .or_insert_with(|| vec![])
                .push(coord.y);
        } else if ch == '>' {
            e_blizzards
                .entry(coord.y)
                .or_insert_with(|| vec![])
                .push(coord.x);
        } else if ch == '<' {
            w_blizzards
                .entry(coord.y)
                .or_insert_with(|| vec![])
                .push(coord.x);
        }
    }

    return Map {
        start: start,
        end: end,
        // main_width and height filter out the wall rows/columns
        main_width: (grid.max.x - grid.min.x + 1 - 2) as usize,
        main_height: (grid.max.y - grid.min.y + 1 - 2) as usize,
        coords: grid.coords.keys().map(|c| *c).collect::<HashSet<Coord>>(),
        n_blizzards: n_blizzards,
        s_blizzards: s_blizzards,
        e_blizzards: e_blizzards,
        w_blizzards: w_blizzards,
    };
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug, Hash)]
struct State {
    pos: Coord,
    round: i32,
}
impl SolverState for State {}

fn has_blizzard(state: &State, map: &Map) -> bool {
    return blizzard_ch(&state.pos, state.round, map, true) != '.';
}

fn blizzard_ch(pos: &Coord, round: i32, map: &Map, fast: bool) -> char {
    // note the positions are 1..10, not 0..9, so we have to adjust when doing the mod
    let height = map.main_height as i32;
    let width = map.main_width as i32;
    let mut ch = '.';
    let mut cnt = 0;
    if let Some(bs) = map.n_blizzards.get(&pos.x) {
        if bs
            .iter()
            .any(|b| mod_one_through_range(b - round, height) == pos.y)
        {
            if fast {
                return '^';
            }
            ch = '^';
            cnt += 1;
        }
    }
    if let Some(bs) = map.s_blizzards.get(&pos.x) {
        if bs
            .iter()
            .any(|b| mod_one_through_range(b + round, height) == pos.y)
        {
            if fast {
                return 'v';
            }
            ch = 'v';
            cnt += 1;
        }
    }
    if let Some(bs) = map.e_blizzards.get(&pos.y) {
        if bs
            .iter()
            .any(|b| mod_one_through_range(b + round, width) == pos.x)
        {
            if fast {
                return '>';
            }
            ch = '>';
            cnt += 1;
        }
    }
    if let Some(bs) = map.w_blizzards.get(&pos.y) {
        if bs
            .iter()
            .any(|b| mod_one_through_range(b - round, width) == pos.x)
        {
            if fast {
                return '<';
            }
            ch = '<';
            cnt += 1;
        }
    }

    if cnt == 0 {
        return '.';
    } else if cnt == 1 {
        return ch;
    } else if cnt == 2 {
        return '2';
    } else if cnt == 3 {
        return '3';
    } else {
        panic!("Unexpected: {}", ch);
    }
}

fn print_map(state: &State, map: &Map) {
    let grid = Grid::from_set(&map.coords, '.');
    print_grid(&grid, &mut |c: &Coord, ch: Option<&char>| {
        if ch.is_none() {
            ' '.to_string()
        } else {
            let b = blizzard_ch(c, state.round, map, false);
            if b != '.' {
                b.to_string()
            } else if *c == state.pos {
                'E'.to_string()
            } else {
                '.'.to_string()
            }
        }
    });
    println!();
}

impl SolverBase<State> for Map {
    fn is_finished(&self, state: &State) -> bool {
        return state.pos == self.end;
    }

    fn print_state(&self, state: &State) -> () {
        println!("{:?}", state);
    }

    fn print_path(&self, path: &Vec<State>) {
        for st in path {
            println!("Round {}", st.round);
            print_map(st, self);
            println!();
        }
    }

    // returns a list of (cost, new state) pairs
    fn gen_possible_moves(&self, state: &State) -> Vec<(i32, State)> {
        let mut moves = four_neighbors(&state.pos)
            .into_iter()
            .collect::<Vec<Coord>>();
        moves.push(state.pos);
        return moves
            .into_iter()
            .map(|c| State { pos: c, round: state.round + 1 })
            .filter(|s| self.coords.contains(&s.pos) && !has_blizzard(s, self))
            .map(|s| (1, s))
            .collect::<Vec<(i32, State)>>();
    }

    fn is_verbose(&self) -> bool {
        return false;
    }
}

fn walk_grid(grid: &Grid<char>, trips: i32) -> i32 {
    let mut map = make_map(grid);
    let mut total_cost: i32 = 0;
    for _ in 0..trips {
        let cur_cost = cost_minimizing_bfs(&map, &State { pos: map.start, round: total_cost });
        total_cost += cur_cost;
        let tmp = map.start;
        map.start = map.end;
        map.end = tmp;
    }
    return total_cost;
}

impl BaseDay for Day24 {
    fn parse(&mut self, input: &mut InputReader) {
        let mut parse_coord = |c: char, _coord: &Coord| -> Option<char> {
            if c == '#' {
                return None;
            } else {
                return Some(c);
            }
        };
        self.vals = parse_grid(input, &mut parse_coord);
    }

    fn pt1(&mut self) -> String {
        return walk_grid(&self.vals, 1).to_string();
    }

    fn pt2(&mut self) -> String {
        return walk_grid(&self.vals, 3).to_string();
    }
}

fn main() {
    let mut day = Day24 { vals: Grid::new() };
    run_day(&mut day);
}
