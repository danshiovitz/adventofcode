#[macro_use] extern crate lazy_static;
#[macro_use] extern crate failure;

use failure::{Error, bail};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::io::{BufReader,BufRead};
use std::fs::File;

#[derive(Debug)]
struct Path {
    steps: Vec<String>,
}

fn parse_line(line: &str) -> Result<Path, Error> {
    let mut ret = Path { steps: Vec::new() };
    let mut chs : Vec<char> = line.chars().collect();
    while chs.len() > 0 {
        let mut ch = chs.remove(0).to_string();
        if ch == "s" || ch == "n" {
            ch.push(chs.remove(0));
        }
        ret.steps.push(ch);
    }
    return Ok(ret);
}

fn read_input(file: &str) -> Result<Vec<Path>, Error> {
    let f = File::open(file)?;
    let br = BufReader::new(f);
    let parsed : Result<Vec<Path>, Error> = br.lines().map(|line| parse_line(&line?)).collect();
    return parsed;
}

fn walk_paths(paths: &Vec<Path>) -> HashSet<(i64, i64, i64)> {
    let mut flipped = HashSet::new();
    for path in paths {
        walk_single_path(path, &mut flipped);
    }
    return flipped;
}

fn get_dirs() -> &'static HashMap<&'static str, (i64, i64, i64)> {
    lazy_static! {
        static ref DIRS: HashMap<&'static str, (i64, i64, i64)> = {
            let mut m = HashMap::new();
            m.insert("ne", (1, -1, 0));
            m.insert("e", (1, 0, -1));
            m.insert("se", (0, 1, -1));
            m.insert("sw", (-1, 1, 0));
            m.insert("w", (-1, 0, 1));
            m.insert("nw", (0, -1, 1));
            m
        };
    }
    return &DIRS;
}

fn walk_single_path(path: &Path, flipped: &mut HashSet<(i64, i64, i64)>) -> () {
    let mut cur = (0, 0, 0);
    for step in &path.steps {
        let dir = get_dirs().get(step.as_str()).unwrap();
        cur = (cur.0 + dir.0, cur.1 + dir.1, cur.2 + dir.2);
    }
    if flipped.contains(&cur) {
        flipped.remove(&cur);
    } else {
        flipped.insert(cur);
    }
}

fn evolve_times(init_state: HashSet<(i64, i64, i64)>, times: i32) -> HashSet<(i64, i64, i64)> {
    let mut cur = init_state.clone();
    for t in 0..times {
        cur = evolve(cur);
        // println!("Day {}: {}", t + 1, cur.len());
    }
    return cur;
}

fn evolve(state: HashSet<(i64, i64, i64)>) -> HashSet<(i64, i64, i64)> {
    let mut next = HashSet::new();

    let mut neighboring_inactive = HashMap::new();
    for cur in &state {
        let mut active_cnt = 0;
        for n in neighbors(*cur) {
            if state.contains(&n) {
                active_cnt += 1;
            } else if let Some(cnt) = neighboring_inactive.get_mut(&n) {
                *cnt += 1;
            } else {
                neighboring_inactive.insert(n, 1);
            }
        }
        if active_cnt == 1 || active_cnt == 2 {
            next.insert(*cur);
        }
    }

    // Now consider the inactive neighbors of those:
    for cur in neighboring_inactive.into_iter().filter(|(_k, v)| *v == 2).map(|(k, _v)| k) {
        next.insert(cur);
    }

    return next;
}

fn neighbors(cur: (i64, i64, i64)) -> HashSet<(i64, i64, i64)> {
    return get_dirs().values().map(|d| (cur.0 + d.0, cur.1 + d.1, cur.2 + d.2)).collect();
}

fn main() {
    let args : Vec<String> = std::env::args().collect();
    if args[1] == "1" {
        println!("Doing part 1");
        let paths = read_input(&args[2]).unwrap();
        let flipped = walk_paths(&paths);
        println!("Flipped: {}", flipped.len());
    } else {
        println!("Doing part 2");
        let paths = read_input(&args[2]).unwrap();
        let flipped = walk_paths(&paths);
        let evolved = evolve_times(flipped, 100);
        println!("Flipped: {}", evolved.len());
    }
}
