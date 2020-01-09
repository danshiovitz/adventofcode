#[macro_use] extern crate lazy_static;

use failure::Error;
use itertools::Itertools;
use regex::Regex;
use std::cell::RefCell;
use std::collections::{BinaryHeap,HashMap,HashSet};
use std::cmp::Ordering;
use std::io::{BufReader,BufRead};
use std::fs::File;

const WIDTH : i32 = 5;
const HEIGHT : i32 = 5;

type Bug = (i32, i32, i32);

fn read_input(file: &str) -> Result<HashSet<Bug>, Error> {
    let f = File::open(file)?;
    let br = BufReader::new(f);
    let mut bugs = HashSet::new();
    let mut y = 0;
    for line in br.lines() {
        let mut x = 0;
        for ch in line?.chars() {
            match ch {
                '#' => {
                    bugs.insert((0, x, y));
                },
                '.' => {},
                _ => panic!("Bad char: {}", ch),
            };
            x += 1;
            assert!(x <= WIDTH);
        }
        y += 1;
        assert!(y <= HEIGHT);
    }
    return Ok(bugs);
}

fn print_level(bugs: &HashSet<Bug>) {
    let lvls : Vec<i32> = bugs.iter().map(|(lvl, _, _)| *lvl).unique().sorted().collect();
    for lvl in lvls {
        println!("Level {}:", lvl);
        for y in 0..HEIGHT {
            let mut line = String::new();
            line.reserve(WIDTH as usize);
            for x in 0..WIDTH {
                if bugs.contains(&(lvl, x, y)) {
                    line.push('#');
                } else {
                    line.push('.');
                }
            }
            println!("{}", line);
        }
        println!();
    }
    println!("------------------");
}

fn evolve_until_dup(bugs: &HashSet<Bug>) -> HashSet<Bug> {
    let neighbor_count = |lvl: i32, x: i32, y: i32, b: &HashSet<Bug>| -> i32 {
        return DIRECTIONS.iter().filter_map(|d| b.get(&(lvl, x+d.x, y+d.y))).count() as i32;
    };

    let mut cur = bugs.clone();
    let mut seen = HashSet::new();
    loop {
        let key : Vec<Bug> = cur.clone().into_iter().sorted().collect();
        if seen.insert(key) == false {
            return cur;
        }
        // print_level(&cur);
        // println!();
        cur = evolve_once(&cur, neighbor_count);
    }
}

fn evolve_times(bugs: &HashSet<Bug>, n: i32) -> HashSet<Bug> {
    let mut neighbor_map : HashMap<(i32, i32), Vec<(i32, i32, i32)>> = HashMap::new();
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let mut ns = Vec::new();
            if y == 0 {
                ns.push((-1, 2, 1));
            } else if x == 2 && y == 3 {
                for c in 0..WIDTH {
                    ns.push((1, c, HEIGHT-1));
                }
            } else {
                ns.push((0, x, y-1));
            }
            if x == 0 {
                ns.push((-1, 1, 2));
            } else if x == 3 && y == 2 {
                for c in 0..HEIGHT {
                    ns.push((1, WIDTH-1, c));
                }
            } else {
                ns.push((0, x-1, y));
            }
            if y == HEIGHT-1 {
                ns.push((-1, 2, 3));
            } else if x == 2 && y == 1 {
                for c in 0..WIDTH {
                    ns.push((1, c, 0));
                }
            } else {
                ns.push((0, x, y+1));
            }
            if x == WIDTH-1 {
                ns.push((-1, 3, 2));
            } else if x == 1 && y == 2 {
                for c in 0..HEIGHT {
                    ns.push((1, 0, c));
                }
            } else {
                ns.push((0, x+1, y));
            }
            neighbor_map.insert((x, y), ns);
        }
    }
    let neighbor_count = |lvl: i32, x: i32, y: i32, b: &HashSet<Bug>| -> i32 {
        if x == WIDTH/2 && y == HEIGHT/2 {
            return 0;
        }
        return neighbor_map.get(&(x, y)).unwrap().iter().
            filter_map(|(nlvl, nx, ny)| b.get(&(lvl+*nlvl, *nx, *ny))).count() as i32;
    };

    let mut cur = bugs.clone();
    for _ in 0..n {
        cur = evolve_once(&cur, neighbor_count);
    }
    return cur;
}

fn evolve_once(bugs: &HashSet<Bug>, neighbor_count: impl Fn(i32, i32, i32, &HashSet<Bug>) -> i32) -> HashSet<Bug> {
    let mut lvls : Vec<i32> = bugs.iter().map(|(lvl, _, _)| *lvl).unique().sorted().collect();
    let min_lvl = *lvls.iter().min().unwrap() - 1;
    let max_lvl = *lvls.iter().max().unwrap() + 1;
    let mut cur = HashSet::new();
    for lvl in min_lvl..=max_lvl {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let bug_count = neighbor_count(lvl, x, y, &bugs);
                if bugs.contains(&(lvl, x, y)) {
                    if bug_count == 1 {
                        cur.insert((lvl, x, y));
                    }
                } else {
                    if bug_count == 1 || bug_count == 2 {
                        cur.insert((lvl, x, y));
                    }
                }
            }
        }
    }
    return cur;
}

fn calc_score(bugs: &HashSet<Bug>) -> i32 {
    let mut total = 0;
    let mut cur = 1;
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if bugs.contains(&(0, x, y)) {
                total += cur;
            }
            cur *= 2;
        }
    }
    return total;
}

#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
struct Direction {
    value: i32,
    x: i32,
    y: i32,
    c: char,
}

static NORTH : Direction = Direction { value: 1, x: 0, y: -1, c: '^' };
static SOUTH : Direction = Direction { value: 2, x: 0, y: 1, c: 'v' };
static WEST : Direction = Direction { value: 3, x: -1, y: 0, c: '<' };
static EAST : Direction = Direction { value: 4, x: 1, y: 0, c: '>' };

static DIRECTIONS : [Direction; 4] = [
    NORTH, SOUTH, WEST, EAST
];

fn main() {
    let args : Vec<String> = std::env::args().collect();
    if args[1] == "test" {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let mut lvl = HashSet::new();
                lvl.insert((0, x, y));
                print_level(&lvl);
                let evolved = evolve_times(&lvl, 1);
                print_level(&evolved);
                println!("========");
            }
        }
    } else if args[1] == "1" {
        println!("Doing part 1");
        let lvl = read_input(&args[2]).unwrap();
        print_level(&lvl);
        let evolved = evolve_until_dup(&lvl);
        print_level(&evolved);
        println!("Biodiversity score: {}", calc_score(&evolved));
    } else {
        println!("Doing part 2");
        let lvl = read_input(&args[2]).unwrap();
        print_level(&lvl);
        let evolved = evolve_times(&lvl, 200);
        print_level(&evolved);
        println!("Final count: {}", evolved.len());
    }
}
