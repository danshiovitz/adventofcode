#[macro_use] extern crate lazy_static;

use failure::Error;
use itertools::Itertools;
use regex::Regex;
use std::io::{BufReader,BufRead};
use std::fs::File;

fn parse_int(val: &str) -> Result<i32, Error> {
    return Ok(val.parse::<i32>()?);
}

fn parse_line(line: &str) -> Result<Vec<i32>, Error> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r",\s*").unwrap();
    }
    return RE.split(&line).map(|val| parse_int(&val)).collect();
}

fn read_input(file: &str) -> Result<Vec<i32>, Error> {
    let f = File::open(file)?;
    let br = BufReader::new(f);
    let parsed : Result<Vec<Vec<i32>>, Error> = br.lines().map(|line| parse_line(&line?)).collect();
    return Ok(parsed?.into_iter().flatten().collect());
}

fn run_program(mut program: Vec<i32>, input: impl Fn() -> i32, output: impl Fn(i32)) -> Vec<i32> {
    let idx = |i: i32, program: &Vec<i32>| -> i32 {
        if i >= program.len() as i32 || i < 0 {
            panic!("Index {} outside of program bound ({})", i, program.len());
        }
        return program[i as usize];
    };
    let get = |pc: i32, num: i32, modes: i32, program: &Vec<i32>| -> i32 {
        let i = pc + num;
        let mut flag = modes;
        for _ in 0..num-1 {
            flag /= 10;
        }
        flag %= 10;
        if flag == 0 {
            return idx(idx(i, program), program);
        } else {
            return idx(i, program);
        }
    };

    let mut pc : i32 = 0;
    loop {
        if pc >= program.len() as i32 || pc < 0 {
            panic!("PC {} outside of program bound ({})", pc, program.len());
        }
        let opcode = program[pc as usize] % 100;
        let modes = program[pc as usize] / 100;
        if opcode == 1 {
            let ix = idx(pc+3, &program);
            program[ix as usize] = get(pc, 1, modes, &program) + get(pc, 2, modes, &program);
            pc += 4;
        } else if opcode == 2 {
            let ix = idx(pc+3, &program);
            program[ix as usize] = get(pc, 1, modes, &program) * get(pc, 2, modes, &program);
            pc += 4;
        } else if opcode == 3 {
            let ix = idx(pc+1, &program);
            program[ix as usize] = input();
            pc += 2;
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
            let ix = idx(pc+3, &program);
            if get(pc, 1, modes, &program) < get(pc, 2, modes, &program) {
                program[ix as usize] = 1;
            } else {
                program[ix as usize] = 0;
            }
            pc += 4;
        } else if opcode == 8 {
            let ix = idx(pc+3, &program);
            if get(pc, 1, modes, &program) == get(pc, 2, modes, &program) {
                program[ix as usize] = 1;
            } else {
                program[ix as usize] = 0;
            }
            pc += 4;
        } else if opcode == 99 {
            println!("Saw 99, exiting");
            return program;
        } else {
            panic!("Unknown opcode at {}: {}", pc, program[pc as usize]);
        }
    }
}

fn print_program(program: &Vec<i32>) {
    println!("Program: {}", program.into_iter().map(|v| v.to_string()).format(","));
}

fn main() {
    let args : Vec<String> = std::env::args().collect();
    if args[1] == "free" {
        println!("Doing free play");
        let mut vals = parse_line(&args[2]).unwrap();
        let mut input = parse_int(&args[3]).unwrap();
        vals = run_program(vals, || { return input; }, |x| { println!("Output: {}", x) });
    } else if args[1] == "1" {
        println!("Doing part 1");
        let mut vals = read_input(&args[2]).unwrap();
        vals = run_program(vals, || { return 1; }, |x| { println!("Output: {}", x) });
    } else {
        println!("Doing part 2");
        let mut vals = read_input(&args[2]).unwrap();
        vals = run_program(vals, || { return 5; }, |x| { println!("Output: {}", x) });
    }
}
