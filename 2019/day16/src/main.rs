#[macro_use] extern crate failure;
#[macro_use] extern crate lazy_static;

use failure::Error;
use itertools::Itertools;
use num_integer::lcm;
use regex::Regex;
use std::collections::HashMap;
use std::io::{BufReader,BufRead};
use std::fs::File;

fn parse_line(line: &str) -> Result<Vec<i32>, Error> {
    return Ok(line.chars().map(|c| c as i32 - '0' as i32).collect());
}

fn read_input(file: &str) -> Result<Vec<i32>, Error> {
    let f = File::open(file)?;
    let br = BufReader::new(f);
    let parsed : Result<Vec<Vec<i32>>, Error> = br.lines().map(|line| parse_line(&line?)).collect();
    return Ok(parsed?.into_iter().flatten().collect());
}

fn do_transform(digits: &Vec<i32>, rounds: i32) -> Vec<i32> {
    let mut output = digits.to_vec();
    let mut by_pos = Vec::new();
    for _ in 0..output.len() {
        by_pos.push(Vec::new());
    }
    for _round in 0..rounds {
        let mut tmp = output.to_vec();
        for od in 0..output.len() {
            let mut sum = 0;
            for id in 0..output.len() {
                sum += output[id] * calc_pattern(od, id);
            }
            tmp[od] = sum.abs() % 10;
        }
        output = tmp;
        // println!("Digits: {:?}", output.to_vec().into_iter().take(20).format(""));
        for p in 0..by_pos.len() {
            by_pos[p].push(output[p]);
        }
    }
    for p in 0..by_pos.len() {
        println!("Digits at {}: {:?}", p, by_pos[p].to_vec().into_iter().take(50).format(""));
    }
    println!("Pattern:");
    for od in 0..digits.len() {
        let mut line = String::new();
        line.reserve(digits.len());
        for id in 0..digits.len() {
            match calc_pattern(od, id) {
                -1 => { line.push('-'); },
                0 => { line.push('.'); },
                1 => { line.push('+'); },
                x => { panic!("Bad val: {}", x); }
            }
        }
        println!(" {}", line);
    }
    return output;
}

fn make_pattern(sz: usize) -> HashMap<(usize, usize), i32> {
    lazy_static! {
        static ref PHASES: Vec<i32> = vec![0, 1, 0, -1];
    }
    let mut ret = HashMap::new();
    for od in 0..sz {
        let width = od + 1;
        let mut phase = 0; // which phase
        let mut ctr = 1; // position within the phase
        for id in 0..sz {
            if ctr >= width {
                ctr = 0;
                phase += 1;
            }
            if phase >= PHASES.len() {
                phase = 0;
            }
            ret.insert((od, id), PHASES[phase]);
            ctr += 1;
        }
    }
    return ret;
}

fn calc_pattern(od: usize, id: usize) -> i32 {
    lazy_static! {
        static ref PHASES: Vec<i32> = vec![0, 1, 0, -1];
    }
    let phase_width = od as i32 + 1;
    let rep_width = phase_width * PHASES.len() as i32;
    let rep_start = (((id as i32 + 1) / rep_width) * rep_width) - 1;
    let idx = id as i32 - rep_start;
    return PHASES[(idx / phase_width) as usize];
}

fn main() {
    let args : Vec<String> = std::env::args().collect();
    if args[1] == "1" {
        println!("Doing part 1");
        let digits = read_input(&args[2]).unwrap();
        let transformed = do_transform(&digits, 4);
        println!("Digits: {:?}", transformed.into_iter().take(20).format(""));
    } else {
        println!("Doing part 2");
    }
}
