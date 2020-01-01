#[macro_use] extern crate lazy_static;

use failure::Error;
use itertools::Itertools;
use regex::Regex;
use std::cell::RefCell;
use std::collections::{BinaryHeap,HashMap,HashSet};
use std::cmp::Ordering;
use std::io::{BufReader,BufRead};
use std::fs::File;

fn parse_int(val: &str) -> Result<i64, Error> {
    return Ok(val.parse::<i64>()?);
}

fn parse_line(line: &str) -> Result<Vec<i64>, Error> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r",\s*").unwrap();
    }
    return RE.split(&line).map(|val| parse_int(&val)).collect();
}

fn read_input(file: &str) -> Result<Vec<i64>, Error> {
    let f = File::open(file)?;
    let br = BufReader::new(f);
    let parsed : Result<Vec<Vec<i64>>, Error> = br.lines().map(|line| parse_line(&line?)).collect();
    return Ok(parsed?.into_iter().flatten().collect());
}

enum ExitType {
    Finish99,
    InputAbort,
}

fn run_program(program_init: &Vec<i64>, mut input: impl FnMut() -> Option<i64>, mut output: impl FnMut(i64)) -> ExitType {
    let relative_base = -99;

    let read_idx = |i: i64, program: &HashMap<i64, i64>| -> i64 {
        if i < 0 {
            panic!("Index {} outside of program bound ({})", i, program.len());
        }
        match program.get(&i) {
            Some(&v) => { return v; },
            None => { return 0; },
        };
    };

    let write_idx = |i: i64, value: i64, program: &mut HashMap<i64, i64>| {
        if i < 0 {
            panic!("Index {} outside of program bound ({})", i, program.len());
        }
        program.insert(i, value);
    };

    let resolve_idx = |pc: i64, num: i64, modes: i64, program: &HashMap<i64, i64>| -> i64 {
        let i = pc + num;
        let mut flag;
        if num > 0 {
            flag = modes;
            for _ in 0..num-1 {
                flag /= 10;
            }
            flag %= 10;
        } else {
            flag = 1;
        }
        if flag == 0 {
            return read_idx(i, program);
        } else if flag == 1 {
            return i;
        } else if flag == 2 {
            return read_idx(i, program) + program[&relative_base];
        } else {
            panic!("Bad flag at {}: {}", i, flag);
        }
    };

    let get = |pc: i64, num: i64, modes: i64, program: &HashMap<i64, i64>| -> i64 {
        return read_idx(resolve_idx(pc, num, modes, program), program);
    };

    let set = |pc: i64, num: i64, modes: i64, value: i64, program: &mut HashMap<i64, i64>| {
        return write_idx(resolve_idx(pc, num, modes, program), value, program);
    };

    let mut program : HashMap<i64, i64> = HashMap::new();
    for i in 0..program_init.len() {
        program.insert(i as i64, program_init[i]);
    }
    program.insert(relative_base, 0);

    let mut pc : i64 = 0;
    loop {
        if pc >= program.len() as i64 || pc < 0 {
            panic!("PC {} outside of program bound ({})", pc, program.len());
        }
        let opcode = get(pc, 0, 0, &program) % 100;
        let modes = get(pc, 0, 0, &program) / 100;
        // println!("Executing at pc={} opcode={}, modes={}", pc, opcode, modes);
        if opcode == 1 {
            let value = get(pc, 1, modes, &program) + get(pc, 2, modes, &program);
            set(pc, 3, modes, value, &mut program);
            pc += 4;
        } else if opcode == 2 {
            let value = get(pc, 1, modes, &program) * get(pc, 2, modes, &program);
            set(pc, 3, modes, value, &mut program);
            pc += 4;
        } else if opcode == 3 {
            let value = input();
            match value {
                Some(v) => {
                    set(pc, 1, modes, v, &mut program);
                    pc += 2;
                },
                None => {
                    return ExitType::InputAbort;
                }
            }
        } else if opcode == 4 {
            output(get(pc, 1, modes, &program));
            pc += 2;
        } else if opcode == 5 || opcode == 6 {
            if (get(pc, 1, modes, &program) == 0) == (opcode == 6) {
                pc = get(pc, 2, modes, &program);
            } else {
                pc += 3;
            }
        } else if opcode == 7 {
            let value = if get(pc, 1, modes, &program) < get(pc, 2, modes, &program) { 1 } else { 0 };
            set(pc, 3, modes, value, &mut program);
            pc += 4;
        } else if opcode == 8 {
            let value = if get(pc, 1, modes, &program) == get(pc, 2, modes, &program) { 1 } else { 0 };
            set(pc, 3, modes, value, &mut program);
            pc += 4;
        } else if opcode == 9 {
            let value = get(pc, 1, modes, &program);
            *program.get_mut(&relative_base).unwrap() += value;
            pc += 2;
        } else if opcode == 99 {
            // println!("Saw 99, exiting");
            return ExitType::Finish99;
        } else {
            panic!("Unknown opcode at {}: {}", pc, get(pc, 0, 0, &program));
        }
    }
}

fn walk_springbot(program: &Vec<i64>) -> i64 {
    let mut line = String::new();
    // I dunno if this is only specific to my input or if this is intended to
    // work in general, but oh well
    let moves = vec![
        "NOT D T",
        "OR C T",
        "NOT T J",  // jump (early) if empty 3 away and not empty 4 away
        "NOT A T",
        "OR T J",   // also jump if it's empty immediately in front
    ];
    for mv in moves {
        line.push_str(mv);
        line.push('\n');
    }
    line.push_str("WALK\n");

    let mut inch : Vec<i64> = line.chars().map(|c| c as u32 as i64).collect();
    let input_fn = || -> Option<i64> {
        if inch.len() > 0 {
            return Some(inch.remove(0));
        } else {
            return None;
        }
    };

    let mut return_code = 0;
    let output_fn = |i: i64| {
        if i > 127 || return_code != 0 {
            println!("Received return code: {}", i);
            return_code = i;
        } else {
            let c = std::char::from_u32(i as u32).unwrap();
            print!("{}", c);
        }
    };

    run_program(&program, input_fn, output_fn);
    return return_code;
}

fn run_springbot(program: &Vec<i64>) -> i64 {
    let mut line = String::new();
    let moves = vec![
        "OR D J",  // jump if safe landing spot
        "OR E T",
        "OR H T",
        "AND T J", // and we can either jump or move from there
        "NOT A T",
        "NOT T T",
        "AND B T",
        "AND C T",
        "NOT T T",
        "AND T J", // and at least one of A-C is false
    ];
    for mv in moves {
        line.push_str(mv);
        line.push('\n');
    }
    line.push_str("RUN\n");

    let mut inch : Vec<i64> = line.chars().map(|c| c as u32 as i64).collect();
    let input_fn = || -> Option<i64> {
        if inch.len() > 0 {
            return Some(inch.remove(0));
        } else {
            return None;
        }
    };

    let mut return_code = 0;
    let output_fn = |i: i64| {
        if i > 127 || return_code != 0 {
            println!("Received return code: {}", i);
            return_code = i;
        } else {
            let c = std::char::from_u32(i as u32).unwrap();
            print!("{}", c);
        }
    };

    run_program(&program, input_fn, output_fn);
    return return_code;
}

fn main() {
    let args : Vec<String> = std::env::args().collect();
    if args[1] == "1" {
        println!("Doing part 1");
        let program = read_input(&args[2]).unwrap();
        let rc = walk_springbot(&program);
        println!("Return code: {}", rc);
    } else {
        println!("Doing part 2");
        let program = read_input(&args[2]).unwrap();
        let rc = run_springbot(&program);
        println!("Return code: {}", rc);
    }
}
