use failure::{Error, bail};
use std::collections::{HashMap, HashSet};
use std::io::{BufReader, BufRead};
use std::fs::File;

type Map = HashSet<(i64, i64, i64, i64)>;

struct Level {
    map: Map,
    min: (i64, i64, i64, i64),
    max: (i64, i64, i64, i64),
}

fn read_input(file: &str) -> Result<Level, Error> {
    let f = File::open(file)?;
    let br = BufReader::new(f);
    let mut lvl = Level { map: HashSet::new(), min: (0, 0, 0, 0), max: (0, 0, 0, 0) };
    let mut max_x = 0;
    let mut max_y = 0;
    for (y, line) in br.lines().enumerate() {
        for (x, c) in line?.chars().enumerate() {
            if c == '#' {
                lvl.map.insert((x as i64, y as i64, 0 as i64, 0 as i64));
            } else if c == '.' {
                continue;
            } else {
                bail!("Unknown map char: {}", c);
            }
            if x as i64 > max_x { max_x = x as i64; }
        }
        if y as i64 > max_y { max_y = y as i64; }
    }
    lvl.max = (max_x, max_y, 0, 0);
    return Ok(lvl);
}

fn print_map(lvl: &Level) -> () {
    for w in lvl.min.3 .. lvl.max.3 + 1 {
        for z in lvl.min.2 .. lvl.max.2 + 1 {
            println!("z={}, w={}", z, w);
            for y in lvl.min.1 .. lvl.max.1 + 1 {
                for x in lvl.min.0 .. lvl.max.0 + 1 {
                    let ch = if lvl.map.contains(&(x, y, z, w)) { '#' } else { '.' };
                    print!("{}", ch);
                }
                println!();
            }
        }
    }
    println!();
}

fn neighbors(cur: (i64, i64, i64, i64), use_w: bool) -> Vec<(i64, i64, i64, i64)> {
    let w_min = if use_w { -1 } else { 0 };
    let w_max = if use_w { 2 } else { 1 };
    let mut ret = Vec::new();
    for x in -1..2 {
        for y in -1..2 {
            for z in -1..2 {
                for w in w_min..w_max {
                    if x == 0 && y == 0 && z == 0 && w == 0 {
                        continue;
                    }
                    ret.push((cur.0 + x, cur.1 + y, cur.2 + z, cur.3 + w));
                }
            }
        }
    }
    return ret;
}

fn cycle(lvl: &Level, use_w: bool) -> Level {
    let mut nxt = HashSet::new();

    let mut neighboring_inactive = HashMap::new();
    for cur in &lvl.map {
        let mut active_cnt = 0;
        for n in neighbors(*cur, use_w) {
            if lvl.map.contains(&n) {
                active_cnt += 1;
            } else if let Some(cnt) = neighboring_inactive.get_mut(&n) {
                *cnt += 1;
            } else {
                neighboring_inactive.insert(n, 1);
            }
        }
        if active_cnt == 2 || active_cnt == 3 {
            nxt.insert(*cur);
        }
    }

    // Now consider the inactive neighbors of those:
    for cur in neighboring_inactive.into_iter().filter(|(_k, v)| *v == 3).map(|(k, _v)| k) {
        nxt.insert(cur);
    }

    let min = (
        nxt.iter().map(|(x, _y, _z, _w)| *x).min().unwrap(),
        nxt.iter().map(|(_x, y, _z, _w)| *y).min().unwrap(),
        nxt.iter().map(|(_x, _y, z, _w)| *z).min().unwrap(),
        nxt.iter().map(|(_x, _y, _z, w)| *w).min().unwrap(),
    );
    let max = (
        nxt.iter().map(|(x, _y, _z, _w)| *x).max().unwrap(),
        nxt.iter().map(|(_x, y, _z, _w)| *y).max().unwrap(),
        nxt.iter().map(|(_x, _y, z, _w)| *z).max().unwrap(),
        nxt.iter().map(|(_x, _y, _z, w)| *w).max().unwrap(),
    );
    return Level { map: nxt, min: min, max: max };
}

fn cycle_times(lvl: &Level, times: i32, use_w: bool, verbose: bool) -> i64 {
    let mut cur = Level { map: lvl.map.clone(), min: lvl.min, max: lvl.max };
    for _ in 0..times {
        if verbose {
            print_map(&cur);
            println!();
        }
        cur = cycle(&cur, use_w);
    }
    if verbose {
        print_map(&cur);
    }
    return cur.map.len() as i64;
}

fn main() {
    let args : Vec<String> = std::env::args().collect();
    if args[1] == "1" {
        println!("Doing part 1");
        let lvl = read_input(&args[2]).unwrap();
        let verbose = args.len() > 3 && args[3].chars().nth(0).unwrap() == 't';
        let cnt = cycle_times(&lvl, 6, false, verbose);
        println!("At final cycle, {}", cnt);
    } else {
        println!("Doing part 2");
        let lvl = read_input(&args[2]).unwrap();
        let verbose = args.len() > 3 && args[3].chars().nth(0).unwrap() == 't';
        let cnt = cycle_times(&lvl, 6, true, verbose);
        println!("At final cycle, {}", cnt);
    }
}
