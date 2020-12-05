#[macro_use] extern crate failure;

use failure::{Error, bail};
use std::collections::HashSet;
use std::io::{BufReader, BufRead};
use std::fs::File;

struct Level {
    map: HashSet<(i64, i64)>,
    min: (i64, i64),
    max: (i64, i64),
}

fn read_input(file: &str) -> Result<Level, Error> {
    let f = File::open(file)?;
    let br = BufReader::new(f);
    let mut lvl = Level { map: HashSet::new(), min: (0, 0), max: (0, 0) };
    let mut max_x = 0;
    let mut max_y = 0;
    for (y, line) in br.lines().enumerate() {
        for (x, c) in line?.chars().enumerate() {
            if c == '#' {
                lvl.map.insert((x as i64, y as i64));
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

fn is_tree(cur: (i64, i64), lvl: &Level) -> bool {
    return lvl.map.contains(&(cur.0 % (lvl.max.0 + 1), cur.1))
}

fn count_trees(start: (i64, i64), slope: (i64, i64), lvl: &Level) -> i64 {
    let mut cnt = 0;
    let mut cur = start;
    while cur.1 < lvl.max.1 {
        cur = (cur.0 + slope.0, cur.1 + slope.1);
        if is_tree(cur, lvl) {
            cnt += 1;
        }
    }
    return cnt;
}

fn main() {
    let args : Vec<String> = std::env::args().collect();
    if args[1] == "1" {
        println!("Doing part 1");
        let lvl = read_input(&args[2]).unwrap();
        let trees = count_trees((0, 0), (3, 1), &lvl);
        println!("Hit trees: {}", trees);
    } else {
        println!("Doing part 2");
        let lvl = read_input(&args[2]).unwrap();
        let mut tot = 1;
        for slope in vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)] {
            let trees = count_trees((0, 0), slope, &lvl);
            println!("Hit trees at slope {:?}: {}", slope, trees);
            tot *= trees;
        }
        println!("Total: {}", tot);
    }
}
