#[macro_use] extern crate lazy_static;

use failure::Error;
use itertools::Itertools;
use regex::Regex;
use std::cell::RefCell;
use std::collections::{BinaryHeap,HashMap,HashSet};
use std::cmp::Ordering;
use std::io::{BufReader,BufRead};
use std::fs::File;

#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
enum TileType {
    VOID,
    FLOOR,
    WALL,
}

struct Level {
    map: HashMap<(i64, i64), TileType>,
    entrance: (i64, i64),
    exit: (i64, i64),
    teleporters: HashMap<(i64, i64), (i64, i64)>,
    teleporter_names: HashMap<(i64, i64), String>,
    min_coords: (i64, i64),
    max_coords: (i64, i64),
}

fn read_input(file: &str) -> Result<Level, Error> {
    let f = File::open(file)?;
    let br = BufReader::new(f);
    let mut y = 0;
    let mut max_x = 0;
    let mut lvl = Level { map: HashMap::new(), entrance: (0,0), exit: (0, 0),
        teleporters: HashMap::new(), teleporter_names: HashMap::new(),
        min_coords: (0,0), max_coords: (0,0) };
    let mut letters : HashMap<(i64, i64), char> = HashMap::new();
    for line in br.lines() {
        let mut x = 0;
        for c in line?.chars() {
            if c == '#' {
                lvl.map.insert((x, y), TileType::WALL);
            } else if c == '.' {
                lvl.map.insert((x, y), TileType::FLOOR);
            } else if c == ' ' {
                lvl.map.insert((x, y), TileType::VOID);
            } else if c >= 'A' && c <= 'Z' {
                lvl.map.insert((x, y), TileType::VOID);
                letters.insert((x, y), c);
            } else {
                panic!("Unexpected tile: {}", c);
            }
            x += 1;
        }
        if x > max_x {
            max_x = x;
        }
        y += 1;
    }

    lvl.min_coords = (0, 0);
    lvl.max_coords = (max_x, y);

    let mut tnames : HashMap<String, Vec<(i64, i64)>> = HashMap::new();
    for tx in 0..max_x {
        for ty in 0..y {
            let ltrw = letters.get(&(tx, ty));
            if ltrw.is_none() {
                continue;
            }
            let ltr = *ltrw.unwrap();
            let (other_ltr, before, after) =
                if let Some(&other) = letters.get(&(tx, ty+1)) {
                    letters.remove(&(tx, ty+1));
                    (other, (tx, ty-1), (tx, ty+2))
                } else {
                    let other = *letters.get(&(tx+1, ty)).unwrap();
                    letters.remove(&(tx+1, ty));
                    (other, (tx-1, ty), (tx+2, ty))
                };
            let name : String = vec![ltr, other_ltr].into_iter().collect();
            let before_tile = *lvl.map.get(&before).unwrap_or(&TileType::VOID);
            let after_tile = *lvl.map.get(&after).unwrap_or(&TileType::VOID);
            let dest =
                if before_tile == TileType::VOID && after_tile == TileType::FLOOR {
                    after
                } else if before_tile == TileType::FLOOR && after_tile == TileType::VOID {
                    before
                } else {
                    panic!("Couldn't figure out destination for {} by {},{}", name, tx, ty);
                };
            match tnames.get_mut(&name) {
                Some(dests) => { dests.push(dest); },
                None => { tnames.insert(name, vec![dest]); },
            }
        }
    }

    for (tname, dests) in tnames.iter() {
        if tname == "AA" {
            assert!(dests.len() == 1);
            lvl.entrance = dests[0];
        } else if tname == "ZZ" {
            assert!(dests.len() == 1);
            lvl.exit = dests[0];
        } else {
            assert!(dests.len() == 2);
            lvl.teleporters.insert(dests[0], dests[1]);
            lvl.teleporters.insert(dests[1], dests[0]);
        }
        for dest in dests {
            lvl.teleporter_names.insert(*dest, tname.clone());
        }
    }

    return Ok(lvl);
}

fn print_level(lvl: &Level) {
    let features : HashMap<(i64, i64), char> =
        vec![(lvl.entrance, '@'), (lvl.exit, '>')].into_iter()
        .chain(lvl.teleporters.keys().map(|p| (*p, if is_outer(p, &lvl) { '^' } else { 'v' })))
        .collect();
    for y in lvl.min_coords.1..=lvl.max_coords.1 {
        let mut line = String::new();
        line.reserve((lvl.max_coords.0 - lvl.min_coords.0 + 1) as usize);
        for x in lvl.min_coords.0..=lvl.max_coords.0 {
            if let Some(c) = features.get(&(x, y)) {
                line.push(*c);
            } else {
                match lvl.map.get(&(x, y)) {
                    Some(TileType::VOID) => line.push(' '),
                    Some(TileType::FLOOR) => line.push('.'),
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

fn find_path(lvl: &Level) -> i32 {
    let mut seen = HashMap::new();
    let mut working = vec![(lvl.entrance, 0)];
    while let Some((pos, dist)) = working.pop() {
        // println!("Considering {},{} - {}", pos.0, pos.1, dist);
        if pos == lvl.exit {
            return dist;
        }
        seen.insert(pos, dist);
        for dir in &DIRECTIONS {
            let next_p = (pos.0 + dir.x, pos.1 + dir.y);
            if seen.contains_key(&next_p) {
                continue;
            }
            match lvl.map.get(&next_p) {
                Some(TileType::FLOOR) => {
                    working.insert(0, (next_p, dist + 1));
                },
                Some(TileType::VOID) => {},
                Some(TileType::WALL) => {},
                None => {},
            }
        }
        if let Some(other) = lvl.teleporters.get(&pos) {
            if seen.contains_key(other) {
                continue;
            }
            working.insert(0, (*other, dist+1));
        }
    }
    panic!("Couldn't find exit!");
}

fn is_outer(pos: &(i64, i64), lvl: &Level) -> bool {
    let width = 3;
    return pos.0 - width <= lvl.min_coords.0 || pos.1 - width <= lvl.min_coords.1 ||
        pos.0 + width >= lvl.max_coords.0 || pos.1 + width >= lvl.max_coords.1;
}

fn find_path_recursive(lvl: &Level) -> i32 {
    let mut seen = HashMap::new();
    let mut working = vec![(lvl.entrance, 0, 0, "AA(0)".to_string())];
    while let Some((pos, depth, dist, path)) = working.pop() {
        // println!("Considering {},{} @ {} - {} - {}", pos.0, pos.1, depth, dist, path);
        if pos == lvl.exit && depth == 0 {
            return dist;
        }
        seen.insert((pos, depth), dist);
        for dir in &DIRECTIONS {
            let next_p = (pos.0 + dir.x, pos.1 + dir.y);
            if seen.contains_key(&(next_p, depth)) {
                continue;
            }
            match lvl.map.get(&next_p) {
                Some(TileType::FLOOR) => {
                    working.insert(0, (next_p, depth, dist + 1, path.clone()));
                },
                Some(TileType::VOID) => {},
                Some(TileType::WALL) => {},
                None => {},
            }
        }
        if let Some(other) = lvl.teleporters.get(&pos) {
            let max_depth = lvl.teleporters.len() + 5; // some prune heuristic
            if (!is_outer(&pos, &lvl) || depth > 0) && depth < max_depth {
                let next_depth = if is_outer(&pos, &lvl) { depth - 1 } else { depth + 1 };
                if !seen.contains_key(&(*other, next_depth)) {
                    if next_depth < 2 || !seen.contains_key(&(*other, next_depth-2)) {
                        let new_path = format!("{},{}({})", path, lvl.teleporter_names.get(&pos).unwrap(), next_depth);
                        working.insert(0, (*other, next_depth, dist+1, new_path));
                    } else {
                        // println!("Not backtracking into {}({})", lvl.teleporter_names.get(&pos).unwrap(), next_depth);
                    }
                }
            }
        }
    }
    panic!("Couldn't find exit!");
}

fn main() {
    let args : Vec<String> = std::env::args().collect();
    if args[1] == "1" {
        println!("Doing part 1");
        let lvl = read_input(&args[2]).unwrap();
        print_level(&lvl);
        let steps = find_path(&lvl);
        println!("Found path in {} steps", steps);
    } else {
        println!("Doing part 2");
        let lvl = read_input(&args[2]).unwrap();
        print_level(&lvl);
        let steps = find_path_recursive(&lvl);
        println!("Found path in {} steps", steps);
    }
}
