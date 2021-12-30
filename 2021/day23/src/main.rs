use std::collections::{HashMap, HashSet};

extern crate common;

use common::framework::{parse_grid, run_day, BaseDay, InputReader};
use common::grid::{Coord, Grid, four_neighbors, print_grid};
use common::solver::{SolverBase, SolverState, cost_minimizing_dfs};

struct Day23 {
    grid: Grid<char>,
    positions: HashMap<char, Vec<Coord>>,
}

fn organize(grid: &Grid<char>, positions: &HashMap<char, Vec<Coord>>) -> i32 {
    let costs = HashMap::from([('A', 1), ('B', 10), ('C', 100), ('D', 1000)]);
    let mut sk: Vec<char> = costs.keys().map(|c| *c).collect();
    sk.sort();

    let mut pchars = vec![];
    for ch in &sk {
        pchars.extend(positions.get(ch).unwrap().iter().map(|_| ch));
    }
    let recurser = Recurser::new(grid, costs.clone(), pchars);
    let mut init_state = State { positions: Vec::new() };
    for ch in &sk {
        init_state.positions.extend(positions.get(ch).unwrap());
    }
    println!("Initial state:");
    recurser.print_state(&init_state);
    return cost_minimizing_dfs(&recurser, &init_state);
}

#[derive(PartialEq, Eq, Clone, Ord, PartialOrd, Debug, Hash)]
struct State {
    positions: Vec<Coord>,  // A A B B C C D D
}

impl SolverState for State {}

struct Recurser {
    moves: HashMap<(Coord, Coord), Vec<Coord>>,
    costs: HashMap<char, i32>,
    destinations: HashMap<char, HashSet<Coord>>,
    hallways: HashSet<Coord>,
    pchars: Vec<char>,
    grid: Grid<char>,
    verbose: bool,
}

impl Recurser {
    pub fn new(grid: &Grid<char>, costs: HashMap<char, i32>, pchars: Vec<char>) -> Self {
        let (moves, destinations, hallways) = calc_moves(grid, &pchars);
        return Recurser {
            moves: moves,
            costs: costs,
            destinations: destinations,
            hallways: hallways,
            pchars: pchars,
            grid: grid.clone(),
            verbose: false,
        };
    }

    // this isn't our room, or someone else is here
    fn in_bad_room(&self, pos: Coord, ch: char, state: &State) -> bool {
        let dests = self.destinations.get(&ch).unwrap();
        if !dests.contains(&pos) {
            return true;
        }
        let others: HashSet<Coord> = state.positions.iter().zip(self.pchars.iter()).filter_map(|(p, oc)| if *oc != ch { Some(*p) } else { None }).collect();
        return others.intersection(&dests).count() > 0;
    }

    fn in_hallway(&self, pos: Coord) -> bool {
        return self.hallways.contains(&pos);
    }

    fn can_walk_to(&self, pos: Coord, ch: char, dest: Coord, state: &State) -> Option<i32> {
        // can we walk there without bumping into people, returns cost
        let moves = self.moves.get(&(pos, dest)).unwrap();
        let others: HashSet<Coord> = state.positions.iter().map(|p| *p).collect();
        if moves.iter().any(|m| others.contains(m)) {
            return None;
        }
        return Some(moves.len() as i32 * self.costs.get(&ch).unwrap());
    }
}

impl SolverBase<State> for Recurser {
    fn is_verbose(&self) -> bool {
        return self.verbose;
    }

    fn is_finished(&self, state: &State) -> bool {
        for (pos, ch) in state.positions.iter().zip(self.pchars.iter()) {
            let dests = self.destinations.get(ch).unwrap();
            if !dests.contains(pos) {
                return false;
            }
        }
        return true;
    }

    fn gen_possible_moves(&self, state: &State) -> Vec<(i32, State)> {
        let mut ret: Vec<(i32, State)> = vec![];
        for idx in 0..state.positions.len() {
            let pos = state.positions[idx];
            let ch = self.pchars[idx];
            // moves are either from the room to a hallway, or a hallway to a room
            if self.in_hallway(pos) {
                let mut dests: Vec<Coord> = (*self.destinations.get(&ch).unwrap()).iter().map(|c| *c).collect();
                // we always move to the deepest one we can - there's never any point
                // stopping earlier
                dests.sort_by(|a, b| b.y.partial_cmp(&a.y).unwrap());
                for dpos in dests.into_iter() {
                    if let Some(cost) = self.can_walk_to(pos, ch, dpos, state) {
                        if self.in_bad_room(dpos, ch, state) {
                            continue;
                        }
                        let mut new_state = state.clone();
                        new_state.positions[idx] = dpos;
                        if self.verbose {
                            println!("Move {} from hallway {:?} to room {:?}", idx, pos, dpos);
                        }
                        ret.push((cost, new_state));
                        // stop once we found one:
                        break;
                    }
                }
            } else if self.in_bad_room(pos, ch, state) {
                for hpos in &self.hallways {
                    if let Some(cost) = self.can_walk_to(pos, ch, *hpos, state) {
                        let mut new_state = state.clone();
                        new_state.positions[idx] = *hpos;
                        if self.verbose {
                            println!("Move {} from room {:?} to hallway {:?}", idx, pos, hpos);
                        }
                        ret.push((cost, new_state));
                    }
                }
            } else {
                // do nothing, we're in the right place
            }
        }
        return ret;
    }

    fn print_state(&self, state: &State) {
        let mut print_one = |c: &Coord, maybe_val: Option<&char>| -> String {
            if maybe_val.is_some() {
                for (idx, pos) in state.positions.iter().enumerate() {
                    if *pos == *c {
                        return self.pchars[idx].to_string();
                    }
                }
                return ".".to_owned();
            }
            return "#".to_owned();
        };
        print_grid(&self.grid, &mut print_one);
        println!();
    }
}

fn calc_moves(grid: &Grid<char>, pchars: &Vec<char>) -> (HashMap<(Coord, Coord), Vec<Coord>>, HashMap<char, HashSet<Coord>>, HashSet<Coord>) {
    let mut hallways: HashSet<Coord> = grid.coords.keys().map(|c| *c).filter(|c| c.y == 1).collect();
    if hallways.len() < 5 {
        panic!("Bad hallway count? {}", hallways.len());
    }

    let mut rms: Vec<Coord> = grid.coords.keys().map(|c| *c).filter(|c| c.y > 1).collect();
    rms.sort();
    if rms.len() != pchars.len() {
        panic!("Bad room count? {} {}", rms.len(), pchars.len());
    }
    for rm in &rms {
        hallways.remove(&Coord { x: rm.x, y: 1 });
    }

    let mut destinations: HashMap<char, HashSet<Coord>> = HashMap::new();
    for ch in pchars {
        if let Some(hs) = destinations.get_mut(ch) {
            hs.insert(rms.remove(0));
        } else {
            destinations.insert(*ch, HashSet::from([rms.remove(0)]));
        }
    }

    let mut moves: HashMap<(Coord, Coord), Vec<Coord>> = HashMap::new();
    for coord in grid.coords.keys() {
        let mut working = Vec::new();
        working.push((*coord, vec![]));
        let mut seen = HashSet::new();
        while !working.is_empty() {
            let (cur, path) = working.remove(0);
            if seen.contains(&cur) {
                continue;
            }
            seen.insert(cur);
            moves.insert((*coord, cur), path.clone());
            for ngh in four_neighbors(&cur) {
                if grid.coords.contains_key(&ngh) {
                    let mut nxt_path = path.clone();
                    nxt_path.push(ngh);
                    working.push((ngh, nxt_path));
                }
            }
        }
    }

    return (moves, destinations, hallways);
}

impl BaseDay for Day23 {
    fn parse(&mut self, input: &mut InputReader) {
        let mut parse_coord = |c: char, coord: &Coord| -> Option<char> {
            if c == '.' {
                return Some(c);
            } else if c == '#' || c == ' ' {
                return None;
            } else {
                if let Some(ps) = self.positions.get_mut(&c) {
                    ps.push(*coord);
                } else {
                    self.positions.insert(c, vec![*coord]);
                }
                return Some('.');
            }
        };
        self.grid = parse_grid(input, &mut parse_coord);
    }

    fn pt1(&mut self) -> String {
        let cost = organize(&self.grid, &self.positions);
        return cost.to_string();
    }

    fn pt2(&mut self) -> String {
        let mut new_grid = self.grid.clone();
        let mut new_positions = self.positions.clone();
        new_grid.max = Coord { x: new_grid.max.x, y: new_grid.max.y + 2 };
        for nx in vec![3, 5, 7, 9] {
            new_grid.coords.insert(Coord {x: nx, y: 4}, '.');
            new_grid.coords.insert(Coord {x: nx, y: 5}, '.');
        }
        for ps in new_positions.values_mut() {
            for c in ps {
                if c.y == 3 {
                    c.y = 5;
                }
            }
        }
        new_positions.get_mut(&'A').unwrap().extend(vec![Coord {x: 9, y: 3}, Coord {x: 7, y: 4}]);
        new_positions.get_mut(&'B').unwrap().extend(vec![Coord {x: 7, y: 3}, Coord {x: 5, y: 4}]);
        new_positions.get_mut(&'C').unwrap().extend(vec![Coord {x: 5, y: 3}, Coord {x: 9, y: 4}]);
        new_positions.get_mut(&'D').unwrap().extend(vec![Coord {x: 3, y: 3}, Coord {x: 3, y: 4}]);

        let cost = organize(&new_grid, &new_positions);
        return cost.to_string();
    }
}

fn main() {
    let mut day = Day23 {
        grid: Grid::new(),
        positions: HashMap::new(),
    };
    run_day(&mut day);
}
