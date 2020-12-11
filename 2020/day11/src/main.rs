#[macro_use] extern crate failure;

use failure::{Error, bail};
use std::collections::HashMap;
use std::io::{BufReader, BufRead};
use std::fs::File;

type Map = HashMap<(i64, i64), char>;

struct Level {
    map: Map,
    min: (i64, i64),
    max: (i64, i64),
}

fn read_input(file: &str) -> Result<Level, Error> {
    let f = File::open(file)?;
    let br = BufReader::new(f);
    let mut lvl = Level { map: HashMap::new(), min: (0, 0), max: (0, 0) };
    let mut max_x = 0;
    let mut max_y = 0;
    for (y, line) in br.lines().enumerate() {
        for (x, c) in line?.chars().enumerate() {
            if c == '#' || c == 'L' {
                lvl.map.insert((x as i64, y as i64), c);
            } else if c == '.' {
                continue;
            } else {
                bail!("Unknown map char: {}", c);
            }
            if x as i64 > max_x { max_x = x as i64; }
        }
        if y as i64 > max_y { max_y = y as i64; }
    }
    lvl.max = (max_x, max_y);
    return Ok(lvl);
}

fn print_map(map: &Map, min: (i64, i64), max: (i64, i64)) -> () {
    for y in min.1 .. max.1 + 1 {
        for x in min.0 .. max.0 + 1 {
            let ch = map.get(&(x, y)).unwrap_or(&'.');
            print!("{}", ch);
        }
        println!();
    }
}

fn sight_one(start: (i64, i64), dir: (i64, i64), lvl: &Level) -> (i64, i64) {
    let mut cur = start;
    loop {
        let nxt = (cur.0 + dir.0, cur.1 + dir.1);
        if nxt.0 < lvl.min.0 || nxt.0 > lvl.max.0 ||
           nxt.1 < lvl.min.1 || nxt.1 > lvl.max.1 ||
           lvl.map.contains_key(&nxt) {
            return nxt;
        }
        cur = nxt;
    }
}

type Neighbors = HashMap<(i64, i64), Vec<(i64, i64)>>;

fn make_neighbors(lvl: &Level, part: i32) -> Neighbors {
    let mut ret = HashMap::new();
    for cur in lvl.map.keys() {
        let mut ns = Vec::new();
        for x in 0..3 {
            for y in 0..3 {
                if x == 1 && y == 1 {
                    continue;
                }
                if part == 1 {
                    ns.push((cur.0 + x - 1, cur.1 + y - 1));
                } else {
                    ns.push(sight_one(*cur, (x - 1, y - 1), &lvl));
                }
            }
        }
        ret.insert(*cur, ns);
    }
    return ret;
}

fn evolve(map: &Map, neighbors: &Neighbors, part: i32) -> Map {
    let mut nxt : Map = HashMap::new();
    for (cur, ch) in map {
        if *ch == 'L' {
            let oc_cnt = neighbors.get(cur).unwrap().iter().map(|n| map.get(n).unwrap_or(&'.')).filter(|c| **c == '#').count() as i64;
            nxt.insert(*cur, if oc_cnt == 0 { '#' } else { 'L' });
        } else if *ch == '#' {
            let min_oc = if part == 1 { 4 } else { 5 };
            let oc_cnt = neighbors.get(cur).unwrap().iter().map(|n| map.get(n).unwrap_or(&'.')).filter(|c| **c == '#').count() as i64;
            nxt.insert(*cur, if oc_cnt >= min_oc { 'L' } else { '#' });
        }
    }
    return nxt;
}

fn evolve_until_static(lvl: &Level, part: i32, verbose: bool) -> i64 {
    let neighbors = make_neighbors(&lvl, part);

    let mut prev : Map = lvl.map.clone();
    loop {
        if verbose {
            print_map(&prev, lvl.min, lvl.max);
            println!();
        }
        let cur = evolve(&prev, &neighbors, part);
        if cur == prev {
            break;
        } else {
            prev = cur;
        }
    }
    if verbose {
        print_map(&prev, lvl.min, lvl.max);
    }
    return prev.iter().map(|(_, ch)| ch).filter(|c| **c == '#').count() as i64;
}

fn main() {
    let args : Vec<String> = std::env::args().collect();
    if args[1] == "1" {
        println!("Doing part 1");
        let lvl = read_input(&args[2]).unwrap();
        let verbose = args.len() > 3 && args[3].chars().nth(0).unwrap() == 't';
        let cnt = evolve_until_static(&lvl, 1, verbose);
        println!("At final evolution, {}", cnt);
    } else {
        println!("Doing part 2");
        let lvl = read_input(&args[2]).unwrap();
        let verbose = args.len() > 3 && args[3].chars().nth(0).unwrap() == 't';
        let cnt = evolve_until_static(&lvl, 2, verbose);
        println!("At final evolution, {}", cnt);
    }
}
