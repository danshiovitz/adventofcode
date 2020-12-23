#[macro_use] extern crate failure;
#[macro_use] extern crate lazy_static;

use failure::{Error, bail};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::io::{BufReader, BufRead};
use std::fs::File;

#[derive(Debug, Clone)]
struct Cups {
    cur: usize,
    values: Vec<u32>,
}

fn parse_cups(line: &str) -> Result<Cups, Error> {
    let digs : Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap() as u32).collect();
    return Ok(Cups { cur: 0, values: digs });
}

fn read_input(file: &str) -> Result<Cups, Error> {
    let f = File::open(file)?;
    let br = BufReader::new(f);

    let vals : Vec<String> = br.lines().map(|line| line.unwrap()).collect();
    if vals.len() != 1 {
        bail!("Expected 1 line in input, found {}", vals.len());
    }
    return parse_cups(vals[0].as_str());
}

fn move_cups(init_cups: &Cups, num_moves: i32, verbose: bool) -> Vec<u32> {
    let mut cur_idx : usize = init_cups.cur;
    let mut values : Vec<u32> = init_cups.values.clone();
    if num_moves > 1000 {
        for cup in values.len() + 1 .. 1000001 {
            values.push(cup as u32);
        }
    }
    for m in 0..num_moves {
        if verbose {
            println!("Move {}", m);
            println!("Cups {:?}, cur_idx={}, cur_val={}", values, cur_idx, values[cur_idx]);
        } else if m % 100000 == 0 {
            println!("Move {}", m);
        }
        let mut moving = Vec::new();
        for _ in 0..3 {
            let move_idx = (cur_idx + 1) % values.len();
            moving.push(values.remove(move_idx));
            if move_idx < cur_idx {
                cur_idx -= 1;
            }
        }
        if verbose {
            println!("Will move {:?}, cur_idx={}, cur_val={}", moving, cur_idx, values[cur_idx]);
        }
        let dest_val = values.iter().sorted().rev().filter(|v| **v < values[cur_idx]).nth(0)
            .or_else(|| values.iter().sorted().rev().filter(|v| **v > values[cur_idx]).nth(0))
            .unwrap();
        for dest_idx in 0..values.len() {
            if values[dest_idx] == *dest_val {
                if verbose {
                    println!("Destination idx={} val={}", dest_idx, dest_val);
                }
                while moving.len() > 0 {
                    values.insert(dest_idx + 1, moving.pop().unwrap());
                    if dest_idx < cur_idx {
                        cur_idx = (cur_idx + 1) % values.len();
                    }
                }
                break;
            }
        }
        cur_idx = (cur_idx + 1) % values.len();
        if verbose {
            println!("");
        }
    }

    // normalize order:
    while values[0] != 1 {
        let fst = values.remove(0);
        values.push(fst);
    }
    return values;
}

fn move_cups_map(init_cups: &Cups, num_moves: i32, verbose: bool) -> Vec<u32> {
    let mut cur_val = init_cups.values[init_cups.cur];
    let mut values : HashMap<u32, u32> = HashMap::new();
    for idx in 0..init_cups.values.len() - 1 {
        values.insert(init_cups.values[idx], init_cups.values[idx+1]);
    }
    if num_moves < 1000 {
        values.insert(init_cups.values[init_cups.values.len() - 1], init_cups.values[0]);
    } else {
        values.insert(init_cups.values[init_cups.values.len() - 1], init_cups.values.len() as u32 + 1);
        for val in (init_cups.values.len() as u32 + 1) .. 1000000 {
            values.insert(val, val + 1);
        }
        values.insert(1000000, init_cups.values[0]);
    }

    let max_val = values.len() as u32;

    for m in 0..num_moves {
        if verbose {
            println!("Move {}", m);
            let mut vv = Vec::new();
            let mut cursor = *values.get(&1).unwrap();
            for _ in 0..9 {
                vv.push(cursor);
                cursor = *values.get(&cursor).unwrap();
            }
            println!("Cups {:?}, cur_val={}", vv, cur_val);
            println!("Raw {:?}", values);
        } else if m % 1000000 == 0 {
            println!("Move {}", m);
        }
        let mut moving = Vec::new();
        let mut cursor = *values.get(&cur_val).unwrap();
        for _ in 0..3 {
            moving.push(cursor);
            cursor = *values.get(&cursor).unwrap();
        }
        // we leave the move vals linked in the map, but
        // detach from cur val:
        values.insert(cur_val, cursor);

        if verbose {
            println!("Will move {:?}", moving);
        }
        let mut dest_val = cur_val;
        loop {
            dest_val = (((dest_val as i64 - 2 + max_val as i64) % max_val as i64) + 1) as u32;
            if !moving.iter().any(|v| *v == dest_val) {
                if verbose {
                    println!("Dest val: {}", dest_val);
                }
                let next_val = *values.get(&dest_val).unwrap();
                if verbose {
                    println!("Dest val: {}  next: {}", dest_val, next_val);
                }
                // splice moving back in
                values.insert(dest_val, moving[0]);
                values.insert(moving[moving.len() - 1], next_val);
                break;
            }
            if verbose {
                println!("Rejected dest_val: {}", dest_val);
            }
        }
        cur_val = *values.get(&cur_val).unwrap();
        if verbose {
            println!("");
        }
    }

    let ret_count = if values.len() > 100 { 3 } else { values.len() - 1 };
    let mut to_return = Vec::new();
    let mut ret_cursor : u32 = 1;
    for _ in 0..ret_count {
        to_return.push(ret_cursor);
        ret_cursor = *values.get(&ret_cursor).unwrap();
    }
    return to_return;
}

fn main() {
    let args : Vec<String> = std::env::args().collect();
    if args[1] == "1" {
        println!("Doing part 1");
        let cups = read_input(&args[2]).unwrap();
        let moves = args[3].parse::<i32>().unwrap();
        let verbose = args.len() > 4 && args[4].chars().nth(0).unwrap() == 't';
        let values = move_cups_map(&cups, moves, verbose);
        println!("Final form: {:?}", values);
    } else {
        println!("Doing part 2");
        let cups = read_input(&args[2]).unwrap();
        let moves = 10000000;
        let values = move_cups_map(&cups, moves, false);
        println!("Final form: {} {} {} = {}", values[0], values[1], values[2], values[1] as i64 * values[2] as i64);
    }
}
