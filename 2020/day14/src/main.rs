#[macro_use] extern crate lazy_static;
#[macro_use] extern crate failure;

use failure::{Error, bail};
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::io::{BufReader,BufRead};
use std::fs::File;

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
enum Opcode {
    MASK,
    SET,
}

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
struct Instruction {
    opcode: Opcode,
    val1: i64,
    val2: i64,
}

#[derive(PartialEq, Eq, Debug)]
struct State {
    memory: HashMap<i64, i64>,
}

fn parse_int(val: &str) -> Result<i64, Error> {
    return Ok(val.parse::<i64>()?);
}

// Given a mask string like 0X10X1, turn it into two values,
// one zero-preserving (011011) and one one-preserving (001001)
fn parse_mask(mask: &str) -> (i64, i64) {
    let zero_mask = mask.replace("X", "1");
    let one_mask = mask.replace("X", "0");
    return (i64::from_str_radix(&zero_mask, 2).unwrap(), i64::from_str_radix(&one_mask, 2).unwrap());
}

fn parse_line(line: &str) -> Result<Instruction, Error> {
    lazy_static! {
        static ref MASK_RE: Regex = Regex::new(r"^mask\s*=\s*([X01]+)$").unwrap();
        static ref SET_RE: Regex = Regex::new(r"^mem\[([0-9]+)\]\s*=\s*([0-9]+)$").unwrap();
    }
    if line.starts_with("mask") {
        match MASK_RE.captures(line) {
            Some(caps) => {
                let (zero_mask, one_mask) = parse_mask(caps.get(1).unwrap().as_str());
                return Ok(Instruction {
                    opcode: Opcode::MASK,
                    val1: zero_mask,
                    val2: one_mask,
                });
            }
            None => {
                bail!("Bad mask line: {}", line);
            }
        }
    } else if line.starts_with("mem") {
        match SET_RE.captures(line) {
            Some(caps) => {
                return Ok(Instruction {
                    opcode: Opcode::SET,
                    val1: parse_int(caps.get(1).unwrap().as_str())?,
                    val2: parse_int(caps.get(2).unwrap().as_str())?,
                });
            }
            None => {
                bail!("Bad set line: {}", line);
            }
        }
    } else {
        bail!("Bad overall line: {}", line);
    }
}

fn read_input(file: &str) -> Result<Vec<Instruction>, Error> {
    let f = File::open(file)?;
    let br = BufReader::new(f);
    let parsed : Result<Vec<Instruction>, Error> = br.lines().map(|line| parse_line(&line?)).collect();
    return parsed;
}

fn apply_mask(mask: (i64, i64), val: i64) -> i64 {
    let val = val & mask.0;
    let val = val | mask.1;
    return val;
}

fn run_program1(instructions: &Vec<Instruction>, init_state: State) -> State {
    let mut state = init_state;
    let mut cur_mask = (0, -1);
    for inst in instructions {
        if inst.opcode == Opcode::MASK {
            cur_mask = (inst.val1, inst.val2);
        } else if inst.opcode == Opcode::SET {
            state.memory.insert(inst.val1, apply_mask(cur_mask, inst.val2));
        }
    }
    return state;
}

fn expand_address_mask(mask: (i64, i64)) -> Vec<(i64, i64)> {
    // ok, in the input mask, mask.0 has a 0 for all the actual 0s, and a
    // 1 for all the Xs and 1s; mask.1 has a 1 for all the actual 1s and a
    // zero for all the Xs and 0s. what we need to produce here is a base
    // mask.0 that has all 1s (and then we'll explicitly set to zeros for some Xs)
    // and a mask.1 that's the same as the input mask (and then we'll explicitly
    // set some to ones for some Xs)
    let mask_bits : i64 = 36;

    let base_mask = (2_i64.pow(mask_bits as u32 + 1) - 1, mask.1);
    let mut ret = Vec::new();
    let x_mask = mask.0 ^ mask.1;
    let mask_ones = one_positions(x_mask, mask_bits);
    for val in 0..2_i64.pow(mask_ones.len() as u32) {
        ret.push(set_ones_in_mask(base_mask, &mask_ones, val));
    }
    return ret;
}

fn one_positions(val: i64, mask_bits: i64) -> Vec<i64> {
    let mut ret = Vec::new();
    let mut flag = 1;
    for idx in 0..mask_bits+1 {
        if (val & flag) != 0 {
            ret.push(idx);
        }
        flag <<= 1;
    }
    return ret;
}

fn set_ones_in_mask(base_mask: (i64, i64), mask_ones: &Vec<i64>, val: i64) -> (i64, i64) {
    let mut mask = base_mask;
    for idx in 0..mask_ones.len() {
        let val_flag = 1 << idx;
        let mask_flag = 1 << mask_ones[idx];
        if (val & val_flag) != 0 {
            mask.1 |= mask_flag;
        } else {
            mask.0 &= !mask_flag;
        }
    }
    return mask;
}

fn run_program2(instructions: &Vec<Instruction>, init_state: State) -> State {
    let mut state = init_state;
    let mut cur_masks = Vec::new();
    for inst in instructions {
        if inst.opcode == Opcode::MASK {
            cur_masks = expand_address_mask((inst.val1, inst.val2));
        } else if inst.opcode == Opcode::SET {
            for cur_mask in &cur_masks {
                state.memory.insert(apply_mask(*cur_mask, inst.val1), inst.val2);
            }
        }
    }
    return state;
}

fn main() {
    let args : Vec<String> = std::env::args().collect();
    if args[1] == "1" {
        println!("Doing part 1");
        let instructions = read_input(&args[2]).unwrap();
        let state = run_program1(&instructions, State { memory: HashMap::new() });
        println!("Final state-sum is {}", state.memory.values().sum::<i64>());
    } else {
        println!("Doing part 2");
        let instructions = read_input(&args[2]).unwrap();
        let state = run_program2(&instructions, State { memory: HashMap::new() });
        println!("Final state-sum is {}", state.memory.values().sum::<i64>());
    }
}
