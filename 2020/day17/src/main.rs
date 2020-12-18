use failure::{Error, bail};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::io::{BufReader, BufRead};
use std::fs::File;

type Map = HashSet<Vec<i64>>;

struct Level {
    map: Map,
    dims: usize,
    min: Vec<i64>,
    max: Vec<i64>,
}

fn read_input(file: &str, dims: usize) -> Result<Level, Error> {
    if dims < 2 {
        bail!("Dimensions val is {}, must be at least 2", dims);
    }
    let f = File::open(file)?;
    let br = BufReader::new(f);
    let mut lvl = Level { map: HashSet::new(), dims: dims, min: vec![0 as i64; dims], max: vec![0 as i64; dims] };
    let mut max_x = 0;
    let mut max_y = 0;
    for (y, line) in br.lines().enumerate() {
        for (x, c) in line?.chars().enumerate() {
            if c == '#' {
                let mut cur = vec![0 as i64; dims];
                cur[0] = x as i64;
                cur[1] = y as i64;
                lvl.map.insert(cur);
            } else if c == '.' {
                continue;
            } else {
                bail!("Unknown map char: {}", c);
            }
            if x as i64 > max_x { max_x = x as i64; }
        }
        if y as i64 > max_y { max_y = y as i64; }
    }
    lvl.max[0] = max_x;
    lvl.max[1] = max_y;
    return Ok(lvl);
}

fn print_one(lvl: &Level, higher: &Vec<i64>) -> () {
    let names = vec!["z", "w", "v", "u"];
    if lvl.dims > names.len() {
        panic!("Not enough names!");
    }
    if higher.len() > 0 {
        println!("{}", higher.iter().enumerate().map(|(idx, val)| format!("{}={}", names[idx], val)).join(", "));
    }
    for y in lvl.min[1] .. lvl.max[1] + 1 {
        for x in lvl.min[0] .. lvl.max[0] + 1 {
            let mut cur = higher.clone();
            cur.insert(0, y);
            cur.insert(0, x);
            let ch = if lvl.map.contains(&cur) { '#' } else { '.' };
            print!("{}", ch);
        }
        println!();
    }
}

fn print_map(lvl: &Level) -> () {
    let mut highers : Vec<Vec<i64>> = (2..lvl.dims).map(|i| lvl.min[i]..(lvl.max[i] + 1)).multi_cartesian_product().collect();
    if highers.len() == 0 {
        highers.push(Vec::new());
    }
    for higher in highers {
        print_one(lvl, &higher);
    }
    println!();
}

fn neighbors(cur: &Vec<i64>) -> Vec<Vec<i64>> {
    return (0..cur.len()).map(|i| (cur[i] - 1)..(cur[i] + 2))
    .multi_cartesian_product().filter(|n| n != cur).collect();
}

fn cycle(lvl: &Level) -> Level {
    let mut nxt = HashSet::new();

    let mut neighboring_inactive = HashMap::new();
    for cur in &lvl.map {
        let mut active_cnt = 0;
        for n in neighbors(cur) {
            if lvl.map.contains(&n) {
                active_cnt += 1;
            } else if let Some(cnt) = neighboring_inactive.get_mut(&n) {
                *cnt += 1;
            } else {
                neighboring_inactive.insert(n, 1);
            }
        }
        if active_cnt == 2 || active_cnt == 3 {
            nxt.insert(cur.clone());
        }
    }

    // Now consider the inactive neighbors of those:
    for cur in neighboring_inactive.into_iter().filter(|(_k, v)| *v == 3).map(|(k, _v)| k) {
        nxt.insert(cur);
    }

    let min = (0..lvl.dims).map(|i| nxt.iter().map(|c| c[i]).min().unwrap()).collect();
    let max = (0..lvl.dims).map(|i| nxt.iter().map(|c| c[i]).max().unwrap()).collect();
    return Level { map: nxt, dims: lvl.dims, min: min, max: max };
}

fn cycle_times(lvl: &Level, times: i32, verbose: bool) -> i64 {
    let mut cur = Level { map: lvl.map.clone(), dims: lvl.dims, min: lvl.min.clone(), max: lvl.max.clone() };
    for _ in 0..times {
        if verbose {
            print_map(&cur);
            println!();
        }
        cur = cycle(&cur);
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
        let lvl = read_input(&args[2], 3).unwrap();
        let verbose = args.len() > 3 && args[3].chars().nth(0).unwrap() == 't';
        let cnt = cycle_times(&lvl, 6, verbose);
        println!("At final cycle, {}", cnt);
    } else {
        println!("Doing part 2");
        let lvl = read_input(&args[2], 4).unwrap();
        let verbose = args.len() > 3 && args[3].chars().nth(0).unwrap() == 't';
        let cnt = cycle_times(&lvl, 6, verbose);
        println!("At final cycle, {}", cnt);
    }
}
