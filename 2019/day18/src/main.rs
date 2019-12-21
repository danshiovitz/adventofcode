#[macro_use] extern crate lazy_static;

use failure::Error;
use itertools::Itertools;
use regex::Regex;
use std::cell::RefCell;
use std::collections::{BinaryHeap,HashMap,HashSet};
use std::cmp::Ordering;
use std::io::{BufReader,BufRead};
use std::iter::FromIterator;
use std::fs::File;

#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
enum TileType {
    EMPTY,
    WALL,
}

struct Level {
    map: HashMap<(i64, i64), TileType>,
    entrances: Vec<(i64, i64)>,
    doors: HashMap<char, (i64, i64)>,
    keys: HashMap<char, (i64, i64)>,
}

fn read_input(file: &str) -> Result<Level, Error> {
    let f = File::open(file)?;
    let br = BufReader::new(f);
    let mut y = 0;
    let mut lvl = Level { map: HashMap::new(), entrances: Vec::new(), doors: HashMap::new(), keys: HashMap::new() };
    for line in br.lines() {
        let mut x = 0;
        for c in line?.chars() {
            if c == '#' {
                lvl.map.insert((x, y), TileType::WALL);
            } else if c == '.' || c == ' ' {
                lvl.map.insert((x, y), TileType::EMPTY);
            } else if c == '@' {
                lvl.map.insert((x, y), TileType::EMPTY);
                lvl.entrances.push((x, y));
            } else if c >= 'A' && c <= 'Z' {
                lvl.map.insert((x, y), TileType::EMPTY);
                lvl.doors.insert(c, (x, y));
            } else if c >= 'a' && c <= 'z' {
                lvl.map.insert((x, y), TileType::EMPTY);
                lvl.keys.insert(c, (x, y));
            } else {
                panic!("Unexpected tile: {}", c);
            }
            x += 1;
        }
        y += 1;
    }
    return Ok(lvl);
}

fn print_level(lvl: &Level) {
    let mut min_x = std::i64::MAX;
    let mut max_x = std::i64::MIN;
    let mut min_y = std::i64::MAX;
    let mut max_y = std::i64::MIN;

    for val in lvl.map.keys() {
        let (x, y) = *val;
        if x < min_x {
            min_x = x;
        }
        if x > max_x {
            max_x = x;
        }
        if y < min_y {
            min_y = y;
        }
        if y > max_y {
            max_y = y;
        }
    }

    let features : HashMap<(i64, i64), char> =
        lvl.entrances.iter().map(|p| (*p, '@'))
        .chain(lvl.doors.iter().map(|(c, p)| (*p, *c)))
        .chain(lvl.keys.iter().map(|(c, p)| (*p, *c)))
        .collect();
    for y in min_y..=max_y {
        let mut line = String::new();
        line.reserve((max_x - min_x + 1) as usize);
        for x in min_x..=max_x {
            if let Some(c) = features.get(&(x, y)) {
                line.push(*c);
            } else {
                match lvl.map.get(&(x, y)) {
                    Some(TileType::EMPTY) => line.push('.'),
                    Some(TileType::WALL) => line.push('#'),
                    None => line.push('?'),
                }
            }
        }
        println!("{}", line);
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
struct Direction {
    value: i64,
    x: i64,
    y: i64,
    c: char,
}

static NORTH : Direction = Direction { value: 1, x: 0, y: -1, c: '^' };
static SOUTH : Direction = Direction { value: 2, x: 0, y: 1, c: 'v' };
static WEST : Direction = Direction { value: 3, x: -1, y: 0, c: '<' };
static EAST : Direction = Direction { value: 4, x: 1, y: 0, c: '>' };

static DIRECTIONS : [Direction; 4] = [
    NORTH, SOUTH, WEST, EAST
];


#[derive(Debug, Eq, PartialEq)]
struct State {
    dist: i32,
    keys: HashSet<char>,
    positions: Vec<(i64, i64)>,
    path: Vec<char>,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        if self.dist != other.dist {
            return other.dist.cmp(&self.dist);
        }
        if self.keys.len() != other.keys.len() {
            return other.keys.len().cmp(&self.keys.len());
        }
        return other.positions[0].cmp(&self.positions[0]);
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn path_seen(pos: &(i64, i64), via: &HashSet<char>, seen: &HashMap<(i64, i64), Vec<HashSet<char>>>) -> bool {
    let existing = seen.get(pos);
    return existing.map_or(false, |vias| vias.iter().any(|ex| ex.is_subset(via)));
}

fn already_reachable(via: &HashSet<char>, dist: i32, reachable: &Vec<(HashSet<char>, i32)>) -> bool {
    for (ex, ex_dist) in reachable {
        if *ex_dist <= dist && ex.is_subset(via) {
            return true;
        }
    }
    return false;
}

fn calc_reachables_doored(start_x: i64, start_y: i64, lvl: &Level) -> HashMap<char, Vec<(HashSet<char>, i32)>> {
    let doors : HashMap<(i64, i64), char> = lvl.doors.iter().map(|(p, c)| (*c, *p)).collect();
    let keys : HashMap<(i64, i64), char> = lvl.keys.iter().map(|(p, c)| (*c, *p)).collect();

    let mut reachables : HashMap<char, Vec<(HashSet<char>, i32)>> = HashMap::new();

    let mut seen : HashMap<(i64, i64), Vec<HashSet<char>>> = HashMap::new();
    let mut working = vec![((start_x, start_y), 0, HashSet::new())];
    while working.len() > 0 {
        let (pos, dist, via) = working.remove(0);
        // println!("Considering {},{} - {} via {}", pos.0, pos.1, dist, via.iter().sorted().collect::<String>());
        match seen.get_mut(&pos) {
            Some(prevs) => { prevs.push(via.clone()); },
            None => { seen.insert(pos, vec![via.clone()]); },
        }
        for dir in &DIRECTIONS {
            let next_p = (pos.0 + dir.x, pos.1 + dir.y);
            if path_seen(&next_p, &via, &seen) {
                continue;
            }
            match lvl.map.get(&next_p) {
                Some(TileType::EMPTY) => {
                    if let Some(door) = doors.get(&next_p) {
                        let mut next_via = via.clone();
                        next_via.insert((*door as u8 - 'A' as u8 + 'a' as u8) as char);
                        working.push((next_p, dist + 1, next_via));
                    } else if let Some(key) = keys.get(&next_p) {
                        match reachables.get_mut(key) {
                            Some(vias) => {
                                if !already_reachable(&via, dist+1, vias) {
                                    // println!("Found new path: {} via {:?}", dist+1, via);
                                    vias.push((via.clone(), dist + 1));
                                }
                            },
                            None => {
                                // println!("Found new path: {} via {:?}", dist+1, via);
                                reachables.insert(*key, vec![(via.clone(), dist + 1)]);
                            }
                        }
                        // reachables.insert(*key, dist + 1);
                        working.push((next_p, dist + 1, via.clone()));
                    } else {
                        working.push((next_p, dist + 1, via.clone()));
                    }
                },
                Some(TileType::WALL) => {},
                None => {},
            }
        }
    }

    return reachables;
}

fn make_next_states(
    dist: i32,
    keys: &HashSet<char>,
    positions: &Vec<(i64, i64)>,
    path: &Vec<char>,
    reachables: &HashMap<(i64, i64), HashMap<char, Vec<(HashSet<char>, i32)>>>,
    lvl: &Level
) -> Vec<State> {
    let mut ret = Vec::new();
    for pos_idx in 0..positions.len() {
        let pos_reachables = reachables.get(&positions[pos_idx]).unwrap();

        for (target, paths) in pos_reachables.iter() {
            if keys.contains(target) {
                continue;
            }
            let next_dist_opt = paths.iter().filter(|(path, _dist)| path.is_subset(keys)).
                map(|(_path, dist)| *dist).max();
            if let Some(next_dist) = next_dist_opt {
                let mut new_path = path.to_vec();
                new_path.push(*target);
                let mut new_keys = keys.clone();
                new_keys.insert(*target);
                let mut new_positions = positions.clone();
                new_positions[pos_idx] = *lvl.keys.get(&target).unwrap();
                ret.push(State { dist: dist + next_dist, keys: new_keys, positions: new_positions, path: new_path });
            }
        }
    }
    return ret;
}

fn collect_keys(lvl: &Level) -> i32 {
    let reachables : HashMap<(i64, i64), HashMap<char, Vec<(HashSet<char>, i32)>>> =
        lvl.entrances.iter().map(|p| (*p, calc_reachables_doored(p.0, p.1, &lvl))).
        chain(lvl.keys.iter().map(|(_c, p)| (*p, calc_reachables_doored(p.0, p.1, &lvl)))).
        collect();

    let mut best_dist = std::i32::MAX;

    let mut stack = Vec::new();
    let mut seen : HashMap<Vec<(i64, i64)>, Vec<(HashSet<char>, i32)>> = HashMap::new();
    stack.push(State { dist: 0, keys: HashSet::new(), positions: lvl.entrances.clone(), path: vec![] });
    'OUTER: while let Some(State { dist, keys, positions, path }) = stack.pop() {
        if keys.len() == lvl.keys.len() {
            if best_dist > dist {
                best_dist = dist;
                println!("new best; {} {:?}", best_dist, path);
            }
            continue;
        }

        if dist >= best_dist {
            continue;
        }

        match seen.get_mut(&positions) {
            Some(prevs) => {
                for prev in &*prevs {
                    if keys.is_subset(&prev.0) && dist >= prev.1 {
                        continue 'OUTER;
                    }
                }
                let mut ridxs : Vec<usize> = (0..prevs.len()).filter(|i| prevs[*i].1 > dist && prevs[*i].0.is_subset(&keys)).collect();
                if ridxs.len() > 0 {
                    ridxs.reverse();
                    for i in ridxs {
                        //println!("Removing {:?},{} in favor of {:?},{}", prevs[i].0, prevs[i].1, keys, dist);
                        prevs.remove(i);
                    }
                }
                prevs.insert(0, (keys.clone(), dist));
            },
            None => {
                seen.insert(positions.clone(),  vec![(keys.clone(), dist)]);
            }
        }

        // println!("have {} keys", keys.len());
        // println!("Have {} states", heap.len());
        // println!("Checking path ({}): {}", dist, path.iter().format(","));

        let mut next_states = make_next_states(dist, &keys, &positions, &path, &reachables, &lvl);
        next_states.sort();
        // next_states.reverse();
        for state in next_states {
            stack.push(state);
        }
    }
    return best_dist;
}

fn quadrify(lvl: &mut Level) {
    if lvl.entrances.len() != 1 {
        assert!(lvl.entrances.len() == 4);
        return;
    }
    let old = lvl.entrances[0];
    lvl.entrances.clear();
    for i in vec![-1, 1] {
        for j in vec![-1, 1] {
            lvl.entrances.push((old.0 + i, old.1 + j));
        }
    }
    lvl.map.insert(old, TileType::WALL);
    for dir in &DIRECTIONS {
        lvl.map.insert((old.0 + dir.x, old.1 + dir.y), TileType::WALL);
    }
}

fn main() {
    let args : Vec<String> = std::env::args().collect();
    if args[1] == "1" {
        println!("Doing part 1");
        let lvl = read_input(&args[2]).unwrap();
        print_level(&lvl);
        let steps = collect_keys(&lvl);
        println!("Collected keys in {} steps", steps);
    } else {
        println!("Doing part 2");
        let mut lvl = read_input(&args[2]).unwrap();
        quadrify(&mut lvl);
        print_level(&lvl);
        let steps = collect_keys(&lvl);
        println!("Collected keys in {} steps", steps);
    }
}
