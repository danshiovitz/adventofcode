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

fn do_transform(digits: &Vec<i32>, rounds: i32, chunksize: usize) -> Vec<i32> {
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
    return output.into_iter().take(chunksize).collect();
}

fn do_transform_fast(digits: &Vec<i32>, rounds: i32, chunksize: usize, use_offset: bool) -> Vec<i32> {
    let repeat = 10000;
    let total_size = digits.len() as i32 * repeat;

    let mut offset = 0;
    if use_offset {
        offset = digits.iter().take(7).format("").to_string().parse::<i32>().unwrap();
    }
    if offset <= total_size / 2 {
        panic!("This transform code is only smart enough to handle the second half of the digits ({}, min {})",
               offset, total_size / 2);
    }

    println!("offset: {}, total: {}, total - offset: {}", offset, total_size, total_size - offset);

    // in the second half of the input, the pattern for digits ABC..Z in the output is
    // A is based on the sum of A..Z
    // B is based on the sum of B..Z
    // ...
    // Z is based on the sum of Z
    // (Note that Z is the very last digit of the input - you can't stop early)
    //
    // Doing a bunch of examples, apparently this means that after r rounds,
    //   A_r = P(r, 0) * A_0 + P(r, 1) * B_0 + ... P(r, n) * Z_0
    //   B_r = P(r, 0) * B_0 + P(r, 1) * C_0 + ... P(r, n-1) * Z_0
    // Where P(r, n) is the r'th item from the (n+r)th row of pascal's triangle
    // Y_r is always ((Z_0 * r) + Y_0) % 10 and Z_r is always Z_0, but that will
    // hopefully fall out naturally
    fn nth_digit(n: i32, digits: &Vec<i32>) -> i32 {
        return n % (digits.len() as i32);
    }

    let mut output = vec![0; chunksize];
    if rounds == 0 {
        for d in 0..chunksize {
            output[d] = nth_digit(offset + (d as i32), &digits);
        }
    } else {
        let mut last_pascal : i32 = 1;
        for idx in 0..(total_size - offset) {
            let row : i32 = 5 + idx - 1;
            let column : i32 = 5 - 1;
            let ll = last_pascal % 10;
            assert!(row >= 0 && column >= 0);
            if row == column {
                last_pascal = 1;
            } else {
                last_pascal *= row;
                last_pascal /= (row - column);
            }

            // let num_digits = std::cmp::min(chunksize as i32, total_size - (offset + idx) + 1);
            // for d in 0..num_digits {
            //     let digit = nth_digit(offset + idx + (d as i32), &digits);
            //     output[d] = (output[d] + ((digit * last_pascal) % 10) as i32) % 10;
            // }

            println!("Pascal: {} {} {}{}{}", last_pascal, last_pascal % 10, ll, row % 10, column % 10);
        }
    }

    return output.into_iter().take(chunksize).collect();
}

fn do_transform_fast_dumber(digits: &Vec<i32>, rounds: i32, chunksize: usize, use_offset: bool) -> Vec<i32> {
    let repeat = 10000;
    let total_size = digits.len() as i32 * repeat;

    let mut offset = 0;
    if use_offset {
        offset = digits.iter().take(7).format("").to_string().parse::<i32>().unwrap();
    }
    if offset <= total_size / 2 {
        panic!("This transform code is only smart enough to handle the second half of the digits ({}, min {})",
               offset, total_size / 2);
    }

    // in the second half of the input, the pattern for digits ABC..Z in the output is
    // A is based on the sum of A..Z
    // B is based on the sum of B..Z
    // ...
    // Z is based on the sum of Z
    // (Note that Z is the very last digit of the input - you can't stop early)
    let mut working = vec![0; (total_size - offset) as usize];
    for idx in 0..working.len() {
        working[idx] = digits[(offset as usize + idx) % digits.len()];
    }

    for _ in 0..rounds {
        let mut tot = 0;
        for idx in (0..working.len()).rev() {
            tot += working[idx];
            working[idx] = (tot % 10);
        }
    }

    return working.into_iter().take(chunksize).collect();
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
        let transformed = do_transform(&digits, 4, 8);
        println!("Digits: {:?}", transformed.into_iter().format(""));
    } else {
        println!("Doing part 2");
        let digits = read_input(&args[2]).unwrap();
        let transformed = do_transform_fast_dumber(&digits, 100, 8, true);
        println!("Digits: {:?}", transformed.into_iter().format(""));
    }
}
